use serde_json_schema::*;
use std::convert::TryFrom;

/// https://json-schema.org/latest/json-schema-core.html#rfc.section.8.1
#[test]
#[should_panic]
fn schema_must_be_a_url() {
    let raw_schema = r#"{
  "$schema": "not a uri",
  "description": "A product from Acme's catalog",
  "type": "object",
  "properties": {
    "productId": {
      "description": "The unique identifier for a product",
      "type": "integer"
    }
  },
  "required": [ "productId" ]
}"#;
    Schema::try_from(raw_schema).unwrap();
}

/// https://json-schema.org/latest/json-schema-core.html#rfc.section.8.2
#[test]
#[should_panic]
fn id_must_be_a_url() {
    let raw_schema = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "not a uri",
  "description": "A product from Acme's catalog",
  "type": "object",
  "properties": {
    "productId": {
      "description": "The unique identifier for a product",
      "type": "integer"
    }
  },
  "required": [ "productId" ]
}"#;
    Schema::try_from(raw_schema).unwrap();
}

/// https://json-schema.org/latest/json-schema-core.html#rfc.section.8.2
#[test]
fn id_is_optional() {
    let raw_schema = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "description": "A product from Acme's catalog",
  "type": "object",
  "properties": {
    "productId": {
      "description": "The unique identifier for a product",
      "type": "integer"
    }
  },
  "required": [ "productId" ]
}"#;

    let schema = Schema::try_from(raw_schema).unwrap();
    println!("{:#?}", schema);
    assert!(schema.id().is_none());
}

/// https://json-schema.org/latest/json-schema-core.html#rfc.section.8.2.4
///
/// TODO: definitions are not yet implemented
#[test]
#[ignore]
fn subschema() {
    let raw_schema = r##"{
    "$id": "http://example.com/root.json",
    "definitions": {
        "A": { "$id": "#foo" },
        "B": {
            "$id": "other.json",
            "definitions": {
                "X": { "$id": "#bar" },
                "Y": { "$id": "t/inner.json" }
            }
        },
        "C": {
            "$id": "urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f"
        }
    }
}"##;

    let schema = Schema::try_from(raw_schema).unwrap();
    println!("{:#?}", schema);
}

/// https://json-schema.org/latest/json-schema-core.html#rfc.section.8.2.4
///
/// TODO: definitions are not yet implemented
#[test]
#[ignore]
fn subschema_no_ids() {
    let raw_schema = r##"{
    "definitions": {
        "A": {},
        "B": {
            "definitions": {
            }
        },
        "C": {
        }
    }
}"##;

    let schema = Schema::try_from(raw_schema).unwrap();
    println!("{:#?}", schema);
}
