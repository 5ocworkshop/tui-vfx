// <FILE>crates/tui-vfx-probe/src/fnc_collect_probe_operational_analysis.rs</FILE> - <DESC>Collect operational analysis from direct probe reports</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Direct engine stage-by-stage success/failure reporting</WCTX>
// <CLOG>0.4.0: carry grouped V3 shader-family labels into shader-stage operational rows so direct probe analysis can report category identity as well as concrete names.
// MINOR: Track how many configured elements share the same effect name so per-effect rows can disclose aggregation ambiguity for duplicate same-name instances</CLOG>

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    ProbeCombinedOperationalAnalysis, ProbeDiagnosticSeverity, ProbeEffectOperationalAnalysis,
    ProbeOperationalAnalysis, ProbeOperationalStatus, ProbeReport, ProbeStageOperationalAnalysis,
};

const STAGES: [&str; 4] = ["sampler", "mask", "shader", "filter"];

pub fn collect_probe_operational_analysis(
    scope: &str,
    reports: &[ProbeReport],
) -> ProbeOperationalAnalysis {
    let mut stages = Vec::with_capacity(STAGES.len());

    for stage in STAGES {
        let configured_count = configured_count_for_stage(stage, reports);
        let mut touched_cells = BTreeSet::new();
        let mut observed_effects = BTreeSet::new();
        let mut observed_event_count = 0usize;
        let mut effects = BTreeMap::<String, (BTreeSet<(usize, u16, u16)>, usize)>::new();

        for (frame_index, report) in reports.iter().enumerate() {
            for cell in &report.cells {
                for event in &cell.trace {
                    if event.stage == stage {
                        observed_event_count += 1;
                        touched_cells.insert((
                            frame_index,
                            cell.widget_local.x,
                            cell.widget_local.y,
                        ));
                        if let Some(effect) =
                            event.effect.as_ref().filter(|effect| !effect.is_empty())
                        {
                            observed_effects.insert(effect.clone());
                            let entry = effects.entry(effect.clone()).or_default();
                            entry
                                .0
                                .insert((frame_index, cell.widget_local.x, cell.widget_local.y));
                            entry.1 += 1;
                        }
                    }
                }
            }
        }

        let status = if configured_count == 0 {
            ProbeOperationalStatus::Inactive
        } else if touched_cells.is_empty() {
            ProbeOperationalStatus::Failure
        } else {
            ProbeOperationalStatus::Success
        };

        let configured_effects = configured_effects_for_stage(stage, reports);
        let mut effect_rows = configured_effects
            .iter()
            .map(|(effect, configured_instances, family)| {
                let aliases = configured_effect_aliases(effect, family.as_deref());
                let (effect_touched_cells, effect_event_count) = aliases
                    .iter()
                    .find_map(|alias| effects.get(alias))
                    .map(|(cells, count)| (cells.len(), *count))
                    .unwrap_or_default();
                ProbeEffectOperationalAnalysis {
                    effect: effect.clone(),
                    family: family.clone(),
                    configured: true,
                    configured_instances: *configured_instances,
                    touched_cells: effect_touched_cells,
                    observed_event_count: effect_event_count,
                    status: if effect_event_count > 0 {
                        ProbeOperationalStatus::Success
                    } else {
                        ProbeOperationalStatus::Failure
                    },
                }
            })
            .collect::<Vec<_>>();
        for (effect, (cells, count)) in effects {
            if configured_effects
                .iter()
                .flat_map(|(configured_effect, _, family)| {
                    configured_effect_aliases(configured_effect, family.as_deref())
                })
                .any(|configured_effect| configured_effect == effect)
            {
                continue;
            }
            effect_rows.push(ProbeEffectOperationalAnalysis {
                effect,
                family: None,
                configured: false,
                configured_instances: 0,
                touched_cells: cells.len(),
                observed_event_count: count,
                status: ProbeOperationalStatus::Warning,
            });
        }

        stages.push(ProbeStageOperationalAnalysis {
            stage: stage.to_string(),
            configured: configured_count > 0,
            configured_count,
            touched_cells: touched_cells.len(),
            observed_event_count,
            observed_effects: observed_effects.into_iter().collect(),
            effects: effect_rows,
            status,
        });
    }

    let mut diagnostic_codes = BTreeSet::new();
    let mut error_diagnostics = 0usize;
    let mut warning_diagnostics = 0usize;
    for report in reports {
        for diagnostic in &report.diagnostics {
            diagnostic_codes.insert(diagnostic.code.clone());
            match diagnostic.severity {
                ProbeDiagnosticSeverity::Warning => warning_diagnostics += 1,
                ProbeDiagnosticSeverity::Error => error_diagnostics += 1,
            }
        }
    }

    let failing_stages = stages
        .iter()
        .filter(|stage| stage.status == ProbeOperationalStatus::Failure)
        .map(|stage| stage.stage.clone())
        .collect::<Vec<_>>();
    let status = if error_diagnostics > 0 || !failing_stages.is_empty() {
        ProbeOperationalStatus::Failure
    } else if warning_diagnostics > 0 {
        ProbeOperationalStatus::Warning
    } else {
        ProbeOperationalStatus::Success
    };

    ProbeOperationalAnalysis {
        scope: scope.to_string(),
        frame_count: reports.len(),
        stages,
        combined: ProbeCombinedOperationalAnalysis {
            status,
            error_diagnostics,
            warning_diagnostics,
            failing_stages,
            diagnostic_codes: diagnostic_codes.into_iter().collect(),
        },
    }
}

fn configured_count_for_stage(stage: &str, reports: &[ProbeReport]) -> usize {
    reports
        .iter()
        .map(|report| match stage {
            "sampler" => usize::from(report.pipeline.sampler.is_some()),
            "mask" => report.pipeline.mask_count,
            "shader" => report.pipeline.shader_count,
            "filter" => report.pipeline.filter_count,
            _ => 0,
        })
        .max()
        .unwrap_or_default()
}

fn configured_effects_for_stage(
    stage: &str,
    reports: &[ProbeReport],
) -> Vec<(String, usize, Option<String>)> {
    let mut effects: BTreeMap<String, (usize, Option<String>)> = BTreeMap::new();
    for report in reports {
        match stage {
            "sampler" => {
                for effect in &report.pipeline.sampler_effects {
                    let entry = effects.entry(effect.clone()).or_insert((0usize, None));
                    entry.0 += 1;
                }
            }
            "mask" => {
                for effect in &report.pipeline.mask_effects {
                    let entry = effects.entry(effect.clone()).or_insert((0usize, None));
                    entry.0 += 1;
                }
            }
            "shader" => {
                for (index, effect) in report.pipeline.shader_effects.iter().enumerate() {
                    let family = report.pipeline.shader_families.get(index).cloned();
                    let entry = effects.entry(effect.clone()).or_insert((0usize, family.clone()));
                    entry.0 += 1;
                    if entry.1.is_none() {
                        entry.1 = family;
                    }
                }
            }
            "filter" => {
                for effect in &report.pipeline.filter_effects {
                    let entry = effects.entry(effect.clone()).or_insert((0usize, None));
                    entry.0 += 1;
                }
            }
            _ => {}
        }
    }
    effects
        .into_iter()
        .map(|(effect, (instances, family))| (effect, instances, family))
        .collect()
}

fn configured_effect_aliases(effect: &str, family: Option<&str>) -> Vec<String> {
    let mut aliases = vec![effect.to_string()];
    if let Some(family) = family {
        aliases.push(format!("{family}:{effect}"));
    }
    aliases
}

// <FILE>crates/tui-vfx-probe/src/fnc_collect_probe_operational_analysis.rs</FILE> - <DESC>Collect operational analysis from direct probe reports</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
