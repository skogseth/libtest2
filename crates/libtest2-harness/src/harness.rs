use libtest_lexarg::OutputFormat;

use crate::{cli, notify, Case, RunError, RunMode, TestContext};

pub trait HarnessState: sealed::_HarnessState_is_Sealed {}

pub struct Harness<State: HarnessState> {
    state: State,
}

pub struct StateInitial {
    start: std::time::Instant,
}
impl HarnessState for StateInitial {}
impl sealed::_HarnessState_is_Sealed for StateInitial {}

impl Harness<StateInitial> {
    pub fn new() -> Self {
        Self {
            state: StateInitial {
                start: std::time::Instant::now(),
            },
        }
    }

    pub fn with_env(self) -> std::io::Result<Harness<StateArgs>> {
        let raw = std::env::args_os();
        self.with_args(raw)
    }

    pub fn with_args(
        self,
        args: impl IntoIterator<Item = impl Into<std::ffi::OsString>>,
    ) -> std::io::Result<Harness<StateArgs>> {
        let raw = expand_args(args)?;
        Ok(Harness {
            state: StateArgs {
                start: self.state.start,
                raw,
            },
        })
    }
}

impl Default for Harness<StateInitial> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StateArgs {
    start: std::time::Instant,
    raw: Vec<std::ffi::OsString>,
}
impl HarnessState for StateArgs {}
impl sealed::_HarnessState_is_Sealed for StateArgs {}

impl Harness<StateArgs> {
    pub fn parse(&self) -> Result<Harness<StateParsed>, cli::LexError<'_>> {
        let mut parser = cli::Parser::new(&self.state.raw);
        let opts = parse(&mut parser)?;

        #[cfg(feature = "color")]
        match opts.color {
            libtest_lexarg::ColorConfig::AutoColor => anstream::ColorChoice::Auto,
            libtest_lexarg::ColorConfig::AlwaysColor => anstream::ColorChoice::Always,
            libtest_lexarg::ColorConfig::NeverColor => anstream::ColorChoice::Never,
        }
        .write_global();

        let notifier = notifier(&opts);

        Ok(Harness {
            state: StateParsed {
                start: self.state.start,
                opts,
                notifier,
            },
        })
    }
}

pub struct StateParsed {
    start: std::time::Instant,
    opts: libtest_lexarg::TestOpts,
    notifier: Box<dyn notify::Notifier>,
}
impl HarnessState for StateParsed {}
impl sealed::_HarnessState_is_Sealed for StateParsed {}

impl Harness<StateParsed> {
    pub fn discover(
        mut self,
        cases: impl IntoIterator<Item = impl Case + 'static>,
    ) -> std::io::Result<Harness<StateDiscovered>> {
        self.state.notifier.notify(
            notify::event::DiscoverStart {
                elapsed_s: Some(notify::Elapsed(self.state.start.elapsed())),
            }
            .into(),
        )?;

        let mut cases = cases
            .into_iter()
            .map(|c| Box::new(c) as Box<dyn Case>)
            .collect();
        discover(
            &self.state.start,
            &self.state.opts,
            &mut cases,
            self.state.notifier.as_mut(),
        )?;

        self.state.notifier.notify(
            notify::event::DiscoverComplete {
                elapsed_s: Some(notify::Elapsed(self.state.start.elapsed())),
            }
            .into(),
        )?;

        Ok(Harness {
            state: StateDiscovered {
                start: self.state.start,
                opts: self.state.opts,
                notifier: self.state.notifier,
                cases,
            },
        })
    }
}

pub struct StateDiscovered {
    start: std::time::Instant,
    opts: libtest_lexarg::TestOpts,
    notifier: Box<dyn notify::Notifier>,
    cases: Vec<Box<dyn Case>>,
}
impl HarnessState for StateDiscovered {}
impl sealed::_HarnessState_is_Sealed for StateDiscovered {}

impl Harness<StateDiscovered> {
    pub fn run(mut self) -> std::io::Result<bool> {
        if self.state.opts.list {
            Ok(true)
        } else {
            run(
                &self.state.start,
                &self.state.opts,
                self.state.cases,
                self.state.notifier.as_mut(),
            )
        }
    }
}

mod sealed {
    #[allow(unnameable_types)]
    #[allow(non_camel_case_types)]
    pub trait _HarnessState_is_Sealed {}
}

pub const ERROR_EXIT_CODE: i32 = 101;

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

fn parse<'p>(parser: &mut cli::Parser<'p>) -> Result<libtest_lexarg::TestOpts, cli::LexError<'p>> {
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
                return Err(cli::LexError::msg("unexpected value")
                    .unexpected(arg)
                    .within(prev_arg));
            }
            _ => {}
        }
        prev_arg = arg;

        let arg = test_opts.parse_next(parser, arg)?;

        if let Some(arg) = arg {
            return Err(cli::LexError::msg("unexpected argument").unexpected(arg));
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

fn notifier(opts: &libtest_lexarg::TestOpts) -> Box<dyn notify::Notifier> {
    #[cfg(feature = "color")]
    let stdout = anstream::stdout();
    #[cfg(not(feature = "color"))]
    let stdout = std::io::stdout();
    match opts.format {
        OutputFormat::Json => Box::new(notify::JsonNotifier::new(stdout)),
        _ if opts.list => Box::new(notify::TerseListNotifier::new(stdout)),
        OutputFormat::Pretty => Box::new(notify::PrettyRunNotifier::new(stdout)),
        OutputFormat::Terse => Box::new(notify::TerseRunNotifier::new(stdout)),
    }
}

fn discover(
    start: &std::time::Instant,
    opts: &libtest_lexarg::TestOpts,
    cases: &mut Vec<Box<dyn Case>>,
    notifier: &mut dyn notify::Notifier,
) -> std::io::Result<()> {
    let matches_filter = |case: &dyn Case, filter: &str| {
        let test_name = case.name();

        match opts.filter_exact {
            true => test_name == filter,
            false => test_name.contains(filter),
        }
    };

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
        notifier.notify(
            notify::event::DiscoverCase {
                name: case.name().to_owned(),
                mode: RunMode::Test,
                selected: retain_case,
                elapsed_s: Some(notify::Elapsed(start.elapsed())),
            }
            .into(),
        )?;
    }
    let mut retain_cases = retain_cases.into_iter();
    cases.retain(|_| retain_cases.next().unwrap());

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

    Ok(())
}

fn run(
    start: &std::time::Instant,
    opts: &libtest_lexarg::TestOpts,
    cases: Vec<Box<dyn Case>>,
    notifier: &mut dyn notify::Notifier,
) -> std::io::Result<bool> {
    notifier.notify(
        notify::event::RunStart {
            elapsed_s: Some(notify::Elapsed(start.elapsed())),
        }
        .into(),
    )?;

    if opts.no_capture {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "`--no-capture` is not supported at this time",
        ));
    }
    if opts.show_output {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "`--show-output` is not supported at this time",
        ));
    }

    let threads = opts.test_threads.map(|t| t.get()).unwrap_or(1);

    let mut context = TestContext::new();
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
    context.set_mode(mode);
    context.set_run_ignored(run_ignored);
    let context = std::sync::Arc::new(context);

    let mut success = true;

    let (exclusive_cases, concurrent_cases) = if threads == 1 || cases.len() == 1 {
        (cases, vec![])
    } else {
        cases
            .into_iter()
            .partition::<Vec<_>, _>(|c| c.exclusive(&context))
    };
    if !concurrent_cases.is_empty() {
        notifier.threaded(true);
        struct RunningTest {
            join_handle: std::thread::JoinHandle<()>,
        }

        impl RunningTest {
            fn join(
                self,
                start: &std::time::Instant,
                event: &notify::event::CaseComplete,
                notifier: &mut dyn notify::Notifier,
            ) -> std::io::Result<()> {
                if self.join_handle.join().is_err() {
                    let kind = notify::MessageKind::Error;
                    let message = Some("panicked after reporting success".to_owned());
                    notifier.notify(
                        notify::event::CaseMessage {
                            name: event.name.clone(),
                            kind,
                            message,
                            elapsed_s: Some(notify::Elapsed(start.elapsed())),
                        }
                        .into(),
                    )?;
                }
                Ok(())
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
                let context = context.clone();
                let context_fallback = context.clone();
                let sync_success = sync_success.clone();
                let sync_success_fallback = sync_success.clone();
                let join_handle = cfg.spawn(move || {
                    let mut notifier = SenderNotifier { tx: tx.clone() };
                    let case_success =
                        run_case(&start, case.as_ref().as_ref(), &context, &mut notifier)
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
                            &context_fallback,
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

            let event = rx.recv().unwrap();
            if let notify::Event::CaseComplete(event) = &event {
                let running_test = running_tests.remove(&event.name).unwrap();
                running_test.join(start, event, notifier)?;
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
            success &= run_case(start, case.as_ref(), &context, notifier)?;
            if !success && opts.fail_fast {
                break;
            }
        }
    }

    notifier.notify(
        notify::event::RunComplete {
            elapsed_s: Some(notify::Elapsed(start.elapsed())),
        }
        .into(),
    )?;

    Ok(success)
}

fn run_case(
    start: &std::time::Instant,
    case: &dyn Case,
    context: &TestContext,
    notifier: &mut dyn notify::Notifier,
) -> std::io::Result<bool> {
    notifier.notify(
        notify::event::CaseStart {
            name: case.name().to_owned(),
            elapsed_s: Some(notify::Elapsed(start.elapsed())),
        }
        .into(),
    )?;

    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        __rust_begin_short_backtrace(|| case.run(context))
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

    let mut case_status = None;
    if let Some(err) = outcome.as_ref().err() {
        let kind = err.status();
        case_status = Some(kind);
        let message = err.cause().map(|c| c.to_string());
        notifier.notify(
            notify::event::CaseMessage {
                name: case.name().to_owned(),
                kind,
                message,
                elapsed_s: Some(notify::Elapsed(start.elapsed())),
            }
            .into(),
        )?;
    }

    notifier.notify(
        notify::event::CaseComplete {
            name: case.name().to_owned(),
            elapsed_s: Some(notify::Elapsed(start.elapsed())),
        }
        .into(),
    )?;

    Ok(case_status != Some(notify::MessageKind::Error))
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
