// <FILE>tui-vfx-core/tests/test_schema_output.rs</FILE> - <DESC>Integration tests for stable schema output</DESC>
// <VERS>VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
// <WCTX>Schema reference auto-generation - Phase 1</WCTX>
// <CLOG>Update expected JSON to include new schema fields</CLOG>

use tui_vfx_core::ConfigSchema;
#[derive(Debug, Clone, ConfigSchema)]
#[allow(dead_code)]
enum ExampleEnum {
    Unit,
    Tuple(
        #[config(min = 0, max = 10, default = 3)] u8,
        #[config(help = "free text")] String,
    ),
    Struct {
        #[config(help = "a constrained float", min = -1.0, max = 1.0, default = 0.2)]
        bulge: f32,
        #[config(hidden)]
        internal: u64,
        #[config(opaque)]
        external: std::time::Duration,
    },
    #[config(hidden)]
    Hidden,
}
#[test]
fn test_schema_json_is_stable() {
    let json = ExampleEnum::schema().to_json_pretty();
    let expected = r#"{
  "kind": "enum",
  "name": "ExampleEnum",
  "description": null,
  "json_name": null,
  "tag_field": null,
  "variants": [
    {
      "kind": "unit",
      "name": "Unit",
      "description": null,
      "json_value": null
    },
    {
      "kind": "tuple",
      "name": "Tuple",
      "description": null,
      "json_value": null,
      "items": [
        {
          "name": "0",
          "json_key": null,
          "meta": {
            "help": null,
            "description": null,
            "default": 3,
            "range": {
              "min": 0,
              "max": 10
            },
            "json_key": null,
            "optional": false
          },
          "schema": {
            "kind": "primitive",
            "type": "u8",
            "range": {
              "min": 0,
              "max": 255
            }
          }
        },
        {
          "name": "1",
          "json_key": null,
          "meta": {
            "help": "free text",
            "description": null,
            "default": null,
            "range": null,
            "json_key": null,
            "optional": false
          },
          "schema": {
            "kind": "primitive",
            "type": "String",
            "range": null
          }
        }
      ]
    },
    {
      "kind": "struct",
      "name": "Struct",
      "description": null,
      "json_value": null,
      "fields": [
        {
          "name": "bulge",
          "json_key": null,
          "meta": {
            "help": "a constrained float",
            "description": null,
            "default": 0.2,
            "range": {
              "min": -1.0,
              "max": 1.0
            },
            "json_key": null,
            "optional": false
          },
          "schema": {
            "kind": "primitive",
            "type": "f32",
            "range": null
          }
        },
        {
          "name": "external",
          "json_key": null,
          "meta": {
            "help": null,
            "description": null,
            "default": null,
            "range": null,
            "json_key": null,
            "optional": false
          },
          "schema": {
            "kind": "opaque",
            "type": "std::time::Duration"
          }
        }
      ]
    }
  ]
}"#;
    assert_eq!(json, expected);
}

// <FILE>tui-vfx-core/tests/test_schema_output.rs</FILE> - <DESC>Integration tests for stable schema output</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2025-12-31T00:00:00Z</VERS>
