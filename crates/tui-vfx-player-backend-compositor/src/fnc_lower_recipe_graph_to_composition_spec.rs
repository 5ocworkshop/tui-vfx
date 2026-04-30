// <FILE>crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs</FILE> - <DESC>Lower player render requests into compositor CompositionSpec modes</DESC>
// <VERS>VERSION: 0.2.1</VERS>
// <WCTX>Native compositor lowering: map bounded v3.1 recipe graph effects into native CompositionSpec content with honest fallback diagnostics.</WCTX>
// <CLOG>0.2.1: PATCH — pass native lowering counts directly when building evidence.
// 0.2.0: MINOR — add native/auto/irResolved lowering modes for filter, mask, sampler, shader, and style debug-recipes.
// 0.1.0: INIT — carry playback timing and record that player IR already contains resolved recipe effects for this bounded slice.</CLOG>

use std::collections::BTreeMap;

use mixed_signals::types::SignalOrFloat;
use serde_json::json;
use tui_vfx_compositor::types::cls_filter_spec::VignetteEdge;
use tui_vfx_compositor::{
    pipeline::{CompositionSpec, ShaderLayerSpec},
    types::{
        ApplyTo, Axis, BindableValue, FilterSpec, MaskSpec, RippleCenter, SamplerSpec,
        WipeDirection,
    },
};
use tui_vfx_contract::{NodeSpec, RecipeDocument, Value, ValueSource};
use tui_vfx_player::{
    PlayerRenderBackendCompositionEvidence, PlayerRenderBackendDiagnostic,
    PlayerRenderBackendRequest, PlayerRenderCompositionMode, PlayerRenderIrReport,
};
use tui_vfx_style::models::{
    BorderSweepShader, ColorConfig, ColorSpace, Gradient, LinearGradientApplyTo,
    LinearGradientShader, SpatialShaderType, StyleRegion,
};

/// Complete lowering result used by the compositor backend adapter.
#[derive(Clone, Debug)]
pub struct LoweredCompositionSpec {
    /// Composition instructions to pass to the compositor pipeline.
    pub spec: CompositionSpec,
    /// Source-content transforms applied before compositor effects while staying source-only.
    pub content_stages: Vec<NativeContentStage>,
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

        match lower_node_into_spec(
            &request.recipe,
            node,
            request,
            &mut spec,
            &mut content_stages,
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
    let native_stage_non_empty = !content_stages.is_empty();
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
            ),
            source_render_mode: "sourceOnly".to_string(),
            native_source_isolated: true,
        },
        spec,
        content_stages,
        diagnostics,
    }
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
) -> NodeLoweringOutcome {
    let warnings = ignored_policy_warnings(node);
    let effect = node.effect.as_str();
    match effect {
        "content.typewriter" => lower_content_typewriter(node, request, content_stages, warnings),
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
        "filter.vignette" => lower_vignette(node, spec, request, warnings),
        "filter.crt" => lower_crt(node, spec, request, warnings),
        "filter.pillButton" => {
            let progress = number_signal_input(node, request, "progress", 0.75);
            spec.filters.push(FilterSpec::Tint {
                color: color_input(node, request, "activeColor").unwrap_or(ColorConfig::Cyan),
                strength: progress,
                apply_to: ApplyTo::Both,
            });
            NodeLoweringOutcome::Lowered { warnings }
        }
        "mask.wipe" => {
            spec.masks.push(MaskSpec::Wipe {
                reveal: Some(wipe_direction_input(node, request, "direction")),
                hide: None,
                direction: None,
                soft_edge: bool_input(node, request, "softEdge", false),
            });
            NodeLoweringOutcome::Lowered { warnings }
        }
        "mask.checkers" => {
            spec.masks.push(MaskSpec::Checkers {
                cell_size: integer_input(node, request, "cellSize", 2).max(1) as u16,
            });
            NodeLoweringOutcome::Lowered { warnings }
        }
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
        "shader.linearGradient" => lower_linear_gradient(node, spec, request, warnings),
        "shader.borderSweep" => lower_border_sweep(recipe, node, spec, request, warnings),
        "style.fadeIn" | "style.fadeOut" => lower_style_fade(node, spec, request, warnings),
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
    const SUPPORTED_INPUTS: &[&str] = &[
        "speed",
        "speedVariance",
        "cursorCharacter",
        "cursorWake",
        "wakeCells",
    ];

    let unsupported_inputs = node
        .inputs
        .keys()
        .map(|key| key.as_str())
        .filter(|key| !SUPPORTED_INPUTS.contains(key))
        .collect::<Vec<_>>();
    if !unsupported_inputs.is_empty() {
        return Some(format!(
            "Effect `content.typewriter` uses input(s) `{}` that have no compositor-backend native content-stage equivalent without dropping authored semantics.",
            unsupported_inputs.join("`, `")
        ));
    }

    if !node.outputs.is_empty() {
        return Some(
            "Effect `content.typewriter` declares graph outputs that the compositor-backend native content stage cannot publish without dropping authored semantics."
                .to_string(),
        );
    }

    if let Some(scope) = &node.scope
        && !matches!(scope, tui_vfx_contract::ScopeSpec::All)
    {
        return Some(
            "Effect `content.typewriter` uses a non-all scope that is not yet supported by the compositor-backend native content stage without dropping authored semantics."
                .to_string(),
        );
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
    request: &PlayerRenderBackendRequest,
    warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    let unsupported_inputs = [
        "edgeColor",
        "applyTo",
        "bgColor",
        "color",
        "fillColor",
        "filledColor",
        "unfilledColor",
        "left",
        "right",
        "progress",
    ]
    .into_iter()
    .filter(|key| node_has_input(node, key))
    .collect::<Vec<_>>();
    if !unsupported_inputs.is_empty() {
        return NodeLoweringOutcome::Unsupported {
            reason: format!(
                "Effect `filter.vignette` uses input(s) `{}` that have no compositor-native FilterSpec equivalent without dropping authored semantics.",
                unsupported_inputs.join("`, `")
            ),
        };
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
    mut warnings: Vec<PlayerRenderBackendDiagnostic>,
) -> NodeLoweringOutcome {
    let from = color_input(node, request, "from").unwrap_or(ColorConfig::Black);
    let to = color_input(node, request, "to").unwrap_or(ColorConfig::White);
    let progress = request.ir.phase_t.clamp(0.0, 1.0);
    let style_color = blend_color_config(from, to, progress);
    if node
        .inputs
        .contains_key(&tui_vfx_contract::EffectInputId::new("easing"))
    {
        warnings.push(PlayerRenderBackendDiagnostic {
            code: "fieldIgnoredWithWarning".to_string(),
            path: format!("graph.nodes.{}.inputs.easing", node.id.as_str()),
            message: "Native compositor style fade currently applies linear progress; authored easing is reported rather than silently ignored.".to_string(),
        });
    }
    spec.filters.push(FilterSpec::Tint {
        color: style_color,
        strength: SignalOrFloat::Static(1.0),
        apply_to: apply_to_input(node, request, "applyTo"),
    });
    NodeLoweringOutcome::Lowered { warnings }
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
    composition_spec_summary_with_content_stages(spec, 0)
}

fn composition_spec_summary_with_content_stages(
    spec: &CompositionSpec,
    content_stage_count: usize,
) -> BTreeMap<String, serde_json::Value> {
    BTreeMap::from([
        ("contentStages".to_string(), json!(content_stage_count)),
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
// <VERS>END OF VERSION: 0.2.1</VERS>
