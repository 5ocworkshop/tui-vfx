// <FILE>crates/tui-vfx-next/src/fnc_apply_proof_node.rs</FILE> - <DESC>Apply one proof graph node adapter</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G2: keep proof adapter execution separate from graph orchestration.</WCTX>
// <CLOG>0.1.0: INIT — apply copy, replace-glyph, dim, and explicit-role proof adapters.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_types::RoleTag;

use crate::{
    ApplyOutcome, CellWrite, CellWritePolicy, CoordinateSpace, EffectId, EffectInputId,
    GraphExecutionError, NodeSpec, ProofEffectAdapter, RoleSpace, RoleWritePolicy, ScopeSpec,
    Surface, SurfaceEngine, Value, ValueKind,
};

pub(crate) fn apply_proof_node(
    adapter: ProofEffectAdapter,
    effect: &EffectId,
    node: &NodeSpec,
    inputs: &BTreeMap<EffectInputId, Value>,
    read: &Surface,
    next: &mut Surface,
) -> Result<ApplyOutcome, GraphExecutionError> {
    match adapter {
        ProofEffectAdapter::Copy => Ok(apply_copy_node(node, read, next)),
        ProofEffectAdapter::ReplaceGlyph => {
            apply_replace_glyph_node(effect, node, inputs, read, next)
        }
        ProofEffectAdapter::Dim => apply_dim_node(effect, node, inputs, read, next),
        ProofEffectAdapter::ExplicitRoleWrite => {
            apply_explicit_role_write_node(effect, node, inputs, read, next)
        }
    }
}

fn apply_copy_node(node: &NodeSpec, read: &Surface, next: &mut Surface) -> ApplyOutcome {
    let scope = node_scope(node);
    let cell_policy = node.cell_write_policy.unwrap_or(CellWritePolicy::WriteCell);
    let role_policy = node
        .role_write_policy
        .clone()
        .unwrap_or(RoleWritePolicy::CopySampledSource);
    SurfaceEngine::apply_from_source(
        read,
        next,
        &scope,
        CoordinateSpace::default(),
        RoleSpace::default(),
        move |cell, _role| CellWrite {
            cell,
            cell_policy,
            role_policy: role_policy.clone(),
        },
    )
}

fn apply_replace_glyph_node(
    effect: &EffectId,
    node: &NodeSpec,
    inputs: &BTreeMap<EffectInputId, Value>,
    read: &Surface,
    next: &mut Surface,
) -> Result<ApplyOutcome, GraphExecutionError> {
    let glyph = input_char(effect, inputs, "glyph")?;
    let scope = node_scope(node);
    let cell_policy = node.cell_write_policy.unwrap_or(CellWritePolicy::WriteCell);
    let role_policy = node
        .role_write_policy
        .clone()
        .unwrap_or(RoleWritePolicy::CopySampledSource);
    Ok(SurfaceEngine::apply_from_source(
        read,
        next,
        &scope,
        CoordinateSpace::default(),
        RoleSpace::default(),
        move |mut cell, _role| {
            cell.ch = glyph;
            CellWrite {
                cell,
                cell_policy,
                role_policy: role_policy.clone(),
            }
        },
    ))
}

fn apply_dim_node(
    effect: &EffectId,
    node: &NodeSpec,
    inputs: &BTreeMap<EffectInputId, Value>,
    read: &Surface,
    next: &mut Surface,
) -> Result<ApplyOutcome, GraphExecutionError> {
    let factor = input_number(effect, inputs, "factor")? as f32;
    let scope = node_scope(node);
    let cell_policy = node.cell_write_policy.unwrap_or(CellWritePolicy::WriteCell);
    let role_policy = node
        .role_write_policy
        .clone()
        .unwrap_or(RoleWritePolicy::PreserveDestination);
    Ok(SurfaceEngine::apply_from_source(
        read,
        next,
        &scope,
        CoordinateSpace::default(),
        RoleSpace::default(),
        move |mut cell, _role| {
            cell.fg = cell.fg.dim(factor);
            cell.bg = cell.bg.dim(factor);
            CellWrite {
                cell,
                cell_policy,
                role_policy: role_policy.clone(),
            }
        },
    ))
}

fn apply_explicit_role_write_node(
    effect: &EffectId,
    node: &NodeSpec,
    inputs: &BTreeMap<EffectInputId, Value>,
    read: &Surface,
    next: &mut Surface,
) -> Result<ApplyOutcome, GraphExecutionError> {
    let role = input_role(effect, inputs, "role")?;
    let scope = node_scope(node);
    let cell_policy = node.cell_write_policy.unwrap_or(CellWritePolicy::WriteCell);
    Ok(SurfaceEngine::apply_from_source(
        read,
        next,
        &scope,
        CoordinateSpace::default(),
        RoleSpace::Destination,
        move |cell, _role| CellWrite {
            cell,
            cell_policy,
            role_policy: RoleWritePolicy::SetExplicit { role: role.clone() },
        },
    ))
}

fn node_scope(node: &NodeSpec) -> ScopeSpec {
    node.scope.clone().unwrap_or(ScopeSpec::All)
}

fn input_value<'a>(
    effect: &EffectId,
    inputs: &'a BTreeMap<EffectInputId, Value>,
    input: &str,
) -> Result<&'a Value, GraphExecutionError> {
    let input_id = EffectInputId::new(input);
    inputs
        .get(&input_id)
        .ok_or_else(|| GraphExecutionError::MissingProofInput {
            effect: effect.clone(),
            input: input_id,
        })
}

fn input_char(
    effect: &EffectId,
    inputs: &BTreeMap<EffectInputId, Value>,
    input: &str,
) -> Result<char, GraphExecutionError> {
    match input_value(effect, inputs, input)? {
        Value::Text(value) | Value::String(value) => Ok(value.chars().next().unwrap_or(' ')),
        value => Err(unsupported_input(
            effect,
            input,
            ValueKind::Text,
            value.kind(),
        )),
    }
}

fn input_number(
    effect: &EffectId,
    inputs: &BTreeMap<EffectInputId, Value>,
    input: &str,
) -> Result<f64, GraphExecutionError> {
    let value = input_value(effect, inputs, input)?;
    value
        .as_range_number()
        .ok_or_else(|| unsupported_input(effect, input, ValueKind::Number, value.kind()))
}

fn input_role(
    effect: &EffectId,
    inputs: &BTreeMap<EffectInputId, Value>,
    input: &str,
) -> Result<RoleTag, GraphExecutionError> {
    match input_value(effect, inputs, input)? {
        Value::Role(role) => Ok(role.clone()),
        value => Err(unsupported_input(
            effect,
            input,
            ValueKind::Role,
            value.kind(),
        )),
    }
}

fn unsupported_input(
    effect: &EffectId,
    input: &str,
    expected: ValueKind,
    actual: ValueKind,
) -> GraphExecutionError {
    GraphExecutionError::UnsupportedProofInput {
        effect: effect.clone(),
        input: EffectInputId::new(input),
        expected,
        actual,
    }
}

// <FILE>crates/tui-vfx-next/src/fnc_apply_proof_node.rs</FILE> - <DESC>Apply one proof graph node adapter</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
