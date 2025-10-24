#[macro_export]
macro_rules! _main_parse {
    (#[main] fn main $($item:tt)*) => {
        static TESTS: $crate::_private::DistributedList<$crate::_private::DynCase> = $crate::_private::DistributedList::root();

        fn main() {
            fn inner $($item)*

            inner();
            $crate::main(TESTS.iter().copied());
        }
    };
}

pub enum Attribute {
    Ignore(Option<&'static str>),
    ShouldPanic(Option<&'static str>),
}

#[macro_export]
macro_rules! _parse_attr {
    (ignore) => {
        $crate::_private::Attribute::Ignore(None)
    };
    (ignore = $reason:expr) => {
        $crate::_private::Attribute::Ignore(Some($reason))
    };
    (should_panic) => {
        $crate::_private::Attribute::ShouldPanic(None)
    };
    (should_panic = $expected:expr) => {
        $crate::_private::Attribute::ShouldPanic(Some($expected))
    };
    (should_panic(expected = $expected:expr)) => {
        $crate::_private::Attribute::ShouldPanic(Some($expected))
    };
    ($($attr:tt)*) => {
        compile_error!(concat!("unknown attribute '", stringify!($($attr)*), "'"));
    };
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)] // accessing item defined by `_main_parse`
macro_rules! _test_parse {
    (#[test] $(#[$($attr:tt)*])* fn $name:ident $($item:tt)*) => {
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

                const ATTRS: (::core::option::Option<::core::option::Option<&'static str>>, ::core::option::Option<::core::option::Option<&'static str>>)  = const {
                    let mut ignore: ::core::option::Option<::core::option::Option<&'static str>> = ::core::option::Option::None;
                    let mut should_panic: ::core::option::Option<::core::option::Option<&'static str>> = ::core::option::Option::None;

                    $(
                        match $crate::_private::parse_attr!($($attr)*) {
                            $crate::_private::Attribute::Ignore(maybe_reason) => {
                                ignore = ::core::option::Option::Some(maybe_reason);
                            }
                            $crate::_private::Attribute::ShouldPanic(maybe_message) => {
                                should_panic = ::core::option::Option::Some(maybe_message);
                            }
                        }
                    )*

                    (ignore, should_panic)
                };

                match ATTRS.0 {
                    ::core::option::Option::Some(::core::option::Option::Some(reason)) => context.ignore_for(reason)?,
                    ::core::option::Option::Some(::core::option::Option::None) => context.ignore()?,
                    ::core::option::Option::None => {}
                }

                if let ::core::option::Option::Some(panic_message) = ATTRS.1 {
                    match (::std::panic::catch_unwind(|| run(context)), panic_message) {
                        (Ok(_), _) => Err($crate::RunError::fail("expected panic")),
                        (Err(_), ::core::option::Option::None) => Ok(()),
                        (Err(e), ::core::option::Option::Some(expected)) => {
                            // The `panic` information is just an `Any` object representing the
                            // value the panic was invoked with. For most panics (which use
                            // `panic!` like `println!`), this is either `&str` or `String`.
                            let payload = e
                                .downcast_ref::<String>()
                                .map(|s| s.as_str())
                                .or_else(|| e.downcast_ref::<&str>().copied());

                            match payload {
                                ::core::option::Option::Some(found) if found == expected => Ok(()),
                                ::core::option::Option::Some(found) => {
                                    let error_msg = format!("unexpected panic message '{found}' (expected '{expected}')");
                                    Err($crate::RunError::fail(error_msg))
                                }
                                ::core::option::Option::None => {
                                    let error_msg = format!("unexpected panic message <<not a string>> (expected '{expected}')");
                                    Err($crate::RunError::fail(error_msg))
                                }
                            }
                        }
                    }
                } else {
                    run(context)
                }
            }
        }
    };
}
