use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::ops::Deref;

/// A segment of a shindan result.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Segment {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: Value,
}

impl Segment {
    pub fn new(type_: &str, data: Value) -> Self {
        Segment {
            type_: type_.to_string(),
            data,
        }
    }

    pub fn get_str(&self) -> Option<String> {
        match self.type_.as_str() {
            "text" => self
                .data
                .get("text")
                .and_then(Value::as_str)
                .map(String::from),
            "image" => self
                .data
                .get("file")
                .and_then(Value::as_str)
                .map(String::from),
            _ => None,
        }
    }
}

/// A collection of segments.
#[derive(Debug, Clone)]
pub struct Segments(pub Vec<Segment>);

impl Deref for Segments {
    type Target = Vec<Segment>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Segments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = self
            .iter()
            .filter_map(|s| s.get_str())
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{}", str)
    }
}
