// <FILE>tui-vfx-core/src/schema/cls_json_writer.rs</FILE> - <DESC>Low-level JSON writer for schema output</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>OFPF slicing</WCTX>
// <CLOG>Extracted JsonWriter state and primitive emitters</CLOG>

use super::types::ScalarValue;

pub(super) struct JsonWriter<'a> {
    pub(super) out: &'a mut String,
}

impl<'a> JsonWriter<'a> {
    pub(super) fn new(out: &'a mut String) -> Self {
        Self { out }
    }

    pub(super) fn indent(&mut self, level: usize) {
        for _ in 0..level {
            self.out.push_str("  ");
        }
    }

    pub(super) fn push_escaped_str(&mut self, value: &str) {
        self.out.push('"');
        for c in value.chars() {
            match c {
                '"' => self.out.push_str("\\\""),
                '\\' => self.out.push_str("\\\\"),
                '\n' => self.out.push_str("\\n"),
                '\r' => self.out.push_str("\\r"),
                '\t' => self.out.push_str("\\t"),
                c if c.is_control() => {
                    let code = c as u32;
                    self.out.push_str("\\u");
                    self.out.push_str(&format!("{:04x}", code));
                }
                _ => self.out.push(c),
            }
        }
        self.out.push('"');
    }

    pub(super) fn value_string(&mut self, value: &str) {
        self.push_escaped_str(value);
    }

    pub(super) fn value_scalar(&mut self, value: &ScalarValue) {
        match value {
            ScalarValue::Bool(b) => self.out.push_str(if *b { "true" } else { "false" }),
            ScalarValue::Number(n) => self.out.push_str(n),
            ScalarValue::String(s) => self.value_string(s),
            ScalarValue::Char(c) => self.value_string(&c.to_string()),
        }
    }
}

// <FILE>tui-vfx-core/src/schema/cls_json_writer.rs</FILE> - <DESC>Low-level JSON writer for schema output</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
