// <FILE>tui-vfx-content/src/transformers/cls_odometer.rs</FILE> - <DESC>Odometer transformer with optional mechanical cycle path covering ordered drums, slot reels, and per-tile spring settle</DESC>
// <VERS>VERSION: 4.1.0</VERS>
// <WCTX>Phase C of TransformContext implementation plan: wire ctx.runtime_params through roll_cycle into resolve_mechanical_cycle_with_context so binding-form fonts resolve at runtime.</WCTX>
// <CLOG>4.1.0: drop underscore on ctx parameter; route ctx.runtime_params through roll_cycle to resolve_mechanical_cycle_with_context, enabling host-supplied font bindings to reach the resolver.</CLOG>

use crate::fonts::FontRegistry;
use crate::mechanical::{
    MechanicalSizing, MechanicalSource, MechanicalTile, NumericRouteHint, TileScheduleMeta,
    blit_tile_grid, extract_tile_text, grid_from_text, grid_to_text, overshoot_face_for,
    paired_grids, resolve_mechanical_cycle_with_context, roll_cycle_window, roll_grid_window,
    route_between, settle_sample_for, tile_progress_for, tile_rects,
};
use crate::traits::{TextTransformer, TransformContext};
use crate::types::{
    MechanicalCascadePolicy, MechanicalContentSource, MechanicalCycleConfig, OdometerDirection,
    OdometerTravel,
};
use std::borrow::Cow;
use tui_vfx_style::traits::ShaderRuntimeParams;
use tui_vfx_types::{Grid, OwnedGrid};

#[derive(Debug, Clone)]
pub struct Odometer {
    direction: OdometerDirection,
    travel: OdometerTravel,
    tile_width: u16,
    tile_height: u16,
    from_message: Option<String>,
    mechanical: Option<MechanicalCycleConfig>,
}

impl Odometer {
    pub fn new(
        direction: OdometerDirection,
        travel: OdometerTravel,
        tile_width: u16,
        tile_height: u16,
        from_message: Option<String>,
    ) -> Self {
        Self {
            direction,
            travel,
            tile_width,
            tile_height,
            from_message,
            mechanical: None,
        }
    }

    /// Builder: attach a mechanical cycle config. `None` and explicit
    /// `MechanicalCycleConfig::default()` (Pair source, Simultaneous
    /// cascade, no settle) both produce the legacy whole-grid roll
    /// behavior byte-for-byte.
    pub fn with_mechanical(mut self, mechanical: Option<MechanicalCycleConfig>) -> Self {
        self.mechanical = mechanical;
        self
    }
}

impl TextTransformer for Odometer {
    /// Apply the odometer effect at the given `progress` (0.0 = full source,
    /// 1.0 = full target).
    ///
    /// `ctx.runtime_params` is forwarded to the cycle resolver so that
    /// binding-form font references (e.g. `font: { "binding": "drum_font" }`)
    /// resolve to the host-supplied font name at runtime. When the binding is
    /// absent from `runtime_params`, the resolver falls back to the registry's
    /// registered default per [Intention 36][int36].
    ///
    /// [int36]: crate
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        ctx: &TransformContext<'_>,
    ) -> Cow<'a, str> {
        if progress >= 1.0 {
            return Cow::Borrowed(target);
        }
        let Some(tile) = MechanicalTile::new(self.tile_width, self.tile_height) else {
            return Cow::Borrowed(target);
        };

        match self.mechanical.as_ref() {
            // Absent or default-Pair → legacy whole-grid roll.
            None => self.roll_legacy_pair(target, progress, tile),
            Some(cfg) if is_legacy_equivalent(cfg) => self.roll_legacy_pair(target, progress, tile),
            Some(cfg) => self.roll_cycle(target, progress, tile, cfg, ctx.runtime_params),
        }
    }
}

impl Odometer {
    fn roll_legacy_pair<'a>(
        &self,
        target: &'a str,
        progress: f64,
        tile: MechanicalTile,
    ) -> Cow<'a, str> {
        let source = paired_grids(
            self.from_message.as_deref(),
            target,
            MechanicalSizing::PadToMax,
        );
        let grid = roll_grid_window(&source, progress, self.direction, self.travel, tile);
        Cow::Owned(grid_to_text(&grid))
    }

    fn roll_cycle<'a>(
        &self,
        target: &'a str,
        progress: f64,
        tile: MechanicalTile,
        cfg: &MechanicalCycleConfig,
        runtime_params: &ShaderRuntimeParams,
    ) -> Cow<'a, str> {
        // Build paired source/target grids using the same padding rules
        // as the legacy path so the cycle path agrees on grid extents.
        let paired = paired_grids(
            self.from_message.as_deref(),
            target,
            MechanicalSizing::PadToMax,
        );
        let grid_w = paired.from.width().max(paired.to.width());
        let grid_h = paired.from.height().max(paired.to.height());

        // Construct the font registry once per roll_cycle invocation —
        // not inside the per-tile loop below. FontRegistry::new() only
        // registers the embedded Line 3x3 default and is the only
        // allocation-introducing part of this slice.
        let registry = FontRegistry::new();

        // Resolve the cycle once. Recipe-load-time validation is the
        // user-facing surface; runtime defensively falls back to the
        // legacy path if resolution fails.
        let Ok(cycle) =
            resolve_mechanical_cycle_with_context(&cfg.source, tile, &registry, runtime_params)
        else {
            return self.roll_legacy_pair(target, progress, tile);
        };

        let rects = tile_rects(grid_w, grid_h, tile);
        if rects.is_empty() {
            return Cow::Borrowed(target);
        }

        let mut output = OwnedGrid::new(grid_w, grid_h);

        // Compute change-set and LSB ordering before the per-tile loop
        // so cascade can reference it in O(1) per tile.
        let from_faces: Vec<String> = rects
            .iter()
            .map(|r| extract_tile_text(&paired.from, *r, tile))
            .collect();
        let to_faces: Vec<String> = rects
            .iter()
            .map(|r| extract_tile_text(&paired.to, *r, tile))
            .collect();
        let changed: Vec<bool> = from_faces
            .iter()
            .zip(to_faces.iter())
            .map(|(f, t)| f != t)
            .collect();
        let total_changed = changed.iter().filter(|c| **c).count();
        let mut lsb_indices = vec![0usize; rects.len()];
        // LSB ordering: last changed tile in row-major iteration is
        // the LSB (lsb_index = 0); first changed tile is the MSB.
        let mut counter = 0usize;
        for i in (0..rects.len()).rev() {
            if changed[i] {
                lsb_indices[i] = counter;
                counter += 1;
            }
        }

        // For NumericDelta direction, compute the multi-tile carry sign
        // once from the source/target numeric strings.
        let numeric_hint =
            numeric_carry_hint(&self.from_message, target, &cfg.source, &cfg.cascade);

        for (i, rect) in rects.iter().enumerate() {
            let from_face = &from_faces[i];
            let to_face = &to_faces[i];

            let meta = TileScheduleMeta {
                tile_index: i,
                total_tiles: rects.len(),
                changed: changed[i],
                changed_lsb_index: lsb_indices[i],
                total_changed,
            };
            let local_progress = tile_progress_for(&cfg.cascade, progress, meta);
            let settle = settle_sample_for(&cfg.settle, local_progress);

            // Resolve route per tile; fall back to single-face render
            // (target) on any resolution error rather than crashing.
            let route_result = if matches!(cfg.source, MechanicalContentSource::Pair) {
                // Pair fast path: synthesize a [from, to] route with
                // both faces normalized via grid_from_text padding.
                pair_route_for_tile(from_face, to_face, tile)
            } else {
                route_between(&cycle, from_face, to_face, cfg.route, numeric_hint, tile).ok()
            };

            let Some(route) = route_result else {
                // Route resolution failed for this tile (e.g., source
                // and target faces aren't both in the cycle and the
                // missing-face policy is Error). Degrade to a legacy
                // pair roll for this tile only — same window motion,
                // same tile size, just no intermediate faces — so the
                // tile animates from source to target rather than
                // freezing at one face.
                let pair_source = MechanicalSource {
                    from: grid_from_text(from_face, MechanicalSizing::PadToMax),
                    to: grid_from_text(to_face, MechanicalSizing::PadToMax),
                };
                let tile_grid =
                    roll_grid_window(&pair_source, progress, self.direction, self.travel, tile);
                blit_tile_grid(&mut output, &tile_grid, *rect, tile);
                continue;
            };

            let overshoot_grid = if matches!(cfg.source, MechanicalContentSource::Pair) {
                None
            } else {
                overshoot_face_for(&cycle, &route).cloned()
            };

            let tile_grid = roll_cycle_window(
                &route,
                settle,
                overshoot_grid.as_ref(),
                self.direction,
                self.travel,
                tile,
            );
            blit_tile_grid(&mut output, &tile_grid, *rect, tile);
        }

        Cow::Owned(grid_to_text(&output))
    }
}

fn is_legacy_equivalent(cfg: &MechanicalCycleConfig) -> bool {
    matches!(cfg.source, MechanicalContentSource::Pair)
        && matches!(cfg.cascade, MechanicalCascadePolicy::Simultaneous)
        && matches!(cfg.settle, crate::types::MechanicalSettleConfig::None)
}

fn pair_route_for_tile(
    from_face: &str,
    to_face: &str,
    tile: MechanicalTile,
) -> Option<crate::mechanical::MechanicalCycleRoute> {
    use crate::mechanical::{ResolvedMechanicalFace, normalize_cycle_face};
    let from_grid = normalize_cycle_face(from_face, tile).ok()?;
    let to_grid = normalize_cycle_face(to_face, tile).ok()?;
    Some(crate::mechanical::MechanicalCycleRoute {
        faces: vec![
            ResolvedMechanicalFace {
                value: from_face.to_string(),
                grid: from_grid,
            },
            ResolvedMechanicalFace {
                value: to_face.to_string(),
                grid: to_grid,
            },
        ],
        selected_direction: crate::types::CycleDirectionPolicy::Forward,
    })
}

fn numeric_carry_hint(
    from_message: &Option<String>,
    target: &str,
    source: &MechanicalContentSource,
    cascade: &MechanicalCascadePolicy,
) -> Option<NumericRouteHint> {
    // Only meaningful when the cascade is NumericCarry on a digit cycle.
    if !matches!(cascade, MechanicalCascadePolicy::NumericCarry { .. }) {
        return None;
    }
    let is_digit_cycle = matches!(
        source,
        MechanicalContentSource::Preset {
            preset: crate::types::MechanicalCyclePreset::DecimalDigits,
            ..
        }
    );
    if !is_digit_cycle {
        return None;
    }
    let from = from_message.as_deref()?;
    // Strip newlines so multi-line numeric padding still parses; only
    // the first row is interpreted as the numeric string.
    let from_str: String = from.chars().take_while(|c| *c != '\n').collect();
    let to_str: String = target.chars().take_while(|c| *c != '\n').collect();
    let from_n: i64 = from_str.parse().ok()?;
    let to_n: i64 = to_str.parse().ok()?;
    if to_n >= from_n {
        Some(NumericRouteHint::Increment)
    } else {
        Some(NumericRouteHint::Decrement)
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_odometer.rs</FILE>
// <VERS>END OF VERSION: 4.1.0</VERS>
