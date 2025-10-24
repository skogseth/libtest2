#[macro_export]
macro_rules! _main_parse {
    (#[main] $(#[$meta:meta])* fn main $($item:tt)*) => {
        static TESTS: $crate::_private::DistributedList<$crate::_private::DynCase> = $crate::_private::DistributedList::root();

        $(#[$meta])*
        fn main() {
            fn inner $($item)*

            inner();
            $crate::main(TESTS.iter().copied());
        }
    };
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)] // accessing item defined by `_main_parse`
macro_rules! _test_parse {
    // Entry point
    (#[test] $(#[$($attr:tt)+])* fn $name:ident $($item:tt)*) => {
        $crate::_private::test_parse!(continue:
            name=$name
            body=[$($item)*]
            attrs=[$(#[$($attr)+])*]
        );
    };

    // Recursively handle attributes:

    // Edge condition (no more attributes to parse)
    (continue: name=$name:ident body=[$($item:tt)*] attrs=[] $(ignore=$ignore:tt)? $(should_panic=$should_panic:tt)?) => {
        $crate::_private::test_parse!(break:
            name=$name
            body=[$($item)*]
            $(ignore=$ignore)?
            $(should_panic=$should_panic)?
        );
    };
    // Process `#[ignore]` macro (NOTE: This will only match if an `#[ignore]` macro has not already been parsed)
    (continue: name=$name:ident body=[$($item:tt)*] attrs=[#[ignore $(= $reason:literal)?] $(#[$($attr:tt)+])*] $(should_panic=$should_panic:tt)?) => {
        $crate::_private::test_parse!(continue:
            name=$name
            body=[$($item)*]
            attrs=[$(#[$($attr)*])*]
            ignore=[$($reason)?]
            $(should_panic=$should_panic)?
        );
    };
    // Process `#[should_panic]` macro (NOTE: This will only match if `#[should_panic]` macro has not already been parsed)
    (continue: name=$name:ident body=[$($item:tt)*] attrs=[#[should_panic $(= $expected:literal)?] $(#[$($attr:tt)+])*] $(ignore=$ignore:tt)?) => {
        $crate::_private::test_parse!(continue:
            name=$name
            body=[$($item)*]
            attrs=[$(#[$($attr)*])*]
            $(ignore=$ignore)?
            should_panic=[$($expected)?]
        );
    };
    // Process `#[should_panic(expected = "..")]` macro (NOTE: Same as branch above)
    (continue: name=$name:ident body=[$($item:tt)*] attrs=[#[should_panic(expected = $expected:literal)] $(#[$($attr:tt)+])*] $(ignore=$ignore:tt)?) => {
        $crate::_private::test_parse!(continue:
            name=$name
            body=[$($item)*]
            attrs=[$(#[$($attr)*])*]
            $(ignore=$ignore)?
            should_panic=[$expected]
        );
    };
    // Discard unknown attributes
    (continue: name=$name:ident body=[$($item:tt)*] attrs=[#[$($unknown_attr:tt)+] $(#[$($attr:tt)+])*] $(ignore=$ignore:tt)? $(should_panic=$should_panic:tt)?) => {
        $crate::_private::test_parse!(continue:
            name=$name
            body=[$($item)*]
            attrs=[$(#[$($attr)*])*]
            $(ignore=$ignore)?
            $(should_panic=$should_panic)?
        );
    };

    // End result
    (break: name=$name:ident body=[$($item:tt)*] $(ignore=$ignore:tt)? $(should_panic=$should_panic:tt)?) => {
        #[allow(non_camel_case_types)]
        struct $name;

        impl $crate::_private::Case for $name {
            fn name(&self) -> &str {
                $crate::_private::push!(crate::TESTS, _: $crate::_private::DynCase = $crate::_private::DynCase(&$name));

                stringify!($name)
            }
            fn kind(&self) -> $crate::_private::TestKind {
                Default::default()
            }
            fn source(&self) -> Option<&$crate::_private::Source> {
                None
            }
            fn exclusive(&self, _: &$crate::TestContext) -> bool {
                false
            }

            fn run(&self, context: &$crate::TestContext) -> $crate::RunResult {
                fn run $($item)*

                $(
                    $crate::_private::parse_ignore!(context, $ignore)?;
                )?

                use $crate::IntoRunResult;
                let result = $crate::_private::run_test!(context, $($should_panic)?);
                IntoRunResult::into_run_result(result)
            }
        }
    };
}
#[macro_export]
macro_rules! _parse_ignore {
    ($context:expr, [$reason:literal]) => {
        $context.ignore_for($reason)
    };
    ($context:expr, []) => {
        $context.ignore()
    };
}

#[macro_export]
macro_rules! _run_test {
    ($context:expr, [$($expected:literal)?]) => {
        $crate::assert_panic!(run($context), $($expected)?)
    };
    ($context:expr $(,)?) => {{
        run($context)
    }};
}

#[macro_export]
macro_rules! assert_panic {
    ($f:expr, $expected:literal $(,)?) => {
        match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| $f)) {
            // The test should have panicked, but didn't.
            ::std::result::Result::Ok(_) => {
                // TODO: Rust includes the source file location here, consider doing the same?
                ::std::result::Result::Err($crate::RunError::fail("test did not panic as expected"))
            }

            // The test panicked, as expected.
            ::std::result::Result::Err(payload) => {
                // The `panic` information is just an `Any` object representing the
                // value the panic was invoked with. For most panics (which use
                // `panic!` like `println!`), this is either `&str` or `String`.
                let maybe_panic_str = payload
                    .downcast_ref::<::std::string::String>()
                    .map(|s| s.as_str())
                    .or_else(|| payload.downcast_ref::<&str>().copied());

                // Enforce `$expected` to be a string literal
                let expected: &'static str = $expected;

                // Check the panic message against the expected message.
                match maybe_panic_str {
                    ::std::option::Option::Some(panic_str) if panic_str.contains(expected) => {
                        ::std::result::Result::Ok(())
                    }

                    ::std::option::Option::Some(panic_str) => {
                        let error_msg = ::std::format!(
                            r#"panic did not contain expected string
      panic message: {panic_str:?}
 expected substring: {expected:?}"#
                        );

                        ::std::result::Result::Err($crate::RunError::fail(error_msg))
                    }

                    ::std::option::Option::None => {
                        let type_id = (*payload).type_id();
                        let error_msg = ::std::format!(
                            r#"expected panic with string value,
 found non-string value: `{type_id:?}`
     expected substring: {expected:?}"#,
                        );

                        ::std::result::Result::Err($crate::RunError::fail(error_msg))
                    }
                }
            }
        }
    };
    ($f:expr $(,)?) => {
        match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| $f)) {
            // The test should have panicked, but didn't.
            ::std::result::Result::Ok(_) => {
                // TODO: Rust includes the source file location here, consider doing the same?
                ::std::result::Result::Err($crate::RunError::fail("test did not panic as expected"))
            }

            // The test panicked, as expected.
            ::std::result::Result::Err(_) => Ok(()),
        }
    };
}
