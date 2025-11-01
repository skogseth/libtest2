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
#[allow(clippy::crate_in_macro_def)] // accessing item defined by `_main_parse`/`_parse_ignore`, and recursively calling the macro itself
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
    (continue: name=$name:ident body=[$($item:tt)*] attrs=[] $(ignore=$ignore:tt)?) => {
        $crate::_private::test_parse!(break:
            name=$name
            body=[$($item)*]
            $(ignore=$ignore)?
        );
    };
    // Process `#[ignore]`/`#[ignore = ".."]` (NOTE: This will only match if an ignore macro has not already been parsed)
    (continue: name=$name:ident body=[$($item:tt)*] attrs=[#[ignore $(= $reason:literal)?] $(#[$($attr:tt)+])*]) => {
        $crate::_private::test_parse!(continue:
            name=$name
            body=[$($item)*]
            attrs=[$(#[$($attr)*])*]
            ignore=[$($reason)?]
        );
    };
    // Ignore subsequent calls to `#[ignore]`/`#[ignore = ".."]`
    (continue: name=$name:ident body=[$($item:tt)*] attrs=[#[ignore $(= $reason:literal)?] $(#[$($attr:tt)+])*] ignore=$ignore:tt) => {
        $crate::_private::test_parse!(continue:
            name=$name
            body=[$($item)*]
            attrs=[$(#[$($attr)*])*]
            ignore=$ignore
        );
    };
    // Emit error on unknown attributes (but continue parsing)
    (continue: name=$name:ident body=[$($item:tt)*] attrs=[#[$($unknown_attr:tt)+] $(#[$($attr:tt)+])*] $(ignore=$ignore:tt)?) => {
        compile_error!(concat!("unknown attribute '", stringify!($($unknown_attr)+), "'"));
        $crate::_private::test_parse!(continue:
            name=$name
            body=[$($item)*]
            attrs=[$(#[$($attr)*])*]
            $(ignore=$ignore)?
        );
    };

    // End result
    (break: name=$name:ident body=[$($item:tt)*] $(ignore=$ignore:tt)?) => {
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

                $crate::_private::parse_ignore!(context, $($ignore)?);

                use $crate::IntoRunResult;
                let result = run(context);
                IntoRunResult::into_run_result(result)
            }
        }
    };
}

#[macro_export]
macro_rules! _parse_ignore {
    ($context:expr, [$reason:literal] $(,)?) => {
        $context.ignore_for($reason)?
    };
    ($context:expr, [] $(,)?) => {
        $context.ignore()?
    };
    ($context:expr $(,)?) => {};
}
