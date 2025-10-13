/// Expands to the test harness
#[macro_export]
macro_rules! _main {
    ( $( $test:path ),* $(,)*) => {
        fn main() {
            let harness = $crate::Harness::new();
            let harness = match harness.with_env() {
                Ok(harness) => harness,
                Err(err) => {
                    eprintln!("{err}");
                    ::std::process::exit(1);
                }
            };
            let harness = match harness.parse() {
                Ok(harness) => harness,
                Err(err) => {
                    eprintln!("{err}");
                    ::std::process::exit(1);
                }
            };
            let harness = match harness.discover([
                $($crate::Trial::test(::std::stringify!($test), $test)),*
            ]) {
                Ok(harness) => harness,
                Err(err) => {
                    eprintln!("{err}");
                    ::std::process::exit($crate::ERROR_EXIT_CODE)
                }
            };
            match harness.run() {
                Ok(true) => ::std::process::exit(0),
                Ok(false) => ::std::process::exit($crate::ERROR_EXIT_CODE),
                Err(err) => {
                    eprintln!("{err}");
                    ::std::process::exit($crate::ERROR_EXIT_CODE)
                }
            }
        }
    }
}
