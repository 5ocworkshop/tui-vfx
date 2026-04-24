// <FILE>tui-vfx-style/src/models/fnc_style_region_deserialize.rs</FILE> - <DESC>Custom Deserialize for StyleRegion — back-compat for legacy bare-string variants (BorderOnly / TextOnly / BackgroundOnly) mapped to canonical Role(RoleTag) form</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.2 audit-1 remediation — extract Deserialize + shadow enum out of cls_style_region.rs to bring the main file back under OFPF cls_ LOC budget</WCTX>
// <CLOG>0.1.0: extracted from cls_style_region.rs (prev lines 219–320); logic bit-preserved. Custom Deserialize captures input as serde_json::Value, short-circuits the three legacy bare strings to Role(...), and forwards every other variant through a shadow enum that mirrors StyleRegion's canonical variants.</CLOG>

use super::cls_bindable_u16::BindableU16;
use super::cls_style_region::{CellCoord, ModuloAxis, StyleRegion};
use serde::Deserialize;
use tui_vfx_types::RoleTag;

/// Shadow enum mirroring `StyleRegion`'s canonical variants exactly.
///
/// Used only by the manual `Deserialize` impl so serde can derive a
/// PascalCase-tagged parse for every variant EXCEPT the three legacy
/// bare strings (`BorderOnly`, `TextOnly`, `BackgroundOnly`), which are
/// handled by `deserialize` before the shadow is invoked.
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
enum StyleRegionShadow {
    All,
    Role(RoleTag),
    Rows(Vec<u16>),
    RowRange {
        start: u16,
        end: u16,
    },
    Cell {
        x: BindableU16,
        y: BindableU16,
    },
    Cells(Vec<CellCoord>),
    Column(u16),
    Columns(Vec<u16>),
    ColumnRange {
        start: u16,
        end: u16,
    },
    Modulo {
        axis: ModuloAxis,
        modulus: u16,
        remainder: u16,
    },
}

impl From<StyleRegionShadow> for StyleRegion {
    fn from(s: StyleRegionShadow) -> Self {
        match s {
            StyleRegionShadow::All => StyleRegion::All,
            StyleRegionShadow::Role(t) => StyleRegion::Role(t),
            StyleRegionShadow::Rows(v) => StyleRegion::Rows(v),
            StyleRegionShadow::RowRange { start, end } => StyleRegion::RowRange { start, end },
            StyleRegionShadow::Cell { x, y } => StyleRegion::Cell { x, y },
            StyleRegionShadow::Cells(v) => StyleRegion::Cells(v),
            StyleRegionShadow::Column(c) => StyleRegion::Column(c),
            StyleRegionShadow::Columns(v) => StyleRegion::Columns(v),
            StyleRegionShadow::ColumnRange { start, end } => {
                StyleRegion::ColumnRange { start, end }
            }
            StyleRegionShadow::Modulo {
                axis,
                modulus,
                remainder,
            } => StyleRegion::Modulo {
                axis,
                modulus,
                remainder,
            },
        }
    }
}

impl<'de> Deserialize<'de> for StyleRegion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let value = serde_json::Value::deserialize(deserializer)?;

        if let serde_json::Value::String(s) = &value {
            match s.as_str() {
                "BorderOnly" => return Ok(StyleRegion::Role(RoleTag::Border)),
                "TextOnly" => return Ok(StyleRegion::Role(RoleTag::Text)),
                "BackgroundOnly" => return Ok(StyleRegion::Role(RoleTag::Background)),
                _ => { /* fall through to shadow parse */ }
            }
        }

        let shadow: StyleRegionShadow = serde_json::from_value(value).map_err(D::Error::custom)?;
        Ok(shadow.into())
    }
}

// <FILE>tui-vfx-style/src/models/fnc_style_region_deserialize.rs</FILE> - <DESC>Custom Deserialize for StyleRegion</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
