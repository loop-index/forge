use std::fmt::Display;

use derive_setters::Setters;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{ToolCallFull, ToolCallId, ToolName};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Setters)]
#[setters(strip_option)]
pub struct ToolResult {
    pub name: ToolName,
    pub call_id: Option<ToolCallId>,
    pub content: Value,
    pub is_error: bool,
}

#[derive(Default, Serialize, Setters)]
#[serde(rename_all = "snake_case", rename = "tool_result")]
#[setters(strip_option)]
struct ToolResultXML {
    tool_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success: Option<Value>,
}

impl ToolResult {
    pub fn new(name: ToolName) -> ToolResult {
        Self {
            name,
            call_id: None,
            content: Value::default(),
            is_error: false,
        }
    }
}

impl From<ToolCallFull> for ToolResult {
    fn from(value: ToolCallFull) -> Self {
        Self {
            name: value.name,
            call_id: value.call_id,
            content: Value::default(),
            is_error: false,
        }
    }
}

impl Display for ToolResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let xml = {
            let xml = ToolResultXML::default().tool_name(self.name.as_str().to_owned());
            if self.is_error {
                xml.error(self.content.clone())
            } else {
                xml.success(self.content.clone())
            }
        };

        // First serialize to string
        let mut out = String::new();
        {
            let ser = quick_xml::se::Serializer::new(&mut out);
            xml.serialize(ser).unwrap();
        }

        // Now reformat with pretty printing
        let mut writer = quick_xml::Writer::new_with_indent(Vec::new(), b' ', 0);
        let mut reader = quick_xml::reader::Reader::from_reader(out.as_bytes());
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(quick_xml::events::Event::Eof) => break,
                Ok(event) => writer.write_event(event).unwrap(),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            }
        }

        let result = writer.into_inner();
        let xml = String::from_utf8(result).unwrap();
        let xml = html_escape::decode_html_entities(&xml);
        write!(f, "{}", xml)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use serde_json::json;

    use super::*;

    #[test]
    fn test_snapshot_minimal() {
        let result = ToolResult::new(ToolName::new("test_tool"));
        assert_snapshot!(result);
    }

    #[test]
    fn test_snapshot_full() {
        let result = ToolResult::new(ToolName::new("complex_tool"))
            .call_id(ToolCallId::new("123"))
            .content(json!({"key": "value", "number": 42}))
            .is_error(true);
        assert_snapshot!(result);
    }

    #[test]
    fn test_snapshot_with_special_chars() {
        let result = ToolResult::new(ToolName::new("xml_tool")).content(json!({
            "text": "Special chars: < > & ' \"",
            "nested": {
                "html": "<div>Test</div>"
            }
        }));
        assert_snapshot!(result);
    }

    #[test]
    fn test_display_minimal() {
        let result = ToolResult::new(ToolName::new("test_tool"));
        assert_snapshot!(result.to_string());
    }

    #[test]
    fn test_display_full() {
        let result = ToolResult::new(ToolName::new("complex_tool"))
            .call_id(ToolCallId::new("123"))
            .content(json!({
                "user": "John Doe",
                "age": 42,
                "address": [{"city": "New York"}, {"city": "Los Angeles"}]
            }));
        assert_snapshot!(result.to_string());
    }

    #[test]
    fn test_display_special_chars() {
        let result = ToolResult::new(ToolName::new("xml_tool")).content(json!({
            "text": "Special chars: < > & ' \"",
            "nested": {
                "html": "<div>Test</div>"
            }
        }));
        assert_snapshot!(result.to_string());
    }
}
