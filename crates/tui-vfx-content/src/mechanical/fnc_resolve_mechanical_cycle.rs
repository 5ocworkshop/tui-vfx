// <FILE>crates/tui-vfx-content/src/mechanical/fnc_resolve_mechanical_cycle.rs</FILE> - <DESC>Compose preset expansion, deterministic shuffle, and face normalization into a ResolvedMechanicalCycle</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2 of mechanical circular content cycles plan: source config to ResolvedMechanicalCycle resolver consumed by route_between.</WCTX>
// <CLOG>0.1.0: introduce resolve_mechanical_cycle handling Pair, Ordered, Preset, Randomized, Weighted; enforce duplicate-face and circular-min-size rules.</CLOG>

use std::collections::HashSet;

use tui_vfx_style::models::BindableString;
use tui_vfx_style::traits::ShaderRuntimeParams;

use crate::fonts::FontRegistry;
use crate::types::{CycleWrapMode, MechanicalContentSource};

use super::cls_resolved_cycle::{ResolvedMechanicalCycle, ResolvedMechanicalFace};
use super::enum_cycle_error::MechanicalCycleError;
use super::fnc_expand_cycle_preset::expand_cycle_preset;
use super::fnc_normalize_cycle_face::normalize_cycle_face;
use super::fnc_weighted_cycle_order::{shuffle_in_place, weighted_cycle_order};
use super::types::MechanicalTile;

/// Resolve a `MechanicalContentSource` into a `ResolvedMechanicalCycle`
/// for the given mechanism tile size.
///
/// `Pair` resolves to an empty cycle; the route helper will inject
/// source and target endpoints when building a route. The non-Pair
/// variants resolve to a fully populated, validated cycle whose face
/// grids are normalized to `tile`.
///
/// Backward-compatible entry point: forwards to `resolve_mechanical_
/// cycle_with_context` with a default font registry and an empty
/// runtime-params map. Callers that want font expansion via a host-
/// supplied registry, or that want bindable-font resolution against
/// runtime params, use the `_with_context` variant directly.
pub(crate) fn resolve_mechanical_cycle(
    source: &MechanicalContentSource,
    tile: MechanicalTile,
) -> Result<ResolvedMechanicalCycle, MechanicalCycleError> {
    let registry = FontRegistry::new();
    let params = ShaderRuntimeParams::new();
    resolve_mechanical_cycle_with_context(source, tile, &registry, &params)
}

/// Font-aware cycle resolution.
///
/// Identical to [`resolve_mechanical_cycle`] except for the `Preset`
/// path, which when `font` is set on the variant expands each preset
/// face string through the resolved font's glyph table before
/// normalization. Resolution precedence for the font:
///
/// 1. `BindableString::Literal(name)` resolves directly against
///    `font_registry` with implicit fallback to the registry's default
///    per Intention 36.
/// 2. `BindableString::Binding(key)` looks up `key` in
///    `runtime_params`; if a Text value is present, that name resolves
///    against `font_registry` (with default fallback). If the binding
///    is absent or the parameter is the wrong type, falls back to the
///    registry's default font.
/// 3. The reserved sentinel `default_font` always routes to the
///    registry's currently-registered default.
pub(crate) fn resolve_mechanical_cycle_with_context(
    source: &MechanicalContentSource,
    tile: MechanicalTile,
    font_registry: &FontRegistry,
    runtime_params: &ShaderRuntimeParams,
) -> Result<ResolvedMechanicalCycle, MechanicalCycleError> {
    match source {
        MechanicalContentSource::Pair => Ok(ResolvedMechanicalCycle {
            faces: Vec::new(),
            wrap: CycleWrapMode::Bounded,
        }),
        MechanicalContentSource::Ordered { faces, wrap } => {
            resolve_from_face_strings(faces, *wrap, tile)
        }
        MechanicalContentSource::Preset { preset, wrap, font } => {
            let raw_faces = expand_cycle_preset(*preset);
            let faces = match font {
                None => raw_faces,
                Some(bindable) => {
                    let table = resolve_font_table(bindable, font_registry, runtime_params);
                    raw_faces.iter().map(|s| table.render_text(s)).collect()
                }
            };
            resolve_from_face_strings(&faces, *wrap, tile)
        }
        MechanicalContentSource::Randomized { faces, seed, wrap } => {
            let mut shuffled = faces.clone();
            shuffle_in_place(&mut shuffled, *seed);
            resolve_from_face_strings(&shuffled, *wrap, tile)
        }
        MechanicalContentSource::Weighted { faces, seed, wrap } => {
            let order = weighted_cycle_order(faces, *seed)?;
            // weighted_cycle_order intentionally retains duplicates
            // (a face with weight 3 appears 3 times). Skip the
            // duplicate-face rejection in resolve_from_face_strings by
            // building the resolved cycle directly here.
            if order.is_empty() {
                return Err(MechanicalCycleError::EmptyFaces);
            }
            let mut resolved_faces = Vec::with_capacity(order.len());
            for value in order {
                let grid = normalize_cycle_face(&value, tile)?;
                resolved_faces.push(ResolvedMechanicalFace { value, grid });
            }
            if matches!(wrap, CycleWrapMode::Circular) && distinct_value_count(&resolved_faces) < 2
            {
                return Err(MechanicalCycleError::CircularRequiresAtLeastTwoFaces);
            }
            Ok(ResolvedMechanicalCycle {
                faces: resolved_faces,
                wrap: *wrap,
            })
        }
    }
}

fn resolve_from_face_strings(
    faces: &[String],
    wrap: CycleWrapMode,
    tile: MechanicalTile,
) -> Result<ResolvedMechanicalCycle, MechanicalCycleError> {
    if faces.is_empty() {
        return Err(MechanicalCycleError::EmptyFaces);
    }
    let mut seen: HashSet<&str> = HashSet::with_capacity(faces.len());
    let mut resolved = Vec::with_capacity(faces.len());
    for value in faces {
        if value.is_empty() {
            return Err(MechanicalCycleError::EmptyFaceValue);
        }
        if !seen.insert(value.as_str()) {
            return Err(MechanicalCycleError::DuplicateFace {
                value: value.clone(),
            });
        }
        let grid = normalize_cycle_face(value, tile)?;
        resolved.push(ResolvedMechanicalFace {
            value: value.clone(),
            grid,
        });
    }
    if matches!(wrap, CycleWrapMode::Circular) && resolved.len() < 2 {
        return Err(MechanicalCycleError::CircularRequiresAtLeastTwoFaces);
    }
    Ok(ResolvedMechanicalCycle {
        faces: resolved,
        wrap,
    })
}

fn distinct_value_count(faces: &[ResolvedMechanicalFace]) -> usize {
    let mut seen: HashSet<&str> = HashSet::with_capacity(faces.len());
    for f in faces {
        seen.insert(f.value.as_str());
    }
    seen.len()
}

/// Resolve a [`BindableString`] font reference against the registry +
/// runtime params.
///
/// Always returns a glyph table by falling back to the registry's
/// registered default per Intention 36 — missing fonts degrade to the
/// project's canonical Line 3x3 face rather than failing the recipe.
fn resolve_font_table(
    bindable: &BindableString,
    registry: &FontRegistry,
    runtime_params: &ShaderRuntimeParams,
) -> crate::fonts::FontGlyphTable {
    let resolved_name = bindable
        .evaluate(runtime_params)
        .map(|s| s.to_string())
        .unwrap_or_else(|| crate::fonts::DEFAULT_FONT_SENTINEL.to_string());
    registry.resolve_or_default(&resolved_name)
}

#[cfg(test)]
mod tests {
    use super::super::fnc_grid_text::grid_to_text;
    use super::*;
    use crate::types::{MechanicalCyclePreset, WeightedCycleFace};

    fn tile(w: u16, h: u16) -> MechanicalTile {
        MechanicalTile::new(w, h).unwrap()
    }

    fn values(cycle: &ResolvedMechanicalCycle) -> Vec<&str> {
        cycle.faces.iter().map(|f| f.value.as_str()).collect()
    }

    #[test]
    fn pair_resolves_to_empty_face_list() {
        let cycle = resolve_mechanical_cycle(&MechanicalContentSource::Pair, tile(1, 1)).unwrap();
        assert!(cycle.faces.is_empty());
    }

    #[test]
    fn ordered_three_faces_preserves_order() {
        let src = MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "B".into(), "C".into()],
            wrap: CycleWrapMode::Circular,
        };
        let cycle = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap();
        assert_eq!(values(&cycle), vec!["A", "B", "C"]);
        assert_eq!(cycle.wrap, CycleWrapMode::Circular);
    }

    #[test]
    fn ordered_face_grid_padded_to_tile() {
        let src = MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "B".into()],
            wrap: CycleWrapMode::Circular,
        };
        let cycle = resolve_mechanical_cycle(&src, tile(3, 1)).unwrap();
        assert_eq!(grid_to_text(&cycle.faces[0].grid), "A  ");
        assert_eq!(grid_to_text(&cycle.faces[1].grid), "B  ");
    }

    #[test]
    fn preset_decimal_digits_resolves_to_ten_faces() {
        let src = MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: None,
        };
        let cycle = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap();
        assert_eq!(cycle.faces.len(), 10);
        assert_eq!(
            values(&cycle),
            vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
        );
    }

    #[test]
    fn preset_with_literal_line_3x3_font_expands_to_3x3_glyphs() {
        // With font = Literal("line-3x3"), each preset digit expands
        // through the Line 3x3 glyph table to a 3x3 multi-line face.
        // The cycle is then normalized against a 3x3 tile.
        let src = MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: Some(BindableString::Literal("line-3x3".to_string())),
        };
        let cycle = resolve_mechanical_cycle(&src, tile(3, 3)).unwrap();
        assert_eq!(cycle.faces.len(), 10);
        // The face VALUE is the multi-line glyph string the preset
        // expanded to; the grid is the same content normalized to 3×3.
        // First face is "0" rendered as the canonical Line 3x3 box.
        let zero_value = &cycle.faces[0].value;
        let zero_lines: Vec<&str> = zero_value.lines().collect();
        assert_eq!(zero_lines.len(), 3);
        assert_eq!(zero_lines[0], "┏━┓");
        assert_eq!(zero_lines[1], "┃ ┃");
        assert_eq!(zero_lines[2], "┗━┛");
    }

    #[test]
    fn preset_with_default_font_sentinel_falls_back_to_registered_default() {
        // BindableString::Literal("default_font") routes through the
        // registry's sentinel resolver to whatever the registered
        // default is — Line 3x3 by construction.
        let src = MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: Some(BindableString::Literal("default_font".to_string())),
        };
        let cycle = resolve_mechanical_cycle(&src, tile(3, 3)).unwrap();
        // Same expansion as the named "line-3x3" path because the
        // registry's default is line-3x3.
        assert_eq!(cycle.faces[0].value.lines().count(), 3);
        assert_eq!(cycle.faces[0].value.lines().next(), Some("┏━┓"));
    }

    #[test]
    fn preset_with_unknown_font_name_falls_back_to_default() {
        // Per Intention 36: missing font names degrade to the registry's
        // default rather than failing the recipe.
        let src = MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: Some(BindableString::Literal("does-not-exist".to_string())),
        };
        let cycle = resolve_mechanical_cycle(&src, tile(3, 3)).unwrap();
        // Falls back to Line 3x3 — first face is the "0" box glyph.
        assert_eq!(cycle.faces[0].value.lines().next(), Some("┏━┓"));
    }

    #[test]
    fn preset_font_binding_falls_back_to_default_without_runtime_params() {
        // BindableString::Binding requires runtime_params to evaluate.
        // The default-arg `resolve_mechanical_cycle` passes an empty
        // ShaderRuntimeParams, so the binding evaluates to None and the
        // resolver falls back to the registry default.
        let src = MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: Some(BindableString::Binding("drum_font".to_string())),
        };
        let cycle = resolve_mechanical_cycle(&src, tile(3, 3)).unwrap();
        assert_eq!(cycle.faces[0].value.lines().next(), Some("┏━┓"));
    }

    #[test]
    fn preset_font_binding_resolves_via_runtime_params() {
        // With the with_context entry point and a runtime-params map
        // that supplies a string for the binding key, the bindable
        // resolves to the named font and the cycle expands accordingly.
        let src = MechanicalContentSource::Preset {
            preset: MechanicalCyclePreset::DecimalDigits,
            wrap: CycleWrapMode::Circular,
            font: Some(BindableString::Binding("drum_font".to_string())),
        };
        let registry = FontRegistry::new();
        let mut params = ShaderRuntimeParams::new();
        params.insert("drum_font", "line-3x3".to_string());
        let cycle =
            resolve_mechanical_cycle_with_context(&src, tile(3, 3), &registry, &params).unwrap();
        assert_eq!(cycle.faces[0].value.lines().next(), Some("┏━┓"));
    }

    #[test]
    fn duplicate_ordered_face_rejected() {
        let src = MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "B".into(), "A".into()],
            wrap: CycleWrapMode::Circular,
        };
        let err = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap_err();
        assert!(
            matches!(err, MechanicalCycleError::DuplicateFace { ref value } if value == "A"),
            "{err:?}",
        );
    }

    #[test]
    fn empty_ordered_face_list_rejected() {
        let src = MechanicalContentSource::Ordered {
            faces: vec![],
            wrap: CycleWrapMode::Circular,
        };
        let err = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap_err();
        assert_eq!(err, MechanicalCycleError::EmptyFaces);
    }

    #[test]
    fn circular_with_one_face_rejected() {
        let src = MechanicalContentSource::Ordered {
            faces: vec!["A".into()],
            wrap: CycleWrapMode::Circular,
        };
        let err = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap_err();
        assert_eq!(err, MechanicalCycleError::CircularRequiresAtLeastTwoFaces);
    }

    #[test]
    fn bounded_with_one_face_is_allowed() {
        let src = MechanicalContentSource::Ordered {
            faces: vec!["A".into()],
            wrap: CycleWrapMode::Bounded,
        };
        let cycle = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap();
        assert_eq!(values(&cycle), vec!["A"]);
        assert_eq!(cycle.wrap, CycleWrapMode::Bounded);
    }

    #[test]
    fn randomized_is_deterministic_per_seed() {
        let src = MechanicalContentSource::Randomized {
            faces: vec!["A".into(), "B".into(), "C".into(), "D".into()],
            seed: 42,
            wrap: CycleWrapMode::Circular,
        };
        let a = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap();
        let b = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap();
        assert_eq!(values(&a), values(&b));
    }

    #[test]
    fn weighted_resolves_with_multiplicity() {
        let src = MechanicalContentSource::Weighted {
            faces: vec![
                WeightedCycleFace {
                    value: "7".into(),
                    weight: 1,
                },
                WeightedCycleFace {
                    value: "$".into(),
                    weight: 3,
                },
            ],
            seed: 99,
            wrap: CycleWrapMode::Circular,
        };
        let cycle = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap();
        assert_eq!(cycle.faces.len(), 4);
        let count_seven = values(&cycle).iter().filter(|v| **v == "7").count();
        let count_dollar = values(&cycle).iter().filter(|v| **v == "$").count();
        assert_eq!(count_seven, 1);
        assert_eq!(count_dollar, 3);
    }

    #[test]
    fn ordered_oversized_face_is_rejected() {
        let src = MechanicalContentSource::Ordered {
            faces: vec!["A".into(), "ABC".into()],
            wrap: CycleWrapMode::Bounded,
        };
        let err = resolve_mechanical_cycle(&src, tile(1, 1)).unwrap_err();
        assert!(matches!(err, MechanicalCycleError::FaceExceedsTile { .. }));
    }
}

// <FILE>crates/tui-vfx-content/src/mechanical/fnc_resolve_mechanical_cycle.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
