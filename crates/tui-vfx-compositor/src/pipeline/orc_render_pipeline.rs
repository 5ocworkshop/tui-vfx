// <FILE>tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs</FILE> - <DESC>Pipeline orchestrator with signal-driven composition</DESC>
// <VERS>VERSION: 13.0.0</VERS>
// <WCTX>2026-04-26 packet Phase 4 — orchestrator now owns per-cell VfxCellContext construction so the sampler-accumulated resolved-coord delta reaches every downstream stage (mask, shader, filter).</WCTX>
// <CLOG>13.0.0: BREAKING — sample_sampler_chain returns SamplerChainOutcome (source + delta); orchestrator threads (delta_x, delta_y) into VfxCellContext::with_sampler_resolution and ShaderContext::with_sampler_resolution; check_prepared_masks and PreparedFilter::apply now take &VfxCellContext.</CLOG>

use super::cls_composition_options::CompositionOptions;
use super::cls_composition_playback_timing::CompositionPlaybackTiming;
use super::cls_grid_pool::GridPool;
use super::cls_prepare_context::PrepareContext;
use super::cls_prepared_filter::{PreparedFilter, prepare_filters};
use super::cls_prepared_mask::{PreparedMask, prepare_masks};
use super::cls_prepared_sampler::{PreparedSampler, prepare_samplers, sample_sampler_chain};
use super::cls_render_area::RenderArea;
use super::fnc_blend_shadow_cell::blend_shadow_cell;
use super::fnc_blend_underlying_shadow_cell::blend_underlying_shadow_cell;
use super::fnc_check_masks::check_prepared_masks;
use super::fnc_grade_shadow_cell::grade_shadow_cell;
use super::orc_pipeline_observability::{
    PerStageInputs, emit_per_stage_entered, emit_per_stage_finished, emit_role_map_materialized,
    emit_simple_stage_entered, emit_simple_stage_finished,
};
use crate::pipeline::cls_composition_options::ShaderWithRegion;
use crate::traits::pipeline_inspector::CompositorInspector;
use tui_vfx_debug::inspection::PipelineStageKind;
use mixed_signals::traits::Phase;
use smallvec::SmallVec;
use std::borrow::Cow;
use std::cell::RefCell;
use std::sync::Arc;
use tui_vfx_shadow::{ShadowCompositeMode, render_shadow};
use tui_vfx_style::models::StyleRegion;
use tui_vfx_style::traits::ShaderContext;
use tui_vfx_types::{Grid, Rect, RoleMap, RoleTag, SemanticScene, Style, VfxCellContext};

// Thread-local cache for the Arc<RoleMap> the shader context receives.
//
// Each render call used to rebuild Arc::new(source_roles.clone()) — a full
// RoleMap clone every frame. Typical workloads keep a long-lived RoleMap
// across frames and mutate it rarely; the cache detects that steady state
// via the combination of source pointer, RoleMap::generation(), and
// dimensions, and returns a cheap Arc::clone instead of a fresh
// Vec<RoleId> copy. Keyed on (ptr, generation, width, height) to cover
// both stable-map-across-frames and defensive detection of pointer reuse.
thread_local! {
    static ROLES_ARC_CACHE: RefCell<Option<CachedRolesArc>> = const { RefCell::new(None) };
}

struct CachedRolesArc {
    source_ptr: *const RoleMap,
    generation: u64,
    width: u16,
    height: u16,
    arc: Arc<RoleMap>,
}

/// Reuse a previously built `Arc<RoleMap>` when the source `RoleMap`
/// appears unchanged since the last call on this thread; otherwise
/// rebuild it and update the cache.
fn cached_roles_arc(source_roles: &RoleMap) -> Arc<RoleMap> {
    let ptr = source_roles as *const RoleMap;
    let generation = source_roles.generation();
    let width = source_roles.width();
    let height = source_roles.height();
    ROLES_ARC_CACHE.with(|cell| {
        let mut slot = cell.borrow_mut();
        if let Some(cached) = slot.as_ref()
            && cached.source_ptr == ptr
            && cached.generation == generation
            && cached.width == width
            && cached.height == height
        {
            return Arc::clone(&cached.arc);
        }
        let arc = Arc::new(source_roles.clone());
        *slot = Some(CachedRolesArc {
            source_ptr: ptr,
            generation,
            width,
            height,
            arc: Arc::clone(&arc),
        });
        arc
    })
}

/// Render pipeline with full spec support and optional inspector.
///
/// This is the L2 (framework-agnostic) compositor. It operates on any type
/// implementing the `Grid` trait, allowing it to work with ratatui, crossterm,
/// egui, or any other TUI framework via an adapter.
///
/// ## Shadow Support
///
/// When `options.shadow` is set, the pipeline:
/// 1. Extends the render area by the shadow offset
/// 2. Renders the shadow first, then the element on top
/// 3. Applies all masks to the combined shadow+element area
///
/// **Important:** The rendered area will be larger than the source dimensions:
/// - Total width = width + |shadow.offset_x|
/// - Total height = height + |shadow.offset_y|
///
/// The `width` and `height` parameters specify the element (source) dimensions.
/// The shadow extends beyond this based on the shadow configuration.
#[allow(clippy::too_many_arguments)]
pub fn render_pipeline(
    source: &dyn Grid,
    source_roles: &RoleMap,
    destination: &mut SemanticScene,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    options: CompositionOptions<'_>,
    inspector: Option<&mut dyn CompositorInspector>,
) {
    // SHADOW PATH: Dispatch to shadow-aware rendering
    if options.shadow.is_some() {
        render_pipeline_with_shadow(
            source,
            source_roles,
            destination,
            width,
            height,
            offset_x,
            offset_y,
            options,
            inspector,
        );
        return;
    }

    // FAST PATH: Check if we can copy directly without effects.
    let effective_sampler_specs = options.effective_samplers();
    let has_sampler = options.has_active_sampler();
    let has_masks = !options.masks.is_empty();
    let has_filters = !options.filters.is_empty();
    let has_shaders = !options.shader_layers.is_empty();

    if !has_sampler && !has_masks && !has_filters && !has_shaders && inspector.is_none() {
        let dest_grid = destination.grid_mut();
        for y in 0..height {
            for x in 0..width {
                if let Some(cell) = source.get(x, y) {
                    dest_grid.set(offset_x + x, offset_y + y, *cell);
                }
            }
        }
        return;
    }

    // SLOW PATH: Effects are active
    let timing = CompositionPlaybackTiming::from_options(&options);
    let samplers = prepare_samplers(options.t, effective_sampler_specs.as_ref());
    let sampler_label = (!samplers.is_empty()).then(|| {
        samplers
            .iter()
            .enumerate()
            .map(|(index, sampler)| format!("{}#{}", sampler.name(), index + 1))
            .collect::<Vec<_>>()
            .join(" -> ")
    });
    let prepared_masks = prepare_masks(options.masks.as_ref());
    let loop_t = timing.effective_loop_t();
    let prepare_ctx =
        PrepareContext::new(loop_t, options.runtime_params.as_ref(), width as u16, height as u16);
    let prepared_filters = prepare_filters(options.filters.as_ref(), &prepare_ctx);

    // Dispatch to inspected or non-inspected loop
    // (Two loops needed due to Rust borrow checker constraints with optional mutable refs)
    let dest_grid: &mut dyn Grid = destination.grid_mut();
    if let Some(inspector) = inspector {
        render_loop_inspected(
            source,
            source_roles,
            dest_grid,
            width,
            height,
            offset_x,
            offset_y,
            &options,
            &samplers,
            &prepared_masks,
            &prepared_filters,
            loop_t,
            sampler_label.as_deref(),
            inspector,
        );
    } else {
        render_loop(
            source,
            source_roles,
            dest_grid,
            width,
            height,
            offset_x,
            offset_y,
            &options,
            &samplers,
            &prepared_masks,
            &prepared_filters,
            loop_t,
        );
    }
}

/// Render pipeline with area struct for reduced argument count.
///
/// This is a convenience wrapper around [`render_pipeline`] that accepts
/// a [`RenderArea`] instead of separate width/height/offset parameters.
pub fn render_pipeline_with_area(
    source: &dyn Grid,
    source_roles: &RoleMap,
    destination: &mut SemanticScene,
    area: RenderArea,
    options: CompositionOptions<'_>,
    inspector: Option<&mut dyn CompositorInspector>,
) {
    render_pipeline(
        source,
        source_roles,
        destination,
        area.width,
        area.height,
        area.offset_x,
        area.offset_y,
        options,
        inspector,
    );
}

/// Shadow-aware rendering path.
///
/// Creates a working buffer with extended dimensions (element + shadow),
/// renders shadow then element, and applies masks when copying to dest.
///
/// Phase A.3 role-awareness: the destination is `&mut SemanticScene` so
/// this function can tag shadow cells with `RoleTag::Shadow` in the
/// destination role map as they are written. Role-filtered extrusion
/// (from `config.source_region`) is honoured via
/// [`tui_vfx_shadow::extract_shadow_envelope`].
#[allow(clippy::too_many_arguments)]
fn render_pipeline_with_shadow(
    source: &dyn Grid,
    source_roles: &RoleMap,
    destination: &mut SemanticScene,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    options: CompositionOptions<'_>,
    mut inspector: Option<&mut dyn CompositorInspector>,
) {
    // Extract shadow spec (caller guarantees it's Some)
    let shadow_spec = options.shadow.as_ref().expect("shadow_spec must be Some");
    let grade_config = shadow_spec.config.grade.unwrap_or_default();
    let shadow_element_rect = options.shadow_element_rect;

    // Calculate extended dimensions
    let (ext_width, ext_height, elem_offset_x, elem_offset_y, element_rect) =
        if let Some(shadow_rect) = shadow_element_rect {
            (width, height, 0, 0, shadow_rect)
        } else {
            let ext_width = width + shadow_spec.extra_width();
            let ext_height = height + shadow_spec.extra_height();
            let elem_offset_x = shadow_spec.element_offset_x();
            let elem_offset_y = shadow_spec.element_offset_y();
            let element_rect = Rect::new(
                elem_offset_x as u16,
                elem_offset_y as u16,
                width as u16,
                height as u16,
            );
            (
                ext_width,
                ext_height,
                elem_offset_x,
                elem_offset_y,
                element_rect,
            )
        };

    // Check out the two working buffers (shadow + element composite, and a
    // post-shadow snapshot) from the thread-local GridPool. Both return to
    // the pool on drop at the end of this function, so steady-state
    // allocation traffic on the render thread stays at zero.
    let mut buffer_guard = GridPool::checkout(ext_width, ext_height);
    let mut shadow_only_guard = GridPool::checkout(ext_width, ext_height);

    // ── Pipeline observability Unit A — Shadow stage emit pair ──────────
    // When an inspector is installed, emit RoleMapMaterialized at function
    // entry (the role map is the source role map for this render) and
    // bracket the actual shadow generation with a per-stage entered/
    // finished pair. step_id=1 because this is the first stage of the
    // shadow-mode pipeline; element-pass stages emitted from the inner
    // render loop later in this function carry their own step_ids.
    let shadow_stage_state = if let Some(insp) = inspector.as_deref_mut() {
        emit_role_map_materialized(insp, source_roles, width as u16, height as u16);
        Some(emit_simple_stage_entered(
            insp,
            PipelineStageKind::Shadow,
            1,
            "Shadow",
            source_roles,
            width as u16,
            height as u16,
        ))
    } else {
        None
    };
    // ────────────────────────────────────────────────────────────────────

    // Render shadow to buffer (uses animation progress for fade sync).
    // Phase A.3: if `config.source_region` is set, the shadow stage
    // computes the role-filtered envelope's bounding rect and extrudes
    // from that instead of the caller-supplied element_rect. The
    // buffer-level render path here doesn't carry a destination
    // SemanticScene, so we derive the effective rect inline.
    let effective_shadow_rect =
        effective_shadow_rect(source, source_roles, element_rect, &shadow_spec.config);
    if let Some(rect) = effective_shadow_rect {
        render_shadow(buffer_guard.as_mut(), rect, &shadow_spec.config, options.t);
    }

    // Emit Shadow StageFinished now that the shadow buffer is complete;
    // downstream element-pass per-cell shadow callbacks (on_shadow_cell_applied)
    // continue to fire as today.
    if let (Some(insp), Some(state)) = (inspector.as_deref_mut(), shadow_stage_state.as_ref()) {
        emit_simple_stage_finished(insp, state);
    }

    // Snapshot the post-shadow grid into the second buffer so the element
    // pass can keep mutating `buffer` without losing the shadow-only view.
    // copy_from_slice is a single memcpy; no allocation.
    shadow_only_guard
        .as_mut()
        .cells_mut()
        .copy_from_slice(buffer_guard.as_ref().cells());

    let buffer = buffer_guard.as_mut();
    let shadow_only_buffer = shadow_only_guard.as_ref();

    // Prepare effects for element rendering
    let timing = CompositionPlaybackTiming::from_options(&options);
    let effective_sampler_specs = options.effective_samplers();
    let samplers = prepare_samplers(options.t, effective_sampler_specs.as_ref());
    let loop_t = timing.effective_loop_t();
    let prepare_ctx =
        PrepareContext::new(loop_t, options.runtime_params.as_ref(), width as u16, height as u16);
    let prepared_filters = prepare_filters(options.filters.as_ref(), &prepare_ctx);
    let shader_t = timing.shader_t();
    // Arc-wrap the source role map once so per-cell ShaderContext cloning
    // is a cheap atomic refcount bump instead of a RoleMap allocation.
    let roles_arc: Arc<RoleMap> = cached_roles_arc(source_roles);

    // Render element content to buffer (on top of shadow)
    let (w16, h16) = (width as u16, height as u16);
    // Resolve shader-layer regions and build the shader area once per frame,
    // not per cell. StyleRegion::resolved is a no-op for bindless regions and
    // a cheap evaluate() pair for Cell{binding, binding}; either way the cost
    // belongs outside the per-cell loop.
    let shader_area = Rect::new(0, 0, w16, h16);
    let resolved_regions: Vec<Cow<'_, StyleRegion>> = options
        .shader_layers
        .iter()
        .map(|layer| layer.region.resolved(&options.runtime_params))
        .collect();

    // Prepared masks are produced now (rather than just before the mask
    // pass below) so the per-stage observability block can emit Mask
    // StageEntered events alongside the Sampler/Shader/Filter entries.
    let prepared_masks = prepare_masks(options.masks.as_ref());

    // ── Pipeline observability Unit A — element-pass per-stage emit ─────
    // Architect-flagged B2: the shadow path's element pass was previously
    // observability-blind for Sampler/Mask/Shader/Filter stages. Wire the
    // same per-stage block render_loop_inspected uses, starting at
    // step_id=2 because step_id=1 is reserved for the Shadow stage that
    // already emitted above.
    let element_stage_block = if let Some(insp) = inspector.as_deref_mut() {
        let mask_labels: Vec<String> = prepared_masks
            .iter()
            .enumerate()
            .map(|(i, mask)| format!("{}#{}", mask.name(), i + 1))
            .collect();
        let filter_labels: Vec<String> = prepared_filters
            .iter()
            .enumerate()
            .map(|(i, filter)| format!("{}#{}", filter.name(), i + 1))
            .collect();
        let shader_pairs: Vec<(&ShaderWithRegion, &StyleRegion)> = options
            .shader_layers
            .iter()
            .zip(resolved_regions.iter())
            .map(|(layer, resolved)| (layer, resolved.as_ref()))
            .collect();
        Some(emit_per_stage_entered(
            insp,
            2,
            &PerStageInputs {
                sampler_label: options.sampler_spec.as_ref().map(|spec| spec.name()),
                mask_labels: &mask_labels,
                shader_layers_with_regions: &shader_pairs,
                filter_labels: &filter_labels,
                source_roles,
                width: w16,
                height: h16,
            },
        ))
    } else {
        None
    };
    let element_skipped_shaders: &[usize] = element_stage_block
        .as_ref()
        .map_or(&[], |block| &block.skipped_shader_indices);
    // ────────────────────────────────────────────────────────────────────

    for y in 0..height {
        for x in 0..width {
            let (local_x, local_y) = (x as u16, y as u16);

            // Sample coordinates (sampler operates on element dimensions).
            // The chain reports both the final source coord and the
            // accumulated delta so downstream stages can react to the
            // sampler-induced displacement.
            let chain_outcome =
                sample_sampler_chain(&samplers, local_x, local_y, w16, h16, options.t);
            let (src_x, src_y) = match (chain_outcome.source_x, chain_outcome.source_y) {
                (Some(sx), Some(sy)) => (sx, sy),
                _ => continue,
            };
            let (delta_x, delta_y) = (chain_outcome.delta_x, chain_outcome.delta_y);

            // Per-stage ctx template: same spatial fields, sampler delta
            // threaded in. `t` is overridden per-stage from a Copy of this.
            let base_ctx = VfxCellContext::new(local_x, local_y, w16, h16, 0, 0, options.t)
                .with_sampler_resolution(delta_x, delta_y);

            // Get source cell
            let Some(source_cell) = source.get(src_x as usize, src_y as usize) else {
                continue;
            };

            let mut out_cell = *source_cell;

            // Apply shaders (coordinates relative to element). Shaders flagged
            // as scope-mismatched by emit_per_stage_entered are skipped via
            // element_skipped_shaders so they do not silently mutate cells
            // for a stage already marked StageSkipped on the trace.
            let source_role = source_roles.get((src_x, src_y));
            for (shader_index, (layer, resolved)) in options
                .shader_layers
                .iter()
                .zip(resolved_regions.iter())
                .enumerate()
            {
                if element_skipped_shaders.contains(&shader_index) {
                    continue;
                }
                if resolved.should_style(local_x, local_y, source_role.clone(), shader_area) {
                    let (ctx_x, ctx_y, ctx_w, ctx_h) = resolved
                        .to_local_coords(local_x, local_y)
                        .unwrap_or((local_x, local_y, w16, h16));
                    let shader_ctx = ShaderContext::new(
                        ctx_x,
                        ctx_y,
                        ctx_w,
                        ctx_h,
                        (offset_x + elem_offset_x) as u16,
                        (offset_y + elem_offset_y) as u16,
                        shader_t,
                        options.phase,
                        Some(options.runtime_params.clone()),
                    )
                    .with_roles(roles_arc.clone())
                    .with_sampler_resolution(delta_x, delta_y);
                    let current_style = Style {
                        fg: out_cell.fg,
                        bg: out_cell.bg,
                        mods: out_cell.mods,
                    };
                    let new_style = layer.shader.style_at(&shader_ctx, current_style);
                    out_cell.fg = new_style.fg;
                    out_cell.bg = new_style.bg;
                    out_cell.mods = new_style.mods;
                }
            }

            // Apply filters — derive per-stage ctx with t = loop_t.
            let mut filter_ctx = base_ctx;
            filter_ctx.t = loop_t;
            for filter in &prepared_filters {
                filter.apply(&mut out_cell, &filter_ctx);
            }

            // Write to buffer at element position
            buffer.set(elem_offset_x + x, elem_offset_y + y, out_cell);
        }
    }

    // Now copy from buffer to dest, applying masks over the extended area
    let mask_t = compute_mask_t(&options);
    let (ext_w16, ext_h16) = (ext_width as u16, ext_height as u16);

    // Collect dest positions of shadow cells so we can tag RoleTag::Shadow
    // in the destination's RoleMap after the grid-write phase completes.
    let mut shadow_role_writes: Vec<(u16, u16)> = Vec::new();

    // Element bounds are frame-constant; both write-back branches used to
    // recompute them per cell.
    let element_left = usize::from(element_rect.x);
    let element_top = usize::from(element_rect.y);
    let element_right = element_left + usize::from(element_rect.width);
    let element_bottom = element_top + usize::from(element_rect.height);

    // Grid-write phase: borrow the destination's grid mutably for the
    // duration of the pixel copy loop. The `destination: &mut SemanticScene`
    // argument re-borrows once the grid phase is done so we can write
    // RoleTag::Shadow for the collected positions.
    let dest: &mut dyn Grid = destination.grid_mut();

    if let Some(inspector) = inspector.as_deref_mut() {
        // Inspected path
        for y in 0..ext_height {
            for x in 0..ext_width {
                let (local_x, local_y) = (x as u16, y as u16);

                // Check mask visibility over extended area. No sampler runs
                // in this write-back loop, so the mask sees a ctx with zero
                // sampler delta (resolved == local).
                let mask_ctx =
                    VfxCellContext::new(local_x, local_y, ext_w16, ext_h16, 0, 0, mask_t);
                if !check_prepared_masks(
                    &mask_ctx,
                    &prepared_masks,
                    options.mask_combine_mode,
                    Some(inspector),
                ) {
                    continue;
                }

                // Get cell from buffer and write to dest
                if let Some(cell) = buffer.get(x, y) {
                    let shadow_cell = shadow_only_buffer.get(x, y);
                    let source_empty = source
                        .get(x, y)
                        .is_none_or(|source_cell| source_cell.is_empty());
                    let in_element = x >= element_left
                        && x < element_right
                        && y >= element_top
                        && y < element_bottom;
                    let shadow_has_coverage =
                        shadow_cell.is_some_and(|shadow_cell| !shadow_cell.is_empty());
                    let shadow_region_candidate = !in_element && shadow_has_coverage;
                    if shadow_region_candidate {
                        inspector.on_shadow_cell_applied(
                            local_x,
                            local_y,
                            shadow_cell.expect("shadow region candidate must have shadow coverage"),
                            source_empty,
                        );
                    }

                    // Skip unfilled cells (space with transparent colors) to preserve
                    // underlying content in shadow corner regions
                    if options.preserve_unfilled
                        && cell.ch == ' '
                        && cell.bg.a == 0
                        && cell.fg.a == 0
                    {
                        continue;
                    }

                    let dest_x = offset_x + x;
                    let dest_y = offset_y + y;

                    let final_cell = if shadow_element_rect.is_some() {
                        if in_element || !shadow_has_coverage {
                            *cell
                        } else if let Some(dest_cell) = dest.get(dest_x, dest_y) {
                            let shadow_cell =
                                shadow_cell.expect("shadow coverage implies a shadow cell");
                            if source_empty {
                                match shadow_spec.config.composite_mode {
                                    ShadowCompositeMode::GlyphOverlay => {
                                        blend_shadow_cell(shadow_cell, dest_cell)
                                    }
                                    ShadowCompositeMode::GradeUnderlying => grade_shadow_cell(
                                        shadow_cell,
                                        dest_cell,
                                        shadow_spec.config.color,
                                        &grade_config,
                                    ),
                                    ShadowCompositeMode::BlendUnderlying => {
                                        blend_underlying_shadow_cell(shadow_cell, dest_cell)
                                    }
                                }
                            } else {
                                match shadow_spec.config.composite_mode {
                                    ShadowCompositeMode::GlyphOverlay => *cell,
                                    ShadowCompositeMode::GradeUnderlying => grade_shadow_cell(
                                        shadow_cell,
                                        cell,
                                        shadow_spec.config.color,
                                        &grade_config,
                                    ),
                                    ShadowCompositeMode::BlendUnderlying => {
                                        blend_underlying_shadow_cell(shadow_cell, cell)
                                    }
                                }
                            }
                        } else {
                            *cell
                        }
                    } else {
                        // Check if this cell is in the shadow region (outside element bounds)
                        if in_element {
                            *cell
                        } else if let Some(dest_cell) = dest.get(dest_x, dest_y) {
                            match shadow_spec.config.composite_mode {
                                ShadowCompositeMode::GlyphOverlay => {
                                    blend_shadow_cell(cell, dest_cell)
                                }
                                ShadowCompositeMode::GradeUnderlying => grade_shadow_cell(
                                    cell,
                                    dest_cell,
                                    shadow_spec.config.color,
                                    &grade_config,
                                ),
                                ShadowCompositeMode::BlendUnderlying => {
                                    blend_underlying_shadow_cell(cell, dest_cell)
                                }
                            }
                        } else {
                            *cell
                        }
                    };

                    inspector.on_cell_rendered(local_x, local_y, &final_cell);
                    dest.set(dest_x, dest_y, final_cell);
                    // Track shadow-region writes for Phase A.3.5 role
                    // tagging. A shadow region is "outside element_rect
                    // with shadow coverage"; the specific final_cell
                    // shape varies by composite mode / source_empty, but
                    // the destination cell IS a shadow contribution.
                    if shadow_region_candidate {
                        shadow_role_writes.push((dest_x as u16, dest_y as u16));
                    }
                }
            }
        }
    } else {
        // Non-inspected path (optimized)
        for y in 0..ext_height {
            for x in 0..ext_width {
                let (local_x, local_y) = (x as u16, y as u16);

                // Check mask visibility over extended area. No sampler runs
                // in this write-back loop, so the mask sees a ctx with zero
                // sampler delta (resolved == local).
                let mask_ctx =
                    VfxCellContext::new(local_x, local_y, ext_w16, ext_h16, 0, 0, mask_t);
                if !check_prepared_masks(
                    &mask_ctx,
                    &prepared_masks,
                    options.mask_combine_mode,
                    None,
                ) {
                    continue;
                }

                // Get cell from buffer and write to dest
                if let Some(cell) = buffer.get(x, y) {
                    let shadow_cell = shadow_only_buffer.get(x, y);
                    // Skip unfilled cells (space with transparent colors) to preserve
                    // underlying content in shadow corner regions
                    if options.preserve_unfilled
                        && cell.ch == ' '
                        && cell.bg.a == 0
                        && cell.fg.a == 0
                    {
                        continue;
                    }

                    let dest_x = offset_x + x;
                    let dest_y = offset_y + y;

                    // Non-inspected path needs these locally for the shadow-
                    // region classification below. element_left/top/right/
                    // bottom are hoisted above the if/else; source_empty,
                    // in_element, shadow_has_coverage, and
                    // shadow_region_candidate are genuinely per-cell.
                    let source_empty_ni = source
                        .get(x, y)
                        .is_none_or(|source_cell| source_cell.is_empty());
                    let in_element_ni = x >= element_left
                        && x < element_right
                        && y >= element_top
                        && y < element_bottom;
                    let shadow_has_coverage_ni =
                        shadow_cell.is_some_and(|shadow_cell| !shadow_cell.is_empty());
                    let shadow_region_candidate_ni = !in_element_ni && shadow_has_coverage_ni;

                    let final_cell = if shadow_element_rect.is_some() {
                        let source_empty = source_empty_ni;
                        let in_element = in_element_ni;
                        let shadow_has_coverage = shadow_has_coverage_ni;
                        if in_element || !shadow_has_coverage {
                            *cell
                        } else if let Some(dest_cell) = dest.get(dest_x, dest_y) {
                            let shadow_cell =
                                shadow_cell.expect("shadow coverage implies a shadow cell");
                            if source_empty {
                                match shadow_spec.config.composite_mode {
                                    ShadowCompositeMode::GlyphOverlay => {
                                        blend_shadow_cell(shadow_cell, dest_cell)
                                    }
                                    ShadowCompositeMode::GradeUnderlying => grade_shadow_cell(
                                        shadow_cell,
                                        dest_cell,
                                        shadow_spec.config.color,
                                        &grade_config,
                                    ),
                                    ShadowCompositeMode::BlendUnderlying => {
                                        blend_underlying_shadow_cell(shadow_cell, dest_cell)
                                    }
                                }
                            } else {
                                match shadow_spec.config.composite_mode {
                                    ShadowCompositeMode::GlyphOverlay => *cell,
                                    ShadowCompositeMode::GradeUnderlying => grade_shadow_cell(
                                        shadow_cell,
                                        cell,
                                        shadow_spec.config.color,
                                        &grade_config,
                                    ),
                                    ShadowCompositeMode::BlendUnderlying => {
                                        blend_underlying_shadow_cell(shadow_cell, cell)
                                    }
                                }
                            }
                        } else {
                            *cell
                        }
                    } else if in_element_ni {
                        *cell
                    } else if let Some(dest_cell) = dest.get(dest_x, dest_y) {
                        match shadow_spec.config.composite_mode {
                            ShadowCompositeMode::GlyphOverlay => blend_shadow_cell(cell, dest_cell),
                            ShadowCompositeMode::GradeUnderlying => grade_shadow_cell(
                                cell,
                                dest_cell,
                                shadow_spec.config.color,
                                &grade_config,
                            ),
                            ShadowCompositeMode::BlendUnderlying => {
                                blend_underlying_shadow_cell(cell, dest_cell)
                            }
                        }
                    } else {
                        *cell
                    };

                    dest.set(dest_x, dest_y, final_cell);
                    if shadow_region_candidate_ni {
                        shadow_role_writes.push((dest_x as u16, dest_y as u16));
                    }
                }
            }
        }
    }

    // Phase A.3.5 role write-back: tag every shadow-region destination
    // cell with RoleTag::Shadow. The dest borrow has dropped here; we
    // re-borrow through `destination` to reach roles_mut().
    if !shadow_role_writes.is_empty() {
        let roles = destination.roles_mut();
        for (dx, dy) in shadow_role_writes {
            roles.set((dx, dy), RoleTag::Shadow);
        }
    }

    // ── Pipeline observability Unit A — element-pass post-loop emit ─────
    // Emit StageFinished for every Sampler/Mask/Shader/Filter stage that
    // emit_per_stage_entered registered above. Skipped shaders short-circuit
    // inside emit_per_stage_finished so the pair is not double-counted.
    if let (Some(insp), Some(block)) =
        (inspector, element_stage_block.as_ref())
    {
        emit_per_stage_finished(insp, block);
    }
    // ────────────────────────────────────────────────────────────────────
}

/// Compute the effective shadow element rectangle, honouring a
/// `source_region` filter when present.
///
/// Returns `None` when the filter is `Some(role)` but no source cells
/// carry that role — in which case the caller should skip rendering the
/// shadow entirely.
fn effective_shadow_rect(
    source: &dyn Grid,
    source_roles: &RoleMap,
    element_rect: Rect,
    config: &tui_vfx_shadow::ShadowConfig,
) -> Option<Rect> {
    match &config.source_region {
        None => Some(element_rect),
        Some(role) => {
            let envelope =
                tui_vfx_shadow::extract_shadow_envelope(source, source_roles, Some(role.clone()));
            envelope.bounding_rect()
        }
    }
}

/// Non-inspected rendering loop (optimized hot path).
#[allow(clippy::too_many_arguments)]
fn render_loop(
    source: &dyn Grid,
    source_roles: &RoleMap,
    dest: &mut dyn Grid,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    options: &CompositionOptions<'_>,
    samplers: &SmallVec<[PreparedSampler; 2]>,
    prepared_masks: &SmallVec<[PreparedMask; 2]>,
    prepared_filters: &SmallVec<[PreparedFilter; 3]>,
    loop_t: f64,
) {
    let (w16, h16) = (width as u16, height as u16);
    let shader_t = CompositionPlaybackTiming::from_options(options).shader_t();
    let mask_t = compute_mask_t(options);
    // Arc-wrap the source role map once for cheap per-cell Arc::clone.
    let roles_arc: Arc<RoleMap> = cached_roles_arc(source_roles);
    // Resolve shader-layer regions and build the shader area once per frame,
    // not per cell.
    let shader_area = Rect::new(0, 0, w16, h16);
    let resolved_regions: Vec<Cow<'_, StyleRegion>> = options
        .shader_layers
        .iter()
        .map(|layer| layer.region.resolved(&options.runtime_params))
        .collect();

    for y in 0..height {
        for x in 0..width {
            let (local_x, local_y) = (x as u16, y as u16);

            // Sample coordinates — capture both source and accumulated delta.
            let chain_outcome =
                sample_sampler_chain(samplers, local_x, local_y, w16, h16, options.t);
            let (src_x, src_y) = match (chain_outcome.source_x, chain_outcome.source_y) {
                (Some(sx), Some(sy)) => (sx, sy),
                _ => continue,
            };
            let (delta_x, delta_y) = (chain_outcome.delta_x, chain_outcome.delta_y);

            // Per-cell ctx with sampler delta threaded in. `t` is overridden
            // per-stage from a Copy of this template.
            let base_ctx = VfxCellContext::new(local_x, local_y, w16, h16, 0, 0, options.t)
                .with_sampler_resolution(delta_x, delta_y);

            // Check mask visibility — derive ctx with t = mask_t.
            let mut mask_ctx = base_ctx;
            mask_ctx.t = mask_t;
            if !check_prepared_masks(&mask_ctx, prepared_masks, options.mask_combine_mode, None) {
                continue;
            }

            // Get source cell
            let Some(source_cell) = source.get(src_x as usize, src_y as usize) else {
                continue;
            };

            let mut out_cell = *source_cell;

            // Apply shaders
            let source_role = source_roles.get((src_x, src_y));
            apply_shaders(
                &mut out_cell,
                local_x,
                local_y,
                w16,
                h16,
                offset_x,
                offset_y,
                shader_t,
                options,
                &resolved_regions,
                shader_area,
                source_role,
                &roles_arc,
                delta_x,
                delta_y,
            );

            // Apply filters — derive ctx with t = loop_t.
            let mut filter_ctx = base_ctx;
            filter_ctx.t = loop_t;
            for filter in prepared_filters {
                filter.apply(&mut out_cell, &filter_ctx);
            }

            dest.set(offset_x + x, offset_y + y, out_cell);
        }
    }
}

/// Inspected rendering loop (with inspector callbacks).
#[allow(clippy::too_many_arguments)]
fn render_loop_inspected(
    source: &dyn Grid,
    source_roles: &RoleMap,
    dest: &mut dyn Grid,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    options: &CompositionOptions<'_>,
    samplers: &SmallVec<[PreparedSampler; 2]>,
    prepared_masks: &SmallVec<[PreparedMask; 2]>,
    prepared_filters: &SmallVec<[PreparedFilter; 3]>,
    loop_t: f64,
    sampler_label: Option<&str>,
    inspector: &mut dyn CompositorInspector,
) {
    let (w16, h16) = (width as u16, height as u16);
    let shader_t = CompositionPlaybackTiming::from_options(options).shader_t();
    let mask_t = compute_mask_t(options);
    // Arc-wrap the source role map once for cheap per-cell Arc::clone.
    let roles_arc: Arc<RoleMap> = cached_roles_arc(source_roles);
    // Resolve shader-layer regions and build the shader area once per frame,
    // not per cell.
    let shader_area = Rect::new(0, 0, w16, h16);
    let resolved_regions: Vec<Cow<'_, StyleRegion>> = options
        .shader_layers
        .iter()
        .map(|layer| layer.region.resolved(&options.runtime_params))
        .collect();

    // ── Pipeline observability Unit A — pre-loop emit ───────────────────
    // Emit RoleMapMaterialized first, then delegate the per-stage
    // StageEntered / ScopeEvaluated emit to the shared helper. Shaders
    // that match zero cells emit StageSkipped immediately and the helper
    // flags them in `block.skipped_shader_indices` so
    // apply_shaders_inspected skips them.
    emit_role_map_materialized(inspector, source_roles, w16, h16);

    let mask_labels: Vec<String> = prepared_masks
        .iter()
        .enumerate()
        .map(|(i, mask)| format!("{}#{}", mask.name(), i + 1))
        .collect();
    let filter_labels: Vec<String> = prepared_filters
        .iter()
        .enumerate()
        .map(|(i, filter)| format!("{}#{}", filter.name(), i + 1))
        .collect();
    let shader_pairs: Vec<(&ShaderWithRegion, &StyleRegion)> = options
        .shader_layers
        .iter()
        .zip(resolved_regions.iter())
        .map(|(layer, resolved)| (layer, resolved.as_ref()))
        .collect();
    let block = emit_per_stage_entered(
        inspector,
        1,
        &PerStageInputs {
            sampler_label,
            mask_labels: &mask_labels,
            shader_layers_with_regions: &shader_pairs,
            filter_labels: &filter_labels,
            source_roles,
            width: w16,
            height: h16,
        },
    );
    let skipped_shader_indices = &block.skipped_shader_indices;
    // ────────────────────────────────────────────────────────────────────

    for y in 0..height {
        for x in 0..width {
            let (local_x, local_y) = (x as u16, y as u16);

            // Sample coordinates — capture both source and accumulated delta.
            let chain_outcome =
                sample_sampler_chain(samplers, local_x, local_y, w16, h16, options.t);
            if let Some(sampler_label) = sampler_label {
                inspector.on_sampler_applied(
                    local_x,
                    local_y,
                    chain_outcome.source_x,
                    chain_outcome.source_y,
                    sampler_label,
                );
            }

            let (src_x, src_y) = match (chain_outcome.source_x, chain_outcome.source_y) {
                (Some(sx), Some(sy)) => (sx, sy),
                _ => continue,
            };
            let (delta_x, delta_y) = (chain_outcome.delta_x, chain_outcome.delta_y);

            // Per-cell ctx with sampler delta threaded in. `t` is overridden
            // per-stage from a Copy of this template.
            let base_ctx = VfxCellContext::new(local_x, local_y, w16, h16, 0, 0, options.t)
                .with_sampler_resolution(delta_x, delta_y);

            // Check masks with inspector — derive ctx with t = mask_t.
            let mut mask_ctx = base_ctx;
            mask_ctx.t = mask_t;
            if !check_prepared_masks(
                &mask_ctx,
                prepared_masks,
                options.mask_combine_mode,
                Some(inspector),
            ) {
                continue;
            }

            // Get source cell
            let Some(source_cell) = source.get(src_x as usize, src_y as usize) else {
                continue;
            };

            let mut out_cell = *source_cell;

            // Apply shaders with inspector
            let source_role = source_roles.get((src_x, src_y));
            apply_shaders_inspected(
                &mut out_cell,
                local_x,
                local_y,
                w16,
                h16,
                offset_x,
                offset_y,
                shader_t,
                options,
                &resolved_regions,
                shader_area,
                inspector,
                source_role,
                &roles_arc,
                skipped_shader_indices,
                delta_x,
                delta_y,
            );

            // Apply filters with inspector — derive ctx with t = loop_t.
            let mut filter_ctx = base_ctx;
            filter_ctx.t = loop_t;
            for (filter_index, filter) in prepared_filters.iter().enumerate() {
                let before_cell = out_cell;
                filter.apply(&mut out_cell, &filter_ctx);
                inspector.on_filter_applied(
                    local_x,
                    local_y,
                    &before_cell,
                    &out_cell,
                    &format!("{}#{}", filter.name(), filter_index + 1),
                );
            }

            inspector.on_cell_rendered(local_x, local_y, &out_cell);
            dest.set(offset_x + x, offset_y + y, out_cell);
        }
    }

    // ── Pipeline observability Unit A — post-loop emit ──────────────────
    // Emit StageFinished for every non-skipped stage. Order does not matter
    // (consumers join on step_id) but for tape readability we emit in
    // the same order stages were entered.
    emit_per_stage_finished(inspector, &block);
}

/// Compute mask progress value (inverted for exit phase).
///
/// Clamps output to 0.0-1.0 for consistency with shader_t behavior,
/// preventing out-of-range mask evaluation if callers overshoot.
#[inline]
fn compute_mask_t(options: &CompositionOptions<'_>) -> f64 {
    let t = match options.phase {
        Some(Phase::End) => 1.0 - options.t,
        _ => options.t,
    };
    t.clamp(0.0, 1.0)
}

/// Apply shader layers to a cell.
///
/// `delta_x` / `delta_y` carry the sampler chain's accumulated resolved-coord
/// delta; the orchestrator threads them through so shader contexts observe
/// per-cell sampler displacement via [`ShaderContext::with_sampler_resolution`]
/// — a `Deref`-reachable read of `ctx.resolved_x` / `ctx.resolved_y`.
#[allow(clippy::too_many_arguments)]
fn apply_shaders(
    out_cell: &mut tui_vfx_types::Cell,
    local_x: u16,
    local_y: u16,
    w16: u16,
    h16: u16,
    offset_x: usize,
    offset_y: usize,
    shader_t: f64,
    options: &CompositionOptions<'_>,
    resolved_regions: &[Cow<'_, StyleRegion>],
    shader_area: Rect,
    source_role: Option<tui_vfx_types::RoleTag>,
    roles_arc: &Arc<RoleMap>,
    delta_x: i32,
    delta_y: i32,
) {
    for (layer, resolved) in options.shader_layers.iter().zip(resolved_regions.iter()) {
        // `resolved` was produced once per layer per frame by the caller.
        if resolved.should_style(local_x, local_y, source_role.clone(), shader_area) {
            let (ctx_x, ctx_y, ctx_w, ctx_h) = resolved
                .to_local_coords(local_x, local_y)
                .unwrap_or((local_x, local_y, w16, h16));

            let shader_ctx = ShaderContext::new(
                ctx_x,
                ctx_y,
                ctx_w,
                ctx_h,
                offset_x as u16,
                offset_y as u16,
                shader_t,
                options.phase,
                Some(options.runtime_params.clone()),
            )
            .with_roles(roles_arc.clone())
            .with_sampler_resolution(delta_x, delta_y);

            let current_style = Style {
                fg: out_cell.fg,
                bg: out_cell.bg,
                mods: out_cell.mods,
            };
            let new_style = layer.shader.style_at(&shader_ctx, current_style);
            out_cell.fg = new_style.fg;
            out_cell.bg = new_style.bg;
            out_cell.mods = new_style.mods;
        }
    }
}

/// Apply shader layers to a cell with inspector callbacks.
///
/// `skipped_shader_indices` lists shaders whose pre-loop scope tally
/// returned matched=0 (StageSkipped already emitted by
/// `emit_shader_entered_or_skipped`). Those shaders are silently bypassed
/// in the cell loop so per-cell `on_shader_applied` events do not fire
/// for a stage already marked skipped on the trace.
#[allow(clippy::too_many_arguments)]
fn apply_shaders_inspected(
    out_cell: &mut tui_vfx_types::Cell,
    local_x: u16,
    local_y: u16,
    w16: u16,
    h16: u16,
    offset_x: usize,
    offset_y: usize,
    shader_t: f64,
    options: &CompositionOptions<'_>,
    resolved_regions: &[Cow<'_, StyleRegion>],
    shader_area: Rect,
    inspector: &mut dyn CompositorInspector,
    source_role: Option<tui_vfx_types::RoleTag>,
    roles_arc: &Arc<RoleMap>,
    skipped_shader_indices: &[usize],
    delta_x: i32,
    delta_y: i32,
) {
    for (shader_index, (layer, resolved)) in options
        .shader_layers
        .iter()
        .zip(resolved_regions.iter())
        .enumerate()
    {
        if skipped_shader_indices.contains(&shader_index) {
            continue;
        }
        if resolved.should_style(local_x, local_y, source_role.clone(), shader_area) {
            let (ctx_x, ctx_y, ctx_w, ctx_h) = resolved
                .to_local_coords(local_x, local_y)
                .unwrap_or((local_x, local_y, w16, h16));

            let shader_ctx = ShaderContext::new(
                ctx_x,
                ctx_y,
                ctx_w,
                ctx_h,
                offset_x as u16,
                offset_y as u16,
                shader_t,
                options.phase,
                Some(options.runtime_params.clone()),
            )
            .with_roles(roles_arc.clone())
            .with_sampler_resolution(delta_x, delta_y);

            let before_style = Style {
                fg: out_cell.fg,
                bg: out_cell.bg,
                mods: out_cell.mods,
            };
            let new_style = layer.shader.style_at(&shader_ctx, before_style);
            out_cell.fg = new_style.fg;
            out_cell.bg = new_style.bg;
            out_cell.mods = new_style.mods;

            inspector.on_shader_applied(
                local_x,
                local_y,
                before_style,
                new_style,
                &layer.inspector_shader_label(shader_index),
            );
        }
    }
}

// <FILE>tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs</FILE> - <DESC>Pipeline orchestrator with signal-driven composition</DESC>
// <VERS>END OF VERSION: 13.0.0</VERS>
