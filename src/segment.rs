use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a segment of a Shindan result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: Value,
}

/// Creates a new Segment.
impl Segment {
    pub fn new(type_: &str, data: Value) -> Self {
        Segment {
            type_: type_.to_string(),
            data,
        }
    }

    /// Gets the text content if the segment is of type "text".
    pub fn get_text(&self) -> Option<String> {
        if self.type_ != "text" { return None; }
        self.data.as_object()
            .and_then(|map| map.get("text"))
            .and_then(Value::as_str)
            .map(String::from)
    }

    /// Gets the image URL if the segment is of type "image".
    pub fn get_image_url(&self) -> Option<String> {
        if self.type_ != "image" { return None; }
        self.data.as_object()
            .and_then(|map| map.get("file"))
            .and_then(Value::as_str)
            .map(String::from)
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_ && self.data == other.data
    }
}

/**
Filters segments by type.

# Examples

```
use shindan_maker::{Segment, filter_segments_by_type};
use serde_json::json;

let segments = vec![
    Segment::new("text", json!({"text": "Hello"})),
    Segment::new("image", json!({"file": "image.jpg"})),
    Segment::new("text", json!({"text": "World"})),
];

let text_segments = filter_segments_by_type(&segments, "text");
assert_eq!(text_segments.len(), 2);
```
*/
pub fn filter_segments_by_type<'a>(segments: &'a [Segment], type_: &str) -> Vec<&'a Segment> {
    segments
        .iter()
        .filter(|segment| segment.type_ == type_)
        .collect()
}