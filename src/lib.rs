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

    pub fn validate(&self, json: &serde_json::Value) -> bool {
        match &self.specification {
            Property::Inline(ref prop) => prop.validate(json),
            Property::Ref(_) => unimplemented!()
        }
    }

}

#[serde(untagged)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Property {
    Inline(SchemaValue),
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

    pub fn validate(&self, json: &serde_json::Value) -> bool {
        use SchemaValue::*;
        use serde_json::Value;

        match (&self, json) {
            (String, Value::String(_)) => true,
            (String, unexpected_value) => {
                eprintln!("expected string found {}", unexpected_value);
                false
            },

            (Array{ items }, Value::Array(elems)) => elems.iter().all(|elem| items.validate(&elem)),
            (Array{ .. }, _) => false,

            (Object{ properties }, Value::Object(hash)) => {
                hash.iter()
                    .all(|(k, v)|
                    properties
                        .get(k)
                        .map(|s| match s {
                            Property::Inline(schema) => schema.validate(v),
                            Property::Ref(_schema) => unimplemented!(),
                        })
                        .unwrap_or(false)
                    )
            },

            (Object{ .. }, _) => false,
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
    fn validate_string_object() {
        let raw_json = include_str!("../test/address.json");
        let raw_schema = include_str!("../test/address.schema.json");

        let json: serde_json::Value = serde_json::from_str(raw_json).unwrap();
        let schema: Schema = serde_json::from_str(raw_schema).unwrap();

        println!("{:#?}{:#?}", schema, json);
        assert!(schema.validate(&json))
    }

}

