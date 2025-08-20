use libtest2::RunError;
use libtest2::RunResult;
use libtest2::TestContext;

libtest2::libtest2_main!(
    check_toph,
    check_katara,
    check_sokka,
    long_computation,
    compile_fail_dummy
);

// Tests

fn check_toph(_context: &TestContext) -> RunResult {
    Ok(())
}
fn check_katara(_context: &TestContext) -> RunResult {
    Ok(())
}
fn check_sokka(_context: &TestContext) -> RunResult {
    Err(RunError::fail("Sokka tripped and fell :("))
}
fn long_computation(context: &TestContext) -> RunResult {
    context.ignore_for("slow")?;

    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}
fn compile_fail_dummy(_context: &TestContext) -> RunResult {
    Ok(())
}
