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
        libtest_json::Event::DiscoverStart { elapsed_s: None },
        str![[r#"{"event":"discover_start"}"#]],
    );
    t(
        libtest_json::Event::DiscoverStart {
            elapsed_s: Some(libtest_json::Elapsed(Default::default())),
        },
        str![[r#"{"event":"discover_start","elapsed_s":"0"}"#]],
    );
}

#[test]
fn discover_case() {
    t(
        libtest_json::Event::DiscoverCase {
            name: "Hello\tworld!".to_owned(),
            mode: libtest_json::RunMode::Test,
            run: true,
            elapsed_s: None,
        },
        str![[r#"{"event":"discover_case","name":"Hello\tworld!"}"#]],
    );

    t(
        libtest_json::Event::DiscoverCase {
            name: "Hello\tworld!".to_owned(),
            mode: libtest_json::RunMode::Bench,
            run: false,
            elapsed_s: Some(libtest_json::Elapsed(Default::default())),
        },
        str![[
            r#"{"event":"discover_case","name":"Hello\tworld!","mode":"bench","run":false,"elapsed_s":"0"}"#
        ]],
    );
}

#[test]
fn discover_complete() {
    t(
        libtest_json::Event::DiscoverComplete { elapsed_s: None },
        str![[r#"{"event":"discover_complete"}"#]],
    );

    t(
        libtest_json::Event::DiscoverComplete {
            elapsed_s: Some(libtest_json::Elapsed(Default::default())),
        },
        str![[r#"{"event":"discover_complete","elapsed_s":"0"}"#]],
    );
}

#[test]
fn suite_start() {
    t(
        libtest_json::Event::RunStart { elapsed_s: None },
        str![[r#"{"event":"run_start"}"#]],
    );
    t(
        libtest_json::Event::RunStart {
            elapsed_s: Some(libtest_json::Elapsed(Default::default())),
        },
        str![[r#"{"event":"run_start","elapsed_s":"0"}"#]],
    );
}

#[test]
fn case_start() {
    t(
        libtest_json::Event::CaseStart {
            name: "Hello\tworld!".to_owned(),
            elapsed_s: None,
        },
        str![[r#"{"event":"case_start","name":"Hello\tworld!"}"#]],
    );
    t(
        libtest_json::Event::CaseStart {
            name: "Hello\tworld!".to_owned(),
            elapsed_s: Some(libtest_json::Elapsed(Default::default())),
        },
        str![[r#"{"event":"case_start","name":"Hello\tworld!","elapsed_s":"0"}"#]],
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
        str![[r#"{"event":"case_complete","name":"Hello\tworld!"}"#]],
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
            r#"{"event":"case_complete","name":"Hello\tworld!","mode":"bench","status":"ignored","message":"This\tfailed","elapsed_s":"0"}"#
        ]],
    );
}

#[test]
fn suite_complete() {
    t(
        libtest_json::Event::RunComplete { elapsed_s: None },
        str![[r#"{"event":"run_complete"}"#]],
    );

    t(
        libtest_json::Event::RunComplete {
            elapsed_s: Some(libtest_json::Elapsed(Default::default())),
        },
        str![[r#"{"event":"run_complete","elapsed_s":"0"}"#]],
    );
}
