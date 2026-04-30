// <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs</FILE> - <DESC>Lower player render requests into compositor CompositionSpec modes</DESC>
// <VERS>VERSION: 0.13.0</VERS>
// <WCTX>Native compositor lowering: map bounded v3.1 recipe graph effects into native CompositionSpec and source-stage content/style/filter work with honest fallback diagnostics.</WCTX>
// <CLOG>0.13.0: MINOR — lower style.italicWindow through a source-owned native style stage.</CLOG>

use std::collections::BTreeMap;

use mixed_signals::types::SignalOrFloat;
use serde_json::json;
use tui_vfx_compositor::types::cls_filter_spec::{ScannerAxis, ScannerMotionMode, VignetteEdge};
use tui_vfx_compositor::{
    pipeline::{CompositionSpec, ShaderLayerSpec},
    types::{
        ApplyTo, Axis, BindableValue, DitherMatrix, FilterSpec, HoverBarPosition, MaskSpec,
        PatternType, RadialOrigin, RippleCenter, SamplerSpec, WipeDirection,
    },
};
use tui_vfx_contract::{NodeSpec, RecipeDocument, Value, ValueSource};
use tui_vfx_player::{
    PlayerRenderBackendCompositionEvidence, PlayerRenderBackendDiagnostic,
    PlayerRenderBackendRequest, PlayerRenderCompositionMode, PlayerRenderIrReport,
    fnc_resolve_value_source::resolve_value_source_with_graph_values,
};
use tui_vfx_style::models::{
    BorderSweepShader, ColorConfig, ColorSpace, Gradient, LinearGradientApplyTo,
    LinearGradientShader, RevealWipeShader, SpatialShaderType, StyleRegion,
};

const SUPPORTED_WIPE_DIRECTIONS: &[&str] = &[
    "leftToRight",
    "rightToLeft",
    "topToBottom",
    "bottomToTop",
    "outFromTopLeft",
    "outFromTopRight",
    "outFromBottomLeft",
    "outFromBottomRight",
    "inToTopLeft",
    "inToTopRight",
    "inToBottomLeft",
    "inToBottomRight",
];

/// Complete lowering result used by the compositor backend adapter.
#[derive(Clone, Debug)]
pub struct LoweredCompositionSpec {
    /// Composition instructions to pass to the compositor pipeline.
    pub spec: CompositionSpec,
    /// Source-content transforms applied before compositor effects while staying source-only.
    pub content_stages: Vec<NativeContentStage>,
    /// Source-style transforms applied before compositor effects while staying source-only.
    pub style_stages: Vec<NativeStyleStage>,
    /// Backend diagnostics describing lowering decisions.
    pub diagnostics: Vec<PlayerRenderBackendDiagnostic>,
    /// Evidence fields copied onto the player backend output.
    pub evidence: PlayerRenderBackendCompositionEvidence,
}

/// Native content transform stage owned by the compositor backend adapter.
#[derive(Clone, Debug, PartialEq)]
pub enum NativeContentStage {
    /// Reveal source glyphs progressively with an optional cursor and wake.
    Typewriter {
        speed: f64,
        speed_variance: f64,
        cursor_character: char,
        cursor_wake: TypewriterCursorWake,
        wake_cells: usize,
    },
    /// Flip unresolved source glyphs through a split-flap character set.
    SplitFlap {
        settle: f64,
        cascade: f64,
        speed: f64,
        cycles: f64,
        charset: String,
        tile_width: usize,
        tile_height: usize,
        jitter: f64,
    },
    /// Roll source glyphs from a prior message into the target source message.
    Odometer {
        direction: String,
        travel: String,
        from_message: String,
        tile_width: usize,
        tile_height: usize,
    },
    /// Reveal/move source glyphs by row/cell routes.
    CellMotion {
        route: String,
        stagger: usize,
        affect: String,
    },
    /// Rotate each source row inside its authored marquee width.
    Marquee {
        direction: String,
        speed: f64,
        width: usize,
    },
    /// Replace progressively resolved glyphs with a target glyph family.
    Morph { target: String },
    /// Replace unresolved glyphs with deterministic scramble glyphs.
    Scramble { seed: usize, charset: String },
    /// Mark wrapping rows with an end-of-line indicator.
    WrapIndicator { every: usize },
    /// Redact unresolved source glyphs with a symbol.
    Redact { symbol: char, reveal: f64 },
    /// Mirror source rows horizontally or vertically.
    Mirror { axis: String },
    /// Replace the source grid with a formatted numeric value.
    Numeric {
        value: f64,
        decimals: usize,
        prefix: String,
        suffix: String,
    },
    /// Dissolve source glyphs toward a replacement character.
    Dissolve {
        replacement: char,
        direction: String,
        seed: usize,
    },
    /// Shift alternating source rows.
    GlitchShift { amount: usize, seed: usize },
    /// Shift source rows between authored start/end columns.
    SlideShift { start_col: i64, end_col: i64 },
    /// Apply player-compatible CRT bow, jitter, and scanline semantics to source rows.
    CrtSampler {
        curvature: f64,
        scanline_strength: f64,
        jitter: f64,
    },
    /// Apply player-compatible CRT jitter row shifting to source rows.
    CrtJitterSampler {
        amplitude: f64,
        frequency: f64,
        seed: usize,
    },
    /// Apply V2-compatible fault-line row displacement to source rows.
    FaultLineSampler {
        seed: u64,
        intensity: f64,
        split_bias: f64,
        width: usize,
        height: usize,
    },
    /// Apply player-compatible cellular mask semantics to source rows.
    CellularMask {
        cell_size: usize,
        seed: usize,
        threshold: f64,
    },
    /// Apply player-compatible blinds mask semantics to source rows.
    BlindsMask { orientation: String, count: usize },
    /// Apply player-compatible diamond mask semantics to source rows.
    DiamondMask { soft_edge: bool },
    /// Apply player-compatible dissolve mask semantics to source rows.
    DissolveMask { seed: u64, chunk_size: usize },
    /// Apply player-compatible iris mask semantics to source rows.
    IrisMask { shape: String, soft_edge: bool },
    /// Apply player-compatible radial mask semantics to source rows.
    RadialMask { soft_edge: bool },
    /// Apply player-compatible wipe/path-reveal mask semantics to source rows.
    WipeMask { direction: String, soft_edge: bool },
}

/// Native style transform stage owned by the compositor backend adapter.
#[derive(Clone, Debug, PartialEq)]
pub enum NativeStyleStage {
    /// Apply foreground/background colors to modulo-selected columns.
    ModuloColumns {
        modulus: usize,
        remainder: usize,
        foreground: String,
        background: String,
    },
    /// Apply deterministic neon flicker styling.
    NeonFlicker {
        color: String,
        stability: f64,
        dim_amount: f64,
        italic_window: bool,
    },
    /// Apply player-compatible color fade styling to existing foreground/background channels.
    ColorFade { target: String, color_space: String },
    /// Apply player-compatible HSL color shift styling to existing channels.
    ColorShift {
        hue_shift: f64,
        saturation_shift: f64,
        lightness_shift: f64,
    },
    /// Apply player-compatible italic-window styling.
    ItalicWindow { start: f64, end: f64 },
    /// Apply player-compatible vignette filter styling.
    Vignette {
        strength: f64,
        edge_color: String,
        apply_to: String,
    },
    /// Apply player-compatible bracket emphasis filter styling.
    BracketEmphasis {
        emphasis_color: String,
        edge_width: usize,
        apply_to: String,
    },
    /// Apply player-compatible edge grow filter styling.
    EdgeGrow {
        direction: String,
        progress: f64,
        edge_color: String,
        apply_to: String,
    },
    /// Apply player-compatible hover bar filter styling.
    HoverBar {
        bar_color: String,
        thickness: usize,
        position: f64,
        apply_to: String,
    },
    /// Apply player-compatible matrix rain filter styling.
    MatrixRain {
        speed_multiplier: f64,
        speed_min: f64,
        speed_max: f64,
        glyph_change_hz: f64,
        density: f64,
        seed: f64,
        trail_min: f64,
        trail_max: f64,
        affect: String,
        chars: String,
        mode: String,
        preset: String,
        head_color: String,
        tail_color: String,
    },
    /// Apply player-compatible sub-pixel bar filter styling.
    SubPixelBar {
        filled_color: String,
        unfilled_color: String,
        progress: f64,
        direction: String,
    },
    /// Apply player-compatible underline wipe filter styling.
    UnderlineWipe {
        underline_color: String,
        progress: f64,
        thickness: usize,
        apply_to: String,
    },
    /// Apply player-compatible highlighter shader styling.
    Highlighter {
        color: String,
        blend_strength: f64,
        text_contrast: f64,
        soft_edge: bool,
        direction: String,
        mode: String,
        row_mask: i64,
        band_width: usize,
    },
    /// Apply player-compatible focus field shader styling.
    FocusField {
        color: String,
        rect_x: f64,
        rect_y: f64,
        rect_width: f64,
        rect_height: f64,
        center_x: f64,
        center_y: f64,
        shape: String,
        radius_x: f64,
        radius_y: f64,
        feather: f64,
        intensity: f64,
    },
    /// Apply player-compatible glisten band shader styling.
    GlistenBand {
        color: String,
        blend_strength: f64,
        angle_deg: f64,
        speed: f64,
        head: f64,
        tail: f64,
        band_width: f64,
        direction: String,
    },
    /// Apply player-compatible wayfinding node shader styling.
    WayfindingNode {
        current_index: usize,
        nodes: usize,
        previous_strength: f64,
        future_strength: f64,
        intensity: f64,
        radius: usize,
        active_color: String,
    },
    /// Apply player-compatible barber-pole shader styling.
    BarberPole {
        stripe_color: String,
        background_color: String,
        stripe_width: usize,
        gap_width: usize,
        angle_deg: f64,
        speed: f64,
        apply_to: String,
    },
    /// Apply player-compatible diffusion shader styling.
    Diffusion {
        color: String,
        center_x: f64,
        center_y: f64,
        radius: f64,
        intensity: f64,
        apply_to: String,
    },
    /// Apply player-compatible radar shader styling.
    Radar {
        color: String,
        speed: f64,
        tail_length: f64,
        apply_to: String,
    },
}

/// Cursor wake behavior for native typewriter content.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TypewriterCursorWake {
    Off,
    Ghost,
    Tint,
}

/// Lower the already-resolved player render IR sample into the legacy IR-resolved compositor spec.
pub fn lower_player_ir_to_composition_spec(
    input: &PlayerRenderIrReport,
) -> (CompositionSpec, Vec<PlayerRenderBackendDiagnostic>) {
    let lowered = lower_ir_resolved_composition_spec(input);
    (lowered.spec, lowered.diagnostics)
}

/// Lower a full backend request into the requested compositor composition mode.
pub fn lower_backend_request_to_composition_spec(
    request: &PlayerRenderBackendRequest,
) -> LoweredCompositionSpec {
    match request.backend_options.composition_mode {
        PlayerRenderCompositionMode::IrResolved => lower_ir_resolved_composition_spec(&request.ir),
        PlayerRenderCompositionMode::Native => lower_native_composition_spec(request),
        PlayerRenderCompositionMode::Auto => lower_auto_composition_spec(request),
    }
}

fn lower_ir_resolved_composition_spec(input: &PlayerRenderIrReport) -> LoweredCompositionSpec {
    let mut spec = CompositionSpec {
        t: input.phase_t,
        loop_t: input.loop_t,
        ..CompositionSpec::default()
    };
    spec.preserve_unfilled = true;

    let diagnostics = vec![PlayerRenderBackendDiagnostic {
        code: "playerIrAlreadyResolved".to_string(),
        path: "graph".to_string(),
        message: "The compositor backend consumed player-resolved styled render IR.".to_string(),
    }];

    LoweredCompositionSpec {
        evidence: PlayerRenderBackendCompositionEvidence {
            composition_mode: "irResolved".to_string(),
            fallback_used: false,
            native_lowering_attempted: false,
            native_lowering_succeeded: false,
            composition_spec_non_empty: false,
            lowered_node_count: 0,
            unlowered_node_count: 0,
            lowered_effect_ids: vec![],
            unlowered_effect_ids: vec![],
            composition_spec_summary: composition_spec_summary(&spec),
            source_render_mode: "postEffectIr".to_string(),
            native_source_isolated: false,
        },
        spec,
        content_stages: Vec::new(),
        style_stages: Vec::new(),
        diagnostics,
    }
}

fn lower_auto_composition_spec(request: &PlayerRenderBackendRequest) -> LoweredCompositionSpec {
    let mut lowered = lower_native_composition_spec(request);
    if lowered.evidence.unlowered_node_count == 0 {
        lowered.evidence.composition_mode = "auto".to_string();
        return lowered;
    }

    let fallback = lower_ir_resolved_composition_spec(&request.ir);
    lowered.spec = fallback.spec;
    lowered.content_stages = Vec::new();
    lowered.style_stages = Vec::new();
    lowered.evidence.composition_mode = "auto".to_string();
    lowered.evidence.fallback_used = true;
    lowered.evidence.native_lowering_attempted = true;
    lowered.evidence.native_lowering_succeeded = false;
    lowered.evidence.composition_spec_non_empty = false;
    lowered.evidence.composition_spec_summary = fallback.evidence.composition_spec_summary;
    lowered.evidence.source_render_mode = "postEffectIr".to_string();
    lowered.evidence.native_source_isolated = false;
    lowered.diagnostics.push(PlayerRenderBackendDiagnostic {
        code: "requiresIrFallback".to_string(),
        path: "graph".to_string(),
        message: "Auto composition mode used player-IR fallback because at least one graph node was not supported by native compositor lowering.".to_string(),
    });
    lowered
}

fn lower_native_composition_spec(request: &PlayerRenderBackendRequest) -> LoweredCompositionSpec {
    let mut spec = CompositionSpec {
        t: request.ir.phase_t,
        loop_t: request.ir.loop_t,
        ..CompositionSpec::default()
    };
    spec.preserve_unfilled = true;
    let mut content_stages = Vec::new();
    let mut style_stages = Vec::new();

    let mut diagnostics = Vec::new();
    let mut lowered_effect_ids = Vec::new();
    let mut unlowered_effect_ids = Vec::new();
    let mut lowered_node_count = 0_usize;
    let mut unlowered_node_count = 0_usize;

    for node_id in &request.recipe.graph.order {
        let Some(node) = request.recipe.graph.nodes.get(node_id) else {
            diagnostics.push(PlayerRenderBackendDiagnostic {
                code: "unsupportedByDescriptor".to_string(),
                path: format!("graph.order.{}", node_id.as_str()),
                message: "Graph order references a node that is not present in the recipe graph."
                    .to_string(),
            });
            unlowered_node_count += 1;
            continue;
        };

        if !node_active_for_backend_request(node, request) {
            diagnostics.push(PlayerRenderBackendDiagnostic {
                code: "nativeNodeSkippedByPhase".to_string(),
                path: format!("graph.nodes.{}.activePhases", node.id.as_str()),
                message: format!(
                    "Skipped `{}` because it is not active during the sampled {:?} phase.",
                    node.effect.as_str(),
                    request.ir.phase
                ),
            });
            continue;
        }

        match lower_node_into_spec(
            &request.recipe,
            node,
            request,
            &mut spec,
            &mut content_stages,
            &mut style_stages,
        ) {
            NodeLoweringOutcome::Lowered { warnings } => {
                lowered_node_count += 1;
                push_unique(&mut lowered_effect_ids, node.effect.as_str().to_string());
                diagnostics.push(PlayerRenderBackendDiagnostic {
                    code: "nativeCompositionSpecApplied".to_string(),
                    path: format!("graph.nodes.{}", node.id.as_str()),
                    message: format!(
                        "Lowered `{}` into compositor-native CompositionSpec content.",
                        node.effect.as_str()
                    ),
                });
                diagnostics.extend(warnings);
            }
            NodeLoweringOutcome::Unsupported { reason } => {
                unlowered_node_count += 1;
                push_unique(&mut unlowered_effect_ids, node.effect.as_str().to_string());
                diagnostics.push(PlayerRenderBackendDiagnostic {
                    code: "unsupportedNativeEffect".to_string(),
                    path: format!("graph.nodes.{}", node.id.as_str()),
                    message: reason,
                });
            }
        }
    }

    if request.recipe.graph.order.is_empty() {
        diagnostics.push(PlayerRenderBackendDiagnostic {
            code: "nativeSourceBaselineApplied".to_string(),
            path: "graph.order".to_string(),
            message: "Recipe has no graph nodes; native mode rendered the source scene with a neutral composition spec and no fallback.".to_string(),
        });
    }

    let composition_spec_non_empty = composition_spec_non_empty(&spec);
    let native_stage_non_empty = !content_stages.is_empty() || !style_stages.is_empty();
    let native_lowering_succeeded = unlowered_node_count == 0;
    LoweredCompositionSpec {
        evidence: PlayerRenderBackendCompositionEvidence {
            composition_mode: "native".to_string(),
            fallback_used: false,
            native_lowering_attempted: true,
            native_lowering_succeeded,
            composition_spec_non_empty: composition_spec_non_empty || native_stage_non_empty,
            lowered_node_count,
            unlowered_node_count,
            lowered_effect_ids,
            unlowered_effect_ids,
            composition_spec_summary: composition_spec_summary_with_content_stages(
                &spec,
                content_stages.len(),
                style_stages.len(),
            ),
            source_render_mode: "sourceOnly".to_string(),
            native_source_isolated: true,
        },
        spec,
        content_stages,
        style_stages,
        diagnostics,
    }
}

fn node_active_for_backend_request(node: &NodeSpec, request: &PlayerRenderBackendRequest) -> bool {
    node.active_phases.is_empty()
        || node
            .active_phases
            .iter()
            .any(|phase| phase == &request.ir.phase)
}

enum NodeLoweringOutcome {
    Lowered {
        warnings: Vec<PlayerRenderBackendDiagnostic>,
    },
    Unsupported {
        reason: String,
    },
}

fn lower_node_into_spec(
    recipe: &RecipeDocument,
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    spec: &mut CompositionSpec,
    content_stages: &mut Vec<NativeContentStage>,
    style_stages: &mut Vec<NativeStyleStage>,
) -> NodeLoweringOutcome {
    let warnings = ignored_policy_warnings(node);
    let effect = node.effect.as_str();
    match effect {
        "content.typewriter" => lower_content_typewriter(node, request, content_stages, warnings),
        "content.splitFlap" => lower_content_split_flap(node, request, content_stages, warnings),
        "content.odometer" => lower_content_odometer(node, request, content_stages, warnings),
        "content.cellMotion" => lower_content_cell_motion(node, request, content_stages, warnings),
        "content.marquee" => lower_content_marquee(node, request, content_stages, warnings),
        "content.morph" => lower_content_morph(node, request, content_stages, warnings),
        "content.scramble" => lower_content_scramble(node, request, content_stages, warnings),
        "content.wrapIndicator" => {
            lower_content_wrap_indicator(node, request, content_stages, warnings)
        }
        "content.redact" => lower_content_redact(node, request, content_stages, warnings),
        "content.mirror" => lower_content_mirror(node, request, content_stages, warnings),
        "content.numeric" => lower_content_numeric(node, request, content_stages, warnings),
        "content.dissolve" => lower_content_dissolve(node, request, content_stages, warnings),
        "content.glitchShift" => {
            lower_content_glitch_shift(node, request, content_stages, warnings)
        }
        "content.scrambleGlitchShift" => {
            lower_content_scramble_glitch_shift(node, request, content_stages, warnings)
        }
        "content.slideShift" => lower_content_slide_shift(node, request, content_stages, warnings),
        "filter.tint" => {
            spec.filters.push(FilterSpec::Tint {
                color: color_input(node, request, "color").unwrap_or(ColorConfig::White),
                strength: number_signal_input(node, request, "strength", 0.5),
                apply_to: apply_to_input(node, request, "applyTo"),
            });
            NodeLoweringOutcome::Lowered { warnings }
        }
        "filter.dim" => {
            spec.filters.push(FilterSpec::Dim {
                factor: number_signal_input(node, request, "factor", 0.5),
                apply_to: apply_to_input(node, request, "applyTo"),
            });
            NodeLoweringOutcome::Lowered { warnings }
        }
        "filter.invert" => {
            spec.filters.push(FilterSpec::Invert {
                apply_to: apply_to_input(node, request, "applyTo"),
            });
            NodeLoweringOutcome::Lowered { warnings }
        }
        "filter.greyscale" => {
            spec.filters.push(FilterSpec::Greyscale {
                strength: number_signal_input(node, request, "strength", 0.8),
                apply_to: apply_to_input(node, request, "applyTo"),
            });
            NodeLoweringOutcome::Lowered { warnings }
        }
        "filter.fadeToCanvas" => lower_fade_to_canvas(node, spec, request, warnings),
        "filter.vignette" => lower_vignette(node, spec, style_stages, request, warnings),
        "filter.crt" => lower_crt(node, spec, request, warnings),
        "filter.patternFill" => lower_pattern_fill(node, spec, request, warnings),
        "filter.kittScanner" => lower_kitt_scanner(node, spec, request, warnings),
        "filter.bracketEmphasis" => {
            lower_filter_bracket_emphasis(node, style_stages, request, warnings)
        }
        "filter.dotIndicator" => lower_filter_dot_indicator(node, spec, request, warnings),
        "filter.edgeGrow" => lower_filter_edge_grow(node, style_stages, request, warnings),
        "filter.hoverBar" => lower_filter_hover_bar(node, spec, request, warnings),
        "filter.matrixRain" => lower_filter_matrix_rain(node, style_stages, request, warnings),
        "filter.subPixelBar" => lower_filter_sub_pixel_bar(node, style_stages, request, warnings),
        "filter.underlineWipe" => {
            lower_filter_underline_wipe(node, style_stages, request, warnings)
        }
        "filter.pillButton" => {
            let progress = number_signal_input(node, request, "progress", 0.75);
            spec.filters.push(FilterSpec::Tint {
                color: color_input(node, request, "activeColor").unwrap_or(ColorConfig::Cyan),
                strength: progress,
                apply_to: ApplyTo::Both,
            });
            NodeLoweringOutcome::Lowered { warnings }
        }
        "mask.wipe" => lower_wipe_mask(node, content_stages, request, warnings),
        "mask.checkers" => {
            spec.masks.push(MaskSpec::Checkers {
                cell_size: integer_input(node, request, "cellSize", 2).max(1) as u16,
            });
            NodeLoweringOutcome::Lowered { warnings }
        }
        "mask.blinds" => lower_blinds_mask(node, content_stages, request, warnings),
        "mask.cellular" => lower_cellular_mask(node, content_stages, request, warnings),
        "mask.diamond" => lower_diamond_mask(node, content_stages, request, warnings),
        "mask.dissolve" => lower_dissolve_mask(node, content_stages, request, warnings),
        "mask.iris" => lower_iris_mask(node, content_stages, request, warnings),
        "mask.none" => lower_none_mask(node, spec, warnings),
        "mask.pathReveal" => lower_path_reveal_mask(node, content_stages, request, warnings),
        "mask.radial" => lower_radial_mask(node, content_stages, request, warnings),
        "mask.wipeCorner" => lower_wipe_corner_mask(node, content_stages, request, warnings),
        "mask.materialize" | "mask.materializeCorner" => {
            lower_materialize_mask(node, spec, request, warnings)
        }
        "mask.noiseDither" => lower_noise_dither_mask(node, spec, request, warnings),
        "sampler.sineWave" => {
            spec.push_sampler(SamplerSpec::SineWave {
                axis: axis_input(node, request, "axis"),
                amplitude: number_signal_input(node, request, "amplitude", 1.0),
                frequency: number_signal_input(node, request, "frequency", 1.0),
                speed: number_signal_input(node, request, "speed", 1.0),
                phase: number_signal_input(node, request, "phaseOffset", 0.0),
            });
            NodeLoweringOutcome::Lowered { warnings }
        }
        "sampler.ripple" => {
            spec.push_sampler(SamplerSpec::Ripple {
                amplitude: number_signal_input(node, request, "amplitude", 1.0),
                wavelength: number_signal_input(node, request, "wavelength", 4.0),
                speed: number_signal_input(node, request, "speed", 1.0),
                center: RippleCenter::Center,
            });
            NodeLoweringOutcome::Lowered { warnings }
        }
        "sampler.crt" => lower_crt_sampler(node, content_stages, request, warnings),
        "sampler.crtJitter" => lower_crt_jitter_sampler(node, content_stages, request, warnings),
        "sampler.faultLine" => {
            lower_fault_line_sampler(node, spec, content_stages, request, warnings)
        }
        "sampler.radialTwist" => lower_radial_twist_sampler(node, spec, request, warnings),
        "sampler.shredder" => lower_shredder_sampler(node, spec, request, warnings),
        "shader.linearGradient" => lower_linear_gradient(node, spec, request, warnings),
        "shader.revealWipe" => lower_reveal_wipe(node, spec, request, warnings),
        "shader.borderSweep" => lower_border_sweep(recipe, node, spec, request, warnings),
        "shader.highlighter" => lower_highlighter_shader(node, style_stages, request, warnings),
        "shader.focusField" => lower_focus_field_shader(node, style_stages, request, warnings),
        "shader.glistenBand" => lower_glisten_band_shader(node, style_stages, request, warnings),
        "shader.wayfindingNode" => {
            lower_wayfinding_node_shader(node, style_stages, request, warnings)
        }
        "shader.barberPole" => lower_barber_pole_shader(node, style_stages, request, warnings),
        "shader.diffusion" => lower_diffusion_shader(node, style_stages, request, warnings),
        "shader.radar" => lower_radar_shader(node, style_stages, request, warnings),
        "style.colorFade" => lower_style_color_fade(node, style_stages, request, warnings),
        "style.colorShift" => lower_style_color_shift(node, style_stages, request, warnings),
        "style.fadeIn" | "style.fadeOut" => lower_style_fade(node, spec, request, warnings),
        "style.italicWindow" => lower_style_italic_window(node, style_stages, request, warnings),
        "style.moduloColumns" => lower_style_modulo_columns(node, style_stages, request, warnings),
        "style.neonFlicker" => lower_style_neon_flicker(node, style_stages, request, warnings),
        other => NodeLoweringOutcome::Unsupported {
            reason: format!("Effect `{other}` is not yet supported by compositor-native lowering."),
        },
    }
}

fn lower_content_typewriter(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_typewriter_reason(node) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::Typewriter {
        speed: number_input(node, request, "speed", 1.0).max(0.0),
        speed_variance: number_input(node, request, "speedVariance", 0.0).clamp(0.0, 1.0),
        cursor_character: enum_input(node, request, "cursorCharacter")
            .unwrap_or("▌")
            .chars()
            .next()
            .unwrap_or('▌'),
        cursor_wake: match enum_input(node, request, "cursorWake").unwrap_or("off") {
            "ghost" => TypewriterCursorWake::Ghost,
            "tint" => TypewriterCursorWake::Tint,
            _ => TypewriterCursorWake::Off,
        },
        wake_cells: integer_input(node, request, "wakeCells", 1).max(0) as usize,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn unsupported_typewriter_reason(node: &NodeSpec) -> Option<String> {
    unsupported_native_content_reason(
        node,
        "content.typewriter",
        &[
            "speed",
            "speedVariance",
            "cursorCharacter",
            "cursorWake",
            "wakeCells",
        ],
    )
}

fn lower_content_split_flap(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_content_reason(
        node,
        "content.splitFlap",
        &[
            "settle",
            "cascade",
            "speed",
            "cycles",
            "charset",
            "tileWidth",
            "tileHeight",
            "jitter",
        ],
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::SplitFlap {
        settle: number_input(node, request, "settle", 1.0).clamp(0.0, 1.0),
        cascade: number_input(node, request, "cascade", 0.0).clamp(0.0, 1.0),
        speed: number_input(node, request, "speed", 1.0).max(0.0),
        cycles: number_input(node, request, "cycles", 1.0).max(0.0),
        charset: enum_input(node, request, "charset")
            .unwrap_or("blocks")
            .to_string(),
        tile_width: native_content_tile_size(node, request, "tileWidth"),
        tile_height: native_content_tile_size(node, request, "tileHeight"),
        jitter: number_input(node, request, "jitter", 0.0).clamp(0.0, 1.0),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_content_odometer(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_content_reason(
        node,
        "content.odometer",
        &[
            "direction",
            "travel",
            "fromMessage",
            "tileWidth",
            "tileHeight",
        ],
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::Odometer {
        direction: enum_input(node, request, "direction")
            .unwrap_or("up")
            .to_string(),
        travel: enum_input(node, request, "travel")
            .unwrap_or("axis")
            .to_string(),
        from_message: enum_input(node, request, "fromMessage")
            .unwrap_or("")
            .to_string(),
        tile_width: native_content_tile_size(node, request, "tileWidth"),
        tile_height: native_content_tile_size(node, request, "tileHeight"),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_content_cell_motion(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_content_reason(
        node,
        "content.cellMotion",
        &["route", "stagger", "affect"],
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::CellMotion {
        route: enum_input(node, request, "route")
            .unwrap_or("fromTop")
            .to_string(),
        stagger: integer_input(node, request, "stagger", 0).max(0) as usize,
        affect: enum_input(node, request, "affect")
            .unwrap_or("all")
            .to_string(),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_content_marquee(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_content_reason(node, "content.marquee", &["direction", "speed", "width"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::Marquee {
        direction: enum_input(node, request, "direction")
            .unwrap_or("left")
            .to_string(),
        speed: number_input(node, request, "speed", 1.0).max(0.0),
        width: integer_input(node, request, "width", 0).max(0) as usize,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_content_morph(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_content_reason(node, "content.morph", &["target"]) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::Morph {
        target: enum_input(node, request, "target")
            .unwrap_or("blocks")
            .to_string(),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_content_scramble(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_content_reason(node, "content.scramble", &["seed", "charset"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::Scramble {
        seed: integer_input(node, request, "seed", 7).max(0) as usize,
        charset: enum_input(node, request, "charset")
            .unwrap_or("#%&?+*")
            .to_string(),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_content_wrap_indicator(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_content_reason(node, "content.wrapIndicator", &["every"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::WrapIndicator {
        every: integer_input(node, request, "every", 1).max(1) as usize,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_content_redact(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_content_reason(node, "content.redact", &["symbol", "reveal"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::Redact {
        symbol: enum_input(node, request, "symbol")
            .unwrap_or("█")
            .chars()
            .next()
            .unwrap_or('█'),
        reveal: number_input(node, request, "reveal", request.ir.phase_t).clamp(0.0, 1.0),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_content_mirror(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_content_reason(node, "content.mirror", &["axis"]) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::Mirror {
        axis: enum_input(node, request, "axis")
            .unwrap_or("horizontal")
            .to_string(),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_content_numeric(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_content_reason(
        node,
        "content.numeric",
        &["value", "decimals", "prefix", "suffix"],
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::Numeric {
        value: number_input(node, request, "value", request.ir.phase_t),
        decimals: integer_input(node, request, "decimals", 0).clamp(0, 9) as usize,
        prefix: enum_input(node, request, "prefix")
            .unwrap_or("")
            .to_string(),
        suffix: enum_input(node, request, "suffix")
            .unwrap_or("")
            .to_string(),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_content_dissolve(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_content_reason(
        node,
        "content.dissolve",
        &["replacement", "direction", "seed"],
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    let replacement = match enum_input(node, request, "replacement").unwrap_or("space") {
        "dot" => '·',
        "block" => '█',
        value => value.chars().next().unwrap_or(' '),
    };
    content_stages.push(NativeContentStage::Dissolve {
        replacement,
        direction: enum_input(node, request, "direction")
            .unwrap_or("random")
            .to_string(),
        seed: integer_input(node, request, "seed", 0).max(0) as usize,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_content_glitch_shift(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_content_reason(node, "content.glitchShift", &["amount", "seed"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::GlitchShift {
        amount: integer_input(node, request, "amount", 1).max(0) as usize,
        seed: integer_input(node, request, "seed", 3).max(0) as usize,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_content_scramble_glitch_shift(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_content_reason(
        node,
        "content.scrambleGlitchShift",
        &["seed", "charset", "amount"],
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    let seed = integer_input(node, request, "seed", 7).max(0) as usize;
    content_stages.push(NativeContentStage::Scramble {
        seed,
        charset: enum_input(node, request, "charset")
            .unwrap_or("#%&?+*")
            .to_string(),
    });
    content_stages.push(NativeContentStage::GlitchShift {
        amount: integer_input(node, request, "amount", 1).max(0) as usize,
        seed,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_content_slide_shift(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    content_stages: &mut Vec<NativeContentStage>,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_content_reason(node, "content.slideShift", &["startCol", "endCol"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::SlideShift {
        start_col: integer_input(node, request, "startCol", -4),
        end_col: integer_input(node, request, "endCol", 0),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn native_content_tile_size(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    input_id: &str,
) -> usize {
    integer_input(node, request, input_id, 1).max(1) as usize
}

fn unsupported_native_content_reason(
    node: &NodeSpec,
    effect_id: &str,
    supported_inputs: &[&str],
) -> Option<String> {
    unsupported_native_reason(
        node,
        effect_id,
        supported_inputs,
        "compositor-backend native content-stage",
    )
}

fn unsupported_native_effect_reason(
    node: &NodeSpec,
    effect_id: &str,
    supported_inputs: &[&str],
) -> Option<String> {
    unsupported_native_reason(
        node,
        effect_id,
        supported_inputs,
        "compositor-native effect",
    )
}

fn unsupported_native_reason(
    node: &NodeSpec,
    effect_id: &str,
    supported_inputs: &[&str],
    native_surface: &str,
) -> Option<String> {
    let unsupported_inputs = node
        .inputs
        .keys()
        .map(|key| key.as_str())
        .filter(|key| !supported_inputs.contains(key))
        .collect::<Vec<_>>();
    if !unsupported_inputs.is_empty() {
        return Some(format!(
            "Effect `{effect_id}` uses input(s) `{}` that have no {native_surface} equivalent without dropping authored semantics.",
            unsupported_inputs.join("`, `"),
        ));
    }

    if !node.outputs.is_empty() {
        return Some(format!(
            "Effect `{effect_id}` declares graph outputs that the {native_surface} cannot publish without dropping authored semantics."
        ));
    }

    if let Some(scope) = &node.scope
        && !matches!(scope, tui_vfx_contract::ScopeSpec::All)
    {
        return Some(format!(
            "Effect `{effect_id}` uses a non-all scope that is not yet supported by the {native_surface} without dropping authored semantics."
        ));
    }

    None
}

fn lower_fade_to_canvas(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    let canvas_color = color_input(node, request, "canvasColor").unwrap_or(ColorConfig::Black);
    let canvas_color_binding = signal_source_id(node, "canvasColor").map(|signal_id| {
        if let Some(color) = request
            .sample
            .signals
            .get(&signal_id)
            .and_then(|value| match value {
                Value::Color(color) => Some(*color),
                _ => None,
            })
        {
            spec.runtime_params
                .insert(signal_id.as_str().to_string(), color);
        }
        signal_id.as_str().to_string()
    });
    let strength_key = if node_has_input(node, "strength") {
        "strength"
    } else {
        "amount"
    };
    spec.filters.push(FilterSpec::FadeToCanvas {
        canvas_color,
        canvas_color_binding,
        strength: BindableValue::from(number_signal_input(node, request, strength_key, 0.5)),
        apply_to: apply_to_input(node, request, "applyTo"),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_vignette(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    let source_style_inputs = ["edgeColor", "applyTo"];
    let direct_filter_only_inputs = ["radius", "sides", "ditherAmount", "temporalDitherHz"];
    let supported_inputs = [
        "strength",
        "radius",
        "sides",
        "ditherAmount",
        "temporalDitherHz",
        "edgeColor",
        "applyTo",
    ];
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "filter.vignette",
        &supported_inputs,
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    let has_source_style_input = source_style_inputs
        .iter()
        .any(|key| node_has_input(node, key));
    if has_source_style_input {
        if direct_filter_only_inputs
            .iter()
            .any(|key| node_has_input(node, key))
        {
            return NodeLoweringOutcome::Unsupported {
                reason: "Effect `filter.vignette` cannot mix source-style-only fields with compositor FilterSpec-only fields without dropping authored semantics.".to_string(),
            };
        }
        let edge_color = color_input(node, request, "edgeColor").unwrap_or(ColorConfig::Rgb {
            r: 10,
            g: 20,
            b: 36,
        });
        let apply_to = match strict_enum_input(
            node,
            request,
            "applyTo",
            "both",
            &["foreground", "background", "both"],
            "filter.vignette",
        ) {
            Ok(apply_to) => apply_to,
            Err(reason) => return NodeLoweringOutcome::Unsupported { reason },
        };
        style_stages.push(NativeStyleStage::Vignette {
            strength: number_input(node, request, "strength", 0.6).clamp(0.0, 1.0),
            edge_color: color_label_from_config(edge_color),
            apply_to,
        });
        return NodeLoweringOutcome::Lowered { warnings };
    }

    spec.filters.push(FilterSpec::Vignette {
        strength: number_signal_input(node, request, "strength", 0.6),
        radius: number_signal_input(node, request, "radius", 0.75),
        sides: vignette_sides_input(node, request, "sides"),
        dither_amount: number_input(node, request, "ditherAmount", 0.0) as f32,
        temporal_dither_hz: number_input(node, request, "temporalDitherHz", 0.0) as f32,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_crt(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    let intensity = number_signal_input(node, request, "intensity", 1.0);
    let scanline_strength = if node_has_input(node, "scanlineStrength") {
        multiply_signal_or_float(
            number_signal_input(node, request, "scanlineStrength", 0.5),
            intensity.clone(),
        )
    } else {
        intensity.clone()
    };
    spec.filters.push(FilterSpec::Crt {
        scanline_strength,
        glow: multiply_signal_or_float(number_signal_input(node, request, "glow", 0.5), intensity),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_pattern_fill(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_effect_reason(node, "filter.patternFill", &["pattern", "density"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    spec.filters.push(FilterSpec::PatternFill {
        pattern: pattern_type_input(node, request, "pattern"),
        color: None,
        only_empty: false,
        density: number_input(node, request, "density", 0.5).clamp(0.0, 1.0) as f32,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_kitt_scanner(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_effect_reason(
        node,
        "filter.kittScanner",
        &[
            "scanColor",
            "trailColor",
            "speed",
            "width",
            "applyTo",
            "progress",
            "bandWidth",
            "bpm",
            "boost",
            "boostSeparatorBg",
            "axis",
            "powerlineMode",
            "motionMode",
        ],
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    let (band_width, band_width_cells) = kitt_band_width_input(node, request);
    let boost_separator_bg_color = color_input(node, request, "boostSeparatorBg");

    spec.filters.push(FilterSpec::KittScanner {
        boost: normalized_boost_input(node, request, "boost", 50),
        band_width,
        scan_color: color_input(node, request, "scanColor"),
        trail_color: color_input(node, request, "trailColor"),
        band_width_cells,
        bpm: optional_number_input(node, request, "bpm").map(|value| value.max(0.0) as f32),
        bps: number_input(node, request, "speed", 1.2).max(0.1) as f32,
        progress: BindableValue::from(number_signal_input(node, request, "progress", 1.0)),
        motion_mode: scanner_motion_mode_input(node, request, "motionMode"),
        axis: scanner_axis_input(node, request, "axis"),
        apply_to: apply_to_input(node, request, "applyTo"),
        powerline_mode: bool_input(node, request, "powerlineMode", false),
        boost_separator_bg: bool_or_color_presence_input(node, request, "boostSeparatorBg", false),
        boost_separator_bg_color,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_blinds_mask(
    node: &NodeSpec,
    content_stages: &mut Vec<NativeContentStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_content_reason(node, "mask.blinds", &["orientation", "count"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    let orientation = match strict_enum_input(
        node,
        request,
        "orientation",
        "horizontal",
        &["horizontal", "vertical"],
        "mask.blinds",
    ) {
        Ok(orientation) => orientation,
        Err(reason) => return NodeLoweringOutcome::Unsupported { reason },
    };
    content_stages.push(NativeContentStage::BlindsMask {
        orientation,
        count: integer_input(node, request, "count", 4).max(1) as usize,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_cellular_mask(
    node: &NodeSpec,
    content_stages: &mut Vec<NativeContentStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_content_reason(node, "mask.cellular", &["cellSize", "seed", "threshold"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    content_stages.push(NativeContentStage::CellularMask {
        cell_size: integer_input(node, request, "cellSize", 2).max(1) as usize,
        seed: integer_input(node, request, "seed", 7).max(0) as usize,
        threshold: number_input(node, request, "threshold", 0.5).clamp(0.0, 1.0),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_diamond_mask(
    node: &NodeSpec,
    content_stages: &mut Vec<NativeContentStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_content_reason(node, "mask.diamond", &["softEdge"]) {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    content_stages.push(NativeContentStage::DiamondMask {
        soft_edge: bool_input(node, request, "softEdge", true),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_dissolve_mask(
    node: &NodeSpec,
    content_stages: &mut Vec<NativeContentStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_content_reason(node, "mask.dissolve", &["seed", "chunkSize"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    content_stages.push(NativeContentStage::DissolveMask {
        seed: integer_input(node, request, "seed", 42).max(0) as u64,
        chunk_size: integer_input(node, request, "chunkSize", 1).max(1) as usize,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_iris_mask(
    node: &NodeSpec,
    content_stages: &mut Vec<NativeContentStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_content_reason(node, "mask.iris", &["shape", "softEdge"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    let shape = match strict_enum_input(
        node,
        request,
        "shape",
        "circle",
        &["circle", "diamond"],
        "mask.iris",
    ) {
        Ok(shape) => shape,
        Err(reason) => return NodeLoweringOutcome::Unsupported { reason },
    };
    content_stages.push(NativeContentStage::IrisMask {
        shape,
        soft_edge: bool_input(node, request, "softEdge", true),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_none_mask(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_reason(node, "mask.none", &[], "compositor-backend native mask")
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    spec.masks.push(MaskSpec::None);
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_path_reveal_mask(
    node: &NodeSpec,
    content_stages: &mut Vec<NativeContentStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_content_reason(node, "mask.pathReveal", &["direction", "softEdge"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    let direction = match strict_enum_input(
        node,
        request,
        "direction",
        "leftToRight",
        SUPPORTED_WIPE_DIRECTIONS,
        "mask.pathReveal",
    ) {
        Ok(direction) => direction,
        Err(reason) => return NodeLoweringOutcome::Unsupported { reason },
    };
    content_stages.push(NativeContentStage::WipeMask {
        direction,
        soft_edge: bool_input(node, request, "softEdge", false),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_wipe_mask(
    node: &NodeSpec,
    content_stages: &mut Vec<NativeContentStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_content_reason(node, "mask.wipe", &["direction", "softEdge"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    let direction = match strict_enum_input(
        node,
        request,
        "direction",
        "leftToRight",
        SUPPORTED_WIPE_DIRECTIONS,
        "mask.wipe",
    ) {
        Ok(direction) => direction,
        Err(reason) => return NodeLoweringOutcome::Unsupported { reason },
    };
    content_stages.push(NativeContentStage::WipeMask {
        direction,
        soft_edge: bool_input(node, request, "softEdge", false),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_radial_mask(
    node: &NodeSpec,
    content_stages: &mut Vec<NativeContentStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_content_reason(node, "mask.radial", &["origin", "softEdge"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    if let Err(reason) = strict_enum_input(
        node,
        request,
        "origin",
        "center",
        &["center"],
        "mask.radial",
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    content_stages.push(NativeContentStage::RadialMask {
        soft_edge: bool_input(node, request, "softEdge", true),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_wipe_corner_mask(
    node: &NodeSpec,
    content_stages: &mut Vec<NativeContentStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_content_reason(node, "mask.wipeCorner", &["direction", "softEdge"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    let direction = match strict_enum_input(
        node,
        request,
        "direction",
        "leftToRight",
        SUPPORTED_WIPE_DIRECTIONS,
        "mask.wipeCorner",
    ) {
        Ok(direction) => direction,
        Err(reason) => return NodeLoweringOutcome::Unsupported { reason },
    };
    content_stages.push(NativeContentStage::WipeMask {
        direction,
        soft_edge: bool_input(node, request, "softEdge", false),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_materialize_mask(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_effect_reason(
        node,
        node.effect.as_str(),
        &["origin", "softEdge", "chunkSize", "noise", "seed"],
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    spec.masks.push(MaskSpec::Materialize {
        origin: radial_origin_input(node, request, "origin"),
        seed: integer_input(node, request, "seed", 0).max(0) as u64,
        chunk_size: positive_u8_input(node, request, "chunkSize", 1),
        noise: number_input(node, request, "noise", 0.18).clamp(0.0, 1.0) as f32,
        soft_edge: bool_input(node, request, "softEdge", true),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_noise_dither_mask(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_effect_reason(node, "mask.noiseDither", &["chunkSize", "seed"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    spec.masks.push(MaskSpec::NoiseDither {
        seed: integer_input(node, request, "seed", 0).max(0) as u64,
        matrix: DitherMatrix::Bayer4,
        chunk_size: positive_u8_input(node, request, "chunkSize", 1),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_crt_sampler(
    node: &NodeSpec,
    content_stages: &mut Vec<NativeContentStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_content_reason(
        node,
        "sampler.crt",
        &["curvature", "scanlineStrength", "jitter"],
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::CrtSampler {
        curvature: number_input(node, request, "curvature", 0.4).clamp(0.0, 1.0),
        scanline_strength: number_input(node, request, "scanlineStrength", 0.35).clamp(0.0, 1.0),
        jitter: number_input(node, request, "jitter", 0.0).max(0.0),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_crt_jitter_sampler(
    node: &NodeSpec,
    content_stages: &mut Vec<NativeContentStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_content_reason(
        node,
        "sampler.crtJitter",
        &["amplitude", "frequency", "seed"],
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    content_stages.push(NativeContentStage::CrtJitterSampler {
        amplitude: number_input(node, request, "amplitude", 1.0).max(0.0),
        frequency: number_input(node, request, "frequency", 2.0).max(0.0),
        seed: integer_input(node, request, "seed", 13).max(0) as usize,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_fault_line_sampler(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    content_stages: &mut Vec<NativeContentStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_effect_reason(
        node,
        "sampler.faultLine",
        &["offset", "seed", "intensity", "splitBias"],
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    if let Some(offset) = optional_i16_input(node, request, "offset") {
        spec.push_sampler(SamplerSpec::FaultLine {
            seed: 0,
            intensity: SignalOrFloat::Static(1.0),
            split_bias: 0.0,
            offset: Some(offset),
        });
    } else {
        spec.push_sampler(SamplerSpec::None);
        content_stages.push(NativeContentStage::FaultLineSampler {
            seed: integer_input(node, request, "seed", 0).max(0) as u64,
            intensity: number_input(node, request, "intensity", 1.0).max(0.0),
            split_bias: number_input(node, request, "splitBias", 0.0).clamp(-1.0, 1.0),
            width: request
                .recipe
                .scenes
                .first()
                .map(|scene| scene.width)
                .unwrap_or(request.source_ir.width),
            height: request
                .recipe
                .scenes
                .first()
                .map(|scene| scene.height)
                .unwrap_or(request.source_ir.height),
        });
    }
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_radial_twist_sampler(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_effect_reason(node, "sampler.radialTwist", &["strength"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    spec.push_sampler(SamplerSpec::RadialTwist {
        twist: number_signal_input(node, request, "strength", 1.0),
        center: RippleCenter::Center,
        radius_floor: SignalOrFloat::Static(0.1),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_shredder_sampler(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_native_effect_reason(
        node,
        "sampler.shredder",
        &[
            "sliceWidth",
            "offset",
            "stripeWidth",
            "oddSpeed",
            "evenSpeed",
        ],
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    let default_stripe_width = if node_has_input(node, "sliceWidth") {
        integer_input(node, request, "sliceWidth", 2)
    } else {
        4
    };
    spec.push_sampler(SamplerSpec::Shredder {
        stripe_width: positive_u16_input(node, request, "stripeWidth", default_stripe_width),
        odd_speed: number_signal_input(node, request, "oddSpeed", 3.0),
        even_speed: number_signal_input(node, request, "evenSpeed", -2.0),
        offset: optional_i16_input(node, request, "offset"),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_linear_gradient(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    let gradient = gradient_input(node, request, "gradient").unwrap_or_default();
    let shader = LinearGradientShader {
        gradient,
        angle_deg: number_input(node, request, "angleDeg", 0.0) as f32,
        apply_to: gradient_apply_to_input(node, request, "applyTo"),
        intensity: number_input(node, request, "intensity", 1.0) as f32,
    };
    spec.shader_layers.push(ShaderLayerSpec {
        shader: SpatialShaderType::LinearGradient(shader),
        region: StyleRegion::All,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_reveal_wipe(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) =
        unsupported_native_effect_reason(node, "shader.revealWipe", &["color", "direction"])
    {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    spec.shader_layers.push(ShaderLayerSpec {
        shader: SpatialShaderType::RevealWipe(RevealWipeShader {
            direction: wipe_direction_input(node, request, "direction"),
            color: color_input(node, request, "color"),
        }),
        region: StyleRegion::All,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_border_sweep(
    recipe: &RecipeDocument,
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    request: &PlayerRenderBackendRequest,
    mut warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    let position_binding = signal_source_id(node, "position");
    if let Some(signal_id) = &position_binding
        && let Some(value) = request.sample.signals.get(signal_id)
        && let Some(number) = value.as_range_number()
    {
        spec.runtime_params
            .insert(signal_id.as_str().to_string(), number);
    }
    if position_binding.is_none()
        && recipe
            .graph
            .signals
            .contains_key(&tui_vfx_contract::SignalId::new("sweepPosition"))
    {
        warnings.push(PlayerRenderBackendDiagnostic {
            code: "fieldIgnoredWithWarning".to_string(),
            path: format!("graph.nodes.{}.inputs.position", node.id.as_str()),
            message: "Border sweep has no signal-backed position input; native lowering will use time-driven sweep position.".to_string(),
        });
    }

    spec.shader_layers.push(ShaderLayerSpec {
        shader: SpatialShaderType::BorderSweep(BorderSweepShader {
            speed: number_input(node, request, "speed", 1.0) as f32,
            length: integer_input(node, request, "length", 5).max(1) as u16,
            color: color_input(node, request, "color").unwrap_or(ColorConfig::Cyan),
            head: None,
            tail: None,
            position_binding: position_binding.map(|id| id.as_str().to_string()),
        }),
        region: StyleRegion::All,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_style_fade(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    let progress = eased_style_progress(node, request);
    let canvas_fade = match node.effect.as_str() {
        "style.fadeIn" if node_has_input(node, "from") && !node_has_input(node, "to") => {
            Some(("from", 1.0 - progress))
        }
        "style.fadeOut" if node_has_input(node, "to") && !node_has_input(node, "from") => {
            Some(("to", progress))
        }
        _ => None,
    };
    if let Some((canvas_input, strength)) = canvas_fade {
        spec.filters.push(FilterSpec::FadeToCanvas {
            canvas_color: color_input(node, request, canvas_input).unwrap_or(ColorConfig::Black),
            canvas_color_binding: None,
            strength: BindableValue::from(SignalOrFloat::Static(strength)),
            apply_to: apply_to_input(node, request, "applyTo"),
        });
        return NodeLoweringOutcome::Lowered { warnings };
    }

    let from = color_input(node, request, "from").unwrap_or(ColorConfig::Black);
    let to = color_input(node, request, "to").unwrap_or(ColorConfig::White);
    let style_color = blend_color_config(from, to, progress as f64);
    spec.filters.push(FilterSpec::Tint {
        color: style_color,
        strength: SignalOrFloat::Static(1.0),
        apply_to: apply_to_input(node, request, "applyTo"),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn eased_style_progress(node: &NodeSpec, request: &PlayerRenderBackendRequest) -> f32 {
    let phase = request.ir.phase_t.clamp(0.0, 1.0) as f32;
    let easing = enum_input(node, request, "easing").unwrap_or("linear");
    match enum_input(node, request, "ease").unwrap_or(easing) {
        "easeIn" => phase * phase,
        "easeOut" => 1.0 - (1.0 - phase) * (1.0 - phase),
        "easeInOut" => {
            if phase < 0.5 {
                2.0 * phase * phase
            } else {
                1.0 - (-2.0 * phase + 2.0).powi(2) / 2.0
            }
        }
        _ => phase,
    }
}

fn lower_style_modulo_columns(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    let supported_inputs = ["foreground", "background"];
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "style.moduloColumns",
        &supported_inputs,
        StyleScopeRequirement::ModuloColumns,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    let Some((modulus, remainder)) = modulo_columns_scope(node) else {
        return NodeLoweringOutcome::Unsupported {
            reason: "Effect `style.moduloColumns` requires a modulo-columns scope for compositor-native style-stage lowering.".to_string(),
        };
    };
    style_stages.push(NativeStyleStage::ModuloColumns {
        modulus,
        remainder,
        foreground: color_label_from_config(
            color_input(node, request, "foreground").unwrap_or(ColorConfig::Cyan),
        ),
        background: color_label_from_config(color_input(node, request, "background").unwrap_or(
            ColorConfig::Rgb {
                r: 15,
                g: 40,
                b: 55,
            },
        )),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_style_neon_flicker(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "style.neonFlicker",
        &["color", "stability", "dimAmount", "italicWindow"],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    style_stages.push(NativeStyleStage::NeonFlicker {
        color: color_label_from_config(color_input(node, request, "color").unwrap_or(
            ColorConfig::Rgb {
                r: 80,
                g: 255,
                b: 220,
            },
        )),
        stability: number_input(node, request, "stability", 0.7).clamp(0.0, 1.0),
        dim_amount: number_input(node, request, "dimAmount", 0.5).clamp(0.0, 1.0),
        italic_window: bool_input(node, request, "italicWindow", false),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_style_color_fade(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "style.colorFade",
        &["target", "colorSpace"],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    style_stages.push(NativeStyleStage::ColorFade {
        target: color_label_input(node, request, "target", (255, 200, 50)),
        color_space: enum_label_input(node, request, "colorSpace", "rgb"),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_style_color_shift(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "style.colorShift",
        &["hueShift", "saturationShift", "lightnessShift"],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    style_stages.push(NativeStyleStage::ColorShift {
        hue_shift: number_input(node, request, "hueShift", 0.0),
        saturation_shift: number_input(node, request, "saturationShift", 0.0),
        lightness_shift: number_input(node, request, "lightnessShift", 0.0),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_style_italic_window(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "style.italicWindow",
        &["start", "end"],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    let start = number_input(node, request, "start", 0.0).clamp(0.0, 1.0);
    let end = number_input(node, request, "end", 1.0).clamp(start, 1.0);
    style_stages.push(NativeStyleStage::ItalicWindow { start, end });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_highlighter_shader(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "shader.highlighter",
        &[
            "color",
            "bandWidth",
            "blendStrength",
            "textContrast",
            "mode",
            "softEdge",
            "direction",
            "rowMask",
        ],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    let direction = match strict_enum_input(
        node,
        request,
        "direction",
        "leftToRight",
        &["leftToRight", "rightToLeft", "topToBottom", "bottomToTop"],
        "shader.highlighter",
    ) {
        Ok(direction) => direction,
        Err(reason) => return NodeLoweringOutcome::Unsupported { reason },
    };
    let mode = match strict_enum_input(
        node,
        request,
        "mode",
        "band",
        &["band", "row", "centerOut"],
        "shader.highlighter",
    ) {
        Ok(mode) => mode,
        Err(reason) => return NodeLoweringOutcome::Unsupported { reason },
    };
    let soft_edge = match strict_enum_input(
        node,
        request,
        "softEdge",
        "true",
        &["true", "false"],
        "shader.highlighter",
    ) {
        Ok(soft_edge) => soft_edge != "false",
        Err(reason) => return NodeLoweringOutcome::Unsupported { reason },
    };

    let band_width = (resolved_number_input(node, request, "bandWidth", 3.0).max(1.0)
        * (1.0 + if soft_edge { 0.5 } else { 0.0 })) as usize;
    style_stages.push(NativeStyleStage::Highlighter {
        color: color_label_from_config(resolved_color_input(node, request, "color").unwrap_or(
            ColorConfig::Rgb {
                r: 255,
                g: 225,
                b: 90,
            },
        )),
        blend_strength: resolved_number_input(node, request, "blendStrength", 1.0).clamp(0.0, 1.0),
        text_contrast: resolved_number_input(node, request, "textContrast", 0.0).clamp(0.0, 1.0),
        soft_edge,
        direction,
        mode,
        row_mask: resolved_integer_input(node, request, "rowMask", -1),
        band_width,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_focus_field_shader(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "shader.focusField",
        &[
            "color",
            "centerX",
            "centerY",
            "radius",
            "intensity",
            "radiusX",
            "radiusY",
            "shape",
            "feather",
            "rectHeight",
            "rectWidth",
            "rectX",
            "rectY",
        ],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    let shape = match strict_enum_input(
        node,
        request,
        "shape",
        "circle",
        &["circle", "ellipse", "rect"],
        "shader.focusField",
    ) {
        Ok(shape) => shape,
        Err(reason) => return NodeLoweringOutcome::Unsupported { reason },
    };
    let rect_x = resolved_number_input(node, request, "rectX", 0.0);
    let rect_y = resolved_number_input(node, request, "rectY", 0.0);
    let rect_width =
        resolved_number_input(node, request, "rectWidth", request.source_ir.width as f64);
    let rect_height =
        resolved_number_input(node, request, "rectHeight", request.source_ir.height as f64);
    let center_x = resolved_number_input(node, request, "centerX", rect_x + rect_width / 2.0);
    let center_y = resolved_number_input(node, request, "centerY", rect_y + rect_height / 2.0);
    let radius_x = resolved_number_input(
        node,
        request,
        "radiusX",
        resolved_number_input(node, request, "radius", 4.0),
    )
    .max(0.5);
    let radius_y = resolved_number_input(node, request, "radiusY", radius_x).max(0.5);
    style_stages.push(NativeStyleStage::FocusField {
        color: color_label_from_config(resolved_color_input(node, request, "color").unwrap_or(
            ColorConfig::Rgb {
                r: 120,
                g: 180,
                b: 255,
            },
        )),
        rect_x,
        rect_y,
        rect_width,
        rect_height,
        center_x,
        center_y,
        shape,
        radius_x,
        radius_y,
        feather: resolved_number_input(node, request, "feather", 0.0).clamp(0.0, 1.0),
        intensity: resolved_number_input(node, request, "intensity", 1.0).clamp(0.0, 1.0),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_glisten_band_shader(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "shader.glistenBand",
        &[
            "color",
            "bandWidth",
            "direction",
            "blendStrength",
            "angleDeg",
            "head",
            "speed",
            "tail",
        ],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    let direction = match strict_enum_input(
        node,
        request,
        "direction",
        "leftToRight",
        &["leftToRight", "rightToLeft"],
        "shader.glistenBand",
    ) {
        Ok(direction) => direction,
        Err(reason) => return NodeLoweringOutcome::Unsupported { reason },
    };
    style_stages.push(NativeStyleStage::GlistenBand {
        color: color_label_from_config(
            resolved_color_input(node, request, "color").unwrap_or(ColorConfig::White),
        ),
        blend_strength: resolved_number_input(node, request, "blendStrength", 1.0).clamp(0.0, 1.0),
        angle_deg: resolved_number_input(node, request, "angleDeg", 0.0),
        speed: resolved_number_input(node, request, "speed", 1.0).max(0.0),
        head: resolved_number_input(node, request, "head", 0.0).clamp(0.0, 1.0),
        tail: resolved_number_input(node, request, "tail", 1.0).clamp(0.0, 1.0),
        band_width: resolved_number_input(node, request, "bandWidth", 2.0).max(1.0),
        direction,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_wayfinding_node_shader(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "shader.wayfindingNode",
        &[
            "currentIndex",
            "activeColor",
            "color",
            "futureStrength",
            "intensity",
            "nodes",
            "previousStrength",
            "radius",
        ],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    style_stages.push(NativeStyleStage::WayfindingNode {
        current_index: resolved_integer_input(node, request, "currentIndex", 0).max(0) as usize,
        nodes: resolved_integer_input(node, request, "nodes", 1).max(1) as usize,
        previous_strength: resolved_number_input(node, request, "previousStrength", 0.3)
            .clamp(0.0, 1.0),
        future_strength: resolved_number_input(node, request, "futureStrength", 0.4)
            .clamp(0.0, 1.0),
        intensity: resolved_number_input(node, request, "intensity", 1.0).clamp(0.0, 1.0),
        radius: resolved_number_input(node, request, "radius", 1.0).max(0.0) as usize,
        active_color: color_label_from_config(
            resolved_color_input(node, request, "activeColor")
                .or_else(|| resolved_color_input(node, request, "color"))
                .unwrap_or(ColorConfig::Rgb {
                    r: 80,
                    g: 255,
                    b: 160,
                }),
        ),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_barber_pole_shader(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "shader.barberPole",
        &[
            "angleDeg",
            "applyTo",
            "backgroundColor",
            "color",
            "gapWidth",
            "speed",
            "stripeColor",
            "stripeWidth",
        ],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    let apply_to = match strict_enum_input(
        node,
        request,
        "applyTo",
        "background",
        &["foreground", "background", "both"],
        "shader.barberPole",
    ) {
        Ok(apply_to) => apply_to,
        Err(reason) => return NodeLoweringOutcome::Unsupported { reason },
    };
    let base_color = resolved_color_input(node, request, "color").unwrap_or(ColorConfig::Rgb {
        r: 255,
        g: 80,
        b: 80,
    });
    style_stages.push(NativeStyleStage::BarberPole {
        stripe_color: color_label_from_config(
            resolved_color_input(node, request, "stripeColor").unwrap_or(base_color),
        ),
        background_color: color_label_from_config(
            resolved_color_input(node, request, "backgroundColor").unwrap_or(ColorConfig::Rgb {
                r: 40,
                g: 10,
                b: 20,
            }),
        ),
        stripe_width: resolved_integer_input(node, request, "stripeWidth", 3).max(1) as usize,
        gap_width: resolved_integer_input(
            node,
            request,
            "gapWidth",
            resolved_integer_input(node, request, "stripeWidth", 3),
        )
        .max(1) as usize,
        angle_deg: resolved_number_input(node, request, "angleDeg", 45.0),
        speed: resolved_number_input(node, request, "speed", 0.0),
        apply_to,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_diffusion_shader(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "shader.diffusion",
        &[
            "applyTo",
            "centerX",
            "centerY",
            "color",
            "intensity",
            "radius",
            "source",
        ],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    let apply_to = match strict_enum_input(
        node,
        request,
        "applyTo",
        "background",
        &["foreground", "background", "both"],
        "shader.diffusion",
    ) {
        Ok(apply_to) => apply_to,
        Err(reason) => return NodeLoweringOutcome::Unsupported { reason },
    };
    style_stages.push(NativeStyleStage::Diffusion {
        color: color_label_from_config(resolved_color_input(node, request, "color").unwrap_or(
            ColorConfig::Rgb {
                r: 80,
                g: 180,
                b: 255,
            },
        )),
        center_x: resolved_number_input(
            node,
            request,
            "centerX",
            request.source_ir.width.saturating_sub(1) as f64 / 2.0,
        ),
        center_y: resolved_number_input(
            node,
            request,
            "centerY",
            request.source_ir.height.saturating_sub(1) as f64 / 2.0,
        ),
        radius: resolved_number_input(node, request, "radius", 8.0).max(0.5),
        intensity: resolved_number_input(node, request, "intensity", 1.0).clamp(0.0, 1.0),
        apply_to,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_radar_shader(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "shader.radar",
        &["applyTo", "color", "speed", "tailLength"],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }
    let apply_to = match strict_enum_input(
        node,
        request,
        "applyTo",
        "foreground",
        &["foreground", "background", "both"],
        "shader.radar",
    ) {
        Ok(apply_to) => apply_to,
        Err(reason) => return NodeLoweringOutcome::Unsupported { reason },
    };
    style_stages.push(NativeStyleStage::Radar {
        color: color_label_from_config(resolved_color_input(node, request, "color").unwrap_or(
            ColorConfig::Rgb {
                r: 80,
                g: 255,
                b: 160,
            },
        )),
        speed: resolved_number_input(node, request, "speed", 1.0).max(0.0),
        tail_length: resolved_number_input(node, request, "tailLength", 0.25).clamp(0.01, 1.0),
        apply_to,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_filter_bracket_emphasis(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "filter.bracketEmphasis",
        &["emphasisColor", "edgeWidth", "applyTo"],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    style_stages.push(NativeStyleStage::BracketEmphasis {
        emphasis_color: color_label_input(node, request, "emphasisColor", (255, 210, 90)),
        edge_width: integer_input(node, request, "edgeWidth", 1).max(0) as usize,
        apply_to: enum_label_input(node, request, "applyTo", "foreground"),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_filter_dot_indicator(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "filter.dotIndicator",
        &[
            "activeColor",
            "inactiveColor",
            "period",
            "applyTo",
            "progress",
            "bgColor",
            "color",
            "indicatorChar",
            "position",
        ],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    spec.filters.push(FilterSpec::DotIndicator {
        indicator_char: char_input(node, request, "indicatorChar", '•'),
        position: hover_bar_position_input(node, request, "position"),
        color: color_alias_input(node, request, &["color", "activeColor"], (100, 150, 200)),
        bg_color: color_alias_input(node, request, &["bgColor", "inactiveColor"], (30, 30, 30)),
        progress: BindableValue::from(number_signal_input(node, request, "progress", 1.0)),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_filter_edge_grow(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "filter.edgeGrow",
        &["direction", "progress", "edgeColor", "applyTo"],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    style_stages.push(NativeStyleStage::EdgeGrow {
        direction: enum_label_input(node, request, "direction", "left"),
        progress: number_input(node, request, "progress", request.ir.phase_t).clamp(0.0, 1.0),
        edge_color: color_label_input(node, request, "edgeColor", (255, 120, 80)),
        apply_to: enum_label_input(node, request, "applyTo", "both"),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_filter_hover_bar(
    node: &NodeSpec,
    spec: &mut CompositionSpec,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "filter.hoverBar",
        &[
            "barColor",
            "bgColor",
            "baseEighths",
            "maxEighths",
            "marginWidth",
            "position",
            "progress",
        ],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    spec.filters.push(FilterSpec::HoverBar {
        base_eighths: number_input(node, request, "baseEighths", 4.0)
            .round()
            .clamp(0.0, 8.0) as u8,
        max_eighths: number_input(node, request, "maxEighths", 12.0)
            .round()
            .clamp(0.0, 16.0) as u8,
        position: hover_bar_position_input(node, request, "position"),
        bar_color: color_input(node, request, "barColor").unwrap_or(ColorConfig::Rgb {
            r: 100,
            g: 150,
            b: 200,
        }),
        bg_color: color_input(node, request, "bgColor").unwrap_or(ColorConfig::Rgb {
            r: 30,
            g: 30,
            b: 30,
        }),
        progress: BindableValue::from(number_signal_input(node, request, "progress", 0.0)),
        margin_width: integer_input(node, request, "marginWidth", 2).clamp(1, 2) as u8,
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_filter_matrix_rain(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "filter.matrixRain",
        &[
            "speedMultiplier",
            "speedMin",
            "speedMax",
            "speed",
            "glyphChangeHz",
            "density",
            "seed",
            "trailMin",
            "trailMax",
            "affect",
            "chars",
            "mode",
            "preset",
            "headColor",
            "tailColor",
        ],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    let speed_min = number_input(node, request, "speedMin", 0.0).max(0.0);
    let speed_default = number_input(node, request, "speed", 1.0);
    let trail_min = integer_input(node, request, "trailMin", 2).max(0) as f64;
    style_stages.push(NativeStyleStage::MatrixRain {
        speed_multiplier: number_input(node, request, "speedMultiplier", 1.0).max(0.0),
        speed_min,
        speed_max: number_input(node, request, "speedMax", speed_default).max(speed_min),
        glyph_change_hz: number_input(node, request, "glyphChangeHz", 8.0).max(0.0),
        density: number_input(node, request, "density", 0.5).clamp(0.0, 1.0),
        seed: integer_input(node, request, "seed", 1).max(0) as f64,
        trail_min,
        trail_max: (integer_input(node, request, "trailMax", 8) as f64).max(trail_min),
        affect: enum_label_input(node, request, "affect", "foreground"),
        chars: enum_label_input(node, request, "chars", "01"),
        mode: enum_label_input(node, request, "mode", "rain"),
        preset: enum_label_input(node, request, "preset", "default"),
        head_color: color_label_input(node, request, "headColor", (40, 255, 80)),
        tail_color: color_label_input(node, request, "tailColor", (20, 120, 40)),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_filter_sub_pixel_bar(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "filter.subPixelBar",
        &[
            "barColor",
            "offset",
            "width",
            "applyTo",
            "progress",
            "direction",
            "filledColor",
            "unfilledColor",
            "animated",
        ],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    style_stages.push(NativeStyleStage::SubPixelBar {
        filled_color: color_label_input(node, request, "filledColor", (100, 220, 255)),
        unfilled_color: color_label_input(node, request, "unfilledColor", (30, 40, 50)),
        progress: number_input(node, request, "progress", request.ir.phase_t).clamp(0.0, 1.0),
        direction: enum_label_input(node, request, "direction", "horizontal"),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

fn lower_filter_underline_wipe(
    node: &NodeSpec,
    style_stages: &mut Vec<NativeStyleStage>,
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    if let Some(reason) = unsupported_style_stage_reason(
        node,
        "filter.underlineWipe",
        &["underlineColor", "progress", "thickness", "applyTo"],
        StyleScopeRequirement::All,
    ) {
        return NodeLoweringOutcome::Unsupported { reason };
    }

    style_stages.push(NativeStyleStage::UnderlineWipe {
        underline_color: color_label_input(node, request, "underlineColor", (120, 220, 255)),
        progress: number_input(node, request, "progress", request.ir.phase_t).clamp(0.0, 1.0),
        thickness: integer_input(node, request, "thickness", 1).max(1) as usize,
        apply_to: enum_label_input(node, request, "applyTo", "foreground"),
    });
    NodeLoweringOutcome::Lowered { warnings }
}

enum StyleScopeRequirement {
    All,
    ModuloColumns,
}

fn unsupported_style_stage_reason(
    node: &NodeSpec,
    effect_id: &str,
    supported_inputs: &[&str],
    scope_requirement: StyleScopeRequirement,
) -> Option<String> {
    let unsupported_inputs = node
        .inputs
        .keys()
        .map(|key| key.as_str())
        .filter(|key| !supported_inputs.contains(key))
        .collect::<Vec<_>>();
    if !unsupported_inputs.is_empty() {
        return Some(format!(
            "Effect `{effect_id}` uses input(s) `{}` that have no compositor-native style-stage equivalent without dropping authored semantics.",
            unsupported_inputs.join("`, `"),
        ));
    }
    if !node.outputs.is_empty() {
        return Some(format!(
            "Effect `{effect_id}` declares graph outputs that the compositor-native style stage cannot publish without dropping authored semantics."
        ));
    }
    match (&scope_requirement, node.scope.as_ref()) {
        (StyleScopeRequirement::All, None | Some(tui_vfx_contract::ScopeSpec::All)) => None,
        (
            StyleScopeRequirement::ModuloColumns,
            Some(tui_vfx_contract::ScopeSpec::ModuloColumns { .. }),
        ) => None,
        _ => Some(format!(
            "Effect `{effect_id}` uses a scope that is not yet supported by the compositor-native style stage without dropping authored semantics."
        )),
    }
}

fn modulo_columns_scope(node: &NodeSpec) -> Option<(usize, usize)> {
    match node.scope.as_ref()? {
        tui_vfx_contract::ScopeSpec::ModuloColumns { modulus, remainder } if *modulus > 0 => {
            Some((*modulus, *remainder))
        }
        _ => None,
    }
}

fn ignored_policy_warnings(node: &NodeSpec) -> Vec<PlayerRenderBackendDiagnostic> {
    let mut warnings = Vec::new();
    if node.cell_write_policy.is_some() {
        warnings.push(PlayerRenderBackendDiagnostic {
            code: "fieldIgnoredWithWarning".to_string(),
            path: format!("graph.nodes.{}.cellWritePolicy", node.id.as_str()),
            message: "Native compositor lowering consumes the player-resolved source grid and does not reinterpret cell write policy in this backend slice.".to_string(),
        });
    }
    if node.role_write_policy.is_some() {
        warnings.push(PlayerRenderBackendDiagnostic {
            code: "fieldIgnoredWithWarning".to_string(),
            path: format!("graph.nodes.{}.roleWritePolicy", node.id.as_str()),
            message: "Native compositor lowering preserves source roles and reports role write policy as handled by the player source grid for this backend slice.".to_string(),
        });
    }
    warnings
}

fn node_has_input(node: &NodeSpec, key: &str) -> bool {
    node.inputs
        .contains_key(&tui_vfx_contract::EffectInputId::new(key))
}

fn input_value<'a>(
    node: &'a NodeSpec,
    request: &'a PlayerRenderBackendRequest,
    key: &str,
) -> Option<&'a Value> {
    runtime_override_value(node, request, key).or_else(|| {
        node.inputs
            .get(&tui_vfx_contract::EffectInputId::new(key))
            .and_then(|source| match source {
                ValueSource::Literal { value } => Some(value),
                ValueSource::Signal { fallback, .. }
                | ValueSource::Parameter { fallback, .. }
                | ValueSource::GraphValue { fallback, .. } => fallback.as_ref(),
                ValueSource::Map { .. } | ValueSource::SampledField { .. } => None,
            })
    })
}

fn resolved_input_value(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
) -> Option<Value> {
    runtime_override_value(node, request, key)
        .cloned()
        .or_else(|| {
            node.inputs
                .get(&tui_vfx_contract::EffectInputId::new(key))
                .and_then(|source| {
                    resolve_value_source_with_graph_values(
                        source,
                        &request.sample.signals,
                        &request.sample.graph_values,
                    )
                })
        })
}

fn runtime_override_value<'a>(
    node: &NodeSpec,
    request: &'a PlayerRenderBackendRequest,
    key: &str,
) -> Option<&'a Value> {
    let candidates = [
        format!("{}.{}", node.id.as_str(), key),
        format!("{}.{}", node.effect.as_str(), key),
        format!(
            "effect:{}:{}:{}",
            node.effect.as_str(),
            node.id.as_str(),
            key
        ),
        key.to_string(),
    ];
    candidates
        .iter()
        .find_map(|candidate| request.sample.runtime_input_overrides.get(candidate))
        .or_else(|| {
            let normalized_candidates = candidates
                .iter()
                .map(|candidate| normalize_runtime_key(candidate))
                .collect::<Vec<_>>();
            request
                .sample
                .runtime_input_overrides
                .iter()
                .find(|(override_key, _)| {
                    let normalized_override = normalize_runtime_key(override_key);
                    normalized_candidates
                        .iter()
                        .any(|candidate| candidate == &normalized_override)
                })
                .map(|(_, value)| value)
        })
}

fn normalize_runtime_key(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn signal_source_id(node: &NodeSpec, key: &str) -> Option<tui_vfx_contract::SignalId> {
    node.inputs
        .get(&tui_vfx_contract::EffectInputId::new(key))
        .and_then(|source| match source {
            ValueSource::Signal { id, .. } => Some(id.clone()),
            _ => None,
        })
}

fn number_signal_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default: f64,
) -> SignalOrFloat {
    if let Some(signal_id) = signal_source_id(node, key) {
        let value = request
            .sample
            .signals
            .get(&signal_id)
            .and_then(Value::as_range_number)
            .unwrap_or_else(|| number_input(node, request, key, default));
        return SignalOrFloat::Static(value as f32);
    }
    SignalOrFloat::Static(number_input(node, request, key, default) as f32)
}

fn number_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default: f64,
) -> f64 {
    input_value(node, request, key)
        .and_then(Value::as_range_number)
        .unwrap_or(default)
}

fn optional_number_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
) -> Option<f64> {
    input_value(node, request, key).and_then(Value::as_range_number)
}

fn optional_i16_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
) -> Option<i16> {
    input_value(node, request, key).and_then(|value| match value {
        Value::Integer(value) => Some((*value).clamp(i16::MIN as i64, i16::MAX as i64) as i16),
        Value::Number(value) => {
            Some((*value as i64).clamp(i16::MIN as i64, i16::MAX as i64) as i16)
        }
        _ => None,
    })
}

fn integer_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default: i64,
) -> i64 {
    input_value(node, request, key)
        .and_then(|value| match value {
            Value::Integer(value) => Some(*value),
            Value::Number(value) => Some(*value as i64),
            _ => None,
        })
        .unwrap_or(default)
}

fn resolved_number_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default: f64,
) -> f64 {
    resolved_input_value(node, request, key)
        .as_ref()
        .and_then(Value::as_range_number)
        .unwrap_or(default)
}

fn resolved_integer_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default: i64,
) -> i64 {
    resolved_input_value(node, request, key)
        .as_ref()
        .and_then(|value| match value {
            Value::Integer(value) => Some(*value),
            Value::Number(value) => Some(*value as i64),
            _ => None,
        })
        .unwrap_or(default)
}

fn positive_u8_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default: i64,
) -> u8 {
    integer_input(node, request, key, default).clamp(1, u8::MAX as i64) as u8
}

fn positive_u16_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default: i64,
) -> u16 {
    integer_input(node, request, key, default).clamp(1, u16::MAX as i64) as u16
}

fn bool_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default: bool,
) -> bool {
    input_value(node, request, key)
        .and_then(|value| match value {
            Value::Boolean(value) => Some(*value),
            _ => None,
        })
        .unwrap_or(default)
}

fn bool_or_color_presence_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default: bool,
) -> bool {
    input_value(node, request, key)
        .map(|value| match value {
            Value::Boolean(value) => *value,
            Value::Color(_) | Value::String(_) | Value::Text(_) => true,
            _ => default,
        })
        .unwrap_or(default)
}

fn multiply_signal_or_float(value: SignalOrFloat, factor: SignalOrFloat) -> SignalOrFloat {
    match (value, factor) {
        (SignalOrFloat::Static(value), SignalOrFloat::Static(factor)) => {
            SignalOrFloat::Static(value * factor)
        }
        (value, SignalOrFloat::Static(1.0)) => value,
        (value, _) => value,
    }
}

fn enum_input<'a>(
    node: &'a NodeSpec,
    request: &'a PlayerRenderBackendRequest,
    key: &str,
) -> Option<&'a str> {
    input_value(node, request, key).and_then(|value| {
        value.as_enum_value().or(match value {
            Value::String(value) | Value::Text(value) => Some(value.as_str()),
            _ => None,
        })
    })
}

fn enum_label_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default: &str,
) -> String {
    enum_input(node, request, key)
        .unwrap_or(default)
        .to_string()
}

fn text_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default: &str,
) -> String {
    input_value(node, request, key)
        .and_then(|value| match value {
            Value::Enum(value) | Value::String(value) | Value::Text(value) => Some(value.clone()),
            _ => None,
        })
        .unwrap_or_else(|| default.to_string())
}

fn char_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default: char,
) -> char {
    let fallback = default.to_string();
    text_input(node, request, key, &fallback)
        .chars()
        .next()
        .unwrap_or(default)
}

fn strict_enum_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default: &str,
    supported_values: &[&str],
    effect_id: &str,
) -> Result<String, String> {
    let value = enum_input(node, request, key).unwrap_or(default);
    if supported_values.contains(&value) {
        return Ok(value.to_string());
    }
    Err(format!(
        "Effect `{effect_id}` uses `{key}` value `{value}` that has no compositor-backend native stage equivalent without dropping authored semantics."
    ))
}

fn color_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
) -> Option<ColorConfig> {
    input_value(node, request, key).and_then(|value| match value {
        Value::Color(color) => Some(ColorConfig::from(*color)),
        Value::String(value) | Value::Text(value) => color_config_from_hex(value),
        _ => None,
    })
}

fn resolved_color_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
) -> Option<ColorConfig> {
    resolved_input_value(node, request, key).and_then(|value| match value {
        Value::Color(color) => Some(ColorConfig::from(color)),
        Value::String(value) | Value::Text(value) => color_config_from_hex(&value),
        _ => None,
    })
}

fn color_label_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default_rgb: (u8, u8, u8),
) -> String {
    color_label_from_config(color_input(node, request, key).unwrap_or(ColorConfig::Rgb {
        r: default_rgb.0,
        g: default_rgb.1,
        b: default_rgb.2,
    }))
}

fn color_alias_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    keys: &[&str],
    default_rgb: (u8, u8, u8),
) -> ColorConfig {
    keys.iter()
        .find_map(|key| color_input(node, request, key))
        .unwrap_or(ColorConfig::Rgb {
            r: default_rgb.0,
            g: default_rgb.1,
            b: default_rgb.2,
        })
}

fn color_config_from_hex(value: &str) -> Option<ColorConfig> {
    let hex = value.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(ColorConfig::Rgb { r, g, b })
}

fn color_label_from_config(color: ColorConfig) -> String {
    let color = tui_vfx_types::Color::from(color);
    if color.a == 0 {
        "transparent".to_string()
    } else {
        format!("rgba({},{},{},{})", color.r, color.g, color.b, color.a)
    }
}

fn gradient_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
) -> Option<Gradient> {
    input_value(node, request, key).and_then(|value| match value {
        Value::Gradient(gradient) => Some(Gradient {
            stops: gradient
                .stops
                .iter()
                .map(|stop| (stop.position as f32, stop.color))
                .collect(),
            space: color_space_from_str(&gradient.space),
        }),
        _ => None,
    })
}

fn color_space_from_str(value: &str) -> ColorSpace {
    match value {
        "hsl" => ColorSpace::Hsl,
        "hct" => ColorSpace::Hct,
        _ => ColorSpace::Rgb,
    }
}

fn apply_to_input(node: &NodeSpec, request: &PlayerRenderBackendRequest, key: &str) -> ApplyTo {
    match enum_input(node, request, key) {
        Some("foreground" | "fg") => ApplyTo::Foreground,
        Some("background" | "bg") => ApplyTo::Background,
        _ => ApplyTo::Both,
    }
}

fn hover_bar_position_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
) -> HoverBarPosition {
    match enum_input(node, request, key) {
        Some("right") => HoverBarPosition::Right,
        Some("top") => HoverBarPosition::Top,
        Some("bottom") => HoverBarPosition::Bottom,
        _ => HoverBarPosition::Left,
    }
}

fn scanner_axis_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
) -> ScannerAxis {
    match enum_input(node, request, key) {
        Some("vertical" | "y" | "Y") => ScannerAxis::Vertical,
        _ => ScannerAxis::Horizontal,
    }
}

fn scanner_motion_mode_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
) -> ScannerMotionMode {
    match enum_input(node, request, key) {
        Some("forwardWrap" | "forward_wrap" | "forward") => ScannerMotionMode::ForwardWrap,
        Some("reverseWrap" | "reverse_wrap" | "reverse") => ScannerMotionMode::ReverseWrap,
        _ => ScannerMotionMode::PingPong,
    }
}

fn normalized_boost_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
    default: u8,
) -> u8 {
    let value = number_input(node, request, key, default as f64);
    let normalized = if (0.0..=1.0).contains(&value) {
        value * 255.0
    } else {
        value
    };
    normalized.round().clamp(0.0, u8::MAX as f64) as u8
}

fn kitt_band_width_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
) -> (f32, Option<u16>) {
    if node_has_input(node, "width") {
        return (0.15, Some(positive_u16_input(node, request, "width", 3)));
    }

    if let Some(value) = optional_number_input(node, request, "bandWidth") {
        if value > 1.0 {
            return (0.15, Some(value.round().clamp(1.0, u16::MAX as f64) as u16));
        }
        return (value.clamp(0.0, 0.5) as f32, None);
    }

    (0.15, None)
}

fn gradient_apply_to_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
) -> LinearGradientApplyTo {
    match enum_input(node, request, key) {
        Some("background" | "bg") => LinearGradientApplyTo::Background,
        Some("both") => LinearGradientApplyTo::Both,
        _ => LinearGradientApplyTo::Foreground,
    }
}

fn axis_input(node: &NodeSpec, request: &PlayerRenderBackendRequest, key: &str) -> Axis {
    match enum_input(node, request, key) {
        Some("y" | "Y") => Axis::Y,
        _ => Axis::X,
    }
}

fn wipe_direction_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
) -> WipeDirection {
    match enum_input(node, request, key) {
        Some("rightToLeft" | "right_to_left") => WipeDirection::RightToLeft,
        Some("topToBottom" | "top_to_bottom") => WipeDirection::TopToBottom,
        Some("bottomToTop" | "bottom_to_top") => WipeDirection::BottomToTop,
        Some("topLeftToBottomRight" | "top_left_to_bottom_right") => {
            WipeDirection::TopLeftToBottomRight
        }
        Some("topRightToBottomLeft" | "top_right_to_bottom_left") => {
            WipeDirection::TopRightToBottomLeft
        }
        Some("bottomLeftToTopRight" | "bottom_left_to_top_right") => {
            WipeDirection::BottomLeftToTopRight
        }
        Some("bottomRightToTopLeft" | "bottom_right_to_top_left") => {
            WipeDirection::BottomRightToTopLeft
        }
        _ => WipeDirection::LeftToRight,
    }
}

fn radial_origin_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
) -> RadialOrigin {
    match enum_input(node, request, key) {
        Some("topLeft" | "top_left") => RadialOrigin::TopLeft,
        Some("topRight" | "top_right") => RadialOrigin::TopRight,
        Some("bottomLeft" | "bottom_left") => RadialOrigin::BottomLeft,
        Some("bottomRight" | "bottom_right") => RadialOrigin::BottomRight,
        _ => RadialOrigin::Center,
    }
}

fn pattern_type_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
) -> PatternType {
    match enum_input(node, request, key) {
        Some("dot" | "dots") => PatternType::Dots,
        Some("stripe" | "stripes") => PatternType::Stripe,
        Some("diagonal") => PatternType::Diagonal,
        Some(value) if value.chars().count() == 1 => PatternType::Single {
            char: value.chars().next().unwrap_or('.'),
        },
        _ => PatternType::Diagonal,
    }
}

fn vignette_sides_input(
    node: &NodeSpec,
    request: &PlayerRenderBackendRequest,
    key: &str,
) -> Vec<VignetteEdge> {
    match enum_input(node, request, key) {
        Some("top") => vec![VignetteEdge::Top],
        Some("bottom") => vec![VignetteEdge::Bottom],
        Some("left") => vec![VignetteEdge::Left],
        Some("right") => vec![VignetteEdge::Right],
        Some("topBottom" | "top_bottom" | "vertical") => {
            vec![VignetteEdge::Top, VignetteEdge::Bottom]
        }
        Some("leftRight" | "left_right" | "horizontal") => {
            vec![VignetteEdge::Left, VignetteEdge::Right]
        }
        Some("all") => vec![
            VignetteEdge::Top,
            VignetteEdge::Bottom,
            VignetteEdge::Left,
            VignetteEdge::Right,
        ],
        _ => Vec::new(),
    }
}

fn blend_color_config(from: ColorConfig, to: ColorConfig, progress: f64) -> ColorConfig {
    let from = tui_vfx_types::Color::from(from);
    let to = tui_vfx_types::Color::from(to);
    let t = progress.clamp(0.0, 1.0);
    ColorConfig::Rgb {
        r: lerp_u8(from.r, to.r, t),
        g: lerp_u8(from.g, to.g, t),
        b: lerp_u8(from.b, to.b, t),
    }
}

fn lerp_u8(from: u8, to: u8, progress: f64) -> u8 {
    (from as f64 + (to as f64 - from as f64) * progress).round() as u8
}

fn composition_spec_summary(spec: &CompositionSpec) -> BTreeMap<String, serde_json::Value> {
    composition_spec_summary_with_content_stages(spec, 0, 0)
}

fn composition_spec_summary_with_content_stages(
    spec: &CompositionSpec,
    content_stage_count: usize,
    style_stage_count: usize,
) -> BTreeMap<String, serde_json::Value> {
    BTreeMap::from([
        ("contentStages".to_string(), json!(content_stage_count)),
        ("styleStages".to_string(), json!(style_stage_count)),
        (
            "samplers".to_string(),
            json!(spec.effective_samplers().len()),
        ),
        ("masks".to_string(), json!(spec.masks.len())),
        ("filters".to_string(), json!(spec.filters.len())),
        ("shaderLayers".to_string(), json!(spec.shader_layers.len())),
        ("shadow".to_string(), json!(spec.shadow.is_some())),
        (
            "timing".to_string(),
            json!({"t": spec.t, "loopT": spec.loop_t}),
        ),
    ])
}

fn composition_spec_non_empty(spec: &CompositionSpec) -> bool {
    !spec.filters.is_empty()
        || !spec.masks.is_empty()
        || !spec.effective_samplers().is_empty()
        || !spec.shader_layers.is_empty()
        || spec.shadow.is_some()
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.contains(&value) {
        values.push(value);
    }
}

// <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs</FILE> - <DESC>Lower player render requests into compositor CompositionSpec modes</DESC>
// <VERS>END OF VERSION: 0.13.0</VERS>
