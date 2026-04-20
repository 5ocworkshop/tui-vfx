// <FILE>crates/tui-vfx-style/tests/test_shader_role_awareness.rs</FILE> - <DESC>Tests for ShaderContext role awareness — the roles field is populated, shaders can read it, and Default gives an empty map</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.3.1 — TDD red→green for ShaderContext carrying a role map; assert shaders can access the current cell's role via ShaderContext.roles</WCTX>
// <CLOG>0.1.0: initial TDD red covering (a) default ShaderContext carries an empty RoleMap; (b) construction via struct literal with a populated RoleMap exposes tags; (c) a shader instance can read the role for its own cell via ShaderContext; (d) Send+Sync preserved.</CLOG>

//! Tests that `ShaderContext` carries a `RoleMap` so shaders can inspect
//! per-cell semantic roles in addition to geometric coordinates.
//!
//! The compositor still uses `StyleRegion::Role(…)` as the primary
//! role-dispatch mechanism (via `should_style`), but giving shaders direct
//! access to the role map lets them branch on neighbour-role information
//! (e.g. "only glow when an adjacent cell has RoleTag::Text") without
//! widening the shader trait itself.

use std::sync::Arc;
use tui_vfx_style::traits::{ShaderContext, ShaderRuntimeParams, StyleShader};
use tui_vfx_types::{Color, RoleMap, RoleTag, Style};

/// A dummy shader that inspects `ctx.roles` at `(ctx.local_x, ctx.local_y)`
/// and flips the foreground colour if the role matches its target.
#[derive(Debug)]
struct RoleAwareShader {
    target: RoleTag,
    match_color: Color,
}

impl StyleShader for RoleAwareShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let role = ctx.roles.get((ctx.local_x, ctx.local_y));
        if role.as_ref() == Some(&self.target) {
            Style {
                fg: self.match_color,
                bg: base.bg,
                mods: base.mods,
            }
        } else {
            base
        }
    }

    fn name(&self) -> &'static str {
        "RoleAwareShader"
    }
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn default_shader_context_carries_empty_role_map() {
    let ctx = ShaderContext::default();
    // An empty default role map has zero dimensions: every get returns None.
    assert!(ctx.roles.get((0, 0)).is_none());
    assert!(ctx.roles.get((5, 5)).is_none());
}

#[test]
fn shader_can_read_role_for_current_cell() {
    let mut roles = RoleMap::new_with_default(8, 4, RoleTag::Background);
    roles.set((3, 1), RoleTag::Border);
    let ctx = ShaderContext {
        local_x: 3,
        local_y: 1,
        width: 8,
        height: 4,
        screen_x: 0,
        screen_y: 0,
        t: 0.0,
        phase: None,
        runtime_params: Arc::new(ShaderRuntimeParams::new()),
        roles: Arc::new(roles),
    };
    let shader = RoleAwareShader {
        target: RoleTag::Border,
        match_color: Color::RED,
    };
    let base = Style {
        fg: Color::WHITE,
        bg: Color::BLACK,
        mods: Default::default(),
    };
    let styled = shader.style_at(&ctx, base);
    assert_eq!(styled.fg, Color::RED, "Border cell should receive match_color");
}

#[test]
fn shader_passes_through_when_role_does_not_match() {
    let roles = RoleMap::new_with_default(4, 4, RoleTag::Background);
    let ctx = ShaderContext {
        local_x: 1,
        local_y: 1,
        width: 4,
        height: 4,
        screen_x: 0,
        screen_y: 0,
        t: 0.0,
        phase: None,
        runtime_params: Arc::new(ShaderRuntimeParams::new()),
        roles: Arc::new(roles),
    };
    let shader = RoleAwareShader {
        target: RoleTag::Border,
        match_color: Color::RED,
    };
    let base = Style {
        fg: Color::WHITE,
        bg: Color::BLACK,
        mods: Default::default(),
    };
    let styled = shader.style_at(&ctx, base);
    assert_eq!(styled.fg, Color::WHITE);
}

#[test]
fn shader_context_send_sync_preserved() {
    assert_send_sync::<ShaderContext>();
}

#[test]
fn shader_context_clone_preserves_roles() {
    let mut roles = RoleMap::new_with_default(4, 4, RoleTag::Background);
    roles.set((2, 2), RoleTag::Title);
    let original = ShaderContext {
        local_x: 2,
        local_y: 2,
        width: 4,
        height: 4,
        screen_x: 0,
        screen_y: 0,
        t: 0.0,
        phase: None,
        runtime_params: Arc::new(ShaderRuntimeParams::new()),
        roles: Arc::new(roles),
    };
    let cloned = original.clone();
    assert_eq!(
        cloned.roles.get((2, 2)),
        Some(RoleTag::Title),
        "cloned role map must carry the same tag"
    );
}

// <FILE>crates/tui-vfx-style/tests/test_shader_role_awareness.rs</FILE> - <DESC>Tests for ShaderContext role awareness</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
