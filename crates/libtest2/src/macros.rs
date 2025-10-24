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

#[macro_export]
#[allow(clippy::crate_in_macro_def)] // accessing item defined by `_main_parse`
macro_rules! _test_parse {
    // Entry point
    (#[test] $(#[$($attr:tt)*])* fn $name:ident $($item:tt)*) => {
        $crate::_private::test_parse! {
            name=$name
            body=[$($item)*]
            unparsed_attrs=[$(#[$($attr)*])*]
            parsed_attrs=[]
            ignore=[]
        }
    };

    // Recursively handle attributes
    (name=$name:ident body=[$($item:tt)*] unparsed_attrs=[] parsed_attrs=[$(#[$($parsed_attr:tt)*])*] ignore=[$($state:expr)?]) => {
        $crate::_private::test_parse! {
            name=$name
            body=[$($item)*]
            attrs=[$(#[$($parsed_attr)*])*]
            ignore=[$($state)?]
        }
    };
    (name=$name:ident body=[$($item:tt)*] unparsed_attrs=[#[ignore] $(#[$($unparsed_attr:tt)*])*] parsed_attrs=[$(#[$($parsed_attr:tt)*])*] ignore=[$($state:expr)?]) => {
        $crate::_private::test_parse! {
            name=$name
            body=[$($item)*]
            unparsed_attrs=[$(#[$($unparsed_attr)*])*]
            parsed_attrs=[#[$(#[$($parsed_attr)*])*]]
            ignore=[()]
        }
    };
    (name=$name:ident body=[$($item:tt)*] unparsed_attrs=[#[ignore = $reason:expr] $(#[$($unparsed_attr:tt)*])*] parsed_attrs=[$(#[$($parsed_attr:tt)*])*] ignore=[$($state:expr)?]) => {
        $crate::_private::test_parse! {
            name=$name
            body=[$($item)*]
            unparsed_attrs=[$(#[$($unparsed_attr)*])*]
            parsed_attrs=[#[$(#[$($parsed_attr)*])*]]
            ignore=[$reason]
        }
    };
    (name=$name:ident body=[$($item:tt)*] unparsed_attrs=[#[$($attr:tt)+] $(#[$($unparsed_attr:tt)*])*] parsed_attrs=[$(#[$($parsed_attr:tt)*])*] ignore=[$($state:expr)?]) => {
        $crate::_private::test_parse! {
            name=$name
            body=[$($item)*]
            unparsed_attrs=[$(#[$($unparsed_attr)*])*]
            parsed_attrs=[#[$($attr)* $(#[$($parsed_attr)*])*]]
            ignore=[$($state)?]
        }
    };

    // End result
    (name=$name:ident body=[$($item:tt)*] attrs=[$(#[$($attr:tt)*])*] ignore=[]) => {
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
                // $(#[$($attr)*])*
                fn run $($item)*

                run(context)
            }
        }
    };
    (name=$name:ident body=[$($item:tt)*] attrs=[$(#[$($attr:tt)*])*] ignore=[$reason:literal]) => {
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
                // $(#[$($attr)*])*
                fn run $($item)*

                let reason: &'static str = $reason;
                context.ignore_for(reason)?;

                run(context)
            }
        }
    };
    (name=$name:ident body=[$($item:tt)*] attrs=[$(#[$($attr:tt)*])*] ignore=[$unused:expr]) => {
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
                // $(#[$($attr)*])*
                fn run $($item)*

                context.ignore()?;

                run(context)
            }
        }
    };
}
