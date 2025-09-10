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

pub use libtest_json::RunMode;

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

    pub fn case(mut self, case: Trial) -> Self {
        self.harness.case(TrialCase { inner: case });
        self
    }

    pub fn cases(mut self, cases: impl IntoIterator<Item = Trial>) -> Self {
        self.harness
            .cases(cases.into_iter().map(|c| TrialCase { inner: c }));
        self
    }

    pub fn main(self) -> ! {
        self.harness.main()
    }
}

pub struct Trial {
    name: String,
    #[allow(clippy::type_complexity)]
    runner: Box<dyn Fn(TestContext<'_>) -> Result<(), RunError> + Send + Sync>,
}

impl Trial {
    pub fn test(
        name: impl Into<String>,
        runner: impl Fn(TestContext<'_>) -> Result<(), RunError> + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            runner: Box::new(runner),
        }
    }
}

struct TrialCase {
    inner: Trial,
}

impl libtest2_harness::Case for TrialCase {
    fn name(&self) -> &str {
        &self.inner.name
    }
    fn kind(&self) -> libtest2_harness::TestKind {
        Default::default()
    }
    fn source(&self) -> Option<&libtest2_harness::Source> {
        None
    }
    fn exclusive(&self, _: &libtest2_harness::TestContext) -> bool {
        false
    }

    fn run(
        &self,
        context: &libtest2_harness::TestContext,
    ) -> Result<(), libtest2_harness::RunError> {
        (self.inner.runner)(TestContext { inner: context }).map_err(|e| e.inner)
    }
}

pub type RunResult = Result<(), RunError>;

#[derive(Debug)]
pub struct RunError {
    inner: libtest2_harness::RunError,
}

impl RunError {
    pub fn with_cause(cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            inner: libtest2_harness::RunError::with_cause(cause),
        }
    }

    pub fn fail(cause: impl std::fmt::Display) -> Self {
        Self {
            inner: libtest2_harness::RunError::fail(cause),
        }
    }
}

#[derive(Debug)]
pub struct TestContext<'t> {
    inner: &'t libtest2_harness::TestContext,
}

impl<'t> TestContext<'t> {
    pub fn ignore(&self) -> Result<(), RunError> {
        self.inner.ignore().map_err(|e| RunError { inner: e })
    }

    pub fn ignore_for(&self, reason: impl std::fmt::Display) -> Result<(), RunError> {
        self.inner
            .ignore_for(reason)
            .map_err(|e| RunError { inner: e })
    }

    pub fn current_mode(&self) -> RunMode {
        self.inner.current_mode()
    }
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
