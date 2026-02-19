// <FILE>tui-vfx-style/tests/models/test_cls_color_config.rs</FILE> - <DESC>Tests for ColorConfig deserialization</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-26</VERS>
// <WCTX>Ergonomic improvements for recipe authoring</WCTX>
// <CLOG>Initial tests for RGB shorthand and tagged deserialization</CLOG>

use tui_vfx_style::models::ColorConfig;

#[test]
fn test_rgb_shorthand_deserialization() {
    // RGB shorthand without "type" field
    let json = r#"{"r": 255, "g": 128, "b": 64}"#;
    let color: ColorConfig = serde_json::from_str(json).unwrap();
    assert_eq!(color, ColorConfig::Rgb { r: 255, g: 128, b: 64 });
}

#[test]
fn test_rgb_tagged_deserialization() {
    // Full tagged format with "type": "rgb"
    let json = r#"{"type": "rgb", "r": 100, "g": 150, "b": 200}"#;
    let color: ColorConfig = serde_json::from_str(json).unwrap();
    assert_eq!(color, ColorConfig::Rgb { r: 100, g: 150, b: 200 });
}

#[test]
fn test_named_color_deserialization() {
    // Named colors still work
    let json = r#"{"type": "red"}"#;
    let color: ColorConfig = serde_json::from_str(json).unwrap();
    assert_eq!(color, ColorConfig::Red);

    let json = r#"{"type": "cyan"}"#;
    let color: ColorConfig = serde_json::from_str(json).unwrap();
    assert_eq!(color, ColorConfig::Cyan);
}

#[test]
fn test_indexed_color_deserialization() {
    let json = r#"{"type": "indexed", "value": 42}"#;
    let color: ColorConfig = serde_json::from_str(json).unwrap();
    assert_eq!(color, ColorConfig::Indexed { value: 42 });
}

#[test]
fn test_rgb_shorthand_with_extra_fields_fails() {
    // If there's no type but also unexpected fields, it should still try RGB
    // and potentially fail on the extra field
    let json = r#"{"r": 255, "g": 128, "b": 64, "extra": "ignored"}"#;
    // serde_json by default ignores unknown fields, so this should work
    let color: ColorConfig = serde_json::from_str(json).unwrap();
    assert_eq!(color, ColorConfig::Rgb { r: 255, g: 128, b: 64 });
}

#[test]
fn test_rgb_serialization_uses_tagged_format() {
    // Serialization should use the tagged format for consistency
    let color = ColorConfig::Rgb { r: 10, g: 20, b: 30 };
    let json = serde_json::to_string(&color).unwrap();
    // Should contain "type":"rgb"
    assert!(json.contains(r#""type":"rgb""#));
}

// <FILE>tui-vfx-style/tests/models/test_cls_color_config.rs</FILE> - <DESC>Tests for ColorConfig deserialization</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-26</VERS>
