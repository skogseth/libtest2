use snapbox::prelude::*;
use snapbox::str;

fn test_cmd() -> snapbox::cmd::Command {
    static BIN: once_cell_polyfill::sync::OnceLock<(std::path::PathBuf, std::path::PathBuf)> =
        once_cell_polyfill::sync::OnceLock::new();
    let (bin, current_dir) = BIN.get_or_init(|| {
        let package_root = crate::util::new_test(
            r#"
#[libtest2::main]
fn main() {}

#[libtest2::test]
fn accidentally_panics(_context: &libtest2::TestContext) {
    panic!("uh oh")
}

#[libtest2::test]
#[should_panic]
fn intentionally_panics(_context: &libtest2::TestContext) {
    panic!("any message would do")
}

#[libtest2::test]
#[should_panic = "intentional"]
fn intentionally_panics_with_message(_context: &libtest2::TestContext) {
    panic!("this is intentional")
}

#[libtest2::test]
#[should_panic = "in a controlled manner"]
fn panics_with_the_wrong_message(_context: &libtest2::TestContext) {
    panic!("with the wrong message")
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

running 4 tests
test accidentally_panics               ... FAILED
test intentionally_panics              ... ok
test intentionally_panics_with_message ... ok
test panics_with_the_wrong_message     ... FAILED

failures:

---- accidentally_panics ----
test panicked: uh oh

---- panics_with_the_wrong_message ----
panic did not contain expected string
      panic message: "with the wrong message"
 expected substring: "in a controlled manner"


failures:
    accidentally_panics
    panics_with_the_wrong_message

test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 4 tests
...

failures:

---- accidentally_panics ----
test panicked: uh oh

---- panics_with_the_wrong_message ----
panic did not contain expected string
      panic message: "with the wrong message"
 expected substring: "in a controlled manner"


failures:
    accidentally_panics
    panics_with_the_wrong_message

test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}
