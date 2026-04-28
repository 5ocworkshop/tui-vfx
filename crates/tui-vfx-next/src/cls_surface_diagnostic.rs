// <FILE>crates/tui-vfx-next/src/cls_surface_diagnostic.rs</FILE> - <DESC>Structured surface diagnostic DTO</DESC>
// <VERS>VERSION: 0.5.1</VERS>
// <WCTX>New kernel Phase D1 verifier fix: make scene diagnostic paths field-stable.</WCTX>
// <CLOG>0.5.1: PATCH — use field-stable scene.element[index].id diagnostic paths.
// 0.5.0: MINOR — add scene element clipping diagnostic constructor.
// 0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract SurfaceDiagnostic and preserve constructors.</CLOG>

use crate::{DiagnosticLevel, SurfaceDiagnosticCode};

/// Structured diagnostic emitted by surface contract operations.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SurfaceDiagnostic {
    /// Severity level.
    pub level: DiagnosticLevel,
    /// Stable machine-readable code.
    pub code: SurfaceDiagnosticCode,
    /// Human-facing summary.
    pub message: String,
    /// Optional path-like location for future recipe/runtime integration.
    pub path: Option<String>,
    /// Optional remediation hint.
    pub hint: Option<String>,
}

impl SurfaceDiagnostic {
    /// Build the zero-cell scope diagnostic.
    pub fn zero_cell_scope(scope: &str) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            code: SurfaceDiagnosticCode::ZeroCellScope,
            message: format!("surface scope matched zero cells: {scope}"),
            path: None,
            hint: Some("Check the scope coordinates or sampled-source role selection.".to_string()),
        }
    }

    /// Build a surface size mismatch diagnostic.
    pub fn surface_size_mismatch(source: (usize, usize), destination: (usize, usize)) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            code: SurfaceDiagnosticCode::SurfaceSizeMismatch,
            message: format!(
                "source surface {source:?} does not match destination surface {destination:?}"
            ),
            path: None,
            hint: Some("Use equal-sized surfaces for the Phase A/B engine.".to_string()),
        }
    }
    /// Build a diagnostic for scene element cells clipped by final scene bounds.
    pub fn scene_element_clipped(
        element_id: &str,
        declaration_index: usize,
        clipped_cells: usize,
    ) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            code: SurfaceDiagnosticCode::SceneElementClipped,
            message: format!(
                "scene element `{element_id}` had {clipped_cells} cell(s) clipped by scene bounds"
            ),
            path: Some(format!("scene.element[{declaration_index}].id")),
            hint: Some(
                "Adjust element placement or scene dimensions if clipped cells are unexpected."
                    .to_string(),
            ),
        }
    }
}

// <FILE>crates/tui-vfx-next/src/cls_surface_diagnostic.rs</FILE> - <DESC>Structured surface diagnostic DTO</DESC>
// <VERS>END OF VERSION: 0.5.1</VERS>
