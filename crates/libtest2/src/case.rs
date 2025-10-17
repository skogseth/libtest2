use libtest2_harness::Case;
use libtest2_harness::Source;
use libtest2_harness::TestKind;

use crate::RunResult;
use crate::TestContext;

#[derive(Copy, Clone)]
pub struct DynCase<T: 'static>(pub &'static dyn Case<Input = T>);

impl<T> Case for DynCase<T> {
    type Input = T;

    fn name(&self) -> &str {
        self.0.name()
    }
    fn kind(&self) -> TestKind {
        self.0.kind()
    }
    fn source(&self) -> Option<&Source> {
        self.0.source()
    }
    fn exclusive(&self, context: &TestContext<T>) -> bool {
        self.0.exclusive(context)
    }

    fn run(&self, context: &TestContext<T>) -> RunResult {
        self.0.run(context)
    }
}

pub struct FnCase<T> {
    name: String,
    #[allow(clippy::type_complexity)]
    runner: Box<dyn Fn(&TestContext<T>) -> RunResult + Send + Sync>,
}

impl<T> FnCase<T> {
    pub fn test(
        name: impl Into<String>,
        runner: impl Fn(&TestContext<T>) -> RunResult + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            runner: Box::new(runner),
        }
    }
}

impl<T: 'static> Case for FnCase<T> {
    type Input = T;

    fn name(&self) -> &str {
        &self.name
    }
    fn kind(&self) -> TestKind {
        Default::default()
    }
    fn source(&self) -> Option<&Source> {
        None
    }
    fn exclusive(&self, _: &TestContext<T>) -> bool {
        false
    }

    fn run(&self, context: &TestContext<T>) -> RunResult {
        (self.runner)(context)
    }
}

pub fn main<T: Send + Clone + 'static>(
    cases: impl IntoIterator<Item = impl Case<Input = T> + 'static>,
    value: T,
) {
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
    match harness.run_with_value(value.clone()) {
        Ok(true) => ::std::process::exit(0),
        Ok(false) => ::std::process::exit(libtest2_harness::ERROR_EXIT_CODE),
        Err(err) => {
            eprintln!("{err}");
            ::std::process::exit(libtest2_harness::ERROR_EXIT_CODE)
        }
    }
}
