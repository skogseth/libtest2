use libtest2_mimic::RunError;
use libtest2_mimic::RunResult;
use libtest2_mimic::TestContext;
use libtest2_mimic::Trial;

fn main() {
    let mut harness = libtest2_mimic::Harness::with_env();
    harness.case(Trial::test("check_toph", check_toph));
    harness.case(Trial::test("check_katara", check_katara));
    harness.case(Trial::test("check_sokka", check_sokka));
    harness.case(Trial::test("long_computation", long_computation));
    harness.case(Trial::test("compile_fail_dummy", compile_fail_dummy));
    harness.main();
}

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
