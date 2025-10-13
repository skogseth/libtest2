use snapbox::prelude::*;
use snapbox::str;

fn test_cmd() -> snapbox::cmd::Command {
    static BIN: once_cell_polyfill::sync::OnceLock<(std::path::PathBuf, std::path::PathBuf)> =
        once_cell_polyfill::sync::OnceLock::new();
    let (bin, current_dir) = BIN.get_or_init(|| {
        let package_root = crate::util::new_test(
            r#"
libtest2::main!(foo, bar, barro);

fn foo(_context: &libtest2::TestContext) -> libtest2::RunResult {
    Ok(())
}

fn bar(_context: &libtest2::TestContext) -> libtest2::RunResult {
    Ok(())
}

fn barro(_context: &libtest2::TestContext) -> libtest2::RunResult {
    Ok(())
}
"#,
            false,
        );
        let bin = crate::util::compile_test(&package_root);
        (bin, package_root)
    });
    snapbox::cmd::Command::new(bin).current_dir(current_dir)
}

fn check(args: &[&str], single: impl IntoData, parallel: impl IntoData) {
    test_cmd()
        .args(args)
        .args(["--test-threads", "1"])
        .assert()
        .success()
        .stdout_eq(single);
    test_cmd().args(args).assert().success().stdout_eq(parallel);
}

#[test]
fn normal() {
    check(
        &[],
        str![[r#"

running 3 tests
test bar   ... ok
test barro ... ok
test foo   ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 3 tests
...

test result: ok. 3 passed; 0 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn filter_one() {
    check(
        &["foo"],
        str![[r#"

running 1 test
test foo ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 2 filtered out; finished in [..]s


"#]],
        str![[r#"

running 1 test
test foo ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 2 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn filter_two() {
    check(
        &["bar"],
        str![[r#"

running 2 tests
test bar   ... ok
test barro ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 1 filtered out; finished in [..]s


"#]],
        str![[r#"

running 2 tests
...

test result: ok. 2 passed; 0 failed; 0 ignored; 1 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn filter_exact() {
    check(
        &["bar", "--exact"],
        str![[r#"

running 1 test
test bar ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 2 filtered out; finished in [..]s


"#]],
        str![[r#"

running 1 test
test bar ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 2 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn filter_two_and_skip() {
    check(
        &["--skip", "barro", "bar"],
        str![[r#"

running 1 test
test bar ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 2 filtered out; finished in [..]s


"#]],
        str![[r#"

running 1 test
test bar ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 2 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn skip_nothing() {
    check(
        &["--skip", "peter"],
        str![[r#"

running 3 tests
test bar   ... ok
test barro ... ok
test foo   ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 3 tests
...

test result: ok. 3 passed; 0 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn skip_two() {
    check(
        &["--skip", "bar"],
        str![[r#"

running 1 test
test foo ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 2 filtered out; finished in [..]s


"#]],
        str![[r#"

running 1 test
test foo ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 2 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn skip_exact() {
    check(
        &["--exact", "--skip", "bar"],
        str![[r#"

running 2 tests
test barro ... ok
test foo   ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 1 filtered out; finished in [..]s


"#]],
        str![[r#"

running 2 tests
...

test result: ok. 2 passed; 0 failed; 0 ignored; 1 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn terse_output() {
    check(
        &["--quiet", "--skip", "foo"],
        str![[r#"

running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 1 filtered out; finished in [..]s


"#]],
        str![[r#"

running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 1 filtered out; finished in [..]s


"#]],
    );
}
