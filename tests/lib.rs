mod basics {
    /// validating the "basics" from https://json-schema.org/understanding-json-schema/basics.html
    use serde_json::json;
    use serde_json_schema::*;

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
}

mod numbers {
    use serde_json::json;
    use serde_json_schema::*;

    #[test]
    fn number_spec() {
        let raw_schema: &str = r#"{
    "$schema": "https://json-schema.org/draft/2020-12/schema",
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
    "$schema": "https://json-schema.org/draft/2020-12/schema",
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
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "https://example.com/product.schema.json",
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
}

mod examples {
    use serde_json_schema::*;

    #[test]
    fn address_example() {
        let raw = include_str!("./fixtures/address.schema.json");
        let schema: Schema = serde_json::from_str(raw).unwrap();
        println!("{:#?}", schema);
    }

    #[test]
    fn from_json_value() {
        let raw = include_str!("./fixtures/address.schema.json");
        let value: serde_json::Value = serde_json::from_str(raw).unwrap();
        let schema: Schema = serde_json::from_str(raw).unwrap();

        let schema2 = Schema::try_from(value).unwrap();

        assert_eq!(schema, schema2);
        println!("{:#?}", schema);
    }

    #[test]
    fn calendar_example() {
        let raw = include_str!("./fixtures/calendar.schema.json");
        let schema: Schema = serde_json::from_str(raw).unwrap();
        println!("{:#?}", schema);
    }

    #[test]
    fn card_example() {
        let raw = include_str!("./fixtures/card.schema.json");
        let schema: Schema = serde_json::from_str(raw).unwrap();
        println!("{:#?}", schema);
    }

    #[test]
    fn draft_version() {
        let schema: Schema =
            serde_json::from_str(include_str!("./fixtures/card.schema.json")).unwrap();
        println!("{:#?}", schema.draft_version());
        assert_eq!(schema.draft_version(), Some("draft"))
    }

    #[test]
    fn green_door_example() {
        let schema: Schema =
            serde_json::from_str(include_str!("./fixtures/green_door.schema.json")).unwrap();
        let json_green_door: serde_json::Value =
            serde_json::from_str(include_str!("./fixtures/green_door.json")).unwrap();
        println!("{:#?}", schema);
        schema.validate(&json_green_door).unwrap();
    }
}

mod spec {
    use serde_json_schema::*;

    #[test]
    fn required_not_required() {
        let raw_schema = r#"{
  "$id": "https://example.com/address.schema.json",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "description": "An address similar to http://microformats.org/wiki/h-card",
  "type": "object",
  "properties": {
  },
  "dependentRequired": {
    "post-office-box": [ "street-address" ],
    "extended-address": [ "street-address" ]
  }

    }"#;
        let schema = Schema::try_from(raw_schema).unwrap();
        assert!(schema.specification().is_some())
    }

    /// https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.1
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

    /// https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.2
    #[test]
    #[should_panic]
    fn id_must_not_contain_spaces() {
        let raw_schema = r#"{
    "$schema": "https://json-schema.org/draft/2020-12/schema",
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
        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.2
    #[test]
    fn id_may_be_a_uuid() {
        let raw_schema = r#"{
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f"
    }"#;
        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.2
    #[test]
    fn id_may_be_a_fragment() {
        let raw_schema = r##"{
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "#foo"
    }"##;
        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.2
    #[test]
    fn id_may_be_a_path() {
        let raw_schema = r#"{
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "t/inner.json"
    }"#;
        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.2
    #[test]
    fn id_is_optional() {
        let raw_schema = r#"{
    "$schema": "https://json-schema.org/draft/2020-12/schema",
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

    /// https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.2.4
    ///
    /// JSON Schema 2020-12 uses $defs instead of definitions
    #[test]
    fn subschema() {
        let raw_schema = r##"{
        "$id": "https://example.com/root.json",
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$defs": {
            "A": { "$id": "#foo" },
            "B": {
                "$id": "other.json",
                "$defs": {
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

    /// https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.2.4
    ///
    /// JSON Schema 2020-12 uses $defs instead of definitions
    #[test]
    #[ignore]
    fn subschema_no_ids() {
        let raw_schema = r#"{
        "$defs": {
            "A": {},
            "B": {
                "$defs": {
                }
            },
            "C": {
            }
        }
    }"#;

        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }
}

mod validation {
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
            serde_json::from_str(include_str!("./fixtures/address.missing-non-required.json"))
                .unwrap();
        // TODO: make more concrete error type
        schema.validate(&json_missing).unwrap();
    }
}

/// Tests for JSON Schema 2020-12 specific features
mod json_schema_2020_12 {
    use serde_json::json;
    use serde_json_schema::*;

    /// Test prefixItems for tuple validation (2020-12 feature)
    #[test]
    fn prefix_items() {
        let raw_schema = r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "prefixItems": [
                { "type": "string" },
                { "type": "integer" }
            ],
            "items": { "type": "boolean" }
        }"#;

        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);

        // Valid: matches prefixItems types
        schema.validate(&json!(["hello", 42])).unwrap();
        // Valid: extra items match items schema
        schema.validate(&json!(["hello", 42, true, false])).unwrap();
    }

    /// Test $anchor keyword (2020-12 feature)
    #[test]
    fn anchor() {
        let raw_schema = r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://example.com/schemas/address",
            "$anchor": "address",
            "type": "object",
            "properties": {
                "street": { "type": "string" }
            }
        }"#;

        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// Test $dynamicAnchor and $dynamicRef (2020-12 feature)
    #[test]
    fn dynamic_anchor() {
        let raw_schema = r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": "https://example.com/schemas/base",
            "$dynamicAnchor": "node",
            "type": "object"
        }"#;

        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// Test $vocabulary (2020-12 meta-schema feature)
    #[test]
    fn vocabulary() {
        let raw_schema = r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$vocabulary": {
                "https://json-schema.org/draft/2020-12/vocab/core": true,
                "https://json-schema.org/draft/2020-12/vocab/applicator": true,
                "https://json-schema.org/draft/2020-12/vocab/validation": true
            }
        }"#;

        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// Test $comment (included in 2020-12)
    #[test]
    fn comment() {
        let raw_schema = r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$comment": "This is a comment explaining the schema",
            "type": "string"
        }"#;

        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// Test $defs (2020-12 replacement for definitions)
    #[test]
    fn defs() {
        let raw_schema = r##"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$defs": {
                "positiveInteger": {
                    "type": "integer",
                    "exclusiveMinimum": 0
                }
            },
            "type": "object",
            "properties": {
                "count": { "$ref": "#/$defs/positiveInteger" }
            }
        }"##;

        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// Test dependentRequired (2020-12 replacement for dependencies)
    #[test]
    fn dependent_required() {
        let raw_schema = r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "credit_card": { "type": "string" },
                "billing_address": { "type": "string" }
            },
            "dependentRequired": {
                "credit_card": ["billing_address"]
            }
        }"#;

        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// Test unevaluatedProperties (2020-12 feature)
    #[test]
    fn unevaluated_properties() {
        let raw_schema = r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            },
            "unevaluatedProperties": false
        }"#;

        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// Test unevaluatedItems (2020-12 feature)
    #[test]
    fn unevaluated_items() {
        let raw_schema = r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "prefixItems": [
                { "type": "string" }
            ],
            "unevaluatedItems": false
        }"#;

        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// Test contains with minContains and maxContains (2020-12 enhancements)
    #[test]
    fn contains_with_bounds() {
        let raw_schema = r#"{
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "array",
            "contains": { "type": "integer" },
            "minContains": 1,
            "maxContains": 3
        }"#;

        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }
}
