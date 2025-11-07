#[libtest2::main]
fn main() {}

#[libtest2::test]
fn bad_ok_variant(_: &libtest2::TestContext) -> Result<i32, std::io::Error> {
    Ok(0)
}

#[libtest2::test]
fn bad_err_variant(_: &libtest2::TestContext) -> Result<(), String> {
    Ok(())
}
