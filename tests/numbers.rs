use serde_json::json;
use serde_json_schema::*;
use std::convert::TryFrom;

#[test]
fn number_spec() {
    let raw_schema: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Product",
  "description": "A product from Acme's catalog",
  "type": "number"
}"#;

    let schema = Schema::try_from(raw_schema).unwrap();
    println!("{:#?}", schema);

    schema.validate(&json!(-4.1416)).unwrap();
    schema.validate(&json!(4.1416)).unwrap();
}

#[test]
#[should_panic]
fn number_spec_negative() {
    let raw_schema: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Product",
  "description": "A product from Acme's catalog",
  "type": "number"
}"#;

    let schema = Schema::try_from(raw_schema).unwrap();
    println!("{:#?}", schema);

    schema.validate(&json!("4.1416")).unwrap();
}

#[test]
fn number_types() {
    let raw_schema: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "http://example.com/product.schema.json",
  "title": "Product",
  "description": "A product from Acme's catalog",
  "type": "object",
  "properties": {
    "integer": {
      "description": "integer",
      "type": "integer"
    },
    "number": {
      "description": "number",
      "type": "number"
    }
  }
}"#;

    let schema = Schema::try_from(raw_schema).unwrap();
    println!("{:#?}", schema);

    schema.validate(&json!({ "integer": -42 })).unwrap();
    schema.validate(&json!({ "integer": 42 })).unwrap();
    schema.validate(&json!({ "number": -4.1416 })).unwrap();
    schema.validate(&json!({ "number": 4.1416 })).unwrap();
}
