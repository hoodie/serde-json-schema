/// validating the "basics" from https://json-schema.org/understanding-json-schema/basics.html
use serde_json::json;
use serde_json_schema::*;
use std::convert::TryFrom;

#[test]
fn basics() {
    let json_42 = json!(42);
    let json_string = json!("I'm a string");
    let json_object = json!({ "an": [ "arbitrarily", "nested" ], "data": "structure" });

    let empty_schema = dbg!(Schema::try_from("{}").unwrap());
    empty_schema.validate(&json_42).unwrap();
    empty_schema.validate(&json_string).unwrap();
    empty_schema.validate(&json_object).unwrap();

    let true_schema = dbg!(Schema::try_from("true").unwrap());
    true_schema.validate(&json_42).unwrap();
    true_schema.validate(&json_string).unwrap();
    true_schema.validate(&json_object).unwrap();

    let false_schema = dbg!(Schema::try_from("false").unwrap());
    assert!(false_schema.validate(&json_42).is_err());
    assert!(false_schema.validate(&json_string).is_err());
    assert!(false_schema.validate(&json_object).is_err());
}
