// <FILE>tui-vfx-style/src/models/fnc_style_region_schema.rs</FILE> - <DESC>Hand-written ConfigSchema impl for StyleRegion (Role(RoleTag) variant requires manual schema because RoleTag lives in tui-vfx-types which doesn't depend on tui-vfx-core)</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.2 audit-1 remediation — extract ConfigSchema impl out of cls_style_region.rs to bring the main file back under OFPF cls_ LOC budget</WCTX>
// <CLOG>0.1.0: extracted from cls_style_region.rs (prev lines 322–453); logic bit-preserved. `RoleTag` doesn't implement `ConfigSchema` (it lives in tui-vfx-types which does not depend on tui-vfx-core — adding that dep would invert the layer graph). The derive macro can't synthesize a schema for `Role(RoleTag)`, so the enum is described manually here. OFPF SIZE NOTE: file is slightly above the fnc_ soft target of 120 LOC because the schema describes a 10-variant enum declaratively; further splitting would fragment a single cohesive schema description without clarity gain.</CLOG>

use super::cls_bindable_u16::BindableU16;
use super::cls_style_region::{CellCoord, ModuloAxis, StyleRegion};
use tui_vfx_core::{ConfigSchema, SchemaField, SchemaNode, SchemaVariant};

fn prim(name: &str) -> SchemaNode {
    SchemaNode::Primitive {
        type_name: name.to_string(),
        range: None,
    }
}

fn plain_field(name: &str, schema: SchemaNode) -> SchemaField {
    SchemaField::new(name, schema, Default::default())
}

impl ConfigSchema for StyleRegion {
    fn schema() -> SchemaNode {
        SchemaNode::Enum {
            name: "StyleRegion".to_string(),
            description: Some(
                "Targeting: which cells in a widget receive style effects".to_string(),
            ),
            json_name: None,
            tag_field: None,
            variants: vec![
                SchemaVariant::Unit {
                    name: "All".to_string(),
                    description: Some("Apply to every cell".to_string()),
                    json_value: None,
                },
                SchemaVariant::Tuple {
                    name: "Role".to_string(),
                    description: Some(
                        "Apply to cells whose semantic role (from the source's RoleMap) matches \
                         the given RoleTag. Legacy JSON strings `BorderOnly`, `TextOnly`, and \
                         `BackgroundOnly` parse into this variant via a custom Deserialize impl."
                            .to_string(),
                    ),
                    json_value: None,
                    items: vec![plain_field(
                        "role",
                        SchemaNode::Opaque {
                            type_name: "RoleTag".to_string(),
                        },
                    )],
                },
                SchemaVariant::Tuple {
                    name: "Rows".to_string(),
                    description: Some("Apply to the given row indices".to_string()),
                    json_value: None,
                    items: vec![plain_field(
                        "rows",
                        SchemaNode::Vec {
                            item: Box::new(prim("u16")),
                        },
                    )],
                },
                SchemaVariant::Struct {
                    name: "RowRange".to_string(),
                    description: Some("Apply to rows [start, end)".to_string()),
                    json_value: None,
                    fields: vec![
                        plain_field("start", prim("u16")),
                        plain_field("end", prim("u16")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Cell".to_string(),
                    description: Some("Apply to a single cell at (x, y)".to_string()),
                    json_value: None,
                    fields: vec![
                        plain_field("x", BindableU16::schema()),
                        plain_field("y", BindableU16::schema()),
                    ],
                },
                SchemaVariant::Tuple {
                    name: "Cells".to_string(),
                    description: Some("Apply to a list of cells".to_string()),
                    json_value: None,
                    items: vec![plain_field(
                        "cells",
                        SchemaNode::Vec {
                            item: Box::new(CellCoord::schema()),
                        },
                    )],
                },
                SchemaVariant::Tuple {
                    name: "Column".to_string(),
                    description: Some("Apply to a single column".to_string()),
                    json_value: None,
                    items: vec![plain_field("column", prim("u16"))],
                },
                SchemaVariant::Tuple {
                    name: "Columns".to_string(),
                    description: Some("Apply to a list of columns".to_string()),
                    json_value: None,
                    items: vec![plain_field(
                        "columns",
                        SchemaNode::Vec {
                            item: Box::new(prim("u16")),
                        },
                    )],
                },
                SchemaVariant::Struct {
                    name: "ColumnRange".to_string(),
                    description: Some("Apply to columns [start, end)".to_string()),
                    json_value: None,
                    fields: vec![
                        plain_field("start", prim("u16")),
                        plain_field("end", prim("u16")),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Modulo".to_string(),
                    description: Some("Apply on a modulo pattern".to_string()),
                    json_value: None,
                    fields: vec![
                        plain_field("axis", ModuloAxis::schema()),
                        plain_field("modulus", prim("u16")),
                        plain_field("remainder", prim("u16")),
                    ],
                },
            ],
        }
    }
}

// <FILE>tui-vfx-style/src/models/fnc_style_region_schema.rs</FILE> - <DESC>Hand-written ConfigSchema impl for StyleRegion</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
