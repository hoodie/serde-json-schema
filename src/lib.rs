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

#![deny(
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
// #![warn(missing_docs)]

use serde::{Deserialize, Serialize};
pub use url::Url;

use std::collections::HashMap;
pub use std::convert::TryFrom;

pub mod error;
pub mod id;
pub mod property;
mod validation;

use crate::error::Result;
use crate::id::*;
use crate::property::*;

/// Represents a full JSON Schema Document
// TODO: root array vs object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Schema(SchemaInner);

/// Represents a full JSON Schema Document
// TODO: root array vs object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum SchemaInner {
    /// The Common case
    Schema(SchemaDefinition),
    /// For Some stupid reason specs can just be `true` or `false`
    Boolean(bool),
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
            SchemaInner::Schema(definition @ SchemaDefinition { .. }) => Some(&definition),
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
        self.as_definition()
            .and_then(|d| d.description.as_ref().map(|s| s.as_str()))
    }

    pub fn specification(&self) -> Option<&PropertyInstance> {
        match &self.0 {
            SchemaInner::Schema(SchemaDefinition {
                specification:
                    Some(Property::Value(specification @ PropertyInstance::Object { .. })),
                ..
            }) => Some(&specification),
            _ => None,
        }
    }

    pub fn properties(&self) -> Option<&HashMap<String, Property>> {
        match self.specification() {
            Some(PropertyInstance::Object { properties, .. }) => Some(properties),
            _ => None,
        }
    }

    pub fn required_properties(&self) -> Option<&Vec<String>> {
        match self.specification() {
            Some(PropertyInstance::Object { required, .. }) => required.as_ref(),
            _ => None,
        }
    }

    pub fn as_null(&self) -> Option<&PropertyInstance> {
        match self.specification() {
            Some(null @ PropertyInstance::Null) => Some(null),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<&PropertyInstance> {
        match self.specification() {
            Some(boolean @ PropertyInstance::Boolean(_)) => Some(boolean),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<&PropertyInstance> {
        match self.specification() {
            Some(integer @ PropertyInstance::Integer { .. }) => Some(integer),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&PropertyInstance> {
        match self.specification() {
            Some(object @ PropertyInstance::Object { .. }) => Some(object),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&PropertyInstance> {
        match self.specification() {
            Some(array @ PropertyInstance::Array { .. }) => Some(array),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<&PropertyInstance> {
        match self.specification() {
            Some(number @ PropertyInstance::Number { .. }) => Some(number),
            _ => None,
        }
    }

    pub fn validate(&self, json: &serde_json::Value) -> std::result::Result<(), Vec<String>> {
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

    pub fn from_value(value: serde_json::Value) -> Result<Self> {
        Ok(serde_json::from_value(value)?)
    }
}

impl<'a> TryFrom<&str> for Schema {
    type Error = crate::error::Error;
    fn try_from(s: &str) -> Result<Schema> {
        Ok(serde_json::from_str(s)?)
    }
}

impl<'a> TryFrom<String> for Schema {
    type Error = crate::error::Error;
    fn try_from(s: String) -> Result<Schema> {
        Ok(serde_json::from_str(&s)?)
    }
}

impl<'a> TryFrom<&str> for SchemaDefinition {
    type Error = crate::error::Error;
    fn try_from(s: &str) -> Result<SchemaDefinition> {
        Ok(serde_json::from_str(&s)?)
    }
}

impl<'a> TryFrom<String> for SchemaDefinition {
    type Error = crate::error::Error;
    fn try_from(s: String) -> Result<SchemaDefinition> {
        Ok(serde_json::from_str(&s)?)
    }
}

/// Represents a full JSON Schema Document, except when it is a boolean
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct SchemaDefinition {
    #[serde(rename = "$id")]
    pub id: Option<SchemaId>,

    #[serde(rename = "$schema")]
    pub schema: Option<Url>,
    pub description: Option<String>,
    // pub properties: HashMap<String, Property>,
    pub dependencies: Option<HashMap<String, Vec<String>>>,

    #[serde(flatten)]
    pub specification: Option<Property>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<HashMap<String, SchemaDefinition>>,
}
