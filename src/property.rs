//! Represents the [Instance Data Model](https://json-schema.org/latest/json-schema-core.html#rfc.section.4.2.1)

use serde::{Deserialize, Serialize};

use std::{collections::HashMap, str::Split};

use crate::{validation::NumberCriteria, Schema};

/// Either a `PropertyInstance` or a reference
#[serde(untagged)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Property {
    Value(PropertyInstance),
    Ref(RefProperty),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefProperty {
    #[serde(rename = "$ref")]
    pub reference: String,
}

#[derive(Debug)]
enum Data<'a> {
    Map(&'a HashMap<String, Property>),
    Prop(&'a Property),
    Instance(&'a PropertyInstance),
    Schema(&'a Schema),
}

fn get_items(p: &Property) -> Option<&PropertyInstance> {
    match p {
        Property::Value(v) => match v {
            PropertyInstance::Array { items } => Some(&**items),
            _ => None,
        },
        _ => None,
    }
}

fn get_properties_instance(p: &PropertyInstance) -> Option<&HashMap<String, Property>> {
    match p {
        PropertyInstance::Object { properties, .. } => Some(properties),
        _ => None,
    }
}

fn get_properties(p: &Property) -> Option<&HashMap<String, Property>> {
    match p {
        Property::Value(v) => get_properties_instance(v),
        _ => None,
    }
}

fn find_ref<'a>(mut path: Split<'a, &'a str>, mut data: Data<'a>) -> Option<Data<'a>> {
    loop {
        let Some(branch) = path.next() else {
            return Some(data);
        };
        data = match (branch, data) {
            ("properties", Data::Instance(v)) => Data::Map(get_properties_instance(v)?),
            ("properties", Data::Map(v)) => Data::Map(get_properties(v.get(branch)?)?),
            ("properties", Data::Prop(v)) => Data::Map(get_properties(&v)?),
            ("properties", Data::Schema(v)) => Data::Map(v.properties()?),
            ("items", Data::Prop(v)) => Data::Instance(get_items(&v)?),
            (_, Data::Map(v)) => Data::Prop(v.get(branch)?),
            _ => return None,
        };
    }
}

impl RefProperty {
    pub fn deref<'a>(&'a self, schema: &'a Schema) -> Option<&PropertyInstance> {
        let reference = self.reference.strip_prefix("#/")?;
        let path = reference.split("/").into_iter();
        match find_ref(path, Data::Schema(schema))? {
            Data::Prop(v) => match v {
                Property::Ref(v) => Some(v.deref(schema)?),
                Property::Value(v) => Some(v),
            },
            Data::Instance(v) => Some(v),
            _ => None,
        }
    }
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
