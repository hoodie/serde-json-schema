use serde_json_schema::*;

#[test]
fn validate_wrong_numbers() {
    let schema: Schema =
        serde_json::from_str(include_str!("./fixtures/green_door.schema.json")).unwrap();
    let json_green_door: serde_json::Value = serde_json::from_str(include_str!(
        "./fixtures/green_door.wrong_number_types.json"
    ))
    .unwrap();
    println!("{:#?}", schema);
    assert_eq!(
        schema.validate(&json_green_door),
        Err(vec![String::from("expected integer found Number(1.2)")])
    );
}

#[test]
fn validate_pure_string_object() {
    let raw_schema = include_str!("./fixtures/address.schema.json");
    let schema: Schema = serde_json::from_str(raw_schema).unwrap();

    let json_correct: serde_json::Value =
        serde_json::from_str(include_str!("./fixtures/address.json")).unwrap();
    schema.validate(&json_correct).unwrap();
}

#[test]
fn validate_root_array() {
    let schema: Schema =
        serde_json::from_str(include_str!("./fixtures/root_array.schema.json")).unwrap();

    let json_array: serde_json::Value =
        serde_json::from_str(include_str!("./fixtures/root_array.json")).unwrap();
    schema.validate(&json_array).unwrap();
}

#[test]
fn validate_find_wrong_type() {
    let raw_schema = include_str!("./fixtures/address.schema.json");
    let schema: Schema = serde_json::from_str(raw_schema).unwrap();

    let json_wrong_type: serde_json::Value =
        serde_json::from_str(include_str!("./fixtures/address.wrong-type.json")).unwrap();
    // TODO: make more concrete error type
    assert!(schema.validate(&json_wrong_type).is_err());
}

#[test]
fn validate_find_missing_required() {
    let raw_schema = include_str!("./fixtures/address.schema.json");
    let schema: Schema = serde_json::from_str(raw_schema).unwrap();

    let json_missing: serde_json::Value =
        serde_json::from_str(include_str!("./fixtures/address.missing.json")).unwrap();
    // TODO: make more concrete error type
    assert!(schema.validate(&json_missing).is_err());
}

#[test]
fn validate_find_missing() {
    let raw_schema = include_str!("./fixtures/address.schema.json");
    let schema: Schema = serde_json::from_str(raw_schema).unwrap();

    let json_missing: serde_json::Value =
        serde_json::from_str(include_str!("./fixtures/address.missing-non-required.json")).unwrap();
    // TODO: make more concrete error type
    schema.validate(&json_missing).unwrap();
}
