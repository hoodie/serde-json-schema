use serde::{Serialize, Deserialize};
use url::Url;

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Schema {
    #[serde(rename = "$id", with = "url_serde")]
    id: Url,

    #[serde(rename = "$schema", with = "url_serde")]
    schema: Url,
    description: String,
    properties: HashMap<String, Property>,
    required: Vec<String>,
    dependencies: Option<HashMap<String, Vec<String>>>
}

impl Schema {
    pub fn draft_version(&self) -> Option<&str> {
        self.schema
            .path_segments()
            .and_then(|mut segments| segments.next())
    }
}

#[serde(untagged)]
#[derive(Debug, Serialize, Deserialize)]
enum Property {
    Inline(InlineProperty),
    Ref(RefProperty),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum InlineProperty {
    #[serde(rename = "string")] String,

    #[serde(rename = "array")] Array {
        items: Box<InlineProperty>
    },

    #[serde(rename = "object")] Object {
        properties: HashMap<String, Property>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct RefProperty {
    #[serde(rename = "$ref")]
    reference: String
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

}

