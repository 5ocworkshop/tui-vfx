// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_resolve_actor_offset_ms.rs</FILE> - <DESC>Resolve actor stagger offsets</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: deterministic elapsed-time stagger math.</WCTX>
// <CLOG>0.1.0: add index, position, distance, and FNV-backed random offsets.</CLOG>

use super::{
    CellActor, CellPlacementContext, CellStagger, CellStaggerAxis, CellStaggerDirection,
    resolve_cell_placement,
};

/// Resolve actor-specific start offset in milliseconds.
pub fn resolve_actor_offset_ms(
    actor: &CellActor,
    stagger: &CellStagger,
    actors: &[CellActor],
    ctx: &CellPlacementContext,
    recipe_or_layer_seed: u64,
) -> u64 {
    match stagger {
        CellStagger::None => 0,
        CellStagger::ByIndex { stride_ms } => {
            actor.selected_ordinal.saturating_mul(*stride_ms as u32) as u64
        }
        CellStagger::ByPosition {
            axis,
            direction,
            stride_ms,
        } => {
            let mut ranked: Vec<_> = actors
                .iter()
                .map(|a| (coord(a, *axis), a.authored_index))
                .collect();
            ranked.sort_by_key(|v| v.0);
            if matches!(direction, CellStaggerDirection::Descending) {
                ranked.reverse();
            }
            let rank = ranked
                .iter()
                .position(|(_, idx)| *idx == actor.authored_index)
                .unwrap_or(0) as u64;
            rank.saturating_mul(*stride_ms)
        }
        CellStagger::ByDistance { origin, stride_ms } => {
            let p = resolve_cell_placement(actor, origin, ctx);
            let d = (actor.authored_x as i32 - p.x).unsigned_abs() as u64
                + (actor.authored_y as i32 - p.y).unsigned_abs() as u64;
            d.saturating_mul(*stride_ms)
        }
        CellStagger::Random {
            seed,
            max_offset_ms,
        } => {
            if *max_offset_ms == 0 {
                0
            } else {
                fnv1a64(
                    recipe_or_layer_seed,
                    actor.authored_index,
                    *seed,
                    b"cell_stagger",
                ) % (max_offset_ms + 1)
            }
        }
    }
}

fn coord(actor: &CellActor, axis: CellStaggerAxis) -> u16 {
    match axis {
        CellStaggerAxis::X => actor.authored_x,
        CellStaggerAxis::Y => actor.authored_y,
    }
}

/// Canonical deterministic seed combiner for Task 23 cell-motion random stagger.
pub fn fnv1a64(recipe_seed: u64, authored_index: u32, user_seed: u64, field_salt: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for b in recipe_seed
        .to_le_bytes()
        .into_iter()
        .chain(authored_index.to_le_bytes())
        .chain(user_seed.to_le_bytes())
        .chain(field_salt.iter().copied())
    {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_resolve_actor_offset_ms.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
