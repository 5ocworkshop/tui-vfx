// <FILE>crates/tui-vfx-player/src/fnc_build_fixture_qc_report.rs</FILE> - <DESC>Build fixture corpus QC reports</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Player evidence tooling: compose render, visual-frame, field, adapter, and smoke reports.</WCTX>
// <CLOG>0.2.0: PATCH — split recipe, summary, and message helpers to keep the coordinator small.</CLOG>

use std::path::{Path, PathBuf};

use tui_vfx_contract::DescriptorCatalog;

use crate::{
    DescriptorPackReport, PlayerFixtureQcReport, PlayerSampleRequest, RecipePlayer,
    build_frame_diff_report, build_frame_timeline_report, build_primitive_field_coverage_report,
    fnc_build_fixture_qc_messages::build_fixture_qc_errors,
    fnc_build_fixture_qc_messages::build_fixture_qc_warnings,
    fnc_build_fixture_qc_recipe_entries::build_fixture_qc_recipe_entries,
    fnc_build_fixture_qc_reports::build_fixture_qc_reports,
    fnc_build_fixture_qc_summary::build_fixture_qc_summary,
    fnc_fixture_qc_smoke_passed::{diff_smoke_passed, timeline_smoke_passed},
    primitive_adapter_gap_paths, render_recipe_file, render_visual_frame_paths,
};

/// Build a clean-room fixture QC report from existing player evidence surfaces.
pub fn build_fixture_qc_report(
    player: &RecipePlayer,
    catalog: &DescriptorCatalog,
    descriptor_packs: Vec<DescriptorPackReport>,
    paths: &[PathBuf],
    root: String,
    request: &PlayerSampleRequest,
) -> Result<PlayerFixtureQcReport, String> {
    let render = crate::PlayerRunReport::new(
        root.clone(),
        paths
            .iter()
            .map(|path| render_recipe_file(player, path, request))
            .collect(),
    );
    let visual_frame = render_visual_frame_paths(
        player,
        descriptor_packs.clone(),
        paths,
        root.clone(),
        request,
    );
    let field_coverage =
        build_primitive_field_coverage_report(root.clone(), descriptor_packs.clone(), paths)?;
    let adapter_gap = primitive_adapter_gap_paths(
        player,
        catalog,
        descriptor_packs.clone(),
        paths,
        root.clone(),
        request,
    );
    let timeline = first_path(paths).map(|path| {
        build_frame_timeline_report(
            player,
            descriptor_packs.clone(),
            path,
            root.clone(),
            request,
            3,
        )
    });
    let diff = first_path(paths).map(|path| {
        build_frame_diff_report(
            player,
            descriptor_packs.clone(),
            path,
            root.clone(),
            request,
            0.0,
            1.0,
        )
    });
    let recipes = build_fixture_qc_recipe_entries(catalog, paths, &render.frames);
    let warnings = build_fixture_qc_warnings(&field_coverage, &adapter_gap, &render.summary);
    let errors = build_fixture_qc_errors(&recipes);
    let summary = build_fixture_qc_summary(
        &render,
        &visual_frame,
        &field_coverage,
        &adapter_gap,
        timeline_smoke_passed(timeline.as_ref()),
        diff_smoke_passed(diff.as_ref()),
        &recipes,
    );
    Ok(PlayerFixtureQcReport::new(
        root,
        descriptor_packs,
        summary,
        build_fixture_qc_reports(
            render,
            visual_frame,
            field_coverage,
            adapter_gap,
            timeline,
            diff,
        )?,
        recipes,
        warnings,
        errors,
    ))
}

fn first_path(paths: &[PathBuf]) -> Option<&Path> {
    paths.first().map(PathBuf::as_path)
}

// <FILE>crates/tui-vfx-player/src/fnc_build_fixture_qc_report.rs</FILE> - <DESC>Build fixture corpus QC reports</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
