//! This module contains functionality related to handling panics

use std::borrow::Cow;

const DID_NOT_PANIC: &str = "test did not panic as expected";

/// Error returned by [`assert_panic`] and [`assert_panic_contains`]
#[derive(Debug)]
pub struct AssertPanicError(Cow<'static, str>);

impl std::fmt::Display for AssertPanicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for AssertPanicError {}

/// Assert that a piece of code is intended to panic
///
/// This will wrap the provided closure and check the result for a panic. If the function fails to panic
/// an error value is returned, otherwise `Ok(())` is returned.
///
/// ```rust
/// # use libtest2::panic::assert_panic;
/// fn panicky_test() {
///     panic!("intentionally fails");
/// }
///
/// let result = assert_panic(panicky_test);
/// assert!(result.is_ok());
/// ```
///
/// If you also want to check that the panic contains a specific message see [`assert_panic_contains`].
///
/// # Notes
/// This function will wrap the provided closure with a call to [`catch_unwind`](`std::panic::catch_unwind`),
/// and will therefore inherit the caveats of this function, most notably that it will be unable to catch
/// panics if they are not implemented via unwinding.
pub fn assert_panic<T, F: FnOnce() -> T>(f: F) -> Result<(), AssertPanicError> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        // The test should have panicked, but didn't.
        Ok(_) => Err(AssertPanicError(Cow::Borrowed(DID_NOT_PANIC))),

        // The test panicked, as expected.
        Err(_) => Ok(()),
    }
}

/// Assert that a piece of code is intended to panic with a specific message
///
/// This will wrap the provided closure and check the result for a panic. If the function fails to panic with
/// a message that contains the expected string an error value is returned, otherwise `Ok(())` is returned.
///
/// ```rust
/// # use libtest2::panic::assert_panic_contains;
/// fn panicky_test() {
///     panic!("intentionally fails");
/// }
///
/// let result = assert_panic_contains(panicky_test, "fail");
/// assert!(result.is_ok());
///
/// let result = assert_panic_contains(panicky_test, "can't find this");
/// assert!(result.is_err());
/// ```
///
/// If you don't want to check that the panic contains a specific message see [`assert_panic`].
///
/// # Notes
/// This function will wrap the provided closure with a call to [`catch_unwind`](`std::panic::catch_unwind`),
/// and will therefore inherit the caveats of this function, most notably that it will be unable to catch
/// panics if they are not implemented via unwinding.
pub fn assert_panic_contains<T, F: FnOnce() -> T>(
    f: F,
    expected: &str,
) -> Result<(), AssertPanicError> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        // The test should have panicked, but didn't.
        Ok(_) => Err(AssertPanicError(Cow::Borrowed(DID_NOT_PANIC))),

        // The test panicked, as expected, but we need to check the panic message
        Err(payload) => check_panic_message(&*payload, expected),
    }
}

#[cold]
fn check_panic_message(
    payload: &dyn std::any::Any,
    expected: &str,
) -> Result<(), AssertPanicError> {
    // The `panic` information is just an `Any` object representing the
    // value the panic was invoked with. For most panics (which use
    // `panic!` like `println!`), this is either `&str` or `String`.
    let maybe_panic_str = payload
        .downcast_ref::<String>()
        .map(|s| s.as_str())
        .or_else(|| payload.downcast_ref::<&str>().copied());

    // Check the panic message against the expected message.
    match maybe_panic_str {
        Some(panic_str) if panic_str.contains(expected) => Ok(()),

        Some(panic_str) => {
            let error_msg = ::std::format!(
                r#"panic did not contain expected string
      panic message: {panic_str:?}
 expected substring: {expected:?}"#
            );

            Err(AssertPanicError(Cow::Owned(error_msg)))
        }

        None => {
            let type_id = (*payload).type_id();
            let error_msg = ::std::format!(
                r#"expected panic with string value,
 found non-string value: `{type_id:?}`
     expected substring: {expected:?}"#,
            );

            Err(AssertPanicError(Cow::Owned(error_msg)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_panic_with_panic() {
        let result = assert_panic(|| panic!("some message"));
        result.unwrap();
    }

    #[test]
    fn assert_panic_no_panic() {
        let result = assert_panic(|| { /* do absolutely nothing */ });
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), DID_NOT_PANIC);
    }

    #[test]
    fn assert_panic_contains_correct_panic_message() {
        let result = assert_panic_contains(|| panic!("some message"), "mess");
        result.unwrap();
    }

    #[test]
    fn assert_panic_contains_no_panic() {
        let result = assert_panic_contains(|| { /* do absolutely nothing */ }, "fail");
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), DID_NOT_PANIC);
    }

    #[test]
    fn assert_panic_contains_wrong_panic_message() {
        let result = assert_panic_contains(|| panic!("some message"), "fail");
        let error = result.unwrap_err();
        assert_eq!(
            error.0,
            r#"panic did not contain expected string
      panic message: "some message"
 expected substring: "fail""#
        );
    }
}
