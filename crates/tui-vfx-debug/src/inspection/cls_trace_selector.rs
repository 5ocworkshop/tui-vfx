// <FILE>crates/tui-vfx-debug/src/inspection/cls_trace_selector.rs</FILE> - <DESC>TraceSelector — predicate for filtering envelopes at sink-time</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — opaque-ID based selectors (Layer, Recipe) reuse tui-vfx-types newtypes so inspection code does not depend on the recipe crate. All selectors match at sink-time so adding new variants does not touch emit sites. `#[non_exhaustive]` preserves room for future selectors.</WCTX>
// <CLOG>0.1.0: initial TraceSelector with Cell, Rect, Role, Layer, Recipe, All variants; matches() predicate tests each event's payload against the selector.</CLOG>

//! Declarative predicate for matching [`TraceEnvelope`]s at sink-time.
//!
//! A [`TraceSelector`] describes **what** the consumer cares about in a
//! trace stream. Multiple selectors combine via OR inside a
//! [`crate::inspection::TraceFilter`].
//!
//! # Design: opaque IDs reuse `tui-vfx-types`
//!
//! [`TraceSelector::Layer`] carries a [`LayerId`] and
//! [`TraceSelector::Recipe`] carries a [`RecipeId`] — both from
//! `tui-vfx-types`. That keeps inspection code independent of the
//! recipe crate, so the layered model from the recipe-scene composer
//! spec (§9.1) is preserved.

use serde::{Deserialize, Serialize};
use tui_vfx_types::{LayerId, RecipeId, Rect, RoleTag};

use super::cls_trace_envelope::TraceEnvelope;
use super::cls_trace_event::TraceEvent;

/// Predicate over [`TraceEnvelope`] payloads.
///
/// Each variant tests a different facet of the envelope:
///
/// - [`Cell`](TraceSelector::Cell) — exact `(x, y)` match on cell-bearing
///   events.
/// - [`Rect`](TraceSelector::Rect) — rectangular containment on
///   cell-bearing events.
/// - [`Role`](TraceSelector::Role) — role match on events that carry a
///   role tag (layer paint / shadow cell application).
/// - [`Layer`](TraceSelector::Layer) — layer-id match.
/// - [`Recipe`](TraceSelector::Recipe) — recipe-id match (envelope-level).
/// - [`All`](TraceSelector::All) — match every envelope.
///
/// `#[non_exhaustive]` — new selector variants may be added without a
/// breaking change; match with a wildcard arm.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TraceSelector {
    /// Match cell-bearing events at `(x, y)`.
    Cell {
        /// Cell x.
        x: u16,
        /// Cell y.
        y: u16,
    },
    /// Match cell-bearing events contained in `rect`.
    Rect(Rect),
    /// Match events that carry a role tag equal to this role.
    Role(RoleTag),
    /// Match events tagged with this `LayerId`.
    Layer(LayerId),
    /// Match events whose envelope `recipe_id` equals this id.
    Recipe(RecipeId),
    /// Match every envelope.
    All,
}

impl TraceSelector {
    /// True if `envelope` is matched by this selector.
    pub fn matches(&self, envelope: &TraceEnvelope) -> bool {
        match self {
            TraceSelector::All => true,
            TraceSelector::Cell { x, y } => cell_xy(&envelope.event)
                .is_some_and(|(ex, ey)| ex == *x && ey == *y),
            TraceSelector::Rect(r) => cell_xy(&envelope.event)
                .is_some_and(|(ex, ey)| rect_contains(r, ex, ey)),
            TraceSelector::Role(role) => event_role(&envelope.event)
                .is_some_and(|ev_role| ev_role == role),
            TraceSelector::Layer(layer) => event_layer_id(&envelope.event)
                .is_some_and(|ev_layer| ev_layer == layer),
            TraceSelector::Recipe(recipe) => envelope
                .recipe_id
                .as_ref()
                .is_some_and(|env_recipe| env_recipe == recipe),
        }
    }
}

/// Return the `(x, y)` of events that carry a cell coordinate.
fn cell_xy(event: &TraceEvent) -> Option<(u16, u16)> {
    match event {
        TraceEvent::SamplerApplied { dest_x, dest_y, .. } => Some((*dest_x, *dest_y)),
        TraceEvent::MaskChecked { x, y, .. } => Some((*x, *y)),
        TraceEvent::ShaderApplied { x, y, .. } => Some((*x, *y)),
        TraceEvent::FilterApplied { x, y, .. } => Some((*x, *y)),
        TraceEvent::ShadowCellApplied { x, y, .. } => Some((*x, *y)),
        TraceEvent::CellRendered { x, y, .. } => Some((*x, *y)),
        TraceEvent::LayerCellPainted { x, y, .. } => Some((*x, *y)),
        _ => None,
    }
}

/// Return the role tag on events that carry one.
fn event_role(event: &TraceEvent) -> Option<&RoleTag> {
    match event {
        TraceEvent::LayerCellPainted { role, .. } => Some(role),
        TraceEvent::ShadowCellApplied { source_role, .. } => source_role.as_ref(),
        _ => None,
    }
}

/// Return the layer id on events that carry one.
fn event_layer_id(event: &TraceEvent) -> Option<&LayerId> {
    match event {
        TraceEvent::LayerStarted { layer_id, .. }
        | TraceEvent::LayerCellPainted { layer_id, .. }
        | TraceEvent::LayerCompleted { layer_id, .. }
        | TraceEvent::LayerSkipped { layer_id, .. } => Some(layer_id),
        _ => None,
    }
}

/// Inclusive containment check for `Rect` (x in [rect.x, rect.x+width),
/// y in [rect.y, rect.y+height)).
fn rect_contains(rect: &Rect, x: u16, y: u16) -> bool {
    x >= rect.x && x < rect.x.saturating_add(rect.width) && y >= rect.y && y < rect.y.saturating_add(rect.height)
}

// <FILE>crates/tui-vfx-debug/src/inspection/cls_trace_selector.rs</FILE> - <DESC>TraceSelector</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
