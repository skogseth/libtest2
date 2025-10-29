use snapbox::prelude::*;
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
fn cat(_context: &libtest2::TestContext) {
}

#[libtest2::test]
fn dog(_context: &libtest2::TestContext) -> libtest2::RunResult {
    Err(libtest2::RunError::fail("was not a good boy"))
}

#[libtest2::test]
fn fox(_context: &libtest2::TestContext) {
}

#[libtest2::test]
fn bunny(context: &libtest2::TestContext) -> libtest2::RunResult {
    context.ignore_for("fails")?;
    Err(libtest2::RunError::fail("jumped too high"))
}

#[libtest2::test]
fn frog(context: &libtest2::TestContext) -> libtest2::RunResult {
    context.ignore_for("slow")?;
    Ok(())
}

#[libtest2::test]
fn owl(context: &libtest2::TestContext) -> libtest2::RunResult {
    context.ignore_for("fails")?;
    Err(libtest2::RunError::fail("broke neck"))
}

#[libtest2::test]
fn fly(context: &libtest2::TestContext) -> libtest2::RunResult {
    context.ignore_for("fails")?;
    Ok(())
}

#[libtest2::test]
#[ignore = "fails"]
fn bear(context: &libtest2::TestContext) -> libtest2::RunResult {
    Err(libtest2::RunError::fail("no honey"))
}

#[libtest2::test]
#[ignore]
fn sheep(context: &libtest2::TestContext) -> libtest2::RunResult {
    Err(libtest2::RunError::fail("got lost blindly following the flock"))
}

#[libtest2::test]
#[ignore = "slow"]
fn horse(context: &libtest2::TestContext) {
}

#[libtest2::test]
fn custom_error(context: &libtest2::TestContext) -> libtest2::RunResult {
    Err(libtest2::RunError::from(std::io::Error::new(std::io::ErrorKind::Other, "I failed")))
}

"#,
            false,
        );
        let bin = crate::util::compile_test(&package_root);
        (bin, package_root)
    });
    snapbox::cmd::Command::new(bin).current_dir(current_dir)
}

fn check(args: &[&str], code: i32, single: impl IntoData, parallel: impl IntoData) {
    test_cmd()
        .args(args)
        .args(["--test-threads", "1"])
        .assert()
        .code(code)
        .stdout_eq(single);
    test_cmd()
        .args(args)
        .assert()
        .code(code)
        .stdout_eq(parallel);
}

#[test]
fn normal() {
    check(
        &[],
        101,
        str![[r#"

running 11 tests
test bear         ... ignored
test bunny        ... ignored
test cat          ... ok
test custom_error ... FAILED
test dog          ... FAILED
test fly          ... ignored
test fox          ... ok
test frog         ... ignored
test horse        ... ignored
test owl          ... ignored
test sheep        ... ignored

failures:

---- custom_error ----
I failed

---- dog ----
was not a good boy


failures:
    custom_error
    dog

test result: FAILED. 2 passed; 2 failed; 7 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 11 tests
...

failures:

---- custom_error ----
I failed

---- dog ----
was not a good boy


failures:
    custom_error
    dog

test result: FAILED. 2 passed; 2 failed; 7 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn test_mode() {
    check(
        &["--test"],
        101,
        str![[r#"

running 11 tests
test bear         ... ignored
test bunny        ... ignored
test cat          ... ok
test custom_error ... FAILED
test dog          ... FAILED
test fly          ... ignored
test fox          ... ok
test frog         ... ignored
test horse        ... ignored
test owl          ... ignored
test sheep        ... ignored

failures:

---- custom_error ----
I failed

---- dog ----
was not a good boy


failures:
    custom_error
    dog

test result: FAILED. 2 passed; 2 failed; 7 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 11 tests
...

failures:

---- custom_error ----
I failed

---- dog ----
was not a good boy


failures:
    custom_error
    dog

test result: FAILED. 2 passed; 2 failed; 7 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn bench_mode() {
    check(
        &["--bench"],
        101,
        str![[r#"

running 11 tests
test bear         ... ignored
test bunny        ... ignored
test cat          ... ok
test custom_error ... FAILED
test dog          ... FAILED
test fly          ... ignored
test fox          ... ok
test frog         ... ignored
test horse        ... ignored
test owl          ... ignored
test sheep        ... ignored

failures:

---- custom_error ----
I failed

---- dog ----
was not a good boy


failures:
    custom_error
    dog

test result: FAILED. 2 passed; 2 failed; 7 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 11 tests
...

failures:

---- custom_error ----
I failed

---- dog ----
was not a good boy


failures:
    custom_error
    dog

test result: FAILED. 2 passed; 2 failed; 7 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn list() {
    check(
        &["--list"],
        0,
        str![[r#"
bear: test
bunny: test
cat: test
custom_error: test
dog: test
fly: test
fox: test
frog: test
horse: test
owl: test
sheep: test

11 tests


"#]],
        str![[r#"
bear: test
bunny: test
cat: test
custom_error: test
dog: test
fly: test
fox: test
frog: test
horse: test
owl: test
sheep: test

11 tests


"#]],
    );
}

#[test]
fn list_ignored() {
    check(
        &["--list", "--ignored"],
        0,
        str![[r#"
bear: test
bunny: test
cat: test
custom_error: test
dog: test
fly: test
fox: test
frog: test
horse: test
owl: test
sheep: test

11 tests


"#]],
        str![[r#"
bear: test
bunny: test
cat: test
custom_error: test
dog: test
fly: test
fox: test
frog: test
horse: test
owl: test
sheep: test

11 tests


"#]],
    );
}

#[test]
fn list_with_filter() {
    check(
        &["--list", "a"],
        0,
        str![[r#"
bear: test
cat: test

2 tests


"#]],
        str![[r#"
bear: test
cat: test

2 tests


"#]],
    );
}

#[test]
fn list_with_specified_order() {
    check(
        &["--list", "--exact", "owl", "fox", "bunny", "frog"],
        0,
        str![[r#"
bunny: test
fox: test
frog: test
owl: test

4 tests


"#]],
        str![[r#"
bunny: test
fox: test
frog: test
owl: test

4 tests


"#]],
    );
}

#[test]
fn filter_c() {
    check(
        &["a"],
        0,
        str![[r#"

running 2 tests
test bear ... ignored
test cat  ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 9 filtered out; finished in [..]s


"#]],
        str![[r#"

running 2 tests
...

test result: ok. 1 passed; 0 failed; 1 ignored; 9 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn filter_o_test() {
    check(
        &["--test", "a"],
        0,
        str![[r#"

running 2 tests
test bear ... ignored
test cat  ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 9 filtered out; finished in [..]s


"#]],
        str![[r#"

running 2 tests
...

test result: ok. 1 passed; 0 failed; 1 ignored; 9 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn filter_o_test_include_ignored() {
    check(
        &["--test", "--include-ignored", "o"],
        101,
        str![[r#"

running 6 tests
test custom_error ... FAILED
test dog          ... FAILED
test fox          ... ok
test frog         ... ok
test horse        ... ok
test owl          ... FAILED

failures:

---- custom_error ----
I failed

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    custom_error
    dog
    owl

test result: FAILED. 3 passed; 3 failed; 0 ignored; 5 filtered out; finished in [..]s


"#]],
        str![[r#"

running 6 tests
...

failures:

---- custom_error ----
I failed

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    custom_error
    dog
    owl

test result: FAILED. 3 passed; 3 failed; 0 ignored; 5 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn filter_o_test_ignored() {
    check(
        &["--test", "--ignored", "o"],
        101,
        str![[r#"

running 6 tests
test custom_error ... FAILED
test dog          ... FAILED
test fox          ... ok
test frog         ... ok
test horse        ... ok
test owl          ... FAILED

failures:

---- custom_error ----
I failed

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    custom_error
    dog
    owl

test result: FAILED. 3 passed; 3 failed; 0 ignored; 5 filtered out; finished in [..]s


"#]],
        str![[r#"

running 6 tests
...

failures:

---- custom_error ----
I failed

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    custom_error
    dog
    owl

test result: FAILED. 3 passed; 3 failed; 0 ignored; 5 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn normal_include_ignored() {
    check(
        &["--include-ignored"],
        101,
        str![[r#"

running 11 tests
test bear         ... FAILED
test bunny        ... FAILED
test cat          ... ok
test custom_error ... FAILED
test dog          ... FAILED
test fly          ... ok
test fox          ... ok
test frog         ... ok
test horse        ... ok
test owl          ... FAILED
test sheep        ... FAILED

failures:

---- bear ----
no honey

---- bunny ----
jumped too high

---- custom_error ----
I failed

---- dog ----
was not a good boy

---- owl ----
broke neck

---- sheep ----
got lost blindly following the flock


failures:
    bear
    bunny
    custom_error
    dog
    owl
    sheep

test result: FAILED. 5 passed; 6 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 11 tests
...

failures:

---- bear ----
no honey

---- bunny ----
jumped too high

---- custom_error ----
I failed

---- dog ----
was not a good boy

---- owl ----
broke neck

---- sheep ----
got lost blindly following the flock


failures:
    bear
    bunny
    custom_error
    dog
    owl
    sheep

test result: FAILED. 5 passed; 6 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn normal_ignored() {
    check(
        &["--ignored"],
        101,
        str![[r#"

running 11 tests
test bear         ... FAILED
test bunny        ... FAILED
test cat          ... ok
test custom_error ... FAILED
test dog          ... FAILED
test fly          ... ok
test fox          ... ok
test frog         ... ok
test horse        ... ok
test owl          ... FAILED
test sheep        ... FAILED

failures:

---- bear ----
no honey

---- bunny ----
jumped too high

---- custom_error ----
I failed

---- dog ----
was not a good boy

---- owl ----
broke neck

---- sheep ----
got lost blindly following the flock


failures:
    bear
    bunny
    custom_error
    dog
    owl
    sheep

test result: FAILED. 5 passed; 6 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 11 tests
...

failures:

---- bear ----
no honey

---- bunny ----
jumped too high

---- custom_error ----
I failed

---- dog ----
was not a good boy

---- owl ----
broke neck

---- sheep ----
got lost blindly following the flock


failures:
    bear
    bunny
    custom_error
    dog
    owl
    sheep

test result: FAILED. 5 passed; 6 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn lots_of_flags() {
    check(
        &["--ignored", "--skip", "g", "--test", "o"],
        101,
        str![[r#"

running 4 tests
test custom_error ... FAILED
test fox          ... ok
test horse        ... ok
test owl          ... FAILED

failures:

---- custom_error ----
I failed

---- owl ----
broke neck


failures:
    custom_error
    owl

test result: FAILED. 2 passed; 2 failed; 0 ignored; 7 filtered out; finished in [..]s


"#]],
        str![[r#"

running 4 tests
...

failures:

---- custom_error ----
I failed

---- owl ----
broke neck


failures:
    custom_error
    owl

test result: FAILED. 2 passed; 2 failed; 0 ignored; 7 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn list_json() {
    check(
        &["-Zunstable-options", "--format=json", "--list", "a"],
        0,
        str![[r#"
[
  {
    "elapsed_s": "[..]",
    "event": "discover_start"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "bunny",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "custom_error",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "dog",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "fly",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "fox",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "frog",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "horse",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "owl",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "sheep",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_complete"
  }
]
"#]]
        .is_json()
        .against_jsonlines(),
        str![[r#"
[
  {
    "elapsed_s": "[..]",
    "event": "discover_start"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "bunny",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "custom_error",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "dog",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "fly",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "fox",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "frog",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "horse",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "owl",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "sheep",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_complete"
  }
]
"#]]
        .is_json()
        .against_jsonlines(),
    );
}

#[test]
fn test_json() {
    check(
        &["-Zunstable-options", "--format=json", "a"],
        0,
        str![[r#"
[
  {
    "elapsed_s": "[..]",
    "event": "discover_start"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "bunny",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "custom_error",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "dog",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "fly",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "fox",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "frog",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "horse",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "owl",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "sheep",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_complete"
  },
  {
    "elapsed_s": "[..]",
    "event": "run_start"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_start",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_message",
    "kind": "ignored",
    "message": "fails",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_complete",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_start",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_complete",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "run_complete"
  }
]
"#]]
        .is_json()
        .against_jsonlines(),
        str![[r#"
[
  {
    "elapsed_s": "[..]",
    "event": "discover_complete"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_complete",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_start"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_start",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_start",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "run_start"
  },
  {
    "elapsed_s": "[..]",
    "event": "run_complete"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "bunny",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "dog",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "fly",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "fox",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "frog",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "owl",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "case_complete",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_message",
    "kind": "ignored",
    "message": "fails",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "horse",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "sheep",
    "selected": false
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "custom_error",
    "selected": false
  }
]
"#]]
        .unordered()
        .is_json()
        .against_jsonlines(),
    );
}

#[test]
fn terse_output() {
    check(
        &["--quiet"],
        101,
        str![[r#"

running 11 tests
ii.FFi.iiii
failures:

---- custom_error ----
I failed

---- dog ----
was not a good boy


failures:
    custom_error
    dog

test result: FAILED. 2 passed; 2 failed; 7 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 11 tests
...
failures:

---- custom_error ----
I failed

---- dog ----
was not a good boy


failures:
    custom_error
    dog

test result: FAILED. 2 passed; 2 failed; 7 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn fail_fast() {
    check(
        &["--fail-fast"],
        101,
        str![[r#"

running 11 tests
test bear         ... ignored
test bunny        ... ignored
test cat          ... ok
test custom_error ... FAILED

failures:

---- custom_error ----
I failed


failures:
    custom_error

test result: FAILED. 1 passed; 1 failed; 2 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"
...

"#]],
    );
}

#[test]
fn fail_fast_json() {
    check(
        &["-Zunstable-options", "--format=json", "--fail-fast"],
        101,
        str![[r#"
[
  {
    "elapsed_s": "[..]",
    "event": "discover_start"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "bunny"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "custom_error"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "dog"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "fly"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "fox"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "frog"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "horse"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "owl"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_case",
    "name": "sheep"
  },
  {
    "elapsed_s": "[..]",
    "event": "discover_complete"
  },
  {
    "elapsed_s": "[..]",
    "event": "run_start"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_start",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_message",
    "kind": "ignored",
    "message": "fails",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_complete",
    "name": "bear"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_start",
    "name": "bunny"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_message",
    "kind": "ignored",
    "message": "fails",
    "name": "bunny"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_complete",
    "name": "bunny"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_start",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_complete",
    "name": "cat"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_start",
    "name": "custom_error"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_message",
    "kind": "error",
    "message": "I failed",
    "name": "custom_error"
  },
  {
    "elapsed_s": "[..]",
    "event": "case_complete",
    "name": "custom_error"
  },
  {
    "elapsed_s": "[..]",
    "event": "run_complete"
  }
]
"#]]
        .is_json()
        .against_jsonlines(),
        str![[r#"
...

"#]],
    );
}
