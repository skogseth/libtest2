use snapbox::prelude::*;
use snapbox::str;

fn test_cmd() -> snapbox::cmd::Command {
    static BIN: once_cell_polyfill::sync::OnceLock<(std::path::PathBuf, std::path::PathBuf)> =
        once_cell_polyfill::sync::OnceLock::new();
    let (bin, current_dir) = BIN.get_or_init(|| {
        let package_root = crate::util::new_test(
            r#"
libtest2::libtest2_main!(cat, dog, fox, bunny, frog, owl, fly, bear);

fn cat(_state: &libtest2::State) -> libtest2::RunResult {
    Ok(())
}

fn dog(_state: &libtest2::State) -> libtest2::RunResult {
    Err(libtest2::RunError::fail("was not a good boy"))
}

fn fox(_state: &libtest2::State) -> libtest2::RunResult {
    Ok(())
}

fn bunny(state: &libtest2::State) -> libtest2::RunResult {
    state.ignore_for("fails")?;
    Err(libtest2::RunError::fail("jumped too high"))
}

fn frog(state: &libtest2::State) -> libtest2::RunResult {
    state.ignore_for("slow")?;
    Ok(())
}

fn owl(state: &libtest2::State) -> libtest2::RunResult {
    state.ignore_for("fails")?;
    Err(libtest2::RunError::fail("broke neck"))
}

fn fly(state: &libtest2::State) -> libtest2::RunResult {
    state.ignore_for("fails")?;
    Ok(())
}

fn bear(state: &libtest2::State) -> libtest2::RunResult {
    state.ignore_for("fails")?;
    Err(libtest2::RunError::fail("no honey"))
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

running 8 tests
test bear  ... ignored
test bunny ... ignored
test cat   ... ok
test dog   ... FAILED
test fly   ... ignored
test fox   ... ok
test frog  ... ignored
test owl   ... ignored

failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 8 tests
...

failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn test_mode() {
    check(
        &["--test"],
        101,
        str![[r#"

running 8 tests
test bear  ... ignored
test bunny ... ignored
test cat   ... ok
test dog   ... FAILED
test fly   ... ignored
test fox   ... ok
test frog  ... ignored
test owl   ... ignored

failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 8 tests
...

failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn bench_mode() {
    check(
        &["--bench"],
        101,
        str![[r#"

running 8 tests
test bear  ... ignored
test bunny ... ignored
test cat   ... ok
test dog   ... FAILED
test fly   ... ignored
test fox   ... ok
test frog  ... ignored
test owl   ... ignored

failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 8 tests
...

failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


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
dog: test
fly: test
fox: test
frog: test
owl: test

8 tests


"#]],
        str![[r#"
bear: test
bunny: test
cat: test
dog: test
fly: test
fox: test
frog: test
owl: test

8 tests


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
dog: test
fly: test
fox: test
frog: test
owl: test

8 tests


"#]],
        str![[r#"
bear: test
bunny: test
cat: test
dog: test
fly: test
fox: test
frog: test
owl: test

8 tests


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
owl: test
fox: test
bunny: test
frog: test

4 tests


"#]],
        str![[r#"
owl: test
fox: test
bunny: test
frog: test

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

test result: ok. 1 passed; 0 failed; 1 ignored; 6 filtered out; finished in [..]s


"#]],
        str![[r#"

running 2 tests
...

test result: ok. 1 passed; 0 failed; 1 ignored; 6 filtered out; finished in [..]s


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

test result: ok. 1 passed; 0 failed; 1 ignored; 6 filtered out; finished in [..]s


"#]],
        str![[r#"

running 2 tests
...

test result: ok. 1 passed; 0 failed; 1 ignored; 6 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn filter_o_test_include_ignored() {
    check(
        &["--test", "--include-ignored", "o"],
        101,
        str![[r#"

running 4 tests
test dog  ... FAILED
test fox  ... ok
test frog ... ok
test owl  ... FAILED

failures:

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    dog
    owl

test result: FAILED. 2 passed; 2 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
        str![[r#"

running 4 tests
...

failures:

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    dog
    owl

test result: FAILED. 2 passed; 2 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn filter_o_test_ignored() {
    check(
        &["--test", "--ignored", "o"],
        101,
        str![[r#"

running 4 tests
test dog  ... FAILED
test fox  ... ok
test frog ... ok
test owl  ... FAILED

failures:

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    dog
    owl

test result: FAILED. 2 passed; 2 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
        str![[r#"

running 4 tests
...

failures:

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    dog
    owl

test result: FAILED. 2 passed; 2 failed; 0 ignored; 4 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn normal_include_ignored() {
    check(
        &["--include-ignored"],
        101,
        str![[r#"

running 8 tests
test bear  ... FAILED
test bunny ... FAILED
test cat   ... ok
test dog   ... FAILED
test fly   ... ok
test fox   ... ok
test frog  ... ok
test owl   ... FAILED

failures:

---- bear ----
no honey

---- bunny ----
jumped too high

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    bear
    bunny
    dog
    owl

test result: FAILED. 4 passed; 4 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 8 tests
...

failures:

---- bear ----
no honey

---- bunny ----
jumped too high

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    bear
    bunny
    dog
    owl

test result: FAILED. 4 passed; 4 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn normal_ignored() {
    check(
        &["--ignored"],
        101,
        str![[r#"

running 8 tests
test bear  ... FAILED
test bunny ... FAILED
test cat   ... ok
test dog   ... FAILED
test fly   ... ok
test fox   ... ok
test frog  ... ok
test owl   ... FAILED

failures:

---- bear ----
no honey

---- bunny ----
jumped too high

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    bear
    bunny
    dog
    owl

test result: FAILED. 4 passed; 4 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 8 tests
...

failures:

---- bear ----
no honey

---- bunny ----
jumped too high

---- dog ----
was not a good boy

---- owl ----
broke neck


failures:
    bear
    bunny
    dog
    owl

test result: FAILED. 4 passed; 4 failed; 0 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn lots_of_flags() {
    check(
        &["--ignored", "--skip", "g", "--test", "o"],
        101,
        str![[r#"

running 2 tests
test fox ... ok
test owl ... FAILED

failures:

---- owl ----
broke neck


failures:
    owl

test result: FAILED. 1 passed; 1 failed; 0 ignored; 6 filtered out; finished in [..]s


"#]],
        str![[r#"

running 2 tests
...

failures:

---- owl ----
broke neck


failures:
    owl

test result: FAILED. 1 passed; 1 failed; 0 ignored; 6 filtered out; finished in [..]s


"#]],
    );
}

#[test]
#[cfg(feature = "json")]
fn list_json() {
    check(
        &["-Zunstable-options", "--format=json", "--list", "a"],
        0,
        str![[r#"
[
  {
    "event": "discover_start",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "bunny",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "dog",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "fly",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "fox",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "frog",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "owl",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "bear",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "cat",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_complete",
    "elapsed_s": "[..]"
  }
]
"#]]
        .is_json()
        .against_jsonlines(),
        str![[r#"
[
  {
    "event": "discover_start",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "bunny",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "dog",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "fly",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "fox",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "frog",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "owl",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "bear",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "cat",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_complete",
    "elapsed_s": "[..]"
  }
]
"#]]
        .is_json()
        .against_jsonlines(),
    );
}

#[test]
#[cfg(feature = "json")]
fn test_json() {
    check(
        &["-Zunstable-options", "--format=json", "a"],
        0,
        str![[r#"
[
  {
    "event": "discover_start",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "bunny",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "dog",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "fly",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "fox",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "frog",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "owl",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "bear",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "cat",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_complete",
    "elapsed_s": "[..]"
  },
  {
    "event": "run_start",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_start",
    "name": "bear",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_message",
    "name": "bear",
    "status": "ignored",
    "message": "fails",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_complete",
    "name": "bear",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_start",
    "name": "cat",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_complete",
    "name": "cat",
    "elapsed_s": "[..]"
  },
  {
    "event": "run_complete",
    "elapsed_s": "[..]"
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
    "event": "run_start",
    "elapsed_s": "[..]"
  },
  {
    "event": "run_complete",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "bunny",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "dog",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "fly",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "fox",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "frog",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "owl",
    "selected": false,
    "elapsed_s": "[..]"
  },
  {
    "event": "case_message",
    "name": "bear",
    "status": "ignored",
    "message": "fails",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_complete",
    "name": "bear",
    "elapsed_s": "[..]"
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

running 8 tests
ii.Fi.ii
failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"

running 8 tests
...
failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 2 passed; 1 failed; 5 ignored; 0 filtered out; finished in [..]s


"#]],
    );
}

#[test]
fn fail_fast() {
    check(
        &["--fail-fast"],
        101,
        str![[r#"

running 8 tests
test bear  ... ignored
test bunny ... ignored
test cat   ... ok
test dog   ... FAILED

failures:

---- dog ----
was not a good boy


failures:
    dog

test result: FAILED. 1 passed; 1 failed; 2 ignored; 0 filtered out; finished in [..]s


"#]],
        str![[r#"
...

"#]],
    );
}

#[test]
#[cfg(feature = "json")]
fn fail_fast_json() {
    check(
        &["-Zunstable-options", "--format=json", "--fail-fast"],
        101,
        str![[r#"
[
  {
    "event": "discover_start",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "bear",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "bunny",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "cat",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "dog",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "fly",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "fox",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "frog",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_case",
    "name": "owl",
    "elapsed_s": "[..]"
  },
  {
    "event": "discover_complete",
    "elapsed_s": "[..]"
  },
  {
    "event": "run_start",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_start",
    "name": "bear",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_message",
    "name": "bear",
    "status": "ignored",
    "message": "fails",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_complete",
    "name": "bear",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_start",
    "name": "bunny",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_message",
    "name": "bunny",
    "status": "ignored",
    "message": "fails",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_complete",
    "name": "bunny",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_start",
    "name": "cat",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_complete",
    "name": "cat",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_start",
    "name": "dog",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_message",
    "name": "dog",
    "status": "failed",
    "message": "was not a good boy",
    "elapsed_s": "[..]"
  },
  {
    "event": "case_complete",
    "name": "dog",
    "elapsed_s": "[..]"
  },
  {
    "event": "run_complete",
    "elapsed_s": "[..]"
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
