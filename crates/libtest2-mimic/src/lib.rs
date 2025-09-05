//! An experimental replacement for libtest-mimic
//!
//! # Usage
//!
//! To use this, you most likely want to add a manual `[[test]]` section to
//! `Cargo.toml` and set `harness = false`. For example:
//!
//! ```toml
//! [[test]]
//! name = "mytest"
//! path = "tests/mytest.rs"
//! harness = false
//! ```
//!
//! And in `tests/mytest.rs` you would call [`Harness::main`] in the `main` function:
//!
//! ```no_run
//! libtest2_mimic::Harness::with_env()
//!     .main();
//! ```
//!

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

pub use libtest2_harness::RunError;
pub use libtest2_harness::RunResult;
pub use libtest2_harness::TestContext;
pub use libtest2_harness::TestKind;

use libtest2_harness::Case;
use libtest2_harness::Source;

pub struct Harness {
    harness: libtest2_harness::Harness,
}

impl Harness {
    pub fn with_args(args: impl IntoIterator<Item = impl Into<std::ffi::OsString>>) -> Self {
        Self {
            harness: libtest2_harness::Harness::with_args(args),
        }
    }

    pub fn with_env() -> Self {
        Self {
            harness: libtest2_harness::Harness::with_env(),
        }
    }

    pub fn case(&mut self, case: Trial) {
        self.harness.case(case)
    }

    pub fn cases(&mut self, cases: impl IntoIterator<Item = Trial>) {
        self.harness.cases(cases)
    }

    pub fn main(self) -> ! {
        self.harness.main()
    }
}

pub struct Trial {
    name: String,
    #[allow(clippy::type_complexity)]
    runner: Box<dyn Fn(&TestContext) -> Result<(), RunError> + Send + Sync>,
}

impl Trial {
    pub fn test(
        name: impl Into<String>,
        runner: impl Fn(&TestContext) -> Result<(), RunError> + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            runner: Box::new(runner),
        }
    }
}

impl Case for Trial {
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

    fn run(&self, context: &TestContext) -> Result<(), RunError> {
        (self.runner)(context)
    }
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
