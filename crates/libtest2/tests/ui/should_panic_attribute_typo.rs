#[libtest2::main]
fn main() {}

#[libtest2::test]
#[should_panic(expect = "something")]
fn test(_: &libtest2::TestContext) {
    panic!("the correct attribute is 'expected = \"...\"'");
}
