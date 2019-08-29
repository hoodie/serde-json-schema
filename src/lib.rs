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
mod validation;

use crate::id::SchemaId;
use crate::validation::*;

/// Represents a full JSON Schema Document
// TODO: root array vs object
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Schema {
    Schema(SchemaDefinition),
    Boolean(bool),
}

/// Represents a full JSON Schema Document
// TODO: root array vs object
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaDefinition {
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
        match self {
            Schema::Schema(SchemaDefinition {
                schema: Some(schema),
                ..
            }) => schema
                .path_segments()
                .and_then(|mut segments| segments.next()),
            _ => None,
        }
    }

    pub fn id(&self) -> Option<&SchemaId> {
        match self {
            Schema::Schema(SchemaDefinition { id: Some(id), .. }) => Some(id),
            _ => None,
        }
    }

    pub fn specification(&self) -> Option<&SchemaInstance> {
        match self {
            Schema::Schema(SchemaDefinition {
                specification: Some(Property::Value(specification @ SchemaInstance::Object { .. })),
                ..
            }) => Some(specification),
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
        match self {
            Schema::Schema(SchemaDefinition {
                specification: Some(Property::Value(ref prop)),
                ..
            }) => prop.validate(json),
            Schema::Schema(SchemaDefinition {
                specification: Some(Property::Ref(_)),
                ..
            }) => unimplemented!(),
            Schema::Boolean(true) => {
                eprintln!(r#"your schema is just "true", everything goes"#);
                Ok(())
            }
            Schema::Boolean(false) => Err(vec![String::from(
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

/// Either a `SchemaInstance` or a reference
#[serde(untagged)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Property {
    Value(SchemaInstance),
    Ref(RefProperty),
}

/// prepresents the [Instance Data Model](https://json-schema.org/latest/json-schema-core.html#rfc.section.4.2.1)
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SchemaInstance {
    Null,

    Boolean(bool),

    Integer {
        #[serde(flatten)]
        criteria: NumberCriteria,
    },
    Object {
        properties: HashMap<String, Property>,
        required: Vec<String>,
    },

    Array {
        items: Box<SchemaInstance>,
    },

    Number {
        #[serde(flatten)]
        criteria: NumberCriteria,
    },

    String,
}

impl SchemaInstance {
    /// TODO: implement [validation](https://json-schema.org/latest/json-schema-validation.html)
    pub fn validate(&self, json: &serde_json::Value) -> Result<(), Vec<String>> {
        use serde_json::Value;
        use SchemaInstance::*;

        match (&self, json) {
            (Null, Value::Null) => Ok(()),
            (Null, unexpected_value) => {
                Err(vec![format!("expected null found {:?}", unexpected_value)])
            }

            (Boolean(_), Value::Bool(_)) => Ok(()),
            (Boolean(_), unexpected_value) => Err(vec![format!(
                "expected boolean found {:?}",
                unexpected_value
            )]),

            (String, Value::String(_)) => Ok(()),
            (String, unexpected_value) => Err(vec![format!(
                "expected string found {:?}",
                unexpected_value
            )]),

            (Number { .. }, Value::Number(_)) => Ok(()),
            (Number { .. }, unexpected_value) => Err(vec![format!(
                "expected number found {:?}",
                unexpected_value
            )]),

            (Integer { .. }, Value::Number(i)) if i.is_i64() => Ok(()),
            (Integer { .. }, unexpected_value) => Err(vec![format!(
                "expected integer found {:?}",
                unexpected_value
            )]),

            (Array { items }, Value::Array(elems)) => {
                let errors: Vec<std::string::String> = elems
                    .iter()
                    .map(|value| items.validate(&value))
                    .filter_map(Result::err)
                    .flat_map(|errors| errors.into_iter())
                    .collect();
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
            (Array { .. }, unexpected_value) => {
                Err(vec![format!("expected array found {:?}", unexpected_value)])
            }

            (
                Object {
                    properties,
                    required,
                },
                Value::Object(object),
            ) => {
                let errors: Vec<std::string::String> = properties
                    .iter()
                    .filter_map(|(k, schema)| {
                        object
                            .get(k)
                            .map(|v| match schema {
                                Property::Value(schema) => schema.validate(v).err(),
                                Property::Ref(_schema) => unimplemented!(),
                            })
                            .unwrap_or_else(|| {
                                if required.iter().any(|x| x == k) {
                                    Some(vec![format!(
                                        "object doesn't contain the required property {:?}",
                                        k
                                    )])
                                } else {
                                    None
                                }
                            })
                    })
                    .flat_map(|errors| errors.into_iter())
                    .collect();
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }

            (Object { .. }, _) => Err(vec![format!("invalid object")]),
        }
    }
}

/// TODO: implement dereferencing
#[derive(Debug, Serialize, Deserialize)]
pub struct RefProperty {
    #[serde(rename = "$ref")]
    pub reference: String,
}
