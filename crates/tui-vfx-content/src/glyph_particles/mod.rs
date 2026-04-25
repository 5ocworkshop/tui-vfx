// <FILE>crates/tui-vfx-content/src/glyph_particles/mod.rs</FILE> - <DESC>Transient glyph particle emitter built on cell-motion semantics</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Task 24: content-layer glyph spawner for TTE-inspired BinaryPath/Spray/Burst effects.</WCTX>
// <CLOG>0.1.0: add schema types and deterministic pure emitter over SemanticScene.</CLOG>

//! Transient glyph particle emitter for content-local V3 effects.
//!
//! The emitter is the sibling of `cell_motion`: cell motion remaps glyphs that
//! already exist in a source scene, while glyph particles spawn additional
//! transient cells from each selected source actor and sample a `CellMotionSpec`
//! per particle.

use serde::{Deserialize, Serialize};
use tui_vfx_geometry::transitions::interpolate_position;
use tui_vfx_geometry::types::{Position, SnappingStrategy};
use tui_vfx_types::{Color, Grid, OwnedGrid, Rect, RoleTag, SemanticScene};

use crate::cell_motion::{
    CellActor, CellMotionOptions, CellMotionPhaseSpec, CellMotionTiming, CellPlacement,
    CellPlacementContext, CellVisibilityMode, collect_cell_actors, resolve_actor_offset_ms,
    resolve_cell_placement,
};

/// Transient glyph particle emitter.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct GlyphParticleEmitterSpec {
    /// Authored target/origin for spawned particles. `authored` means each source
    /// cell's own coordinate, which is the BinaryPath/Spray default.
    #[serde(default = "default_origin")]
    pub origin: CellPlacement,
    /// Number of particles to spawn per selected source cell.
    pub spawn_count: u16,
    /// Glyph palette sampled deterministically per particle.
    #[serde(default = "default_glyph_palette")]
    pub glyph_palette: Vec<char>,
    /// Foreground palette sampled deterministically per particle. Empty means
    /// inherit the source actor foreground.
    #[serde(default)]
    pub color_palette: Vec<Color>,
    /// Per-particle motion sampled in the same coordinate system as cell motion.
    pub motion: crate::cell_motion::CellMotionSpec,
    /// Maximum particle lifetime after its stagger gate opens.
    pub lifetime_ms: u64,
    /// Behavior after `lifetime_ms` elapses.
    #[serde(default)]
    pub on_complete: ParticleEndBehavior,
    /// Deterministic activation budget.
    #[serde(default)]
    pub concurrency: ParticleConcurrency,
    /// User seed mixed with actor and particle identity.
    #[serde(default)]
    pub seed: u64,
}

impl Default for GlyphParticleEmitterSpec {
    fn default() -> Self {
        Self {
            origin: CellPlacement::Authored,
            spawn_count: 1,
            glyph_palette: default_glyph_palette(),
            color_palette: Vec::new(),
            motion: crate::cell_motion::CellMotionSpec::default(),
            lifetime_ms: 1000,
            on_complete: ParticleEndBehavior::Despawn,
            concurrency: ParticleConcurrency::All,
            seed: 0,
        }
    }
}

/// Particle completion behavior once `lifetime_ms` has elapsed.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ParticleEndBehavior {
    /// Remove the transient glyph.
    #[default]
    Despawn,
    /// Keep the particle at the last sampled motion position.
    FreezeInPlace,
    /// Snap the particle back to its configured origin/target coordinate.
    ConvergeToOrigin,
}

/// Deterministic activation policy for particles.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ParticleConcurrency {
    /// Emit every selected particle.
    #[default]
    All,
    /// Emit particles whose deterministic hash is below `fraction`.
    RandomSample { fraction: f32 },
    /// Emit every Nth particle in deterministic actor/particle order.
    RoundRobin { stride: u16 },
}

/// Aggregate emitter stats for probes and tests.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct GlyphParticleStats {
    pub source_actor_count: u32,
    pub candidate_particle_count: u32,
    pub emitted_particle_count: u32,
    pub skipped_by_concurrency_count: u32,
    pub hidden_before_start_count: u32,
    pub completed_particle_count: u32,
    pub clipped_particle_count: u32,
}

/// Emitted scene plus stats.
#[derive(Clone, Debug)]
pub struct GlyphParticleResult {
    pub scene: SemanticScene,
    pub stats: GlyphParticleStats,
}

/// Apply transient glyph particles over a source scene for one sampled frame.
pub fn emit_glyph_particles(
    scene: &SemanticScene,
    spec: &GlyphParticleEmitterSpec,
    timing: &CellMotionTiming,
    local_frame: Rect,
    options: &CellMotionOptions,
) -> GlyphParticleResult {
    let Some(phase_spec) = spec.motion.phase_spec(timing.phase) else {
        return GlyphParticleResult {
            scene: scene.clone(),
            stats: GlyphParticleStats::default(),
        };
    };
    if spec.spawn_count == 0 || spec.glyph_palette.is_empty() || phase_spec.validate().is_err() {
        return GlyphParticleResult {
            scene: scene.clone(),
            stats: GlyphParticleStats::default(),
        };
    }

    let (source_actors, _) = collect_cell_actors(scene, &phase_spec);
    let mut stats = GlyphParticleStats {
        source_actor_count: source_actors.len() as u32,
        candidate_particle_count: source_actors
            .len()
            .saturating_mul(spec.spawn_count as usize) as u32,
        ..GlyphParticleStats::default()
    };
    let mut out = scene.clone();
    let selected_bounds = selected_bounds(&source_actors, local_frame);
    let ctx = CellPlacementContext {
        local_frame,
        selected_bounds,
    };

    for actor in &source_actors {
        for particle_idx in 0..spec.spawn_count {
            let ordinal = actor
                .selected_ordinal
                .saturating_mul(spec.spawn_count as u32)
                + particle_idx as u32;
            if !particle_active(
                &spec.concurrency,
                actor,
                particle_idx,
                spec.seed,
                options.recipe_or_layer_seed,
            ) {
                stats.skipped_by_concurrency_count += 1;
                continue;
            }
            let particle_actor = particle_actor(
                actor,
                ordinal,
                particle_idx,
                spec,
                &ctx,
                options.recipe_or_layer_seed,
            );
            let offset_ms = resolve_actor_offset_ms(
                &particle_actor,
                &phase_spec.stagger,
                &source_actors,
                &ctx,
                options.recipe_or_layer_seed ^ spec.seed,
            );
            if timing.phase_elapsed_ms < offset_ms {
                stats.hidden_before_start_count += 1;
                continue;
            }
            let age_ms = timing.phase_elapsed_ms - offset_ms;
            let completed = spec.lifetime_ms > 0 && age_ms > spec.lifetime_ms;
            let Some(pos) = particle_position(
                &particle_actor,
                &phase_spec,
                &ctx,
                age_ms,
                completed,
                spec.on_complete,
                &mut stats,
            ) else {
                continue;
            };
            let Some((x, y)) = clip(pos, local_frame) else {
                stats.clipped_particle_count += 1;
                continue;
            };
            if let Some(cell) = out.grid_mut().get_mut(x as usize, y as usize) {
                *cell = particle_actor.cell;
            }
            out.roles_mut().set((x, y), particle_actor.role);
            stats.emitted_particle_count += 1;
        }
    }

    GlyphParticleResult { scene: out, stats }
}

fn particle_actor(
    actor: &CellActor,
    ordinal: u32,
    particle_idx: u16,
    spec: &GlyphParticleEmitterSpec,
    ctx: &CellPlacementContext,
    recipe_seed: u64,
) -> CellActor {
    let origin = resolve_cell_placement(actor, &spec.origin, ctx);
    let glyph_i = (hash64(
        recipe_seed,
        actor.authored_index,
        particle_idx as u32,
        spec.seed,
        b"glyph",
    ) % spec.glyph_palette.len() as u64) as usize;
    let mut cell = actor.cell.with_char(spec.glyph_palette[glyph_i]);
    if !spec.color_palette.is_empty() {
        let color_i = (hash64(
            recipe_seed,
            actor.authored_index,
            particle_idx as u32,
            spec.seed,
            b"color",
        ) % spec.color_palette.len() as u64) as usize;
        cell = cell.with_fg(spec.color_palette[color_i]);
    }
    CellActor {
        authored_index: actor.authored_index.saturating_mul(spec.spawn_count as u32)
            + particle_idx as u32,
        selected_ordinal: ordinal,
        authored_x: origin.x.max(0) as u16,
        authored_y: origin.y.max(0) as u16,
        cell,
        role: RoleTag::Text,
    }
}

fn particle_position(
    actor: &CellActor,
    phase: &CellMotionPhaseSpec,
    ctx: &CellPlacementContext,
    age_ms: u64,
    completed_lifetime: bool,
    end: ParticleEndBehavior,
    stats: &mut GlyphParticleStats,
) -> Option<Position> {
    let from = resolve_cell_placement(actor, &phase.from, ctx);
    let to = resolve_cell_placement(actor, &phase.to, ctx);
    if completed_lifetime {
        stats.completed_particle_count += 1;
        return match end {
            ParticleEndBehavior::Despawn => None,
            ParticleEndBehavior::FreezeInPlace => Some(sample_particle_position(
                from,
                phase
                    .via
                    .as_ref()
                    .map(|p| resolve_cell_placement(actor, p, ctx)),
                to,
                1.0,
                phase,
            )),
            ParticleEndBehavior::ConvergeToOrigin => Some(to),
        };
    }
    if phase.duration_ms == 0 {
        return match phase.visibility.after_complete {
            CellVisibilityMode::Hidden => None,
            CellVisibilityMode::AtFrom => Some(from),
            CellVisibilityMode::AtTo | CellVisibilityMode::Hold => Some(to),
        };
    }
    let mut t = (age_ms as f64 / phase.duration_ms as f64).clamp(0.0, 1.0);
    if let Some(steps) = phase.quantize_steps {
        let denom = steps.saturating_sub(1).max(1) as f64;
        t = (t * denom).round() / denom;
    }
    let eased = phase.easing.ease(t) as f64;
    Some(sample_particle_position(
        from,
        phase
            .via
            .as_ref()
            .map(|p| resolve_cell_placement(actor, p, ctx)),
        to,
        eased,
        phase,
    ))
}

fn sample_particle_position(
    from: Position,
    via: Option<Position>,
    to: Position,
    t: f64,
    phase: &CellMotionPhaseSpec,
) -> Position {
    let path = match (&phase.route, via) {
        (tui_vfx_geometry::types::PathType::Bezier { .. }, Some(v)) => {
            tui_vfx_geometry::types::PathType::Bezier {
                control_x: v.x as f32,
                control_y: v.y as f32,
            }
        }
        _ => phase.route.clone(),
    };
    let (x, y) = interpolate_position(from, to, t, &path);
    match phase.snap {
        SnappingStrategy::Floor => Position::new(x.floor() as i32, y.floor() as i32),
        SnappingStrategy::Round | SnappingStrategy::Stochastic { .. } => {
            Position::new(x.round() as i32, y.round() as i32)
        }
    }
}

fn particle_active(
    policy: &ParticleConcurrency,
    actor: &CellActor,
    particle_idx: u16,
    seed: u64,
    recipe_seed: u64,
) -> bool {
    match policy {
        ParticleConcurrency::All => true,
        ParticleConcurrency::RandomSample { fraction } => {
            let threshold = fraction.clamp(0.0, 1.0);
            let value = (hash64(
                recipe_seed,
                actor.authored_index,
                particle_idx as u32,
                seed,
                b"active",
            ) % 10_000) as f32
                / 10_000.0;
            value < threshold
        }
        ParticleConcurrency::RoundRobin { stride } => {
            let stride = (*stride).max(1) as u32;
            (actor.selected_ordinal + particle_idx as u32).is_multiple_of(stride)
        }
    }
}

fn selected_bounds(actors: &[CellActor], frame: Rect) -> Option<Rect> {
    let min_x = actors.iter().map(|a| a.authored_x).min()?;
    let max_x = actors.iter().map(|a| a.authored_x).max()?;
    let min_y = actors.iter().map(|a| a.authored_y).min()?;
    let max_y = actors.iter().map(|a| a.authored_y).max()?;
    Some(Rect::new(
        frame.x.saturating_add(min_x),
        frame.y.saturating_add(min_y),
        max_x - min_x + 1,
        max_y - min_y + 1,
    ))
}

fn clip(pos: Position, frame: Rect) -> Option<(u16, u16)> {
    if pos.x < frame.x as i32
        || pos.y < frame.y as i32
        || pos.x >= frame.right() as i32
        || pos.y >= frame.bottom() as i32
    {
        None
    } else {
        Some((
            (pos.x - frame.x as i32) as u16,
            (pos.y - frame.y as i32) as u16,
        ))
    }
}

fn hash64(
    recipe_seed: u64,
    authored_index: u32,
    particle_idx: u32,
    user_seed: u64,
    salt: &[u8],
) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for b in recipe_seed
        .to_le_bytes()
        .into_iter()
        .chain(authored_index.to_le_bytes())
        .chain(particle_idx.to_le_bytes())
        .chain(user_seed.to_le_bytes())
        .chain(salt.iter().copied())
    {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn default_origin() -> CellPlacement {
    CellPlacement::Authored
}

fn default_glyph_palette() -> Vec<char> {
    vec!['*']
}

#[allow(dead_code)]
fn _blank_scene_like(scene: &SemanticScene) -> SemanticScene {
    SemanticScene::from_grid_with_default_role(
        OwnedGrid::new(scene.grid().width(), scene.grid().height()),
        RoleTag::Background,
    )
}

// <FILE>crates/tui-vfx-content/src/glyph_particles/mod.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
