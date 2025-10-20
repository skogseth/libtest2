use libtest2_harness::Case;
use libtest2_harness::Source;
use libtest2_harness::TestKind;

use crate::RunResult;
use crate::TestContext;

#[derive(Copy, Clone)]
pub struct TestDef<F = fn(&TestContext) -> RunResult> {
    pub name: &'static str,
    pub kind: TestKind,
    pub exclusive: bool,
    pub function: F,
}

impl<F> Case for TestDef<F>
where
    F: Fn(&TestContext) -> RunResult,
    F: Send + Sync + 'static,
{
    fn name(&self) -> &str {
        self.name
    }
    fn kind(&self) -> TestKind {
        self.kind
    }
    fn source(&self) -> Option<&Source> {
        None
    }
    fn exclusive(&self, _context: &TestContext) -> bool {
        self.exclusive
    }
    fn run(&self, context: &TestContext) -> RunResult {
        (self.function)(context)
    }
}

pub struct FnCase {
    name: String,
    #[allow(clippy::type_complexity)]
    runner: Box<dyn Fn(&TestContext) -> RunResult + Send + Sync>,
}

impl FnCase {
    pub fn test(
        name: impl Into<String>,
        runner: impl Fn(&TestContext) -> RunResult + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            runner: Box::new(runner),
        }
    }
}

impl Case for FnCase {
    fn name(&self) -> &str {
        &self.name
    }
    fn kind(&self) -> TestKind {
        Default::default()
    }
    fn source(&self) -> Option<&Source> {
        None
    }
    fn exclusive(&self, _: &TestContext) -> bool {
        false
    }

    fn run(&self, context: &TestContext) -> RunResult {
        (self.runner)(context)
    }
}

pub fn main(cases: impl IntoIterator<Item = impl Case + 'static>) {
    let harness = libtest2_harness::Harness::new();
    let harness = match harness.with_env() {
        Ok(harness) => harness,
        Err(err) => {
            eprintln!("{err}");
            ::std::process::exit(1);
        }
    };
    let harness = match harness.parse() {
        Ok(harness) => harness,
        Err(err) => {
            eprintln!("{err}");
            ::std::process::exit(1);
        }
    };
    let mut cases = cases.into_iter().collect::<Vec<_>>();
    cases.sort_by_key(|c| c.name().to_owned());
    let harness = match harness.discover(cases) {
        Ok(harness) => harness,
        Err(err) => {
            eprintln!("{err}");
            ::std::process::exit(libtest2_harness::ERROR_EXIT_CODE)
        }
    };
    match harness.run() {
        Ok(true) => ::std::process::exit(0),
        Ok(false) => ::std::process::exit(libtest2_harness::ERROR_EXIT_CODE),
        Err(err) => {
            eprintln!("{err}");
            ::std::process::exit(libtest2_harness::ERROR_EXIT_CODE)
        }
    }
}
