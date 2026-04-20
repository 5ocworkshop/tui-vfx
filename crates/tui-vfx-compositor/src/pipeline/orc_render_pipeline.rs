// <FILE>tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs</FILE> - <DESC>Pipeline orchestrator with signal-driven composition</DESC>
// <VERS>VERSION: 11.6.0</VERS>
// <WCTX>Phase 0 P0.1 — build PrepareContext carrying runtime_params before filter preparation</WCTX>
// <CLOG>Construct a PrepareContext from loop_t + options.runtime_params and hand it to prepare_filters at both callsites</CLOG>

use super::cls_composition_options::CompositionOptions;
use super::cls_prepare_context::PrepareContext;
use super::cls_prepared_filter::{PreparedFilter, prepare_filters};
use super::cls_prepared_mask::{PreparedMask, prepare_masks};
use super::cls_prepared_sampler::{PreparedSampler, prepare_sampler};
use super::cls_render_area::RenderArea;
use super::fnc_check_masks::check_prepared_masks;
use super::fnc_grade_shadow_cell::grade_shadow_cell;
use crate::traits::pipeline_inspector::CompositorInspector;
use crate::types::cls_sampler_spec::SamplerSpec;
use mixed_signals::traits::Phase;
use smallvec::SmallVec;
use tui_vfx_shadow::{ShadowCompositeMode, render_shadow};
use tui_vfx_style::traits::ShaderContext;
use tui_vfx_types::{Cell, Grid, OwnedGrid, Rect, Style};

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
    dest: &mut dyn Grid,
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
            source, dest, width, height, offset_x, offset_y, options, inspector,
        );
        return;
    }

    // FAST PATH: Check if we can copy directly without effects.
    let has_sampler = options
        .sampler_spec
        .as_ref()
        .is_some_and(|s| !matches!(s, SamplerSpec::None));
    let has_masks = !options.masks.is_empty();
    let has_filters = !options.filters.is_empty();
    let has_shaders = !options.shader_layers.is_empty();

    if !has_sampler && !has_masks && !has_filters && !has_shaders && inspector.is_none() {
        for y in 0..height {
            for x in 0..width {
                if let Some(cell) = source.get(x, y) {
                    dest.set(offset_x + x, offset_y + y, *cell);
                }
            }
        }
        return;
    }

    // SLOW PATH: Effects are active
    let sampler = prepare_sampler(options.t, &options.sampler_spec);
    let prepared_masks = prepare_masks(options.masks.as_ref());
    let loop_t = options.loop_t.unwrap_or(options.t);
    let prepare_ctx = PrepareContext::new(loop_t, options.runtime_params.as_ref());
    let prepared_filters = prepare_filters(options.filters.as_ref(), &prepare_ctx);

    // Dispatch to inspected or non-inspected loop
    // (Two loops needed due to Rust borrow checker constraints with optional mutable refs)
    if let Some(inspector) = inspector {
        render_loop_inspected(
            source,
            dest,
            width,
            height,
            offset_x,
            offset_y,
            &options,
            &sampler,
            &prepared_masks,
            &prepared_filters,
            loop_t,
            inspector,
        );
    } else {
        render_loop(
            source,
            dest,
            width,
            height,
            offset_x,
            offset_y,
            &options,
            &sampler,
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
    dest: &mut dyn Grid,
    area: RenderArea,
    options: CompositionOptions<'_>,
    inspector: Option<&mut dyn CompositorInspector>,
) {
    render_pipeline(
        source,
        dest,
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
#[allow(clippy::too_many_arguments)]
fn render_pipeline_with_shadow(
    source: &dyn Grid,
    dest: &mut dyn Grid,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    options: CompositionOptions<'_>,
    inspector: Option<&mut dyn CompositorInspector>,
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
            (ext_width, ext_height, elem_offset_x, elem_offset_y, element_rect)
        };

    // Create working buffer for shadow + element
    let mut buffer = OwnedGrid::new(ext_width, ext_height);

    // Render shadow to buffer (uses animation progress for fade sync)
    render_shadow(&mut buffer, element_rect, &shadow_spec.config, options.t);
    let shadow_only_buffer = buffer.clone();

    // Prepare effects for element rendering
    let sampler = prepare_sampler(options.t, &options.sampler_spec);
    let loop_t = options.loop_t.unwrap_or(options.t);
    let prepare_ctx = PrepareContext::new(loop_t, options.runtime_params.as_ref());
    let prepared_filters = prepare_filters(options.filters.as_ref(), &prepare_ctx);
    let shader_t = options.loop_t.unwrap_or(options.t).clamp(0.0, 1.0);

    // Render element content to buffer (on top of shadow)
    let (w16, h16) = (width as u16, height as u16);
    for y in 0..height {
        for x in 0..width {
            let (local_x, local_y) = (x as u16, y as u16);

            // Sample coordinates (sampler operates on element dimensions)
            let (src_x, src_y) = match sampler.sample(local_x, local_y, w16, h16, options.t) {
                (Some(sx), Some(sy)) => (sx, sy),
                _ => continue,
            };

            // Get source cell
            let Some(source_cell) = source.get(src_x as usize, src_y as usize) else {
                continue;
            };

            let mut out_cell = *source_cell;

            // Apply shaders (coordinates relative to element)
            apply_shaders(
                &mut out_cell,
                local_x,
                local_y,
                w16,
                h16,
                offset_x + elem_offset_x,
                offset_y + elem_offset_y,
                shader_t,
                &options,
            );

            // Apply filters
            for filter in &prepared_filters {
                filter.apply(&mut out_cell, local_x, local_y, w16, h16, loop_t);
            }

            // Write to buffer at element position
            buffer.set(elem_offset_x + x, elem_offset_y + y, out_cell);
        }
    }

    // Now copy from buffer to dest, applying masks over the extended area
    let prepared_masks = prepare_masks(options.masks.as_ref());
    let mask_t = compute_mask_t(&options);
    let (ext_w16, ext_h16) = (ext_width as u16, ext_height as u16);

    if let Some(inspector) = inspector {
        // Inspected path
        for y in 0..ext_height {
            for x in 0..ext_width {
                let (local_x, local_y) = (x as u16, y as u16);

                // Check mask visibility over extended area
                if !check_prepared_masks(
                    local_x,
                    local_y,
                    ext_w16,
                    ext_h16,
                    mask_t,
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
                    let element_left = usize::from(element_rect.x);
                    let element_top = usize::from(element_rect.y);
                    let element_right = element_left + usize::from(element_rect.width);
                    let element_bottom = element_top + usize::from(element_rect.height);
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
                                ShadowCompositeMode::GlyphOverlay => blend_shadow_cell(cell, dest_cell),
                                ShadowCompositeMode::GradeUnderlying => grade_shadow_cell(
                                    cell,
                                    dest_cell,
                                    shadow_spec.config.color,
                                    &grade_config,
                                ),
                            }
                        } else {
                            *cell
                        }
                    };

                    inspector.on_cell_rendered(local_x, local_y, &final_cell);
                    dest.set(dest_x, dest_y, final_cell);
                }
            }
        }
    } else {
        // Non-inspected path (optimized)
        for y in 0..ext_height {
            for x in 0..ext_width {
                let (local_x, local_y) = (x as u16, y as u16);

                // Check mask visibility over extended area
                if !check_prepared_masks(
                    local_x,
                    local_y,
                    ext_w16,
                    ext_h16,
                    mask_t,
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

                    let final_cell = if shadow_element_rect.is_some() {
                        let source_empty = source
                            .get(x, y)
                            .is_none_or(|source_cell| source_cell.is_empty());
                        let element_left = usize::from(element_rect.x);
                        let element_top = usize::from(element_rect.y);
                        let element_right = element_left + usize::from(element_rect.width);
                        let element_bottom = element_top + usize::from(element_rect.height);
                        let in_element = x >= element_left
                            && x < element_right
                            && y >= element_top
                            && y < element_bottom;
                        let shadow_has_coverage =
                            shadow_cell.is_some_and(|shadow_cell| !shadow_cell.is_empty());
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
                                }
                            }
                        } else {
                            *cell
                        }
                    } else {
                        let in_element = x >= elem_offset_x
                            && x < elem_offset_x + width
                            && y >= elem_offset_y
                            && y < elem_offset_y + height;

                        if in_element {
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
                            }
                        } else {
                            *cell
                        }
                    };

                    dest.set(dest_x, dest_y, final_cell);
                }
            }
        }
    }
}

/// Non-inspected rendering loop (optimized hot path).
#[allow(clippy::too_many_arguments)]
fn render_loop(
    source: &dyn Grid,
    dest: &mut dyn Grid,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    options: &CompositionOptions<'_>,
    sampler: &PreparedSampler,
    prepared_masks: &SmallVec<[PreparedMask; 2]>,
    prepared_filters: &SmallVec<[PreparedFilter; 3]>,
    loop_t: f64,
) {
    let (w16, h16) = (width as u16, height as u16);
    let shader_t = options.loop_t.unwrap_or(options.t).clamp(0.0, 1.0);
    let mask_t = compute_mask_t(options);

    for y in 0..height {
        for x in 0..width {
            let (local_x, local_y) = (x as u16, y as u16);

            // Sample coordinates
            let (src_x, src_y) = match sampler.sample(local_x, local_y, w16, h16, options.t) {
                (Some(sx), Some(sy)) => (sx, sy),
                _ => continue,
            };

            // Check mask visibility
            if !check_prepared_masks(
                local_x,
                local_y,
                w16,
                h16,
                mask_t,
                prepared_masks,
                options.mask_combine_mode,
                None,
            ) {
                continue;
            }

            // Get source cell
            let Some(source_cell) = source.get(src_x as usize, src_y as usize) else {
                continue;
            };

            let mut out_cell = *source_cell;

            // Apply shaders
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
            );

            // Apply filters
            for filter in prepared_filters {
                filter.apply(&mut out_cell, local_x, local_y, w16, h16, loop_t);
            }

            dest.set(offset_x + x, offset_y + y, out_cell);
        }
    }
}

/// Inspected rendering loop (with inspector callbacks).
#[allow(clippy::too_many_arguments)]
fn render_loop_inspected(
    source: &dyn Grid,
    dest: &mut dyn Grid,
    width: usize,
    height: usize,
    offset_x: usize,
    offset_y: usize,
    options: &CompositionOptions<'_>,
    sampler: &PreparedSampler,
    prepared_masks: &SmallVec<[PreparedMask; 2]>,
    prepared_filters: &SmallVec<[PreparedFilter; 3]>,
    loop_t: f64,
    inspector: &mut dyn CompositorInspector,
) {
    let (w16, h16) = (width as u16, height as u16);
    let shader_t = options.loop_t.unwrap_or(options.t).clamp(0.0, 1.0);
    let mask_t = compute_mask_t(options);

    for y in 0..height {
        for x in 0..width {
            let (local_x, local_y) = (x as u16, y as u16);

            // Sample coordinates
            let (src_local_x, src_local_y) = sampler.sample(local_x, local_y, w16, h16, options.t);
            inspector.on_sampler_applied(
                local_x,
                local_y,
                src_local_x,
                src_local_y,
                &format!("{}#1", sampler.name()),
            );

            let (src_x, src_y) = match (src_local_x, src_local_y) {
                (Some(sx), Some(sy)) => (sx, sy),
                _ => continue,
            };

            // Check masks with inspector
            if !check_prepared_masks(
                local_x,
                local_y,
                w16,
                h16,
                mask_t,
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
                inspector,
            );

            // Apply filters with inspector
            for (filter_index, filter) in prepared_filters.iter().enumerate() {
                let before_cell = out_cell;
                filter.apply(&mut out_cell, local_x, local_y, w16, h16, loop_t);
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
) {
    for layer in &options.shader_layers {
        // Resolve any runtime-parameter bindings (e.g. Cell { x, y } with
        // BindableU16::Binding coords) into concrete literals for this
        // frame's runtime_params. Returns Cow::Borrowed for bindless regions.
        let resolved = layer.region.resolved(&options.runtime_params);
        if resolved.should_style(local_x, local_y, w16, h16) {
            let (ctx_x, ctx_y, ctx_w, ctx_h) = resolved
                .to_local_coords(local_x, local_y)
                .unwrap_or((local_x, local_y, w16, h16));

            let shader_ctx = ShaderContext {
                local_x: ctx_x,
                local_y: ctx_y,
                width: ctx_w,
                height: ctx_h,
                screen_x: offset_x as u16,
                screen_y: offset_y as u16,
                t: shader_t,
                phase: options.phase,
                runtime_params: options.runtime_params.clone(),
            };

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
    inspector: &mut dyn CompositorInspector,
) {
    for (shader_index, layer) in options.shader_layers.iter().enumerate() {
        let resolved = layer.region.resolved(&options.runtime_params);
        if resolved.should_style(local_x, local_y, w16, h16) {
            let (ctx_x, ctx_y, ctx_w, ctx_h) = resolved
                .to_local_coords(local_x, local_y)
                .unwrap_or((local_x, local_y, w16, h16));

            let shader_ctx = ShaderContext {
                local_x: ctx_x,
                local_y: ctx_y,
                width: ctx_w,
                height: ctx_h,
                screen_x: offset_x as u16,
                screen_y: offset_y as u16,
                t: shader_t,
                phase: options.phase,
                runtime_params: options.runtime_params.clone(),
            };

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
                &format!("{}#{}", layer.shader.name(), shader_index + 1),
            );
        }
    }
}

/// Blend a shadow cell with the destination cell.
///
/// For shadow cells, we want to composite with the underlying content:
/// - If the cell's bg has alpha < 255, blend it with dest's bg
/// - If the cell's fg has alpha < 255, blend it with dest's bg (NOT fg!)
///   because the fg in shadow cells represents the shadow portion of
///   half-block characters, which should darken the background underneath
/// - Keep the cell's character and modifiers
///
/// This allows half-block shadow characters to show underlying content
/// through the "surface" portions while maintaining the shadow effect.
#[inline]
fn blend_shadow_cell(shadow_cell: &Cell, dest_cell: &Cell) -> Cell {
    let blended_bg = if shadow_cell.bg.a < 255 && shadow_cell.bg.a > 0 {
        // Semi-transparent: blend with dest bg
        shadow_cell.bg.blend_over(dest_cell.bg)
    } else if shadow_cell.bg.a == 0 {
        // Fully transparent: use dest bg (show-through portion)
        dest_cell.bg
    } else {
        // Opaque: use shadow bg
        shadow_cell.bg
    };

    // For shadow cells, fg represents the shadow portion of half-block characters.
    // We blend it with dest's BG (not fg) because shadows darken the background.
    let blended_fg = if shadow_cell.fg.a < 255 && shadow_cell.fg.a > 0 {
        // Semi-transparent shadow: blend with dest bg for darkening effect
        shadow_cell.fg.blend_over(dest_cell.bg)
    } else if shadow_cell.fg.a == 0 {
        // Fully transparent: use dest bg
        dest_cell.bg
    } else {
        // Opaque: use shadow fg
        shadow_cell.fg
    };

    Cell::styled(shadow_cell.ch, blended_fg, blended_bg, shadow_cell.mods)
        .with_mod_alpha(shadow_cell.mod_alpha)
}

// <FILE>tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs</FILE> - <DESC>Pipeline orchestrator with signal-driven composition</DESC>
// <VERS>END OF VERSION: 11.5.0</VERS>
