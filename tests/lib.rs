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
        assert_eq!(schema.draft_version(), Some("draft-07"))
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
  "$schema": "http://json-schema.org/draft-07/schema#",
  "description": "An address similar to http://microformats.org/wiki/h-card",
  "type": "object",
  "properties": {
  },
  "dependencies": {
    "post-office-box": [ "street-address" ],
    "extended-address": [ "street-address" ]
  }

    }"#;
        let schema = Schema::try_from(raw_schema).unwrap();
        assert!(schema.specification().is_some())
    }

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
    fn id_must_not_contain_spaces() {
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
        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// https://json-schema.org/latest/json-schema-core.html#rfc.section.8.2
    #[test]
    fn id_may_be_a_uuid() {
        let raw_schema = r#"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f"
    }"#;
        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// https://json-schema.org/latest/json-schema-core.html#rfc.section.8.2
    #[test]
    fn id_may_be_a_fragment() {
        let raw_schema = r##"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "#foo"
    }"##;
        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
    }

    /// https://json-schema.org/latest/json-schema-core.html#rfc.section.8.2
    #[test]
    fn id_may_be_a_path() {
        let raw_schema = r##"{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "$id": "t/inner.json"
    }"##;
        let schema = Schema::try_from(raw_schema).unwrap();
        println!("{:#?}", schema);
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

mod suite {
    use serde::{Deserialize, Serialize};
    use serde_json_schema::*;

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    struct Suite {
        description: String,
        schema: Schema,
        tests: Vec<Test>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    struct Test {
        description: String,
        data: serde_json::Value,
        valid: bool,
    }

    #[test]
    fn suite_dialect1() {
        let raw_suite = include_str!("./fixtures/geographical-location.schema.json");
        let schema: Schema = serde_json::from_str(raw_suite).unwrap();
        println!("{:#?}", schema);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }

    #[test]
    fn suite_dialect2() {
        let raw_suite = include_str!("./test-suite/tests/draft7/required.json");
        let suites: Vec<Suite> = serde_json::from_str(raw_suite).unwrap();
        let one = suites.get(0).unwrap().clone().schema;
        println!("{:#?}", one);
        println!("{}", serde_json::to_string_pretty(&one).unwrap());
    }

    #[test]
    #[ignore]
    fn suite_required() {
        let raw_suite = include_str!("./test-suite/tests/draft7/required.json");

        let suites: Vec<Suite> = serde_json::from_str(raw_suite).unwrap();
        for suite in &suites {
            println!("\n\n{:?}", suite.description);
            println!("{:#?}", suite.schema);
            for test in &suite.tests {
                if suite.schema.validate(&test.data).is_ok() != test.valid {
                    println!(
                        "{suite}::{test} expected to validate to {expected:?}",
                        suite = suite.description,
                        test = test.description,
                        expected = test.valid,
                    );
                }
            }
            // println!("{:}", serde_json::to_string_pretty(&test.schema).unwrap());
        }
    }

}
