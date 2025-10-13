/// Expands to the test harness
#[macro_export]
macro_rules! _main {
    ( $( $test:path ),* $(,)*) => {
        fn main() {
            $crate::main([
                $($crate::FnCase::test(::std::stringify!($test), $test)),*
        ]);
        }
    }
}
