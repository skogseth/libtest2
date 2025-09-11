//! An experimental replacement for libtest
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
//! And in `tests/mytest.rs` you would call [`libtest2_main`], passing it each of your tests:
//!
//! ```no_run
//! # use libtest2::RunError;
//! # use libtest2::RunResult;
//! # use libtest2::TestContext;
//! # use libtest2::libtest2_main;
//! fn check_toph(_context: &TestContext) -> RunResult {
//!     Ok(())
//! }
//!
//! libtest2_main!(check_toph);
//! ```
//!

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

pub use libtest2_harness::Harness;
pub use libtest2_harness::RunError;
pub use libtest2_harness::RunResult;
pub use libtest2_harness::TestContext;
pub use libtest2_harness::TestKind;
pub use libtest2_harness::ERROR_EXIT_CODE;

use libtest2_harness::Case;
use libtest2_harness::Source;

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

/// Expands to the test harness
#[macro_export]
macro_rules! libtest2_main {
    ( $( $test:path ),* $(,)*) => {
        fn main() {
            let harness = $crate::Harness::new();
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
            let harness = match harness.discover([
                $($crate::Trial::test(::std::stringify!($test), $test)),*
            ]) {
                Ok(harness) => harness,
                Err(err) => {
                    eprintln!("{err}");
                    ::std::process::exit($crate::ERROR_EXIT_CODE)
                }
            };
            match harness.run() {
                Ok(true) => ::std::process::exit(0),
                Ok(false) => ::std::process::exit($crate::ERROR_EXIT_CODE),
                Err(err) => {
                    eprintln!("{err}");
                    ::std::process::exit($crate::ERROR_EXIT_CODE)
                }
            }
        }
    }
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
