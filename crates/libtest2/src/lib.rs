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
//! And in `tests/mytest.rs` you would call [`main!`], passing it each of your tests:
//!
//! ```no_run
//! # use libtest2::RunError;
//! # use libtest2::RunResult;
//! # use libtest2::TestContext;
//! fn check_toph(_context: &TestContext) -> RunResult {
//!     Ok(())
//! }
//!
//! libtest2::main!(check_toph);
//! ```
//!

#![cfg_attr(docsrs, feature(doc_cfg))]
//#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

mod macros;
mod trial;

#[doc(hidden)]
pub mod _private {
    pub use crate::_main as main;
}

pub use _private::main;
pub use libtest2_harness::RunError;
pub use libtest2_harness::RunResult;
pub use libtest2_harness::TestContext;
pub use trial::main;
pub use trial::Trial;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
