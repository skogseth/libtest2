use libtest_lexarg::OutputFormat;

use crate::{cli, notify, Case, RunError, RunMode, State};

pub struct Harness {
    raw: std::io::Result<Vec<std::ffi::OsString>>,
    cases: Vec<Box<dyn Case>>,
}

impl Harness {
    pub fn with_args(args: impl IntoIterator<Item = impl Into<std::ffi::OsString>>) -> Self {
        let raw = expand_args(args);
        Self { raw, cases: vec![] }
    }

    pub fn with_env() -> Self {
        let raw = std::env::args_os();
        let raw = expand_args(raw);
        Self { raw, cases: vec![] }
    }

    pub fn case(mut self, case: impl Case + 'static) -> Self {
        self.cases.push(Box::new(case));
        self
    }

    pub fn cases(mut self, cases: impl IntoIterator<Item = impl Case + 'static>) -> Self {
        for case in cases {
            self.cases.push(Box::new(case));
        }
        self
    }

    pub fn main(mut self) -> ! {
        let start = std::time::Instant::now();

        let raw = match self.raw {
            Ok(raw) => raw,
            Err(err) => {
                eprintln!("{err}");
                std::process::exit(1)
            }
        };
        let mut parser = cli::Parser::new(&raw);
        let opts = parse(&mut parser).unwrap_or_else(|err| {
            eprintln!("{err}");
            std::process::exit(1)
        });

        #[cfg(feature = "color")]
        match opts.color {
            libtest_lexarg::ColorConfig::AutoColor => anstream::ColorChoice::Auto,
            libtest_lexarg::ColorConfig::AlwaysColor => anstream::ColorChoice::Always,
            libtest_lexarg::ColorConfig::NeverColor => anstream::ColorChoice::Never,
        }
        .write_global();

        let mut notifier = notifier(&opts).unwrap_or_else(|err| {
            eprintln!("{err}");
            std::process::exit(1)
        });
        discover(&start, &opts, &mut self.cases, notifier.as_mut()).unwrap_or_else(|err| {
            eprintln!("{err}");
            std::process::exit(1)
        });

        if !opts.list {
            match run(&start, &opts, self.cases, notifier.as_mut()) {
                Ok(true) => {}
                Ok(false) => std::process::exit(ERROR_EXIT_CODE),
                Err(e) => {
                    eprintln!("error: io error when listing tests: {e:?}");
                    std::process::exit(ERROR_EXIT_CODE)
                }
            }
        }

        std::process::exit(0)
    }
}

const ERROR_EXIT_CODE: i32 = 101;

fn parse<'p>(
    parser: &mut cli::Parser<'p>,
) -> Result<libtest_lexarg::TestOpts, cli::ErrorContext<'p>> {
    let mut test_opts = libtest_lexarg::TestOptsBuilder::new();

    let bin = parser
        .next_raw()
        .expect("first arg, no pending values")
        .unwrap_or(std::ffi::OsStr::new("test"));
    let mut prev_arg = cli::Arg::Value(bin);
    while let Some(arg) = parser.next_arg() {
        match arg {
            cli::Arg::Short("h") | cli::Arg::Long("help") => {
                let bin = bin.to_string_lossy();
                let options_help = libtest_lexarg::OPTIONS_HELP.trim();
                let after_help = libtest_lexarg::AFTER_HELP.trim();
                println!(
                    "Usage: {bin} [OPTIONS] [FILTER]...

{options_help}

{after_help}"
                );
                std::process::exit(0);
            }
            // All values are the same, whether escaped or not, so its a no-op
            cli::Arg::Escape(_) => {
                prev_arg = arg;
                continue;
            }
            cli::Arg::Unexpected(_) => {
                return Err(cli::ErrorContext::msg("unexpected value")
                    .unexpected(arg)
                    .within(prev_arg));
            }
            _ => {}
        }
        prev_arg = arg;

        let arg = test_opts.parse_next(parser, arg)?;

        if let Some(arg) = arg {
            return Err(cli::ErrorContext::msg("unexpected argument").unexpected(arg));
        }
    }

    let mut opts = test_opts.finish()?;
    // If the platform is single-threaded we're just going to run
    // the test synchronously, regardless of the concurrency
    // level.
    let supports_threads = !cfg!(target_os = "emscripten") && !cfg!(target_family = "wasm");
    opts.test_threads = if cfg!(feature = "threads") && supports_threads {
        opts.test_threads
            .or_else(|| std::thread::available_parallelism().ok())
    } else {
        None
    };
    Ok(opts)
}

fn expand_args(
    args: impl IntoIterator<Item = impl Into<std::ffi::OsString>>,
) -> std::io::Result<Vec<std::ffi::OsString>> {
    let mut expanded = Vec::new();
    for arg in args {
        let arg = arg.into();
        if let Some(argfile) = arg.to_str().and_then(|s| s.strip_prefix("@")) {
            expanded.extend(parse_argfile(std::path::Path::new(argfile))?);
        } else {
            expanded.push(arg);
        }
    }
    Ok(expanded)
}

fn parse_argfile(path: &std::path::Path) -> std::io::Result<Vec<std::ffi::OsString>> {
    // Logic taken from rust-lang/rust's `compiler/rustc_driver_impl/src/args.rs`
    let content = std::fs::read_to_string(path)?;
    Ok(content.lines().map(|s| s.into()).collect())
}

fn notifier(opts: &libtest_lexarg::TestOpts) -> std::io::Result<Box<dyn notify::Notifier>> {
    #[cfg(feature = "color")]
    let stdout = anstream::stdout();
    #[cfg(not(feature = "color"))]
    let stdout = std::io::stdout();
    let notifier: Box<dyn notify::Notifier> = match opts.format {
        #[cfg(feature = "json")]
        OutputFormat::Json => Box::new(notify::JsonNotifier::new(stdout)),
        #[cfg(not(feature = "json"))]
        OutputFormat::Json => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "`--format=json` is not supported",
            ));
        }
        _ if opts.list => Box::new(notify::TerseListNotifier::new(stdout)),
        OutputFormat::Pretty => Box::new(notify::PrettyRunNotifier::new(stdout)),
        OutputFormat::Terse => Box::new(notify::TerseRunNotifier::new(stdout)),
    };
    Ok(notifier)
}

fn discover(
    start: &std::time::Instant,
    opts: &libtest_lexarg::TestOpts,
    cases: &mut Vec<Box<dyn Case>>,
    notifier: &mut dyn notify::Notifier,
) -> std::io::Result<()> {
    notifier.notify(notify::Event::DiscoverStart {
        elapsed_s: Some(notify::Elapsed(start.elapsed())),
    })?;

    let matches_filter = |case: &dyn Case, filter: &str| {
        let test_name = case.name();

        match opts.filter_exact {
            true => test_name == filter,
            false => test_name.contains(filter),
        }
    };

    // Do this first so it applies to both discover and running
    cases.sort_unstable_by_key(|case| {
        let priority = if opts.filters.is_empty() {
            Some(0)
        } else {
            opts.filters
                .iter()
                .position(|filter| matches_filter(case.as_ref(), filter))
        };
        let name = case.name().to_owned();
        (priority, name)
    });

    let mut retain_cases = Vec::with_capacity(cases.len());
    for case in cases.iter() {
        let filtered_in = opts.filters.is_empty()
            || opts
                .filters
                .iter()
                .any(|filter| matches_filter(case.as_ref(), filter));
        let filtered_out =
            !opts.skip.is_empty() && opts.skip.iter().any(|sf| matches_filter(case.as_ref(), sf));
        let retain_case = filtered_in && !filtered_out;
        retain_cases.push(retain_case);
        notifier.notify(notify::Event::DiscoverCase {
            name: case.name().to_owned(),
            mode: RunMode::Test,
            run: retain_case,
            elapsed_s: Some(notify::Elapsed(start.elapsed())),
        })?;
    }
    let mut retain_cases = retain_cases.into_iter();
    cases.retain(|_| retain_cases.next().unwrap());

    notifier.notify(notify::Event::DiscoverComplete {
        elapsed_s: Some(notify::Elapsed(start.elapsed())),
    })?;

    Ok(())
}

fn run(
    start: &std::time::Instant,
    opts: &libtest_lexarg::TestOpts,
    cases: Vec<Box<dyn Case>>,
    notifier: &mut dyn notify::Notifier,
) -> std::io::Result<bool> {
    notifier.notify(notify::Event::RunStart {
        elapsed_s: Some(notify::Elapsed(start.elapsed())),
    })?;

    if opts.no_capture {
        todo!("`--no-capture` is not yet supported");
    }
    if opts.show_output {
        todo!("`--show-output` is not yet supported");
    }

    let threads = opts.test_threads.map(|t| t.get()).unwrap_or(1);

    let mut state = State::new();
    let run_ignored = match opts.run_ignored {
        libtest_lexarg::RunIgnored::Yes | libtest_lexarg::RunIgnored::Only => true,
        libtest_lexarg::RunIgnored::No => false,
    };
    let mode = match (opts.run_tests, opts.bench_benchmarks) {
        (true, true) => {
            return Err(std::io::Error::other(
                "`--test` and `-bench` are mutually exclusive",
            ));
        }
        (true, false) => RunMode::Test,
        (false, true) => RunMode::Bench,
        (false, false) => unreachable!("libtest-lexarg` should always ensure at least one is set"),
    };
    state.set_mode(mode);
    state.set_run_ignored(run_ignored);
    let state = std::sync::Arc::new(state);

    let mut success = true;

    let (exclusive_cases, concurrent_cases) = if threads == 1 || cases.len() == 1 {
        (cases, vec![])
    } else {
        cases
            .into_iter()
            .partition::<Vec<_>, _>(|c| c.exclusive(&state))
    };
    if !concurrent_cases.is_empty() {
        notifier.threaded(true);
        struct RunningTest {
            join_handle: std::thread::JoinHandle<()>,
        }

        impl RunningTest {
            fn join(self, event: &mut notify::Event) {
                if self.join_handle.join().is_err() {
                    if let notify::Event::CaseComplete {
                        status, message, ..
                    } = event
                    {
                        if status.is_none() {
                            *status = Some(notify::RunStatus::Failed);
                            *message = Some("panicked after reporting success".to_owned());
                        }
                    }
                }
            }
        }

        // Use a deterministic hasher
        type TestMap = std::collections::HashMap<
            String,
            RunningTest,
            std::hash::BuildHasherDefault<std::collections::hash_map::DefaultHasher>,
        >;

        let sync_success = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(success));
        let mut running_tests: TestMap = Default::default();
        let mut pending = 0;
        let (tx, rx) = std::sync::mpsc::channel::<notify::Event>();
        let mut remaining = std::collections::VecDeque::from(concurrent_cases);
        while pending > 0 || !remaining.is_empty() {
            while pending < threads && !remaining.is_empty() {
                let case = remaining.pop_front().unwrap();
                let name = case.name().to_owned();

                let cfg = std::thread::Builder::new().name(name.clone());
                let start = *start;
                let tx = tx.clone();
                let case = std::sync::Arc::new(case);
                let case_fallback = case.clone();
                let state = state.clone();
                let state_fallback = state.clone();
                let sync_success = sync_success.clone();
                let sync_success_fallback = sync_success.clone();
                let join_handle = cfg.spawn(move || {
                    let mut notifier = SenderNotifier { tx: tx.clone() };
                    let case_success =
                        run_case(&start, case.as_ref().as_ref(), &state, &mut notifier)
                            .expect("`SenderNotifier` is infallible");
                    if !case_success {
                        sync_success.store(case_success, std::sync::atomic::Ordering::Relaxed);
                    }
                });
                match join_handle {
                    Ok(join_handle) => {
                        running_tests.insert(name.clone(), RunningTest { join_handle });
                        pending += 1;
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // `ErrorKind::WouldBlock` means hitting the thread limit on some
                        // platforms, so run the test synchronously here instead.
                        let case_success = run_case(
                            &start,
                            case_fallback.as_ref().as_ref(),
                            &state_fallback,
                            notifier,
                        )
                        .expect("`SenderNotifier` is infallible");
                        if !case_success {
                            sync_success_fallback
                                .store(case_success, std::sync::atomic::Ordering::Relaxed);
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            let mut event = rx.recv().unwrap();
            if let notify::Event::CaseComplete { name, .. } = &event {
                let running_test = running_tests.remove(name).unwrap();
                running_test.join(&mut event);
                pending -= 1;
            }
            notifier.notify(event)?;
            success &= sync_success.load(std::sync::atomic::Ordering::SeqCst);
            if !success && opts.fail_fast {
                break;
            }
        }
    }

    if !exclusive_cases.is_empty() {
        notifier.threaded(false);
        for case in exclusive_cases {
            success &= run_case(start, case.as_ref(), &state, notifier)?;
            if !success && opts.fail_fast {
                break;
            }
        }
    }

    notifier.notify(notify::Event::RunComplete {
        elapsed_s: Some(notify::Elapsed(start.elapsed())),
    })?;

    Ok(success)
}

fn run_case(
    start: &std::time::Instant,
    case: &dyn Case,
    state: &State,
    notifier: &mut dyn notify::Notifier,
) -> std::io::Result<bool> {
    notifier.notify(notify::Event::CaseStart {
        name: case.name().to_owned(),
        elapsed_s: Some(notify::Elapsed(start.elapsed())),
    })?;

    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        __rust_begin_short_backtrace(|| case.run(state))
    }))
    .unwrap_or_else(|e| {
        // The `panic` information is just an `Any` object representing the
        // value the panic was invoked with. For most panics (which use
        // `panic!` like `println!`), this is either `&str` or `String`.
        let payload = e
            .downcast_ref::<String>()
            .map(|s| s.as_str())
            .or_else(|| e.downcast_ref::<&str>().copied());

        let msg = match payload {
            Some(payload) => format!("test panicked: {payload}"),
            None => "test panicked".to_owned(),
        };
        Err(RunError::fail(msg))
    });

    let err = outcome.as_ref().err();
    let status = err.map(|e| e.status());
    let message = err.and_then(|e| e.cause().map(|c| c.to_string()));
    notifier.notify(notify::Event::CaseComplete {
        name: case.name().to_owned(),
        mode: RunMode::Test,
        status,
        message,
        elapsed_s: Some(notify::Elapsed(start.elapsed())),
    })?;

    Ok(status != Some(notify::RunStatus::Failed))
}

/// Fixed frame used to clean the backtrace with `RUST_BACKTRACE=1`.
#[inline(never)]
fn __rust_begin_short_backtrace<T, F: FnOnce() -> T>(f: F) -> T {
    let result = f();

    // prevent this frame from being tail-call optimised away
    std::hint::black_box(result)
}

#[derive(Clone, Debug)]
struct SenderNotifier {
    tx: std::sync::mpsc::Sender<notify::Event>,
}

impl notify::Notifier for SenderNotifier {
    fn notify(&mut self, event: notify::Event) -> std::io::Result<()> {
        // If the sender doesn't care, neither do we
        let _ = self.tx.send(event);
        Ok(())
    }
}
