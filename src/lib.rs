//! This crates provides simply the `Schema` struct. It resembles the latest (draft-07) [json-schema core spec](https://json-schema.org/latest/json-schema-core.html).
//! If this spec is no longer up-to-date by the time you read this, please open a [new issue](https://github.com/hoodie/serde-json-schema/issues/new).
//!
//! If this type seems a bit confusing, then it's because json-schema is a bit too flexible.
//!
//! ## Usage
//!
//! ```
//! use serde_json_schema::Schema;
//! # use std::convert::TryFrom;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use std::fs;
//! let schema_file = fs::read_to_string("./examples/address.schema.json")?;
//! let address_schema = Schema::try_from(schema_file)?;
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use url::Url;

use std::collections::HashMap;
use std::convert::TryFrom;

mod id;
mod specification;
mod validation;

use specification::*;

pub use crate::id::SchemaId;

/// Represents a full JSON Schema Document
// TODO: root array vs object
#[derive(Debug, Serialize, Deserialize)]
pub struct Schema(SchemaInner);

/// Represents a full JSON Schema Document
// TODO: root array vs object
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum SchemaInner {
    /// The Common case
    Schema(SchemaDefinition),
    /// For Some stupid reason specs can just be `true` or `false`
    Boolean(bool),
}

/// Represents a full JSON Schema Document
// TODO: root array vs object
#[derive(Debug, Serialize, Deserialize)]
struct SchemaDefinition {
    #[serde(rename = "$id")]
    //id: Option<Url>,
    id: Option<SchemaId>,

    #[serde(rename = "$schema")]
    schema: Option<Url>,
    description: Option<String>,
    // pub properties: HashMap<String, Property>,
    dependencies: Option<HashMap<String, Vec<String>>>,

    #[serde(flatten)]
    specification: Option<Property>,

    definitions: Option<HashMap<String, SchemaDefinition>>,
}

impl Schema {
    pub fn draft_version(&self) -> Option<&str> {
        match &self.0 {
            SchemaInner::Schema(SchemaDefinition {
                schema: Some(schema),
                ..
            }) => schema
                .path_segments()
                .and_then(|mut segments| segments.next()),
            _ => None,
        }
    }

    fn as_definition(&self) -> Option<&SchemaDefinition> {
        match &self.0 {
            SchemaInner::Schema(definition@SchemaDefinition { .. }) => Some(&definition),
            _ => None,
        }
    }

    pub fn id(&self) -> Option<&SchemaId> {
        self.as_definition().and_then(|d| d.id.as_ref())
    }

    pub fn schema(&self) -> Option<&Url> {
        self.as_definition().and_then(|d| d.schema.as_ref())
    }

    pub fn description(&self) -> Option<&str> {
        self.as_definition().and_then(|d| d.description.as_ref().map(|s| s.as_str()))
    }

    pub fn specification(&self) -> Option<&SchemaInstance> {
        match &self.0 {
            SchemaInner::Schema(SchemaDefinition {
                specification: Some(Property::Value(specification @ SchemaInstance::Object { .. })),
                ..
            }) => Some(&specification),
            _ => None,
        }
    }

    pub fn properties(&self) -> Option<&HashMap<String, Property>> {
        match self.specification() {
            Some(SchemaInstance::Object { properties, .. }) => Some(properties),
            _ => None,
        }
    }

    pub fn required_properties(&self) -> Option<&Vec<String>> {
        match self.specification() {
            Some(SchemaInstance::Object { required, .. }) => Some(required),
            _ => None,
        }
    }

    pub fn as_null(&self) -> Option<&SchemaInstance> {
        match self.specification() {
            Some(null @ SchemaInstance::Null) => Some(null),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<&SchemaInstance> {
        match self.specification() {
            Some(boolean @ SchemaInstance::Boolean(_)) => Some(boolean),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<&SchemaInstance> {
        match self.specification() {
            Some(integer @ SchemaInstance::Integer { .. }) => Some(integer),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&SchemaInstance> {
        match self.specification() {
            Some(object @ SchemaInstance::Object { .. }) => Some(object),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&SchemaInstance> {
        match self.specification() {
            Some(array @ SchemaInstance::Array { .. }) => Some(array),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<&SchemaInstance> {
        match self.specification() {
            Some(number @ SchemaInstance::Number { .. }) => Some(number),
            _ => None,
        }
    }

    pub fn validate(&self, json: &serde_json::Value) -> Result<(), Vec<String>> {
        match self.0 {
            SchemaInner::Schema(SchemaDefinition {
                specification: Some(Property::Value(ref prop)),
                ..
            }) => prop.validate(json),
            SchemaInner::Schema(SchemaDefinition {
                specification: Some(Property::Ref(_)),
                ..
            }) => unimplemented!(),
            SchemaInner::Boolean(true) => {
                eprintln!(r#"your schema is just "true", everything goes"#);
                Ok(())
            }
            SchemaInner::Boolean(false) => Err(vec![String::from(
                r##""the scheme "false" will never validate"##,
            )]),
            _ => Ok(()),
        }
    }
}

impl<'a> TryFrom<&str> for Schema {
    type Error = serde_json::error::Error;
    fn try_from(s: &str) -> Result<Schema, Self::Error> {
        serde_json::from_str(s)
    }
}

impl<'a> TryFrom<String> for Schema {
    type Error = serde_json::error::Error;
    fn try_from(s: String) -> Result<Schema, Self::Error> {
        serde_json::from_str(&s)
    }
}

impl<'a> TryFrom<&str> for SchemaDefinition {
    type Error = serde_json::error::Error;
    fn try_from(s: &str) -> Result<SchemaDefinition, Self::Error> {
        serde_json::from_str(s)
    }
}

impl<'a> TryFrom<String> for SchemaDefinition {
    type Error = serde_json::error::Error;
    fn try_from(s: String) -> Result<SchemaDefinition, Self::Error> {
        serde_json::from_str(&s)
    }
}
