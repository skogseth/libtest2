#![cfg(feature = "alloc")]

use snapbox::prelude::*;
use snapbox::str;

use json_write::ToJsonKey;
use json_write::ToJsonValue;

#[track_caller]
fn t<N: ToJsonValue + core::fmt::Debug>(value: N, expected: impl IntoData) {
    let key = "key".to_json_key();
    let string = value.to_json_value();
    let object = format!("{{ {key}: {string} }}");
    let parsed = format!("{:?}", object.parse::<serde_json::Value>());
    let results = Results {
        value,
        string,
        parsed,
    };
    snapbox::assert_data_eq!(results.to_debug(), expected.raw());
}

#[derive(Debug)]
#[allow(dead_code)]
struct Results<N: core::fmt::Debug> {
    value: N,
    string: String,
    parsed: String,
}

#[test]
fn positive() {
    t(
        42,
        str![[r#"
Results {
    value: 42,
    string: "42",
    parsed: "Ok(Object {\"key\": Number(42)})",
}

"#]],
    );
}

#[test]
fn negative() {
    t(
        -42,
        str![[r#"
Results {
    value: -42,
    string: "-42",
    parsed: "Ok(Object {\"key\": Number(-42)})",
}

"#]],
    );
}
