use snapbox::prelude::*;
use snapbox::str;

fn test_cmd() -> snapbox::cmd::Command {
    static BIN: once_cell_polyfill::sync::OnceLock<(std::path::PathBuf, std::path::PathBuf)> =
        once_cell_polyfill::sync::OnceLock::new();
    let (bin, current_dir) = BIN.get_or_init(|| {
        let package_root = crate::util::new_test(
            r#"
libtest2::libtest2_main!(one, two, three, one_two);

fn one(_state: &libtest2::State) -> libtest2::RunResult {
    Ok(())
}

fn two(_state: &libtest2::State) -> libtest2::RunResult {
    Ok(())
}

fn three(_state: &libtest2::State) -> libtest2::RunResult {
    Ok(())
}

fn one_two(_state: &libtest2::State) -> libtest2::RunResult {
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

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
        str![[r#"

running 0 tests
...

test result: ok. 0 passed; 0 failed; 0 ignored; 4 filtered out; finished in [..]s


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

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
        str![[r#"

running 0 tests
...

test result: ok. 0 passed; 0 failed; 0 ignored; 4 filtered out; finished in [..]s


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

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
        str![[r#"

running 0 tests
...

test result: ok. 0 passed; 0 failed; 0 ignored; 4 filtered out; finished in [..]s


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

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
        str![[r#"

running 0 tests
...

test result: ok. 0 passed; 0 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn invalid() {
    let argfile = std::path::Path::new("highly-improbably-non-existent-file.txt");
    check(
        &[],
        argfile,
        0,
        str![[r#"

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
        str![[r#"

running 0 tests
...

test result: ok. 0 passed; 0 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
    );
}
