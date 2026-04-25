// <FILE>crates/tui-vfx-content/tests/test_glyph_particles.rs</FILE> - <DESC>Tests for transient glyph particle emitter</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Task 24: lock content-layer glyph spawner behavior inspired by TTE Spray/BinaryPath.</WCTX>
// <CLOG>0.1.0: add deterministic spawn, palette, concurrency, lifetime, and motion tests.</CLOG>

use tui_vfx_content::cell_motion::*;
use tui_vfx_content::glyph_particles::*;
use tui_vfx_types::{Cell, Color, Grid, OwnedGrid, RoleMap, RoleTag, SemanticScene};

fn scene(rows: &[&str]) -> SemanticScene {
    let h = rows.len();
    let w = rows.first().map_or(0, |r| r.chars().count());
    let mut grid = OwnedGrid::new(w, h);
    let roles = RoleMap::new_with_default(w as u16, h as u16, RoleTag::Text);
    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch != '.' {
                grid.set(x, y, Cell::new(ch).with_fg(Color::WHITE));
            }
        }
    }
    SemanticScene::new(grid, roles)
}

fn ch(s: &SemanticScene, x: u16, y: u16) -> char {
    s.cell((x, y)).unwrap().ch
}

fn timing(ms: u64) -> CellMotionTiming {
    CellMotionTiming {
        phase: CellMotionPhase::Enter,
        phase_elapsed_ms: ms,
        phase_t: 0.0,
        absolute_t_ms: ms as f64,
        reduced_motion: false,
        seed: 0,
    }
}

fn emitter(from: CellPlacement, to: CellPlacement) -> GlyphParticleEmitterSpec {
    GlyphParticleEmitterSpec {
        origin: CellPlacement::Authored,
        spawn_count: 1,
        glyph_palette: vec!['0'],
        color_palette: vec![Color::CYAN],
        motion: CellMotionSpec {
            enter: Some(CellMotionPhaseSpec {
                duration_ms: 100,
                from,
                to,
                ..Default::default()
            }),
            exit: None,
        },
        lifetime_ms: 1000,
        on_complete: ParticleEndBehavior::Despawn,
        concurrency: ParticleConcurrency::All,
        seed: 0,
    }
}

#[test]
fn glyph_particles_spawn_transient_glyphs_over_source() {
    let sc = scene(&["A."]);
    let res = emit_glyph_particles(
        &sc,
        &emitter(
            CellPlacement::AuthoredOffset { dx: 1, dy: 0 },
            CellPlacement::Authored,
        ),
        &timing(0),
        sc.area(),
        &CellMotionOptions::default(),
    );
    assert_eq!(ch(&res.scene, 0, 0), 'A');
    assert_eq!(ch(&res.scene, 1, 0), '0');
    assert_eq!(res.scene.cell((1, 0)).unwrap().fg, Color::CYAN);
    assert_eq!(res.stats.source_actor_count, 1);
    assert_eq!(res.stats.emitted_particle_count, 1);
}

#[test]
fn glyph_particles_despawn_after_lifetime() {
    let sc = scene(&["A."]);
    let mut spec = emitter(
        CellPlacement::AuthoredOffset { dx: 1, dy: 0 },
        CellPlacement::Authored,
    );
    spec.lifetime_ms = 10;
    let res = emit_glyph_particles(
        &sc,
        &spec,
        &timing(20),
        sc.area(),
        &CellMotionOptions::default(),
    );
    assert_eq!(ch(&res.scene, 0, 0), 'A');
    assert_eq!(ch(&res.scene, 1, 0), ' ');
    assert_eq!(res.stats.completed_particle_count, 1);
}

#[test]
fn glyph_particles_freeze_after_lifetime_when_configured() {
    let sc = scene(&["A."]);
    let mut spec = emitter(
        CellPlacement::AuthoredOffset { dx: 1, dy: 0 },
        CellPlacement::Authored,
    );
    spec.lifetime_ms = 10;
    spec.on_complete = ParticleEndBehavior::FreezeInPlace;
    let res = emit_glyph_particles(
        &sc,
        &spec,
        &timing(20),
        sc.area(),
        &CellMotionOptions::default(),
    );
    assert_eq!(ch(&res.scene, 0, 0), '0');
}

#[test]
fn glyph_particles_round_robin_concurrency_limits_candidates() {
    let sc = scene(&["ABCD"]);
    let mut spec = emitter(CellPlacement::Authored, CellPlacement::Authored);
    spec.concurrency = ParticleConcurrency::RoundRobin { stride: 2 };
    let res = emit_glyph_particles(
        &sc,
        &spec,
        &timing(0),
        sc.area(),
        &CellMotionOptions::default(),
    );
    assert_eq!(res.stats.candidate_particle_count, 4);
    assert_eq!(res.stats.emitted_particle_count, 2);
    assert_eq!(res.stats.skipped_by_concurrency_count, 2);
}

#[test]
fn glyph_particles_random_sample_is_deterministic() {
    let sc = scene(&["ABCDE"]);
    let mut spec = emitter(CellPlacement::Authored, CellPlacement::Authored);
    spec.concurrency = ParticleConcurrency::RandomSample { fraction: 0.5 };
    spec.seed = 7;
    let a = emit_glyph_particles(
        &sc,
        &spec,
        &timing(0),
        sc.area(),
        &CellMotionOptions::default(),
    );
    let b = emit_glyph_particles(
        &sc,
        &spec,
        &timing(0),
        sc.area(),
        &CellMotionOptions::default(),
    );
    assert_eq!(a.stats, b.stats);
    assert_eq!(a.scene.grid().cells(), b.scene.grid().cells());
}

// <FILE>crates/tui-vfx-content/tests/test_glyph_particles.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
