//! Represents the [Instance Data Model](https://json-schema.org/draft/2020-12/json-schema-core.html#section-4.2.1)

use serde::{Deserialize, Deserializer, Serialize};

use std::{collections::HashMap, str::Split};

use crate::{
    validation::{NumberCriteria, StringCriteria},
    Schema,
};

/// Either a `PropertyInstance` or a reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Property {
    Value(PropertyInstance),
    Ref(RefProperty),
}

/// A reference to another schema using $ref or $dynamicRef
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RefProperty {
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// JSON Schema 2020-12: $dynamicRef for dynamic referencing
    #[serde(rename = "$dynamicRef", skip_serializing_if = "Option::is_none")]
    pub dynamic_reference: Option<String>,
}

// Custom deserializer for RefProperty that only matches if at least one ref is present
impl<'de> Deserialize<'de> for RefProperty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RefPropertyHelper {
            #[serde(rename = "$ref")]
            reference: Option<String>,
            #[serde(rename = "$dynamicRef")]
            dynamic_reference: Option<String>,
        }

        let helper = RefPropertyHelper::deserialize(deserializer)?;

        // Only succeed if at least one ref field is present
        if helper.reference.is_none() && helper.dynamic_reference.is_none() {
            return Err(serde::de::Error::custom(
                "RefProperty requires at least $ref or $dynamicRef",
            ));
        }

        Ok(RefProperty {
            reference: helper.reference,
            dynamic_reference: helper.dynamic_reference,
        })
    }
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
        Property::Value(PropertyInstance::Array {
            items: Some(items), ..
        }) => Some(items.as_ref()),
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

fn find_ref<'a>(mut path: Split<'a, char>, mut data: Data<'a>) -> Option<Data<'a>> {
    loop {
        let Some(branch) = path.next() else {
            return Some(data);
        };
        data = match (branch, data) {
            ("properties", Data::Instance(v)) => Data::Map(get_properties_instance(v)?),
            ("properties", Data::Map(v)) => Data::Map(get_properties(v.get(branch)?)?),
            ("properties", Data::Prop(v)) => Data::Map(get_properties(v)?),
            ("properties", Data::Schema(v)) => Data::Map(v.properties()?),
            ("items", Data::Prop(v)) => Data::Instance(get_items(v)?),
            (_, Data::Map(v)) => Data::Prop(v.get(branch)?),
            _ => return None,
        };
    }
}

impl RefProperty {
    pub fn deref<'a>(&'a self, schema: &'a Schema) -> Option<&'a PropertyInstance> {
        // Try $ref first, then fall back to $dynamicRef
        let ref_value = self
            .reference
            .as_ref()
            .or(self.dynamic_reference.as_ref())?;
        let reference = ref_value.strip_prefix("#/")?;
        let path = reference.split('/');
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

/// Represents the [Instance Data Model](https://json-schema.org/draft/2020-12/json-schema-core.html#section-4.2.1)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PropertyInstance {
    Null,

    Boolean,

    Integer {
        #[serde(flatten)]
        criteria: NumberCriteria,
    },
    Object {
        #[serde(default)]
        properties: HashMap<String, Property>,
        #[serde(skip_serializing_if = "Option::is_none")]
        required: Option<Vec<String>>,
        /// JSON Schema 2020-12: additionalProperties
        #[serde(
            rename = "additionalProperties",
            skip_serializing_if = "Option::is_none"
        )]
        additional_properties: Option<Box<AdditionalProperties>>,
        /// JSON Schema 2020-12: unevaluatedProperties
        #[serde(
            rename = "unevaluatedProperties",
            skip_serializing_if = "Option::is_none"
        )]
        unevaluated_properties: Option<Box<AdditionalProperties>>,
        /// JSON Schema 2020-12: patternProperties
        #[serde(rename = "patternProperties", skip_serializing_if = "Option::is_none")]
        pattern_properties: Option<HashMap<String, Property>>,
        /// Minimum number of properties
        #[serde(rename = "minProperties", skip_serializing_if = "Option::is_none")]
        min_properties: Option<u64>,
        /// Maximum number of properties
        #[serde(rename = "maxProperties", skip_serializing_if = "Option::is_none")]
        max_properties: Option<u64>,
        /// Property names schema
        #[serde(rename = "propertyNames", skip_serializing_if = "Option::is_none")]
        property_names: Option<Box<PropertyInstance>>,
    },

    Array {
        /// JSON Schema 2020-12: items accepts a single schema (for items beyond prefixItems)
        #[serde(skip_serializing_if = "Option::is_none")]
        items: Option<Box<PropertyInstance>>,
        /// JSON Schema 2020-12: prefixItems for tuple validation
        #[serde(rename = "prefixItems", skip_serializing_if = "Option::is_none")]
        prefix_items: Option<Vec<Property>>,
        /// JSON Schema 2020-12: unevaluatedItems
        #[serde(rename = "unevaluatedItems", skip_serializing_if = "Option::is_none")]
        unevaluated_items: Option<Box<AdditionalProperties>>,
        /// Contains validation
        #[serde(skip_serializing_if = "Option::is_none")]
        contains: Option<Box<Property>>,
        /// Minimum number of contains
        #[serde(rename = "minContains", skip_serializing_if = "Option::is_none")]
        min_contains: Option<u64>,
        /// Maximum number of contains
        #[serde(rename = "maxContains", skip_serializing_if = "Option::is_none")]
        max_contains: Option<u64>,
        /// Minimum items
        #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
        min_items: Option<u64>,
        /// Maximum items
        #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
        max_items: Option<u64>,
        /// Unique items constraint
        #[serde(rename = "uniqueItems", skip_serializing_if = "Option::is_none")]
        unique_items: Option<bool>,
    },

    Number {
        #[serde(flatten)]
        criteria: NumberCriteria,
    },

    String {
        #[serde(flatten)]
        criteria: StringCriteria,
    },
}

/// Represents either a schema or a boolean for additionalProperties/unevaluatedProperties/unevaluatedItems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Boolean(bool),
    Schema(Property),
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

            (Boolean, Value::Bool(_)) => Ok(()),
            (Boolean, unexpected_value) => Err(vec![format!(
                "expected boolean found {:?}",
                unexpected_value
            )]),

            (String { .. }, Value::String(_)) => Ok(()),
            (String { .. }, unexpected_value) => Err(vec![format!(
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

            (
                Array {
                    items,
                    prefix_items,
                    ..
                },
                Value::Array(elems),
            ) => {
                let mut errors = Vec::new();

                // Validate prefix items if present (tuple validation)
                if let Some(prefix) = prefix_items {
                    for (i, (elem, schema)) in elems.iter().zip(prefix.iter()).enumerate() {
                        match schema {
                            Property::Value(schema) => {
                                if let Err(e) = schema.validate(elem) {
                                    errors.extend(e.into_iter().map(|e| format!("[{}]: {}", i, e)));
                                }
                            }
                            Property::Ref(_) => unimplemented!("$ref in prefixItems"),
                        }
                    }
                }

                // Validate remaining items with items schema
                let prefix_len = prefix_items.as_ref().map(|p| p.len()).unwrap_or(0);
                if let Some(items_schema) = items {
                    for (i, elem) in elems.iter().enumerate().skip(prefix_len) {
                        if let Err(e) = items_schema.validate(elem) {
                            errors.extend(e.into_iter().map(|e| format!("[{}]: {}", i, e)));
                        }
                    }
                }

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
                    ..
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
