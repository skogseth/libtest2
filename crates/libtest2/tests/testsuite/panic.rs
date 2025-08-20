use snapbox::prelude::*;
use snapbox::str;

fn test_cmd() -> snapbox::cmd::Command {
    static BIN: once_cell_polyfill::sync::OnceLock<(std::path::PathBuf, std::path::PathBuf)> =
        once_cell_polyfill::sync::OnceLock::new();
    let (bin, current_dir) = BIN.get_or_init(|| {
        let package_root = crate::util::new_test(
            r#"
libtest2::libtest2_main!(passes, panics);

fn passes(_context: &libtest2::TestContext) -> libtest2::RunResult {
    Ok(())
}

fn panics(_context: &libtest2::TestContext) -> libtest2::RunResult {
    panic!("uh oh")
}
"#,
            false,
        );
        let bin = crate::util::compile_test(&package_root);
        (bin, package_root)
    });
    snapbox::cmd::Command::new(bin).current_dir(current_dir)
}

fn check(args: &[&str], code: i32, single: impl IntoData, parallel: impl IntoData) {
    test_cmd()
        .args(args)
        .args(["--test-threads", "1"])
        .assert()
        .code(code)
        .stdout_eq(single);
    test_cmd()
        .args(args)
        .assert()
        .code(code)
        .stdout_eq(parallel);
}

#[test]
fn normal() {
    check(
        &[],
        101,
        str![[r#"

running 2 tests
test panics ... FAILED
test passes ... ok

failures:

---- panics ----
test panicked: uh oh


failures:
    panics

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 2 tests
...

failures:

---- panics ----
test panicked: uh oh


failures:
    panics

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}
