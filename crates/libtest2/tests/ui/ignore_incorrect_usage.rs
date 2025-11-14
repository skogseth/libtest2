#[libtest2::main]
fn main() {}

#[libtest2::test]
#[ignore(reason = "invalid syntax right here")]
fn test(_: &libtest2::TestContext) {
    panic!("We just need to compile this");
}
