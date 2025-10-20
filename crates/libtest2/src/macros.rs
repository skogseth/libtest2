#[macro_export]
macro_rules! _main_parse {
    (#[main] fn main $($item:tt)*) => {
        static TESTS: $crate::_private::DistributedList<$crate::_private::TestDef> = $crate::_private::DistributedList::root();

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
    (#[test($tag:ident, $t:ty)] fn $name:ident $($item:tt)*) => {
        #[allow(bad_style)]
        const $name: () = const {
            fn run $($item)*

            $crate::_private::push!($tag, _: $crate::_private::TestDef<$t> = $crate::_private::TestDef {
                name: stringify!($name),
                kind: $crate::_private::TestKind::IntegrationTest,
                exclusive: false,
                function: run,
            });
        };
    };
    (#[test] fn $name:ident $($item:tt)*) => {
        #[allow(bad_style)]
        const $name: () = const {
            fn run $($item)*

            $crate::_private::push!(crate::TESTS, _: $crate::_private::TestDef = $crate::_private::TestDef {
                name: stringify!($name),
                kind: $crate::_private::TestKind::IntegrationTest,
                exclusive: false,
                function: run,
            });
        };
    };
}
