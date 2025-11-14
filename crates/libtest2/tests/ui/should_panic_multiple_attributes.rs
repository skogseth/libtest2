#[libtest2::main]
fn main() {}

#[libtest2::test]
#[should_panic]
#[should_panic]
fn test_1(_: &libtest2::TestContext) {}

#[libtest2::test]
#[should_panic]
#[should_panic(reason = "something")]
fn test_2(_: &libtest2::TestContext) {}

#[libtest2::test]
#[should_panic = "anything"]
#[should_panic]
fn test_3(_: &libtest2::TestContext) {}
