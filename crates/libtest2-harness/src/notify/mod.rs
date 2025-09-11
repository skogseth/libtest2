mod json;
#[cfg(not(feature = "color"))]
mod no_style;
mod pretty;
#[cfg(feature = "color")]
mod style;
mod summary;
mod terse;

pub(crate) use json::*;
#[cfg(not(feature = "color"))]
pub(crate) use no_style::*;
pub(crate) use pretty::*;
#[cfg(feature = "color")]
pub(crate) use style::*;
pub(crate) use summary::*;
pub(crate) use terse::*;

pub(crate) trait Notifier {
    fn threaded(&mut self, _yes: bool) {}

    fn notify(&mut self, event: Event) -> std::io::Result<()>;
}

pub(crate) use libtest_json::*;

pub use libtest_json::RunMode;
