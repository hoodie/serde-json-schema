use json_pointer::JsonPointer;
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use url::Url;

use std::fmt;
use std::str::FromStr;

/// Either a `Url` or a `JsonPointer`
#[derive(Debug)]
pub enum SchemaId {
    Url(Url),
    Pointer(JsonPointer<String, Vec<String>>),
    Fragment(Fragment),
    Path(Path),
}

#[derive(Debug)]
pub struct Fragment(String);
#[derive(Debug)]
pub struct InvalidFragment;

impl FromStr for Fragment {
    type Err = InvalidFragment;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0) {
            Some('#') => Ok(Fragment(s[1..].to_owned())),
            _ => Err(InvalidFragment),
        }
    }
}

impl ToString for Fragment {
    fn to_string(&self) -> String {
        format!("#{}", self.0)
    }
}

#[derive(Debug)]
pub struct Path(String);
#[derive(Debug)]
pub struct InvalidPath;

impl FromStr for Path {
    type Err = InvalidPath;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().any(char::is_whitespace) {
            Err(InvalidPath)
        } else {
            Ok(Path(s.to_string()))
        }
    }
}

impl ToString for Path {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl<'de> Deserialize<'de> for SchemaId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SchemaIdVisitor)
    }
}

struct SchemaIdVisitor;

impl<'de> Visitor<'de> for SchemaIdVisitor {
    type Value = SchemaId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a string that follows that \"/Date(...)/\" format "
        )
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Fragment::from_str(s)
            .map(SchemaId::Fragment)
            .or_else(|_| Url::parse(s).map(SchemaId::Url))
            .or_else(|_| JsonPointer::from_str(s).map(SchemaId::Pointer))
            .or_else(|_| Path::from_str(s).map(SchemaId::Path))
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(s), &self))
    }
}

impl ToString for SchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Pointer(p) => p.to_string(),
            Self::Url(u) => u.to_string(),
            Self::Fragment(f) => f.to_string(),
            Self::Path(p) => p.to_string(),
        }
    }
}

impl Serialize for SchemaId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
