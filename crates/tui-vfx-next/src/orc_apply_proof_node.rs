// <FILE>crates/tui-vfx-next/src/orc_apply_proof_node.rs</FILE> - <DESC>Apply one proof graph node adapter</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase G4: return proof effect outputs and consume spatial fields.</WCTX>
// <CLOG>0.3.0: MINOR — produce effect outputs and handle field-driven dim inputs.
// 0.2.0: MINOR — apply foreground/background-only proof nodes.
// 0.1.0: INIT — apply copy, replace-glyph, dim, and explicit-role proof adapters.</CLOG>

use std::collections::BTreeMap;

use crate::{
    ApplyOutcome, CellWrite, CellWritePolicy, CoordinateSpace, EffectId, EffectInputId,
    EffectOutputId, GraphExecutionError, NodeSpec, NumberCellField, ProofEffectAdapter, ProofValue,
    RoleSpace, RoleWritePolicy, ScopeSpec, Surface, SurfaceEngine,
    fnc_apply_dim_with_number_field::apply_dim_with_number_field,
    fnc_read_proof_input::{ProofNumberInput, input_char, input_color, input_number, input_role},
};

pub(crate) fn apply_proof_node(
    adapter: ProofEffectAdapter,
    effect: &EffectId,
    node: &NodeSpec,
    inputs: &BTreeMap<EffectInputId, ProofValue>,
    read: &Surface,
    next: &mut Surface,
) -> Result<ProofNodeApplication, GraphExecutionError> {
    let outcome = match adapter {
        ProofEffectAdapter::Copy => apply_copy_node(node, read, next),
        ProofEffectAdapter::ReplaceGlyph => {
            apply_replace_glyph_node(effect, node, inputs, read, next)?
        }
        ProofEffectAdapter::Dim => apply_dim_node(effect, node, inputs, read, next)?,
        ProofEffectAdapter::ExplicitRoleWrite => {
            apply_explicit_role_write_node(effect, node, inputs, read, next)?
        }
        ProofEffectAdapter::SetForeground => {
            apply_color_node(effect, node, inputs, read, next, true)?
        }
        ProofEffectAdapter::SetBackground => {
            apply_color_node(effect, node, inputs, read, next, false)?
        }
        ProofEffectAdapter::ConsumeNumber => {
            let _ = input_number(effect, inputs, "factor")?;
            ApplyOutcome::default()
        }
        ProofEffectAdapter::SpatialScalarField => {
            return Ok(spatial_scalar_field_application(read));
        }
    };
    Ok(ProofNodeApplication {
        outcome,
        effect_outputs: BTreeMap::new(),
    })
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
    inputs: &BTreeMap<EffectInputId, ProofValue>,
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
    inputs: &BTreeMap<EffectInputId, ProofValue>,
    read: &Surface,
    next: &mut Surface,
) -> Result<ApplyOutcome, GraphExecutionError> {
    let factor = input_number(effect, inputs, "factor")?;
    let scope = node_scope(node);
    let cell_policy = node.cell_write_policy.unwrap_or(CellWritePolicy::WriteCell);
    let role_policy = node
        .role_write_policy
        .clone()
        .unwrap_or(RoleWritePolicy::PreserveDestination);
    match factor {
        ProofNumberInput::Frame(factor) => {
            let factor = factor as f32;
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
        ProofNumberInput::Field(field) => Ok(apply_dim_with_number_field(
            read,
            next,
            &scope,
            field,
            cell_policy,
            role_policy,
        )),
    }
}

fn apply_explicit_role_write_node(
    effect: &EffectId,
    node: &NodeSpec,
    inputs: &BTreeMap<EffectInputId, ProofValue>,
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

fn apply_color_node(
    effect: &EffectId,
    node: &NodeSpec,
    inputs: &BTreeMap<EffectInputId, ProofValue>,
    read: &Surface,
    next: &mut Surface,
    foreground: bool,
) -> Result<ApplyOutcome, GraphExecutionError> {
    let color = input_color(effect, inputs, "color")?;
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
            if foreground {
                cell.fg = color;
            } else {
                cell.bg = color;
            }
            CellWrite {
                cell,
                cell_policy,
                role_policy: role_policy.clone(),
            }
        },
    ))
}

#[derive(Clone, Debug)]
pub(crate) struct ProofNodeApplication {
    pub(crate) outcome: ApplyOutcome,
    pub(crate) effect_outputs: BTreeMap<EffectOutputId, ProofValue>,
}

fn spatial_scalar_field_application(read: &Surface) -> ProofNodeApplication {
    let field = NumberCellField::normalized_x(read.width(), read.height());
    ProofNodeApplication {
        outcome: ApplyOutcome {
            matched_cells: read.width() * read.height(),
            written_cells: 0,
            diagnostics: vec![],
        },
        effect_outputs: BTreeMap::from([(
            EffectOutputId::new("value"),
            ProofValue::NumberCellField(field),
        )]),
    }
}

fn node_scope(node: &NodeSpec) -> ScopeSpec {
    node.scope.clone().unwrap_or(ScopeSpec::All)
}

// <FILE>crates/tui-vfx-next/src/orc_apply_proof_node.rs</FILE> - <DESC>Apply one proof graph node adapter</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
