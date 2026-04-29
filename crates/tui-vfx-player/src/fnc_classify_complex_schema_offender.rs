// <FILE>crates/tui-vfx-player/src/fnc_classify_complex_schema_offender.rs</FILE> - <DESC>Classify complex offender rows</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.12 schema lock: encode complex owner-audit clusters from the briefing-backed audit.</WCTX>
// <CLOG>0.1.0: INIT — map complex scene, GUI, and backend holdbacks to concrete readiness kinds.</CLOG>

/// Return an explicit readiness kind for complex recipes that cannot be inferred from fields alone.
pub(crate) fn classify_complex_schema_offender_path(path: &str) -> Option<&'static str> {
    if is_scene_local_pipeline(path) {
        Some("sceneSemantics")
    } else if is_gui_human_review(path) {
        Some("guiHumanReview")
    } else if path == "complex/complex_shadow_mask_sampler_shader_filter_native_mix.json" {
        Some("backendRenderer")
    } else {
        None
    }
}

fn is_scene_local_pipeline(path: &str) -> bool {
    matches!(
        path,
        "complex/complex_filter_to_mask_sourced_output.json"
            | "complex/complex_nested_parallel_sequences.json"
            | "complex/complex_parallel_channel_filters.json"
            | "complex/complex_parallel_content_scopes.json"
            | "complex/complex_parallel_disjoint_shader_style.json"
            | "complex/complex_parallel_multi_sampler_disjoint.json"
            | "complex/complex_parallel_role_scopes.json"
            | "complex/complex_parallel_sequence_branches.json"
            | "complex/complex_sequence_sampler_then_shader.json"
            | "complex/complex_sequence_sampler_then_style_effect.json"
            | "complex/complex_sequence_shader_then_sampler.json"
            | "complex/complex_sequence_shadow_sampler_then_style_effect.json"
            | "complex/complex_sequence_shadow_style_effect_then_sampler.json"
            | "complex/complex_sequence_style_effect_then_sampler.json"
    )
}

fn is_gui_human_review(path: &str) -> bool {
    matches!(
        path,
        "complex/complex_parallel_overlap_conflict_snapshot.json"
            | "complex/v3_scheduler_overlap_conflict_mixed_family.json"
    )
}

// <FILE>crates/tui-vfx-player/src/fnc_classify_complex_schema_offender.rs</FILE> - <DESC>Classify complex offender rows</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
