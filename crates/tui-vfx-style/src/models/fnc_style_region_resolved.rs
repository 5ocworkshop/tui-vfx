// <FILE>tui-vfx-style/src/models/fnc_style_region_resolved.rs</FILE> - <DESC>Resolve any BindableU16 fields on a StyleRegion against a frame's runtime parameters</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 3b: lift BindableU16 into RowRange/ColumnRange/Modulo. Resolution logic moves into its own sibling so cls_style_region.rs stays close to its OFPF size budget while gaining four resolvable variants.</WCTX>
// <CLOG>Initial extraction from cls_style_region.rs::resolved + new arms for RowRange / ColumnRange / Modulo. Borrows when every BindableU16 field is already a Literal; clones once to a Cow::Owned with all fields lowered to Literal otherwise.</CLOG>

//! `resolved`: lower any `BindableU16::Binding` fields on a `StyleRegion` to
//! `BindableU16::Literal` by evaluating against the current frame's
//! `ShaderRuntimeParams`.
//!
//! The function is invoked once per layer per frame from the render pipeline
//! (`render_loop`, `render_loop_inspected`, `render_pipeline_with_shadow`).
//! It is intentionally a free function so the per-frame caller can decide
//! whether to reuse a borrowed region or pay the clone cost only when
//! resolution is actually needed.
//!
//! # Behavior by variant
//!
//! | Variant       | Resolves                                          |
//! |---------------|---------------------------------------------------|
//! | `Cell`        | `x`, `y`                                          |
//! | `RowRange`    | `start`, `end`                                    |
//! | `ColumnRange` | `start`, `end`                                    |
//! | `Modulo`      | `modulus`, `remainder` (axis is not bindable)     |
//! | all others    | unchanged (`Cow::Borrowed(self)`)                 |
//!
//! Missing bindings fall back to `0` (matches the pre-Phase-3b behavior of
//! `Cell` and the contract documented on `BindableU16::evaluate`). Any
//! variant with all-literal fields short-circuits to `Cow::Borrowed`.

use std::borrow::Cow;

use super::cls_bindable_u16::BindableU16;
use super::cls_style_region::StyleRegion;
use crate::traits::ShaderRuntimeParams;

/// Return a copy of `region` with any `BindableU16::Binding` fields lowered
/// to `BindableU16::Literal` against `runtime_params`. Borrows when no
/// resolution is needed.
pub fn resolved<'a>(
    region: &'a StyleRegion,
    runtime_params: &ShaderRuntimeParams,
) -> Cow<'a, StyleRegion> {
    match region {
        StyleRegion::Cell { x, y } => {
            if both_literal(x, y) {
                Cow::Borrowed(region)
            } else {
                Cow::Owned(StyleRegion::Cell {
                    x: lower(x, runtime_params),
                    y: lower(y, runtime_params),
                })
            }
        }
        StyleRegion::RowRange { start, end } => {
            if both_literal(start, end) {
                Cow::Borrowed(region)
            } else {
                Cow::Owned(StyleRegion::RowRange {
                    start: lower(start, runtime_params),
                    end: lower(end, runtime_params),
                })
            }
        }
        StyleRegion::ColumnRange { start, end } => {
            if both_literal(start, end) {
                Cow::Borrowed(region)
            } else {
                Cow::Owned(StyleRegion::ColumnRange {
                    start: lower(start, runtime_params),
                    end: lower(end, runtime_params),
                })
            }
        }
        StyleRegion::Modulo {
            axis,
            modulus,
            remainder,
        } => {
            if both_literal(modulus, remainder) {
                Cow::Borrowed(region)
            } else {
                Cow::Owned(StyleRegion::Modulo {
                    axis: *axis,
                    modulus: lower(modulus, runtime_params),
                    remainder: lower(remainder, runtime_params),
                })
            }
        }
        _ => Cow::Borrowed(region),
    }
}

#[inline]
fn both_literal(a: &BindableU16, b: &BindableU16) -> bool {
    matches!(
        (a, b),
        (BindableU16::Literal(_), BindableU16::Literal(_))
    )
}

#[inline]
fn lower(b: &BindableU16, runtime_params: &ShaderRuntimeParams) -> BindableU16 {
    BindableU16::Literal(b.evaluate(runtime_params).unwrap_or(0))
}

// <FILE>tui-vfx-style/src/models/fnc_style_region_resolved.rs</FILE> - <DESC>Resolve BindableU16 fields on StyleRegion</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
