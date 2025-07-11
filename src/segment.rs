use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use std::ops::Deref;

/// A segment of a shindan result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: Value,
}

impl Segment {
    /**
    Create a new segment.

    # Arguments
    - `type_` - The type of the segment.
    - `data` - The data of the segment.

    # Returns
    A new segment.

    # Examples
    ```
    use serde_json::json;
    use shindan_maker::Segment;

    let segment = Segment::new("text", json!({"text": "Hello, world!"}));
    ```
    */
    pub fn new(type_: &str, data: Value) -> Self {
        Segment {
            type_: type_.to_string(),
            data,
        }
    }

    /**
    Get the string representation of the segment.

    # Returns
    - `Some(String)`: The string representation of the segment.
    - `None`: If the segment type is not text or image.

    # Examples
    ```
    use serde_json::json;
    use shindan_maker::Segment;

    let segment = Segment::new("text", json!({"text": "Hello, world!"}));
    assert_eq!(segment.get_str(), Some("Hello, world!".to_string()));
    ```
    */
    pub fn get_str(&self) -> Option<String> {
        match self.type_.as_str() {
            "text" => self
                .data
                .as_object()
                .and_then(|map| map.get("text"))
                .and_then(Value::as_str)
                .map(String::from),
            "image" => self
                .data
                .as_object()
                .and_then(|map| map.get("file"))
                .and_then(Value::as_str)
                .map(String::from),
            _ => None,
        }
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_ && self.data == other.data
    }
}

impl Eq for Segment {}

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
            .map(|segment| segment.get_str().unwrap())
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}", str)
    }
}
