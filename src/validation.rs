use serde::{Deserialize, Serialize};

/// Number validation Criteria
/// Supports both draft-07 and 2020-12 syntax
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NumberCriteria {
    /// Minimum value (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<serde_json::Number>,

    /// Maximum value (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<serde_json::Number>,

    /// Exclusive minimum value (2020-12 uses number, draft-07 used boolean)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<serde_json::Value>,

    /// Exclusive maximum value (2020-12 uses number, draft-07 used boolean)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<serde_json::Value>,

    /// Value must be a multiple of this number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<serde_json::Number>,
}

/// String validation criteria
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StringCriteria {
    /// Minimum string length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u64>,

    /// Maximum string length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,

    /// Regular expression pattern the string must match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    /// Semantic format (e.g., "date-time", "email", "uri")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// JSON Schema 2020-12: Content media type
    #[serde(rename = "contentMediaType", skip_serializing_if = "Option::is_none")]
    pub content_media_type: Option<String>,

    /// JSON Schema 2020-12: Content encoding (e.g., "base64")
    #[serde(rename = "contentEncoding", skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,

    /// JSON Schema 2020-12: Content schema for encoded content
    #[serde(rename = "contentSchema", skip_serializing_if = "Option::is_none")]
    pub content_schema: Option<Box<serde_json::Value>>,
}
