use serde::{Deserialize, Serialize};
use url::Url;

use std::collections::HashMap;

// TODO: root array vs object
#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "$id")]
    pub id: Url,

    #[serde(rename = "$schema")]
    pub schema: Url,
    pub description: String,
    // pub properties: HashMap<String, Property>,
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
            Property::Ref(_) => unimplemented!(),
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
#[serde(rename_all = "camelCase")]
pub struct NumberCriteria {
    exclusive_minimum: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SchemaValue {
    String,
    Integer {
        #[serde(flatten)]
        criteria: NumberCriteria,
    },
    Number {
        #[serde(flatten)]
        criteria: NumberCriteria,
    },

    Array {
        items: Box<SchemaValue>,
    },

    Object {
        properties: HashMap<String, Property>,
        required: Option<Vec<String>>,
    },
}

impl SchemaValue {
    pub fn validate(&self, json: &serde_json::Value) -> Result<(), Vec<String>> {
        use serde_json::Value;
        use SchemaValue::*;

        match (&self, json) {
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
                                    Some(vec![format!("object doesn't contain {}", k)])
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

#[derive(Debug, Serialize, Deserialize)]
pub struct RefProperty {
    #[serde(rename = "$ref")]
    pub reference: String,
}
