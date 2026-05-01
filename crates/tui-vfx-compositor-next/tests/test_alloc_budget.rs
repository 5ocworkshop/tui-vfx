// <FILE>crates/tui-vfx-compositor-next/tests/test_alloc_budget.rs</FILE> - <DESC>Steady-state allocation-count gate for render_pipeline_with_shadow, protecting the Phase 1a + 1b wins (buffer pool, role-map Arc cache, region-resolution hoist, element-rect hoist)</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1a perf — durable regression gate that asserts zero heap allocations in the steady state of the shadow render path</WCTX>
// <CLOG>0.1.0: initial inline CountingAllocator + shadowed_render_is_zero_alloc_in_steady_state test. Exercises render_pipeline with a shadow spec; warms the GridPool bucket and the ROLES_ARC_CACHE entry on the first call; subsequent calls must not allocate.</CLOG>

//! # Alloc-budget gate for the shadow render path
//!
//! A counting [`GlobalAlloc`] replaces the system allocator in this integration-
//! test binary so we can count heap allocations over a measured scope. The
//! test:
//!
//! 1. Builds a minimal shadow recipe (empty shaders / filters / masks; a
//!    default [`ShadowConfig`]).
//! 2. Calls [`render_pipeline`] once to warm the [`GridPool`] bucket for the
//!    extended grid size and to populate [`ROLES_ARC_CACHE`] with an
//!    `Arc<RoleMap>` for the source.
//! 3. Snapshots the allocator counter, calls [`render_pipeline`] once more,
//!    and snapshots again.
//! 4. Asserts the delta is zero.
//!
//! If this test regresses, a new allocation slipped into the shadow hot path.
//! The counting allocator is scoped to this test binary only; other test
//! binaries use the default system allocator.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use tui_vfx_compositor_next::pipeline::{CompositionOptions, ShadowSpec, render_pipeline};
use tui_vfx_shadow::ShadowConfig;
use tui_vfx_types::{Color, OwnedGrid, RoleMap, RoleTag, SemanticScene};

struct CountingAllocator {
    allocs: AtomicUsize,
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocs.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator {
    allocs: AtomicUsize::new(0),
};

fn alloc_count() -> usize {
    GLOBAL.allocs.load(Ordering::Relaxed)
}

#[test]
fn shadowed_render_is_zero_alloc_in_steady_state() {
    // Build a minimal fixture: 10x10 source, empty effects, a default
    // shadow config. Shadow path exercises GridPool (buffer pooling) and
    // ROLES_ARC_CACHE (per-frame Arc clone avoidance).
    let source = OwnedGrid::new(10, 10);
    let source_roles = RoleMap::all_background(10, 10);
    let mut scene =
        SemanticScene::from_grid_with_default_role(OwnedGrid::new(14, 14), RoleTag::Background);
    let shadow_config = ShadowConfig::new(Color::rgb(0, 0, 0));
    let shadow_spec = ShadowSpec::new(shadow_config);

    // Pre-clone the options so the clone cost is outside the measurement
    // scope. CompositionOptions with only a shadow set has empty SmallVecs
    // (inline storage) and an Arc'd runtime_params, so the clone itself
    // should be zero-alloc, but measuring only the render keeps the test
    // honest about what's being asserted.
    let options_warm = CompositionOptions::default().with_shadow(shadow_spec.clone());
    let options_measure = CompositionOptions::default().with_shadow(shadow_spec);

    // Warm-up: populates the GridPool bucket for the extended grid size and
    // stores an entry in ROLES_ARC_CACHE keyed on the source_roles pointer.
    render_pipeline(
        &source,
        &source_roles,
        &mut scene,
        10,
        10,
        0,
        0,
        options_warm,
        None,
    );

    // Measure one steady-state render. The expectation: zero new allocations.
    let before = alloc_count();
    render_pipeline(
        &source,
        &source_roles,
        &mut scene,
        10,
        10,
        0,
        0,
        options_measure,
        None,
    );
    let after = alloc_count();

    let delta = after - before;
    assert_eq!(
        delta, 0,
        "shadow render path allocated {delta} times in steady state; \
         phase 1a + 1b wins have regressed. Expected pool + arc cache \
         to cover the shadow buffer, snapshot buffer, and role-map Arc.",
    );
}

// <FILE>crates/tui-vfx-compositor-next/tests/test_alloc_budget.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
