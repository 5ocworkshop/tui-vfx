// <FILE>crates/tui-vfx-player/src/fnc_apply_graph_effects.rs</FILE> - <DESC>Apply graph effect adapters to player frame evidence</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Player graph execution: prefer topology/value-bus semantics with graph.order fallback.</WCTX>
// <CLOG>0.6.0: MINOR — execute sequence/parallel topology, publish graph-value inputs, and warn on deterministic parallel conflicts.
// 0.5.1: PATCH — consolidate text-grid adapter synchronization.
// 0.5.0: MINOR — route K2.9 simple mask adapters.
// 0.4.0: MINOR — route field-aware filter adapters through player styled grids.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{
    DescriptorCatalog, EffectDescriptor, GraphStep, GraphValueId, GraphValueMergePolicy, NodeId,
    NodeOutputSource, NodeSpec, ParallelMergePolicy, RecipeDocument, Value, ValueKind, ValueSource,
};

use crate::{
    PlayerError, PlayerSampleRequest, PlayerStyledGrid, PlayerWarning,
    fnc_apply_content_primitive::apply_content_primitive,
    fnc_apply_distortion_sampler_primitives::apply_distortion_sampler_primitive,
    fnc_apply_filter_primitive::apply_filter_primitive,
    fnc_apply_mask_checkers::apply_mask_checkers,
    fnc_apply_mask_dissolve::apply_mask_dissolve,
    fnc_apply_mask_wipe::apply_mask_wipe,
    fnc_apply_sampler_ripple::apply_sampler_ripple,
    fnc_apply_sampler_sine_wave::apply_sampler_sine_wave,
    fnc_apply_simple_mask_primitives::{
        apply_mask_blinds, apply_mask_cellular, apply_mask_diamond, apply_mask_iris,
        apply_mask_materialize, apply_mask_radial,
    },
    fnc_apply_styled_primitive::apply_styled_primitive,
    fnc_resolve_value_source::resolve_value_source_with_graph_values,
};

/// Apply supported graph effects and collect unsupported adapter diagnostics.
pub fn apply_graph_effects(
    recipe: &RecipeDocument,
    catalog: Option<&DescriptorCatalog>,
    request: &mut PlayerSampleRequest,
    rows: &mut [String],
    styled_grid: &mut PlayerStyledGrid,
    errors: &mut Vec<PlayerError>,
    warnings: &mut Vec<PlayerWarning>,
) {
    if let Some(topology) = &recipe.graph.topology {
        execute_graph_step(
            recipe,
            catalog,
            topology,
            request,
            rows,
            styled_grid,
            errors,
            warnings,
        );
    } else {
        execute_node_order(recipe, catalog, request, rows, styled_grid, errors);
    }
}

/// Apply one explicit graph topology step to an already prepared local surface.
pub fn apply_graph_step_effects(
    recipe: &RecipeDocument,
    catalog: Option<&DescriptorCatalog>,
    step: &GraphStep,
    request: &mut PlayerSampleRequest,
    rows: &mut [String],
    styled_grid: &mut PlayerStyledGrid,
    errors: &mut Vec<PlayerError>,
    warnings: &mut Vec<PlayerWarning>,
) {
    execute_graph_step(
        recipe,
        catalog,
        step,
        request,
        rows,
        styled_grid,
        errors,
        warnings,
    );
}

fn execute_graph_step(
    recipe: &RecipeDocument,
    catalog: Option<&DescriptorCatalog>,
    step: &GraphStep,
    request: &mut PlayerSampleRequest,
    rows: &mut [String],
    styled_grid: &mut PlayerStyledGrid,
    errors: &mut Vec<PlayerError>,
    warnings: &mut Vec<PlayerWarning>,
) {
    match step {
        GraphStep::Node { node } => {
            execute_node_id(recipe, catalog, node, request, rows, styled_grid, errors)
        }
        GraphStep::Sequence { children } => {
            for child in children {
                execute_graph_step(
                    recipe,
                    catalog,
                    child,
                    request,
                    rows,
                    styled_grid,
                    errors,
                    warnings,
                );
            }
        }
        GraphStep::Parallel {
            children,
            merge_policy,
            value_merge_policy,
        } => execute_parallel_step(
            recipe,
            catalog,
            children,
            *merge_policy,
            *value_merge_policy,
            request,
            rows,
            styled_grid,
            errors,
            warnings,
        ),
    }
}

fn execute_node_order(
    recipe: &RecipeDocument,
    catalog: Option<&DescriptorCatalog>,
    request: &mut PlayerSampleRequest,
    rows: &mut [String],
    styled_grid: &mut PlayerStyledGrid,
    errors: &mut Vec<PlayerError>,
) {
    for node_id in &recipe.graph.order {
        execute_node_id(recipe, catalog, node_id, request, rows, styled_grid, errors);
    }
}

fn execute_node_id(
    recipe: &RecipeDocument,
    catalog: Option<&DescriptorCatalog>,
    node_id: &NodeId,
    request: &mut PlayerSampleRequest,
    rows: &mut [String],
    styled_grid: &mut PlayerStyledGrid,
    errors: &mut Vec<PlayerError>,
) {
    let Some(node) = recipe.graph.nodes.get(node_id) else {
        errors.push(PlayerError::new(
            "unknownTopologyNode",
            "graph.topology",
            format!(
                "Graph topology references missing node `{}`",
                node_id.as_str()
            ),
            Some("Declare the topology node under graph.nodes before rendering."),
            serde_json::json!({ "node": node_id.as_str() }),
        ));
        return;
    };
    if !node_active_for_request(node, request) {
        return;
    }
    push_graph_value_input_diagnostics(recipe, catalog, node, request, errors);
    apply_node(node, request, rows, styled_grid, errors);
    publish_node_outputs(node, request, errors);
}

fn node_active_for_request(node: &NodeSpec, request: &PlayerSampleRequest) -> bool {
    node.active_phases.is_empty()
        || node
            .active_phases
            .iter()
            .any(|phase| phase == &request.phase)
}

fn push_graph_value_input_diagnostics(
    recipe: &RecipeDocument,
    catalog: Option<&DescriptorCatalog>,
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    errors: &mut Vec<PlayerError>,
) {
    for (input_id, source) in &node.inputs {
        let ValueSource::GraphValue { id, fallback } = source else {
            continue;
        };
        let Some(value) = request.graph_values.get(id) else {
            if fallback.is_none() {
                errors.push(PlayerError::new(
                    "missingGraphValue",
                    format!(
                        "graph.nodes.{}.inputs.{}",
                        node.id.as_str(),
                        input_id.as_str()
                    ),
                    format!(
                        "Node `{}` input `{}` requires graph value `{}` but no value was published",
                        node.id.as_str(),
                        input_id.as_str(),
                        id.as_str()
                    ),
                    Some(
                        "Publish the graph value earlier in the topology or author an explicit fallback.",
                    ),
                    serde_json::json!({
                        "node": node.id.as_str(),
                        "input": input_id.as_str(),
                        "graphValue": id.as_str()
                    }),
                ));
            }
            continue;
        };
        let Some(expected) = expected_graph_value_kind(recipe, catalog, node, input_id) else {
            continue;
        };
        let actual = value.kind();
        if actual != expected {
            errors.push(PlayerError::new(
                "graphValueKindMismatch",
                format!(
                    "graph.nodes.{}.inputs.{}",
                    node.id.as_str(),
                    input_id.as_str()
                ),
                format!(
                    "Node `{}` input `{}` expected graph value `{}` to be {} but received {}",
                    node.id.as_str(),
                    input_id.as_str(),
                    id.as_str(),
                    value_kind_label(expected),
                    value_kind_label(actual)
                ),
                Some("Publish a graph value whose kind matches the consuming descriptor input."),
                serde_json::json!({
                    "node": node.id.as_str(),
                    "input": input_id.as_str(),
                    "graphValue": id.as_str(),
                    "expected": value_kind_label(expected),
                    "actual": value_kind_label(actual)
                }),
            ));
        }
    }
}

fn expected_graph_value_kind(
    recipe: &RecipeDocument,
    catalog: Option<&DescriptorCatalog>,
    node: &NodeSpec,
    input_id: &tui_vfx_contract::EffectInputId,
) -> Option<ValueKind> {
    effect_descriptor(recipe, catalog, node).and_then(|descriptor| {
        descriptor
            .inputs
            .get(input_id)
            .map(|input| input.value.kind)
    })
}

fn effect_descriptor<'a>(
    recipe: &'a RecipeDocument,
    catalog: Option<&'a DescriptorCatalog>,
    node: &NodeSpec,
) -> Option<&'a EffectDescriptor> {
    recipe.graph.effects.get(&node.effect).or_else(|| {
        catalog.and_then(|catalog| {
            catalog
                .packs
                .values()
                .find_map(|pack| pack.effects.get(&node.effect))
        })
    })
}

fn value_kind_label(kind: ValueKind) -> &'static str {
    match kind {
        ValueKind::Null => "null",
        ValueKind::Boolean => "boolean",
        ValueKind::Integer => "integer",
        ValueKind::Number => "number",
        ValueKind::String => "string",
        ValueKind::Text => "text",
        ValueKind::Color => "color",
        ValueKind::Gradient => "gradient",
        ValueKind::Duration => "duration",
        ValueKind::Enum => "enum",
        ValueKind::Role => "role",
        ValueKind::Scope => "scope",
        ValueKind::Rect => "rect",
        ValueKind::Structured => "structured",
    }
}

fn apply_node(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    rows: &mut [String],
    styled_grid: &mut PlayerStyledGrid,
    errors: &mut Vec<PlayerError>,
) {
    let sync_text_grid = match node.effect.as_str() {
        "filter.dim"
        | "filter.tint"
        | "filter.invert"
        | "filter.greyscale"
        | "filter.pillButton"
        | "filter.fadeToCanvas"
        | "filter.patternFill"
        | "filter.crt"
        | "filter.matrixRain"
        | "filter.vignette"
        | "filter.bracketEmphasis"
        | "filter.dotIndicator"
        | "filter.edgeGrow"
        | "filter.hoverBar"
        | "filter.kittScanner"
        | "filter.underlineWipe"
        | "filter.subPixelBar" => {
            apply_filter_primitive(node, request, styled_grid);
            false
        }
        "mask.none" => false,
        "content.typewriter"
        | "content.marquee"
        | "content.splitFlap"
        | "content.wrapIndicator"
        | "content.scramble"
        | "content.morph"
        | "content.redact"
        | "content.mirror"
        | "content.numeric"
        | "content.dissolve"
        | "content.odometer"
        | "content.cellMotion"
        | "content.slideShift"
        | "content.glitchShift"
        | "content.scrambleGlitchShift" => {
            apply_content_primitive(node, request, rows);
            true
        }
        "sampler.sineWave" => {
            apply_sampler_sine_wave(node, request, rows);
            true
        }
        "mask.wipe" | "mask.wipeCorner" => {
            apply_mask_wipe(node, request, rows);
            true
        }
        "mask.cellular" => {
            apply_mask_cellular(node, request, rows);
            true
        }
        "mask.checkers" => {
            apply_mask_checkers(node, request, rows);
            true
        }
        "mask.dissolve" | "mask.noiseDither" => {
            apply_mask_dissolve(node, request, rows);
            true
        }
        "mask.blinds" => {
            apply_mask_blinds(node, request, rows);
            true
        }
        "mask.radial" => {
            apply_mask_radial(node, request, rows);
            true
        }
        "mask.materialize" | "mask.materializeCorner" => {
            apply_mask_materialize(node, request, rows);
            true
        }
        "mask.iris" => {
            apply_mask_iris(node, request, rows);
            true
        }
        "mask.diamond" => {
            apply_mask_diamond(node, request, rows);
            true
        }
        "sampler.ripple" => {
            apply_sampler_ripple(node, request, rows);
            true
        }
        "sampler.shredder"
        | "sampler.faultLine"
        | "sampler.radialTwist"
        | "sampler.crt"
        | "sampler.crtJitter" => {
            apply_distortion_sampler_primitive(node, request, rows);
            true
        }
        _ if apply_styled_primitive(node, request, styled_grid) => false,
        effect => {
            errors.push(PlayerError::new(
                "unsupportedEffectAdapter",
                format!("graph.nodes.{}.effect", node.id.as_str()),
                format!("No player adapter registered for {effect}"),
                Some(
                    "Implement the effect adapter or keep this fixture in unsupported smoke status.",
                ),
                serde_json::json!({ "effect": effect, "node": node.id.as_str() }),
            ));
            false
        }
    };
    if sync_text_grid {
        styled_grid.sync_glyphs_from_rows(rows);
    }
}

fn publish_node_outputs(
    node: &NodeSpec,
    request: &mut PlayerSampleRequest,
    errors: &mut Vec<PlayerError>,
) {
    for (value_id, output) in &node.outputs {
        match &output.source {
            NodeOutputSource::Input { id } => {
                let Some(source) = node.inputs.get(id) else {
                    errors.push(PlayerError::new(
                        "missingNodeOutputInput",
                        format!("graph.nodes.{}.outputs.{}", node.id.as_str(), value_id.as_str()),
                        format!(
                            "Node output `{}` references missing input `{}`",
                            value_id.as_str(),
                            id.as_str()
                        ),
                        Some("Publish only inputs authored on the node."),
                        serde_json::json!({ "node": node.id.as_str(), "output": value_id.as_str(), "input": id.as_str() }),
                    ));
                    continue;
                };
                if let Some(value) = resolve_value_source_with_graph_values(
                    source,
                    &request.signals,
                    &request.graph_values,
                ) {
                    request.graph_values.insert(value_id.clone(), value);
                }
            }
            NodeOutputSource::EffectOutput { id } => errors.push(PlayerError::new(
                "unsupportedEffectOutput",
                format!("graph.nodes.{}.outputs.{}", node.id.as_str(), value_id.as_str()),
                format!(
                    "Player adapter does not publish effect output `{}` for node `{}`",
                    id.as_str(),
                    node.id.as_str()
                ),
                Some("Use input re-emission for current player graph evidence, or implement a real effect-output adapter."),
                serde_json::json!({ "node": node.id.as_str(), "output": value_id.as_str(), "effectOutput": id.as_str() }),
            )),
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn execute_parallel_step(
    recipe: &RecipeDocument,
    catalog: Option<&DescriptorCatalog>,
    children: &[GraphStep],
    merge_policy: ParallelMergePolicy,
    value_merge_policy: GraphValueMergePolicy,
    request: &mut PlayerSampleRequest,
    rows: &mut [String],
    styled_grid: &mut PlayerStyledGrid,
    errors: &mut Vec<PlayerError>,
    warnings: &mut Vec<PlayerWarning>,
) {
    let base_rows = rows.to_vec();
    let base_grid = styled_grid.clone();
    let base_values = request.graph_values.clone();
    let mut merged_rows = base_rows.clone();
    let mut merged_grid = base_grid.clone();
    let mut row_writes = BTreeMap::<(usize, usize), usize>::new();
    let mut style_writes = BTreeMap::<(usize, usize), usize>::new();
    let mut value_writes = BTreeMap::<GraphValueId, usize>::new();
    let mut merged_values = BTreeMap::new();

    for (branch_index, child) in children.iter().enumerate() {
        let mut branch_rows = base_rows.clone();
        let mut branch_grid = base_grid.clone();
        let mut branch_request = request.clone();
        branch_request.graph_values = base_values.clone();
        execute_graph_step(
            recipe,
            catalog,
            child,
            &mut branch_request,
            &mut branch_rows,
            &mut branch_grid,
            errors,
            warnings,
        );
        merge_rows(
            &base_rows,
            &branch_rows,
            &mut merged_rows,
            branch_index,
            merge_policy,
            &mut row_writes,
            errors,
            warnings,
        );
        merge_styled_grid(
            &base_grid,
            &branch_grid,
            &mut merged_grid,
            branch_index,
            merge_policy,
            &mut style_writes,
            errors,
            warnings,
        );
        merge_graph_values(
            &base_values,
            &branch_request.graph_values,
            &mut merged_values,
            branch_index,
            value_merge_policy,
            &mut value_writes,
            errors,
            warnings,
        );
    }

    rows.clone_from_slice(&merged_rows);
    *styled_grid = merged_grid;
    request.graph_values.extend(merged_values);
}

#[allow(clippy::too_many_arguments)]
fn merge_rows(
    base_rows: &[String],
    branch_rows: &[String],
    merged_rows: &mut [String],
    branch_index: usize,
    merge_policy: ParallelMergePolicy,
    row_writes: &mut BTreeMap<(usize, usize), usize>,
    errors: &mut Vec<PlayerError>,
    warnings: &mut Vec<PlayerWarning>,
) {
    for (y, branch_row) in branch_rows.iter().enumerate() {
        let base_chars = base_rows
            .get(y)
            .map(|row| row.chars().collect::<Vec<_>>())
            .unwrap_or_default();
        let mut merged_chars = merged_rows[y].chars().collect::<Vec<_>>();
        for (x, branch_char) in branch_row.chars().enumerate() {
            if base_chars.get(x) == Some(&branch_char) {
                continue;
            }
            if let Some(previous) = row_writes.insert((x, y), branch_index) {
                push_surface_conflict(
                    merge_policy,
                    branch_index,
                    previous,
                    x,
                    y,
                    "glyph",
                    errors,
                    warnings,
                );
            }
            if x < merged_chars.len() {
                merged_chars[x] = branch_char;
            }
        }
        merged_rows[y] = merged_chars.into_iter().collect();
    }
}

#[allow(clippy::too_many_arguments)]
fn merge_styled_grid(
    base_grid: &PlayerStyledGrid,
    branch_grid: &PlayerStyledGrid,
    merged_grid: &mut PlayerStyledGrid,
    branch_index: usize,
    merge_policy: ParallelMergePolicy,
    style_writes: &mut BTreeMap<(usize, usize), usize>,
    errors: &mut Vec<PlayerError>,
    warnings: &mut Vec<PlayerWarning>,
) {
    for (base, branch) in base_grid.cells().iter().zip(branch_grid.cells()) {
        if base == branch {
            continue;
        }
        if let Some(previous) = style_writes.insert((branch.x, branch.y), branch_index) {
            push_surface_conflict(
                merge_policy,
                branch_index,
                previous,
                branch.x,
                branch.y,
                "style",
                errors,
                warnings,
            );
        }
        merged_grid.set_cell_style(
            branch.x,
            branch.y,
            &branch.foreground,
            &branch.background,
            branch.modifiers.clone(),
            branch.role.clone(),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn merge_graph_values(
    base_values: &BTreeMap<GraphValueId, Value>,
    branch_values: &BTreeMap<GraphValueId, Value>,
    merged_values: &mut BTreeMap<GraphValueId, Value>,
    branch_index: usize,
    value_merge_policy: GraphValueMergePolicy,
    value_writes: &mut BTreeMap<GraphValueId, usize>,
    errors: &mut Vec<PlayerError>,
    warnings: &mut Vec<PlayerWarning>,
) {
    let changed = branch_values
        .iter()
        .filter(|(id, value)| base_values.get(*id) != Some(*value))
        .map(|(id, value)| (id.clone(), value.clone()))
        .collect::<Vec<_>>();
    for (id, value) in changed {
        if let Some(previous) = value_writes.insert(id.clone(), branch_index) {
            match value_merge_policy {
                GraphValueMergePolicy::ChildOrderLastWriterWins => warnings.push(
                    PlayerWarning::new(
                        "parallelGraphValueConflict",
                        "graph.topology",
                        format!(
                            "Parallel branches {previous} and {branch_index} both published graph value `{}`; later branch wins",
                            id.as_str()
                        ),
                        Some("Use explicit branch-local value ids or an error-on-conflict policy when this is not intentional."),
                    ),
                ),
                GraphValueMergePolicy::ErrorOnSameValueConflict => errors.push(PlayerError::new(
                    "parallelGraphValueConflict",
                    "graph.topology",
                    format!(
                        "Parallel branches {previous} and {branch_index} both published graph value `{}`",
                        id.as_str()
                    ),
                    Some("Rename one graph value or use childOrderLastWriterWins when deterministic overwrite is intended."),
                    serde_json::json!({ "graphValue": id.as_str(), "firstBranch": previous, "secondBranch": branch_index }),
                )),
            }
        }
        merged_values.insert(id, value);
    }
}

#[allow(clippy::too_many_arguments)]
fn push_surface_conflict(
    merge_policy: ParallelMergePolicy,
    branch_index: usize,
    previous: usize,
    x: usize,
    y: usize,
    channel: &str,
    errors: &mut Vec<PlayerError>,
    warnings: &mut Vec<PlayerWarning>,
) {
    match merge_policy {
        ParallelMergePolicy::ChildOrderLastWriterWins => warnings.push(PlayerWarning::new(
            "parallelSurfaceConflict",
            "graph.topology",
            format!(
                "Parallel branches {previous} and {branch_index} both wrote {channel} at ({x},{y}); later branch wins"
            ),
            Some("Use disjoint scopes or an error-on-conflict merge policy when overlap is not intentional."),
        )),
        ParallelMergePolicy::ErrorOnSameChannelConflict => errors.push(PlayerError::new(
            "parallelSurfaceConflict",
            "graph.topology",
            format!(
                "Parallel branches {previous} and {branch_index} both wrote {channel} at ({x},{y})"
            ),
            Some("Use disjoint scopes or childOrderLastWriterWins when deterministic overwrite is intended."),
            serde_json::json!({ "x": x, "y": y, "channel": channel, "firstBranch": previous, "secondBranch": branch_index }),
        )),
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_graph_effects.rs</FILE> - <DESC>Apply graph effect adapters to player frame evidence</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
