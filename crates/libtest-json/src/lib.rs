//! Definition of the json output for libtest

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]
#![allow(clippy::todo)]

mod event;

pub use event::Elapsed;
pub use event::Event;
pub use event::RunMode;
pub use event::RunStatus;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
