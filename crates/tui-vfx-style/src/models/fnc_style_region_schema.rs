// <FILE>tui-vfx-style/src/models/fnc_style_region_schema.rs</FILE> - <DESC>Hand-written ConfigSchema impl for StyleRegion. The hand-written impl freezes json_value=None across all variants, deliberately diverging from what the live derive would emit under the enum's #[serde(rename_all = "PascalCase")] attribute.</DESC>
// <VERS>VERSION: 0.2.1</VERS>
// <WCTX>Packet 1.9.A.followup US-008: lift the hand-written StyleRegion ConfigSchema impl with intentional-divergence-from-derive-output justification. The prior file DESC claimed the impl was hand-written because "RoleTag in tui-vfx-types doesn't depend on tui-vfx-core" — that claim was stale (tui-vfx-types DOES declare tui-vfx-core as a dependency per its Cargo.toml, and RoleTag has its own ConfigSchema impl). The real reason the impl is hand-written is a deliberate schema-shape divergence around json_value emission.</WCTX>
// <CLOG>0.2.1: PATCH — add CONFIGSCHEMA-JUSTIFICATION comment above the hand-written impl (kind=intentional-divergence-from-derive-output). Comment also corrects the baseline file's prior stale claim about cross-crate-trait-dep. Update file DESC to name the actual reason. No behavior change to schema output.</CLOG>

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

// CONFIGSCHEMA-JUSTIFICATION: intentional-divergence-from-derive-output: the StyleRegion enum carries `#[serde(rename_all = "PascalCase")]` (cls_style_region.rs:80), so the live derive at tui-vfx-core-macros would emit `json_value: Some("All")`, `Some("Role")`, etc. on every variant. The hand-written impl here freezes `json_value: None` across all variants — a deliberate schema-shape choice that downstream consumers (capabilities.toml, generated docs) currently rely on. Migration is feasible but produces a non-trivial schema diff that requires a separate decision packet. Note: the baseline file's prior `note` claiming "RoleTag in tui-vfx-types which does not depend on tui-vfx-core" was stale — tui-vfx-types/Cargo.toml does declare that dependency, and RoleTag has its own ConfigSchema impl, so the cross-crate-trait-dep rationale never applied.
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
                        plain_field("start", BindableU16::schema()),
                        plain_field("end", BindableU16::schema()),
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
                        plain_field("start", BindableU16::schema()),
                        plain_field("end", BindableU16::schema()),
                    ],
                },
                SchemaVariant::Struct {
                    name: "Modulo".to_string(),
                    description: Some("Apply on a modulo pattern".to_string()),
                    json_value: None,
                    fields: vec![
                        plain_field("axis", ModuloAxis::schema()),
                        plain_field("modulus", BindableU16::schema()),
                        plain_field("remainder", BindableU16::schema()),
                    ],
                },
            ],
        }
    }
}

// <FILE>tui-vfx-style/src/models/fnc_style_region_schema.rs</FILE> - <DESC>Hand-written ConfigSchema impl for StyleRegion</DESC>
// <VERS>END OF VERSION: 0.2.1</VERS>
