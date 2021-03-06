//! Represents the [Instance Data Model](https://json-schema.org/latest/json-schema-core.html#rfc.section.4.2.1)

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::validation::NumberCriteria;

/// Either a `PropertyInstance` or a reference
#[serde(untagged)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Property {
    Value(PropertyInstance),
    Ref(RefProperty),
}
/// TODO: implement dereferencing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefProperty {
    #[serde(rename = "$ref")]
    pub reference: String,
}

/// Represents the [Instance Data Model](https://json-schema.org/latest/json-schema-core.html#rfc.section.4.2.1)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PropertyInstance {
    Null,

    Boolean(bool),

    Integer {
        #[serde(flatten)]
        criteria: NumberCriteria,
    },
    Object {
        properties: HashMap<String, Property>,
        required: Option<Vec<String>>,
    },

    Array {
        items: Box<PropertyInstance>,
    },

    Number {
        #[serde(flatten)]
        criteria: NumberCriteria,
    },

    String,
}

impl PropertyInstance {
    /// TODO: implement [validation](https://json-schema.org/latest/json-schema-validation.html)
    pub fn validate(&self, json: &serde_json::Value) -> Result<(), Vec<String>> {
        use serde_json::Value;
        use PropertyInstance::*;

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
                                if required.iter().flat_map(|v| v.iter()).any(|x| x == k) {
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
