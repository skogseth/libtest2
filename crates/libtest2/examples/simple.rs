use libtest2::RunError;
use libtest2::RunResult;
use libtest2::TestContext;

#[libtest2::main]
fn main() {}

// Tests

#[libtest2::test]
fn check_toph(_context: &TestContext) {}
#[libtest2::test]
fn check_katara(_context: &TestContext) {}
#[libtest2::test]
fn check_sokka(_context: &TestContext) -> RunResult {
    Err(RunError::fail("Sokka tripped and fell :("))
}
#[libtest2::test]
#[ignore = "slow"]
fn long_computation(_context: &TestContext) {
    std::thread::sleep(std::time::Duration::from_secs(1));
}
#[libtest2::test]
fn compile_fail_dummy(_context: &TestContext) {}
