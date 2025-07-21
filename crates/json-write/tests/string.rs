#![cfg(feature = "alloc")]
#![allow(clippy::dbg_macro)] // unsure why config isn't working

use snapbox::prelude::*;
use snapbox::str;

use json_write::ToJsonKey;
use json_write::ToJsonValue;

#[track_caller]
fn t(decoded: &str, expected: impl IntoData) {
    let key = decoded.to_json_key();
    let string = decoded.to_json_value();
    let object = format!("{{ {key}: {string} }}");
    let parsed = format!("{:?}", object.parse::<serde_json::Value>());
    let results = Results {
        decoded,
        key,
        string,
        parsed,
    };
    snapbox::assert_data_eq!(results.to_debug(), expected.raw());
}

#[derive(Debug)]
#[allow(dead_code)]
struct Results<'i> {
    decoded: &'i str,
    key: String,
    string: String,
    parsed: String,
}

#[test]
fn empty() {
    t(
        "",
        str![[r#"
Results {
    decoded: "",
    key: "\"\"",
    string: "\"\"",
    parsed: "Ok(Object {\"\": String(\"\")})",
}

"#]],
    );
}

#[test]
fn alpha() {
    t(
        "helloworld",
        str![[r#"
Results {
    decoded: "helloworld",
    key: "\"helloworld\"",
    string: "\"helloworld\"",
    parsed: "Ok(Object {\"helloworld\": String(\"helloworld\")})",
}

"#]],
    );
}

#[test]
fn ident() {
    t(
        "_hello-world_",
        str![[r#"
Results {
    decoded: "_hello-world_",
    key: "\"_hello-world_\"",
    string: "\"_hello-world_\"",
    parsed: "Ok(Object {\"_hello-world_\": String(\"_hello-world_\")})",
}

"#]],
    );
}

#[test]
fn one_single_quote() {
    t(
        "'hello'world'",
        str![[r#"
Results {
    decoded: "'hello'world'",
    key: "\"'hello'world'\"",
    string: "\"'hello'world'\"",
    parsed: "Ok(Object {\"'hello'world'\": String(\"'hello'world'\")})",
}

"#]],
    );
}

#[test]
fn two_single_quote() {
    t(
        "''hello''world''",
        str![[r#"
Results {
    decoded: "''hello''world''",
    key: "\"''hello''world''\"",
    string: "\"''hello''world''\"",
    parsed: "Ok(Object {\"''hello''world''\": String(\"''hello''world''\")})",
}

"#]],
    );
}

#[test]
fn three_single_quote() {
    t(
        "'''hello'''world'''",
        str![[r#"
Results {
    decoded: "'''hello'''world'''",
    key: "\"'''hello'''world'''\"",
    string: "\"'''hello'''world'''\"",
    parsed: "Ok(Object {\"'''hello'''world'''\": String(\"'''hello'''world'''\")})",
}

"#]],
    );
}

#[test]
fn one_double_quote() {
    t(
        r#""hello"world""#,
        str![[r#"
Results {
    decoded: "\"hello\"world\"",
    key: "\"\\\"hello\\\"world\\\"\"",
    string: "\"\\\"hello\\\"world\\\"\"",
    parsed: "Ok(Object {\"\\\"hello\\\"world\\\"\": String(\"\\\"hello\\\"world\\\"\")})",
}

"#]],
    );
}

#[test]
fn two_double_quote() {
    t(
        r#"""hello""world"""#,
        str![[r#"
Results {
    decoded: "\"\"hello\"\"world\"\"",
    key: "\"\\\"\\\"hello\\\"\\\"world\\\"\\\"\"",
    string: "\"\\\"\\\"hello\\\"\\\"world\\\"\\\"\"",
    parsed: "Ok(Object {\"\\\"\\\"hello\\\"\\\"world\\\"\\\"\": String(\"\\\"\\\"hello\\\"\\\"world\\\"\\\"\")})",
}

"#]],
    );
}

#[test]
fn three_double_quote() {
    t(
        r#""""hello"""world""""#,
        str![[r#"
Results {
    decoded: "\"\"\"hello\"\"\"world\"\"\"",
    key: "\"\\\"\\\"\\\"hello\\\"\\\"\\\"world\\\"\\\"\\\"\"",
    string: "\"\\\"\\\"\\\"hello\\\"\\\"\\\"world\\\"\\\"\\\"\"",
    parsed: "Ok(Object {\"\\\"\\\"\\\"hello\\\"\\\"\\\"world\\\"\\\"\\\"\": String(\"\\\"\\\"\\\"hello\\\"\\\"\\\"world\\\"\\\"\\\"\")})",
}

"#]],
    );
}

#[test]
fn mixed_quote_1() {
    t(
        r#""'"#,
        str![[r#"
Results {
    decoded: "\"'",
    key: "\"\\\"'\"",
    string: "\"\\\"'\"",
    parsed: "Ok(Object {\"\\\"'\": String(\"\\\"'\")})",
}

"#]],
    );
}

#[test]
fn mixed_quote_2() {
    t(
        r#"mixed quoted \"start\" 'end'' mixed quote"#,
        str![[r#"
Results {
    decoded: "mixed quoted \\\"start\\\" 'end'' mixed quote",
    key: "\"mixed quoted \\\\\\\"start\\\\\\\" 'end'' mixed quote\"",
    string: "\"mixed quoted \\\\\\\"start\\\\\\\" 'end'' mixed quote\"",
    parsed: "Ok(Object {\"mixed quoted \\\\\\\"start\\\\\\\" 'end'' mixed quote\": String(\"mixed quoted \\\\\\\"start\\\\\\\" 'end'' mixed quote\")})",
}

"#]],
    );
}

#[test]
fn escape() {
    t(
        r#"\windows\system32\"#,
        str![[r#"
Results {
    decoded: "\\windows\\system32\\",
    key: "\"\\\\windows\\\\system32\\\\\"",
    string: "\"\\\\windows\\\\system32\\\\\"",
    parsed: "Ok(Object {\"\\\\windows\\\\system32\\\\\": String(\"\\\\windows\\\\system32\\\\\")})",
}

"#]],
    );
}

#[test]
fn cr() {
    t(
        "\rhello\rworld\r",
        str![[r#"
Results {
    decoded: "\rhello\rworld\r",
    key: "\"\\rhello\\rworld\\r\"",
    string: "\"\\rhello\\rworld\\r\"",
    parsed: "Ok(Object {\"\\rhello\\rworld\\r\": String(\"\\rhello\\rworld\\r\")})",
}

"#]],
    );
}

#[test]
fn lf() {
    t(
        "\nhello\nworld\n",
        str![[r#"
Results {
    decoded: "\nhello\nworld\n",
    key: "\"\\nhello\\nworld\\n\"",
    string: "\"\\nhello\\nworld\\n\"",
    parsed: "Ok(Object {\"\\nhello\\nworld\\n\": String(\"\\nhello\\nworld\\n\")})",
}

"#]],
    );
}

#[test]
fn crlf() {
    t(
        "\r\nhello\r\nworld\r\n",
        str![[r#"
Results {
    decoded: "\r\nhello\r\nworld\r\n",
    key: "\"\\r\\nhello\\r\\nworld\\r\\n\"",
    string: "\"\\r\\nhello\\r\\nworld\\r\\n\"",
    parsed: "Ok(Object {\"\\r\\nhello\\r\\nworld\\r\\n\": String(\"\\r\\nhello\\r\\nworld\\r\\n\")})",
}

"#]],
    );
}

#[test]
fn tab() {
    t(
        "\thello\tworld\t",
        str![[r#"
Results {
    decoded: "\thello\tworld\t",
    key: "\"\\thello\\tworld\\t\"",
    string: "\"\\thello\\tworld\\t\"",
    parsed: "Ok(Object {\"\\thello\\tworld\\t\": String(\"\\thello\\tworld\\t\")})",
}

"#]],
    );
}
