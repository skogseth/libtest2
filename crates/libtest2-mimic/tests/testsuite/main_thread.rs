use snapbox::str;

#[test]
fn check_test_on_main_thread() {
    let package_root = crate::util::new_test(
        r#"
fn main() {
    use libtest2_mimic::Trial;
    let outer_thread = std::thread::current().id();
    let mut harness = libtest2_mimic::Harness::with_env();
    harness.cases(vec![
        Trial::test("check", move |_| {
            assert_eq!(outer_thread, std::thread::current().id());
            Ok(())
        })
    ]);
    harness.main();
}
"#,
        false,
    );
    let bin = crate::util::compile_test(&package_root);
    snapbox::cmd::Command::new(bin)
        .current_dir(package_root)
        .assert()
        .success()
        .stdout_eq(str![[r#"

running 1 test
...

"#]]);
}
