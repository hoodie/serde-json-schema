use serde::{Serialize, Deserialize};
use url::Url;

use std::collections::HashMap;

// TODO: root array vs object
#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "$id", with = "url_serde")]
    pub id: Url,

    #[serde(rename = "$schema", with = "url_serde")]
    pub schema: Url,
    pub description: String,
    // pub properties: HashMap<String, Property>,
    // pub required: Vec<String>,
    pub dependencies: Option<HashMap<String, Vec<String>>>,


    #[serde(flatten)]
    specification: Property,
}

impl Schema {
    pub fn draft_version(&self) -> Option<&str> {
        self.schema
            .path_segments()
            .and_then(|mut segments| segments.next())
    }

    pub fn validate(&self, json: &serde_json::Value) -> Result<(), Vec<String>> {
        match &self.specification {
            Property::Value(ref prop) => prop.validate(json),
            Property::Ref(_) => unimplemented!()
        }
    }

}

#[serde(untagged)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Property {
    Value(SchemaValue),
    Ref(RefProperty),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SchemaValue {
    #[serde(rename = "string")] String,

    #[serde(rename = "array")] Array {
        items: Box<SchemaValue>
    },

    #[serde(rename = "object")] Object {
        properties: HashMap<String, Property>,
    },
}

impl SchemaValue {

    pub fn validate(&self, json: &serde_json::Value) -> Result<(), Vec<String>> {
        use SchemaValue::*;
        use serde_json::Value;

        match (&self, json) {
            (String, Value::String(_)) => Ok(()),
            (String, unexpected_value) => {
                Err(vec![format!("expected string found {:?}", unexpected_value)])
            },

            (Array{ items }, Value::Array(elems)) => {
                let errors: Vec<std::string::String> = elems
                    .iter()
                    .map(|value| items.validate(&value))
                    .filter_map(Result::err)
                    .flat_map(|errors| errors.into_iter())
                    .collect();
                if errors.is_empty() { Ok(()) } else {
                    Err(errors)
                }
            },
            (Array{ .. }, unexpected_value) => {
                Err(vec![format!("expected array found {:?}", unexpected_value)])
            },

            (Object{ properties }, Value::Object(object)) => {
                let errors: Vec<std::string::String> = properties
                    .iter()
                    .filter_map(|(k, schema)| object
                        .get(k)
                        .map(|v| match schema {
                            Property::Value(schema) => schema.validate(v).err(),
                            Property::Ref(_schema) => unimplemented!(),
                        })
                        .unwrap_or(Some(vec![format!("object doesn't contain {}", k)]))
                    )
                    .flat_map(|errors| errors.into_iter())
                    .collect();
                if errors.is_empty() { Ok(()) } else {
                    Err(errors)
                }
            },

            (Object{ .. }, _) => Err(vec![format!("invalid object")]),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefProperty {
    #[serde(rename = "$ref")]
    pub reference: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_example() {
        let raw = include_str!("../test/address.schema.json");
        let schema: Schema = serde_json::from_str(raw).unwrap();
        println!("{:#?}", schema);
    }

    #[test]
    fn calendar_example() {
        let raw = include_str!("../test/calendar.schema.json");
        let schema: Schema = serde_json::from_str(raw).unwrap();
        println!("{:#?}", schema);
    }

    #[test]
    fn card_example() {
        let raw = include_str!("../test/card.schema.json");
        let schema: Schema = serde_json::from_str(raw).unwrap();
        println!("{:#?}", schema);
    }

    #[test]
    fn draft_version() {
        let raw = include_str!("../test/card.schema.json");
        let schema: Schema = serde_json::from_str(raw).unwrap();
        println!("{:#?}", schema.draft_version());
        assert_eq!(schema.draft_version(), Some("draft-07"))
    }

    #[test]
    fn validate_pure_string_object() {
        let raw_schema = include_str!("../test/address.schema.json");
        let schema: Schema = serde_json::from_str(raw_schema).unwrap();

        let json_correct: serde_json::Value =
            serde_json::from_str(include_str!("../test/address.json")).unwrap();
        schema.validate(&json_correct).unwrap();
    }

    #[test]
    fn validate_find_wrong_type() {
        let raw_schema = include_str!("../test/address.schema.json");
        let schema: Schema = serde_json::from_str(raw_schema).unwrap();

        let json_wrong_type: serde_json::Value =
            serde_json::from_str(include_str!("../test/address.wrong-type.json")).unwrap();
        // TODO: make more concrete error type
        assert!(schema.validate(&json_wrong_type).is_err());
    }

    #[test]
    fn validate_find_missing() {
        let raw_schema = include_str!("../test/address.schema.json");
        let schema: Schema = serde_json::from_str(raw_schema).unwrap();

        let json_missing: serde_json::Value =
            serde_json::from_str(include_str!("../test/address.missing.json")).unwrap();
        // TODO: make more concrete error type
        assert!(schema.validate(&json_missing).is_err());

    }

}

