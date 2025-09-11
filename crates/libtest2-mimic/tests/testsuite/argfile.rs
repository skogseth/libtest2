use snapbox::prelude::*;
use snapbox::str;

fn test_cmd() -> snapbox::cmd::Command {
    static BIN: once_cell_polyfill::sync::OnceLock<(std::path::PathBuf, std::path::PathBuf)> =
        once_cell_polyfill::sync::OnceLock::new();
    let (bin, current_dir) = BIN.get_or_init(|| {
        let package_root = crate::util::new_test(
            r#"
fn main() {
    use libtest2_mimic::Trial;
    use libtest2_mimic::RunError;
    libtest2_mimic::Harness::with_env()
        .discover([
            Trial::test("one", |_| Ok(())),
            Trial::test("two", |_| Ok(())),
            Trial::test("three", |_| Ok(())),
            Trial::test("one_two", |_| Ok(())),
        ])
        .main();
}
"#,
            false,
        );
        let bin = crate::util::compile_test(&package_root);
        (bin, package_root)
    });
    snapbox::cmd::Command::new(bin).current_dir(current_dir)
}

fn check(
    args: &[&str],
    argfile: &std::path::Path,
    code: i32,
    single: impl IntoData,
    parallel: impl IntoData,
) {
    test_cmd()
        .arg(format!("@{}", argfile.to_str().unwrap()))
        .args(args)
        .args(["--test-threads", "1"])
        .assert()
        .code(code)
        .stdout_eq(single);
    test_cmd()
        .arg(format!("@{}", argfile.to_str().unwrap()))
        .args(args)
        .assert()
        .code(code)
        .stdout_eq(parallel);
}

#[test]
fn empty() {
    let argfile = crate::util::new_file("argfile-", ".txt", "");
    check(
        &[],
        &argfile,
        0,
        str![[r#"

running 4 tests
test one     ... ok
test one_two ... ok
test three   ... ok
test two     ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 4 tests
...

test result: ok. 4 passed; 0 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn list() {
    let argfile = crate::util::new_file("argfile-", ".txt", "--list");
    check(
        &[],
        &argfile,
        0,
        str![[r#"
one: test
two: test
three: test
one_two: test

4 tests


"#]],
        str![[r#"
one: test
two: test
...

4 tests


"#]],
    );
}

#[test]
fn multiline() {
    let argfile = crate::util::new_file(
        "argfile-",
        ".txt",
        "one
two
--exact",
    );
    check(
        &[],
        &argfile,
        0,
        str![[r#"

running 2 tests
test one ... ok
test two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 2 filtered out; finished in [..]s


"#]],
        str![[r#"

running 2 tests
...

test result: ok. 2 passed; 0 failed; 0 ignored; 2 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn mixed() {
    let argfile = crate::util::new_file(
        "argfile-", ".txt", "one
two",
    );
    check(
        &["--exact"],
        &argfile,
        0,
        str![[r#"

running 2 tests
test one ... ok
test two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 2 filtered out; finished in [..]s


"#]],
        str![[r#"

running 2 tests
...

test result: ok. 2 passed; 0 failed; 0 ignored; 2 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn invalid() {
    let argfile = std::path::Path::new("highly-improbably-non-existent-file.txt");
    check(&[], argfile, 1, str![""], str![""]);
}
