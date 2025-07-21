#![cfg(feature = "serde")]
#![cfg(feature = "json")]

use snapbox::prelude::*;
use snapbox::str;

#[track_caller]
fn t(input: libtest_json::Event, snapshot: impl IntoData) {
    let actual_encoded = input.to_jsonline();
    let expected_encoded = serde_json::to_string(&input).unwrap();
    snapbox::assert_data_eq!(&actual_encoded, expected_encoded.raw());
    snapbox::assert_data_eq!(&actual_encoded, snapshot.raw());

    let _ = serde_json::from_str::<libtest_json::Event>(&actual_encoded).unwrap();
}

#[test]
fn discover_start() {
    t(
        libtest_json::Event::DiscoverStart,
        str![[r#"{"event":"discover-start"}"#]],
    );
}

#[test]
fn discover_case() {
    t(
        libtest_json::Event::DiscoverCase {
            name: "Hello\tworld!".to_owned(),
            mode: libtest_json::RunMode::Test,
            run: true,
        },
        str![[r#"{"event":"discover-case","name":"Hello\tworld!"}"#]],
    );

    t(
        libtest_json::Event::DiscoverCase {
            name: "Hello\tworld!".to_owned(),
            mode: libtest_json::RunMode::Bench,
            run: false,
        },
        str![[r#"{"event":"discover-case","name":"Hello\tworld!","mode":"bench","run":false}"#]],
    );
}

#[test]
fn discover_complete() {
    t(
        libtest_json::Event::DiscoverComplete { elapsed_s: None },
        str![[r#"{"event":"discover-complete","elapsed_s":null}"#]],
    );

    t(
        libtest_json::Event::DiscoverComplete {
            elapsed_s: Some(libtest_json::Elapsed(Default::default())),
        },
        str![[r#"{"event":"discover-complete","elapsed_s":"0"}"#]],
    );
}

#[test]
fn suite_start() {
    t(
        libtest_json::Event::SuiteStart,
        str![[r#"{"event":"suite-start"}"#]],
    );
}

#[test]
fn case_start() {
    t(
        libtest_json::Event::CaseStart {
            name: "Hello\tworld!".to_owned(),
        },
        str![[r#"{"event":"case-start","name":"Hello\tworld!"}"#]],
    );
}

#[test]
fn case_complete() {
    t(
        libtest_json::Event::CaseComplete {
            name: "Hello\tworld!".to_owned(),
            mode: libtest_json::RunMode::Test,
            status: None,
            message: None,
            elapsed_s: None,
        },
        str![[r#"{"event":"case-complete","name":"Hello\tworld!","elapsed_s":null}"#]],
    );

    t(
        libtest_json::Event::CaseComplete {
            name: "Hello\tworld!".to_owned(),
            mode: libtest_json::RunMode::Bench,
            status: Some(libtest_json::RunStatus::Ignored),
            message: Some("This\tfailed".to_owned()),
            elapsed_s: Some(libtest_json::Elapsed(Default::default())),
        },
        str![[
            r#"{"event":"case-complete","name":"Hello\tworld!","mode":"bench","status":"ignored","message":"This\tfailed","elapsed_s":"0"}"#
        ]],
    );
}

#[test]
fn suite_complete() {
    t(
        libtest_json::Event::SuiteComplete { elapsed_s: None },
        str![[r#"{"event":"suite-complete","elapsed_s":null}"#]],
    );

    t(
        libtest_json::Event::SuiteComplete {
            elapsed_s: Some(libtest_json::Elapsed(Default::default())),
        },
        str![[r#"{"event":"suite-complete","elapsed_s":"0"}"#]],
    );
}
