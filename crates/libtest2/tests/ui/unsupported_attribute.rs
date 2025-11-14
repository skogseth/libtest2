#[libtest2::main]
fn main() {}

#[libtest2::test]
#[unsafe(no_mangle)] // we just need some unknown attribute here
fn test(_: &libtest2::TestContext) {}
