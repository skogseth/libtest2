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
fn foo(_context: &libtest2::TestContext) {}

mod some_module {
    #[libtest2::test]
    fn foo(_context: &libtest2::TestContext) {}
}
"#,
            false,
        );
        let bin = crate::util::compile_test(&package_root);
        (bin, package_root)
    });
    snapbox::cmd::Command::new(bin).current_dir(current_dir)
}

#[test]
fn check() {
    let data = str![[r#"

running 2 tests
test foo ... ok
test foo ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]];

    test_cmd()
        .args(["--test-threads", "1"])
        .assert()
        .success()
        .stdout_eq(data);
}
