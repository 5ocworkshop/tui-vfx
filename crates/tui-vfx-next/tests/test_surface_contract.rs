// <FILE>crates/tui-vfx-next/tests/test_surface_contract.rs</FILE> - <DESC>Phase A/B/C v3.1 surface contract tests</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>New kernel Phase D0 verifier fix: update tests for strict named-field schema-visible enum variants.</WCTX>
// <CLOG>0.5.0: TEST — update ScopeSpec constructors for strict schema-visible named-field variants.
// 0.4.2: TEST — make cell/role/skip/sampled-role pipeline tests depend on Stage 1 mutations, not initial surface state.</CLOG>

use tui_vfx_next::{
    CellWrite, CellWritePolicy, CoordinateSpace, DimEffect, ExplicitRoleWriteEffect, PipelineStage,
    RoleSpace, RoleWritePolicy, ScopeSpec, ShiftSampler, Surface, SurfaceDiagnosticCode,
    SurfaceEngine, SurfacePipeline,
};
use tui_vfx_types::{Cell, Color, Modifiers, Rect, RoleTag};

fn cell(ch: char) -> Cell {
    Cell::styled(ch, Color::WHITE, Color::BLACK, Modifiers::NONE)
}

fn surface(width: usize, height: usize, role: RoleTag) -> Surface {
    Surface::new(width, height, role)
}

#[test]
fn copy_preserves_sampled_source_roles() {
    let mut source = surface(2, 1, RoleTag::Background);
    source.set_cell_and_role(0, 0, cell('A'), RoleTag::Text);
    source.set_cell_and_role(1, 0, cell('B'), RoleTag::Border);
    let mut destination = surface(2, 1, RoleTag::Highlight);

    let outcome = SurfaceEngine::copy(&source, &mut destination, &ScopeSpec::All);

    assert_eq!(outcome.written_cells, 2);
    assert_eq!(destination.cell(0, 0).unwrap().ch, 'A');
    assert_eq!(destination.role(0, 0), Some(&RoleTag::Text));
    assert_eq!(destination.cell(1, 0).unwrap().ch, 'B');
    assert_eq!(destination.role(1, 0), Some(&RoleTag::Border));
}

#[test]
fn visual_effect_preserves_roles() {
    let mut source = surface(1, 1, RoleTag::Text);
    source.set_cell_and_role(0, 0, cell('A'), RoleTag::Text);
    let mut destination = surface(1, 1, RoleTag::Border);

    let outcome = SurfaceEngine::apply_dim(
        &source,
        &mut destination,
        &ScopeSpec::All,
        DimEffect::new(0.5),
    );

    assert_eq!(outcome.written_cells, 1);
    assert_eq!(destination.cell(0, 0).unwrap().ch, 'A');
    assert_eq!(destination.cell(0, 0).unwrap().fg, Color::gray(128));
    assert_eq!(destination.role(0, 0), Some(&RoleTag::Border));
}

#[test]
fn role_scope_affects_only_matching_roles() {
    let mut source = surface(3, 1, RoleTag::Background);
    source.set_cell_and_role(0, 0, cell('T'), RoleTag::Text);
    source.set_cell_and_role(1, 0, cell('B'), RoleTag::Border);
    source.set_cell_and_role(2, 0, cell('G'), RoleTag::Background);
    let mut destination = surface(3, 1, RoleTag::Highlight);
    destination.set_cell(0, 0, cell('x'));
    destination.set_cell(1, 0, cell('y'));
    destination.set_cell(2, 0, cell('z'));

    let outcome = SurfaceEngine::copy(
        &source,
        &mut destination,
        &ScopeSpec::Role {
            role: RoleTag::Text,
        },
    );

    assert_eq!(outcome.matched_cells, 1);
    assert_eq!(destination.cell(0, 0).unwrap().ch, 'T');
    assert_eq!(destination.role(0, 0), Some(&RoleTag::Text));
    assert_eq!(destination.cell(1, 0).unwrap().ch, 'y');
    assert_eq!(destination.role(1, 0), Some(&RoleTag::Highlight));
    assert_eq!(destination.cell(2, 0).unwrap().ch, 'z');
    assert_eq!(destination.role(2, 0), Some(&RoleTag::Highlight));
}

#[test]
fn skipped_cells_preserve_destination_cell_and_role() {
    let mut source = surface(2, 1, RoleTag::Text);
    source.set_cell_and_role(0, 0, cell('A'), RoleTag::Text);
    source.set_cell_and_role(1, 0, cell('B'), RoleTag::Text);
    let mut destination = surface(2, 1, RoleTag::Border);
    destination.set_cell(0, 0, cell('x'));
    destination.set_cell(1, 0, cell('y'));

    let outcome = SurfaceEngine::copy(
        &source,
        &mut destination,
        &ScopeSpec::Rect {
            rect: Rect::new(0, 0, 1, 1),
        },
    );

    assert_eq!(outcome.written_cells, 1);
    assert_eq!(destination.cell(0, 0).unwrap().ch, 'A');
    assert_eq!(destination.role(0, 0), Some(&RoleTag::Text));
    assert_eq!(destination.cell(1, 0).unwrap().ch, 'y');
    assert_eq!(destination.role(1, 0), Some(&RoleTag::Border));
}

#[test]
fn zero_cell_scope_emits_diagnostic() {
    let source = surface(1, 1, RoleTag::Text);
    let mut destination = surface(1, 1, RoleTag::Border);
    destination.set_cell(0, 0, cell('D'));

    let outcome = SurfaceEngine::copy(
        &source,
        &mut destination,
        &ScopeSpec::Role {
            role: RoleTag::Shadow,
        },
    );

    assert_eq!(outcome.matched_cells, 0);
    assert_eq!(outcome.written_cells, 0);
    assert_eq!(outcome.diagnostics.len(), 1);
    assert_eq!(
        outcome.diagnostics[0].code,
        SurfaceDiagnosticCode::ZeroCellScope
    );
    assert_eq!(destination.cell(0, 0).unwrap().ch, 'D');
    assert_eq!(destination.role(0, 0), Some(&RoleTag::Border));
}

#[test]
fn explicit_role_write_sets_role() {
    let mut destination = surface(2, 1, RoleTag::Text);
    destination.set_cell(0, 0, cell('a'));
    destination.set_cell(1, 0, cell('b'));
    let effect = ExplicitRoleWriteEffect::new(cell('░'), RoleTag::Shadow);

    let outcome = SurfaceEngine::apply_explicit_role_write(
        &mut destination,
        &ScopeSpec::ColumnRange { start: 1, end: 2 },
        &effect,
    );

    assert_eq!(outcome.written_cells, 1);
    assert_eq!(destination.cell(0, 0).unwrap().ch, 'a');
    assert_eq!(destination.role(0, 0), Some(&RoleTag::Text));
    assert_eq!(destination.cell(1, 0).unwrap().ch, '░');
    assert_eq!(destination.role(1, 0), Some(&RoleTag::Shadow));
}

#[test]
fn empty_transparent_cell_is_not_the_same_as_skip() {
    let mut source = surface(1, 1, RoleTag::Text);
    source.set_cell_and_role(0, 0, Cell::default(), RoleTag::Text);

    let mut write_empty = surface(1, 1, RoleTag::Border);
    write_empty.set_cell(0, 0, cell('D'));
    let write_outcome = SurfaceEngine::copy(&source, &mut write_empty, &ScopeSpec::All);

    let mut skip_empty = surface(1, 1, RoleTag::Border);
    skip_empty.set_cell(0, 0, cell('D'));
    let skip_outcome = SurfaceEngine::apply_from_source(
        &source,
        &mut skip_empty,
        &ScopeSpec::All,
        Default::default(),
        RoleSpace::default(),
        |sampled_cell, _sampled_role| CellWrite {
            cell: sampled_cell,
            cell_policy: CellWritePolicy::SkipTransparentEmpty,
            role_policy: tui_vfx_next::RoleWritePolicy::CopySampledSource,
        },
    );

    assert_eq!(write_outcome.written_cells, 1);
    assert!(write_empty.cell(0, 0).unwrap().is_empty());
    assert_eq!(write_empty.role(0, 0), Some(&RoleTag::Text));

    assert_eq!(skip_outcome.matched_cells, 1);
    assert_eq!(skip_outcome.written_cells, 0);
    assert_eq!(skip_empty.cell(0, 0).unwrap().ch, 'D');
    assert_eq!(skip_empty.role(0, 0), Some(&RoleTag::Border));
}

#[test]
fn scope_role_space_defaults_to_sampled_source() {
    let mut source = surface(1, 1, RoleTag::Text);
    source.set_cell_and_role(0, 0, cell('S'), RoleTag::Text);
    let mut destination = surface(1, 1, RoleTag::Border);
    destination.set_cell(0, 0, cell('D'));

    let outcome = SurfaceEngine::copy(
        &source,
        &mut destination,
        &ScopeSpec::Role {
            role: RoleTag::Text,
        },
    );

    assert_eq!(outcome.written_cells, 1);
    assert_eq!(destination.cell(0, 0).unwrap().ch, 'S');
    assert_eq!(destination.role(0, 0), Some(&RoleTag::Text));
}

#[test]
fn shift_sampler_copies_sampled_source_cell() {
    let mut source = surface(3, 1, RoleTag::Background);
    source.set_cell_and_role(0, 0, cell('A'), RoleTag::Text);
    source.set_cell_and_role(1, 0, cell('B'), RoleTag::Border);
    source.set_cell_and_role(2, 0, cell('C'), RoleTag::Title);
    let mut destination = surface(3, 1, RoleTag::Highlight);
    destination.set_cell(0, 0, cell('x'));
    destination.set_cell(1, 0, cell('y'));
    destination.set_cell(2, 0, cell('z'));

    let outcome = SurfaceEngine::copy_with_sampler(
        &source,
        &mut destination,
        &ScopeSpec::All,
        &ShiftSampler::new(1, 0),
    );

    assert_eq!(outcome.matched_cells, 2);
    assert_eq!(outcome.written_cells, 2);
    assert_eq!(destination.cell(0, 0).unwrap().ch, 'B');
    assert_eq!(destination.cell(1, 0).unwrap().ch, 'C');
    assert_eq!(destination.cell(2, 0).unwrap().ch, 'z');
}

#[test]
fn shift_sampler_copies_sampled_source_role() {
    let mut source = surface(2, 1, RoleTag::Background);
    source.set_cell_and_role(0, 0, cell('A'), RoleTag::Text);
    source.set_cell_and_role(1, 0, cell('B'), RoleTag::Title);
    let mut destination = surface(2, 1, RoleTag::Highlight);
    destination.set_cell(0, 0, cell('x'));
    destination.set_cell(1, 0, cell('y'));

    let outcome = SurfaceEngine::copy_with_sampler(
        &source,
        &mut destination,
        &ScopeSpec::All,
        &ShiftSampler::new(1, 0),
    );

    assert_eq!(outcome.written_cells, 1);
    assert_eq!(destination.cell(0, 0).unwrap().ch, 'B');
    assert_eq!(destination.role(0, 0), Some(&RoleTag::Title));
    assert_eq!(destination.cell(1, 0).unwrap().ch, 'y');
    assert_eq!(destination.role(1, 0), Some(&RoleTag::Highlight));
}

#[test]
fn role_scope_uses_sampled_source_role_with_shift() {
    let mut source = surface(2, 1, RoleTag::Background);
    source.set_cell_and_role(0, 0, cell('B'), RoleTag::Border);
    source.set_cell_and_role(1, 0, cell('T'), RoleTag::Text);
    let mut destination = surface(2, 1, RoleTag::Border);
    destination.set_cell(0, 0, cell('x'));
    destination.set_cell(1, 0, cell('y'));

    let outcome = SurfaceEngine::copy_with_sampler(
        &source,
        &mut destination,
        &ScopeSpec::Role {
            role: RoleTag::Text,
        },
        &ShiftSampler::new(1, 0),
    );

    assert_eq!(outcome.matched_cells, 1);
    assert_eq!(outcome.written_cells, 1);
    assert_eq!(destination.cell(0, 0).unwrap().ch, 'T');
    assert_eq!(destination.role(0, 0), Some(&RoleTag::Text));
    assert_eq!(destination.cell(1, 0).unwrap().ch, 'y');
    assert_eq!(destination.role(1, 0), Some(&RoleTag::Border));
}

#[test]
fn geometry_scope_uses_destination_local_with_shift() {
    let mut source = surface(2, 1, RoleTag::Text);
    source.set_cell_and_role(0, 0, cell('A'), RoleTag::Text);
    source.set_cell_and_role(1, 0, cell('B'), RoleTag::Text);
    let mut destination = surface(2, 1, RoleTag::Highlight);
    destination.set_cell(0, 0, cell('x'));
    destination.set_cell(1, 0, cell('y'));

    let outcome = SurfaceEngine::copy_with_sampler(
        &source,
        &mut destination,
        &ScopeSpec::Rect {
            rect: Rect::new(0, 0, 1, 1),
        },
        &ShiftSampler::new(1, 0),
    );

    assert_eq!(outcome.matched_cells, 1);
    assert_eq!(outcome.written_cells, 1);
    assert_eq!(destination.cell(0, 0).unwrap().ch, 'B');
    assert_eq!(destination.role(0, 0), Some(&RoleTag::Text));
    assert_eq!(destination.cell(1, 0).unwrap().ch, 'y');
    assert_eq!(destination.role(1, 0), Some(&RoleTag::Highlight));
}

#[test]
fn out_of_bounds_sample_preserves_destination() {
    let mut source = surface(2, 1, RoleTag::Text);
    source.set_cell_and_role(0, 0, cell('A'), RoleTag::Text);
    source.set_cell_and_role(1, 0, cell('B'), RoleTag::Title);
    let mut destination = surface(2, 1, RoleTag::Highlight);
    destination.set_cell(0, 0, cell('x'));
    destination.set_cell(1, 0, cell('y'));

    let outcome = SurfaceEngine::copy_with_sampler(
        &source,
        &mut destination,
        &ScopeSpec::All,
        &ShiftSampler::new(1, 0),
    );

    assert_eq!(outcome.matched_cells, 1);
    assert_eq!(outcome.written_cells, 1);
    assert_eq!(destination.cell(0, 0).unwrap().ch, 'B');
    assert_eq!(destination.role(0, 0), Some(&RoleTag::Title));
    assert_eq!(destination.cell(1, 0).unwrap().ch, 'y');
    assert_eq!(destination.role(1, 0), Some(&RoleTag::Highlight));
}

#[test]
fn zero_cell_scope_with_sampler_emits_diagnostic() {
    let mut source = surface(2, 1, RoleTag::Background);
    source.set_cell_and_role(0, 0, cell('A'), RoleTag::Border);
    source.set_cell_and_role(1, 0, cell('B'), RoleTag::Background);
    let mut destination = surface(2, 1, RoleTag::Highlight);
    destination.set_cell(0, 0, cell('x'));
    destination.set_cell(1, 0, cell('y'));

    let outcome = SurfaceEngine::copy_with_sampler(
        &source,
        &mut destination,
        &ScopeSpec::Role {
            role: RoleTag::Text,
        },
        &ShiftSampler::new(1, 0),
    );

    assert_eq!(outcome.matched_cells, 0);
    assert_eq!(outcome.written_cells, 0);
    assert_eq!(outcome.diagnostics.len(), 1);
    assert_eq!(
        outcome.diagnostics[0].code,
        SurfaceDiagnosticCode::ZeroCellScope
    );
    assert_eq!(destination.cell(0, 0).unwrap().ch, 'x');
    assert_eq!(destination.role(0, 0), Some(&RoleTag::Highlight));
    assert_eq!(destination.cell(1, 0).unwrap().ch, 'y');
    assert_eq!(destination.role(1, 0), Some(&RoleTag::Highlight));
}

#[test]
fn destination_role_space_can_be_selected() {
    let mut source = surface(2, 1, RoleTag::Background);
    source.set_cell_and_role(0, 0, cell('A'), RoleTag::Background);
    source.set_cell_and_role(1, 0, cell('T'), RoleTag::Text);
    let sampler = ShiftSampler::new(1, 0);

    let mut sampled_role_destination = surface(2, 1, RoleTag::Border);
    sampled_role_destination.set_cell(0, 0, cell('x'));
    sampled_role_destination.set_cell(1, 0, cell('y'));
    let sampled_outcome = SurfaceEngine::apply_from_source_with_sampler(
        &source,
        &mut sampled_role_destination,
        &ScopeSpec::Role {
            role: RoleTag::Border,
        },
        CoordinateSpace::default(),
        RoleSpace::SampledSource,
        &sampler,
        |sampled_cell, _sampled_role| CellWrite {
            cell: sampled_cell,
            cell_policy: CellWritePolicy::WriteCell,
            role_policy: RoleWritePolicy::CopySampledSource,
        },
    );

    let mut destination_role_destination = surface(2, 1, RoleTag::Border);
    destination_role_destination.set_cell(0, 0, cell('x'));
    destination_role_destination.set_cell(1, 0, cell('y'));
    let destination_outcome = SurfaceEngine::apply_from_source_with_sampler(
        &source,
        &mut destination_role_destination,
        &ScopeSpec::Role {
            role: RoleTag::Border,
        },
        CoordinateSpace::default(),
        RoleSpace::Destination,
        &sampler,
        |sampled_cell, _sampled_role| CellWrite {
            cell: sampled_cell,
            cell_policy: CellWritePolicy::WriteCell,
            role_policy: RoleWritePolicy::CopySampledSource,
        },
    );

    assert_eq!(sampled_outcome.written_cells, 0);
    assert_eq!(sampled_role_destination.cell(0, 0).unwrap().ch, 'x');
    assert_eq!(sampled_role_destination.role(0, 0), Some(&RoleTag::Border));

    assert_eq!(destination_outcome.written_cells, 1);
    assert_eq!(destination_role_destination.cell(0, 0).unwrap().ch, 'T');
    assert_eq!(
        destination_role_destination.role(0, 0),
        Some(&RoleTag::Text)
    );
    assert_eq!(destination_role_destination.cell(1, 0).unwrap().ch, 'y');
    assert_eq!(
        destination_role_destination.role(1, 0),
        Some(&RoleTag::Border)
    );
}

#[test]
fn transparent_empty_sample_write_is_not_skip() {
    let mut source = surface(2, 1, RoleTag::Background);
    source.set_cell_and_role(0, 0, cell('A'), RoleTag::Background);
    source.set_cell_and_role(1, 0, Cell::default(), RoleTag::Text);
    let mut destination = surface(2, 1, RoleTag::Border);
    destination.set_cell(0, 0, cell('D'));
    destination.set_cell(1, 0, cell('E'));

    let outcome = SurfaceEngine::copy_with_sampler(
        &source,
        &mut destination,
        &ScopeSpec::All,
        &ShiftSampler::new(1, 0),
    );

    assert_eq!(outcome.matched_cells, 1);
    assert_eq!(outcome.written_cells, 1);
    assert!(destination.cell(0, 0).unwrap().is_empty());
    assert_eq!(destination.role(0, 0), Some(&RoleTag::Text));
    assert_eq!(destination.cell(1, 0).unwrap().ch, 'E');
    assert_eq!(destination.role(1, 0), Some(&RoleTag::Border));
}

#[test]
fn pipeline_later_stage_reads_earlier_stage_cells() {
    let mut source = surface(1, 1, RoleTag::Background);
    source.set_cell_and_role(0, 0, cell('x'), RoleTag::Background);

    let outcome = SurfacePipeline::new()
        .then(PipelineStage::explicit_role_write(
            "write-stage-cell",
            ScopeSpec::All,
            ExplicitRoleWriteEffect::new(cell('a'), RoleTag::Text),
        ))
        .then(PipelineStage::replace_glyph(
            "rewrite-stage-output",
            ScopeSpec::All,
            'a',
            'Z',
        ))
        .run(&source);

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'Z');
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Text));
}

#[test]
fn pipeline_later_stage_reads_earlier_stage_roles() {
    let mut source = surface(1, 1, RoleTag::Background);
    source.set_cell_and_role(0, 0, cell('S'), RoleTag::Background);

    let outcome = SurfacePipeline::new()
        .then(PipelineStage::explicit_role_write(
            "write-text-role",
            ScopeSpec::All,
            ExplicitRoleWriteEffect::new(cell('T'), RoleTag::Text),
        ))
        .then(PipelineStage::explicit_role_write(
            "role-scoped-shadow",
            ScopeSpec::Role {
                role: RoleTag::Text,
            },
            ExplicitRoleWriteEffect::new(cell('░'), RoleTag::Shadow),
        ))
        .run(&source);

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, '░');
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Shadow));
}

#[test]
fn pipeline_stage_order_is_semantic() {
    let mut source = surface(1, 1, RoleTag::Text);
    source.set_cell_and_role(0, 0, cell('a'), RoleTag::Text);

    let role_then_rewrite = SurfacePipeline::new()
        .then(PipelineStage::explicit_role_write(
            "mark-shadow",
            ScopeSpec::All,
            ExplicitRoleWriteEffect::new(cell('a'), RoleTag::Shadow),
        ))
        .then(PipelineStage::replace_glyph(
            "rewrite-shadow",
            ScopeSpec::Role {
                role: RoleTag::Shadow,
            },
            'a',
            'Z',
        ))
        .run(&source);
    let rewrite_then_role = SurfacePipeline::new()
        .then(PipelineStage::replace_glyph(
            "rewrite-shadow",
            ScopeSpec::Role {
                role: RoleTag::Shadow,
            },
            'a',
            'Z',
        ))
        .then(PipelineStage::explicit_role_write(
            "mark-shadow",
            ScopeSpec::All,
            ExplicitRoleWriteEffect::new(cell('a'), RoleTag::Shadow),
        ))
        .run(&source);

    assert_eq!(role_then_rewrite.surface.cell(0, 0).unwrap().ch, 'Z');
    assert_eq!(rewrite_then_role.surface.cell(0, 0).unwrap().ch, 'a');
}

#[test]
fn visual_stage_preserves_prior_stage_roles() {
    let mut source = surface(1, 1, RoleTag::Text);
    source.set_cell_and_role(0, 0, cell('A'), RoleTag::Title);

    let outcome = SurfacePipeline::new()
        .then(PipelineStage::copy("copy-title", ScopeSpec::All))
        .then(PipelineStage::dim(
            "dim-visual",
            ScopeSpec::All,
            DimEffect::new(0.5),
        ))
        .run(&source);

    assert_eq!(outcome.surface.cell(0, 0).unwrap().fg, Color::gray(128));
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Title));
}

#[test]
fn stage_skip_preserves_current_surface() {
    let mut source = surface(2, 1, RoleTag::Background);
    source.set_cell_and_role(0, 0, cell('A'), RoleTag::Background);
    source.set_cell_and_role(1, 0, cell('B'), RoleTag::Background);

    let outcome = SurfacePipeline::new()
        .then(PipelineStage::explicit_role_write(
            "stage-one-current",
            ScopeSpec::All,
            ExplicitRoleWriteEffect::new(cell('M'), RoleTag::Shadow),
        ))
        .then(PipelineStage::copy_with_sampler(
            "skip-right",
            ScopeSpec::All,
            ShiftSampler::new(1, 0),
        ))
        .run(&source);

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'M');
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Shadow));
    assert_eq!(outcome.surface.cell(1, 0).unwrap().ch, 'M');
    assert_eq!(outcome.surface.role(1, 0), Some(&RoleTag::Shadow));
}

#[test]
fn stage_zero_cell_scope_diagnostic_names_stage() {
    let source = surface(1, 1, RoleTag::Text);

    let outcome = SurfacePipeline::new()
        .then(PipelineStage::copy(
            "missing-shadow-stage",
            ScopeSpec::Role {
                role: RoleTag::Shadow,
            },
        ))
        .run(&source);

    assert_eq!(outcome.diagnostics.len(), 1);
    assert_eq!(
        outcome.diagnostics[0].code,
        SurfaceDiagnosticCode::ZeroCellScope
    );
    assert!(
        outcome.diagnostics[0]
            .message
            .contains("missing-shadow-stage")
    );
    assert_eq!(
        outcome.diagnostics[0].path.as_deref(),
        Some("pipeline.stage[0].missing-shadow-stage")
    );
}

#[test]
fn pipeline_diagnostics_are_deterministic() {
    let source = surface(1, 1, RoleTag::Text);

    let outcome = SurfacePipeline::new()
        .then(PipelineStage::copy(
            "first-miss",
            ScopeSpec::Role {
                role: RoleTag::Shadow,
            },
        ))
        .then(PipelineStage::copy(
            "second-miss",
            ScopeSpec::Role {
                role: RoleTag::Border,
            },
        ))
        .run(&source);

    let paths: Vec<_> = outcome
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.path.as_deref())
        .collect();
    assert_eq!(
        paths,
        vec![
            Some("pipeline.stage[0].first-miss"),
            Some("pipeline.stage[1].second-miss")
        ]
    );
}

#[test]
fn pipeline_keeps_phase_b_sampled_role_semantics() {
    let mut source = surface(2, 1, RoleTag::Background);
    source.set_cell_and_role(0, 0, cell('A'), RoleTag::Border);
    source.set_cell_and_role(1, 0, cell('B'), RoleTag::Border);

    let outcome = SurfacePipeline::new()
        .then(PipelineStage::explicit_role_write(
            "materialize-current-role",
            ScopeSpec::ColumnRange { start: 1, end: 2 },
            ExplicitRoleWriteEffect::new(cell('T'), RoleTag::Text),
        ))
        .then(PipelineStage::copy_with_sampler(
            "sample-role-from-current",
            ScopeSpec::Role {
                role: RoleTag::Text,
            },
            ShiftSampler::new(1, 0),
        ))
        .run(&source);

    assert_eq!(outcome.surface.cell(0, 0).unwrap().ch, 'T');
    assert_eq!(outcome.surface.role(0, 0), Some(&RoleTag::Text));
    assert_eq!(outcome.surface.cell(1, 0).unwrap().ch, 'T');
    assert_eq!(outcome.surface.role(1, 0), Some(&RoleTag::Text));
}

// <FILE>crates/tui-vfx-next/tests/test_surface_contract.rs</FILE> - <DESC>Phase A/B/C v3.1 surface contract tests</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
