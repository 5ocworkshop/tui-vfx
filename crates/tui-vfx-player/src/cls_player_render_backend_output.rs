// <FILE>crates/tui-vfx-player/src/cls_player_render_backend_output.rs</FILE> - <DESC>Player-owned render backend output DTO</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Native compositor source isolation: add deterministic letter-cell evidence for legacy-oracle comparisons.</WCTX>
// <CLOG>0.4.0: MINOR — expose alphanumeric cell color histograms for V2-to-v3.1 parity checks.
// 0.3.0: MINOR — add source render mode and native source isolation evidence.
// 0.2.0: MINOR — add explicit composition-mode/fallback/native-lowering fields and optional changed-cell evidence.
// 0.1.0: INIT — add backend output and diagnostic DTOs consumed from PlayerRenderIrReport.</CLOG>

use std::collections::BTreeMap;

use crate::{
    PlayerError, PlayerRenderCell, PlayerRenderIrReport, PlayerWarning,
    fnc_render_hash::render_hash,
};

/// Deterministic output from a player-owned render backend.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRenderBackendOutput {
    /// Stable backend output report schema label.
    pub schema_version: &'static str,
    /// Backend implementation label.
    pub backend: &'static str,
    /// Canonical recipe id from the player render IR.
    pub recipe_id: String,
    /// Optional recipe file path from filesystem-backed renders.
    pub recipe_path: Option<String>,
    /// Sample timing metadata used to render this frame.
    pub sample: PlayerRenderBackendSample,
    /// Text rows emitted by the backend.
    pub rows: Vec<String>,
    /// Sparse styled-cell evidence emitted by the backend.
    pub styled_cells: Vec<PlayerRenderCell>,
    /// Player render hash before backend lowering.
    pub render_hash: u64,
    /// Deterministic backend output hash after lowering.
    pub backend_hash: u64,
    /// Count of styled cells that carry non-default visual styling.
    pub non_default_styled_cells: usize,
    /// Deterministic alphanumeric-cell evidence for legacy tooling comparisons.
    pub letter_cell_evidence: PlayerRenderBackendLetterCellEvidence,
    /// Requested composition strategy reported by backend adapters.
    pub composition_mode: String,
    /// Whether the backend used fallback instead of the requested native path.
    pub fallback_used: bool,
    /// Whether backend-native lowering was attempted for this output.
    pub native_lowering_attempted: bool,
    /// Whether backend-native lowering succeeded for every required node.
    pub native_lowering_succeeded: bool,
    /// Whether the backend emitted non-empty native composition instructions.
    pub composition_spec_non_empty: bool,
    /// Count of graph/effect nodes successfully lowered into native backend instructions.
    pub lowered_node_count: usize,
    /// Count of graph/effect nodes not lowered into native backend instructions.
    pub unlowered_node_count: usize,
    /// Stable effect ids successfully lowered.
    pub lowered_effect_ids: Vec<String>,
    /// Stable effect ids not lowered.
    pub unlowered_effect_ids: Vec<String>,
    /// Backend-owned summary of the emitted composition instructions.
    pub composition_spec_summary: BTreeMap<String, serde_json::Value>,
    /// Source substrate used by the backend adapter.
    pub source_render_mode: String,
    /// Whether native backend execution used pre-effect source-only player IR.
    pub native_source_isolated: bool,
    /// Optional cell-diff evidence for before/after comparisons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed_cells: Option<usize>,
    /// Non-fatal player warnings forwarded from render IR.
    pub warnings: Vec<PlayerWarning>,
    /// Hard player errors forwarded from render IR.
    pub errors: Vec<PlayerError>,
    /// Backend-owned non-fatal diagnostics.
    pub diagnostics: Vec<PlayerRenderBackendDiagnostic>,
    /// Backend-owned machine metadata.
    pub backend_metadata: BTreeMap<String, serde_json::Value>,
}

/// Backend output sample timing metadata.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRenderBackendSample {
    /// Requested lifecycle phase label.
    pub phase: String,
    /// Requested normalized phase progress.
    pub phase_t: f64,
    /// Optional requested loop progress.
    pub loop_t: Option<f64>,
    /// Recipe clock mode used to interpret this sample.
    pub clock_mode: String,
    /// Recipe clock loop period in milliseconds when applicable.
    pub clock_period_ms: Option<f64>,
    /// Monotonic elapsed sample time in milliseconds when known.
    pub absolute_t_ms: Option<f64>,
}

/// Player-owned render backend diagnostic.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRenderBackendDiagnostic {
    /// Stable machine-facing diagnostic code.
    pub code: String,
    /// JSON-ish path associated with the diagnostic.
    pub path: String,
    /// Human-readable diagnostic summary.
    pub message: String,
}

impl PlayerRenderBackendOutput {
    /// Build a backend output with the shared backend report schema label.
    pub fn new(
        backend: &'static str,
        rows: Vec<String>,
        styled_cells: Vec<PlayerRenderCell>,
        diagnostics: Vec<PlayerRenderBackendDiagnostic>,
    ) -> Self {
        let backend_hash = backend_hash_for(backend, &rows, &styled_cells);
        Self {
            schema_version: "v3.1.player.renderBackend.1",
            backend,
            recipe_id: String::new(),
            recipe_path: None,
            sample: PlayerRenderBackendSample {
                phase: "unknown".to_string(),
                phase_t: 0.0,
                loop_t: None,
                clock_mode: "unspecified".to_string(),
                clock_period_ms: None,
                absolute_t_ms: None,
            },
            rows,
            non_default_styled_cells: styled_cells
                .iter()
                .filter(|cell| cell_has_non_default_style(cell))
                .count(),
            letter_cell_evidence: letter_cell_evidence_for(&styled_cells),
            styled_cells,
            render_hash: 0,
            backend_hash,
            warnings: vec![],
            errors: vec![],
            diagnostics,
            composition_mode: "irResolved".to_string(),
            fallback_used: false,
            native_lowering_attempted: false,
            native_lowering_succeeded: false,
            composition_spec_non_empty: false,
            lowered_node_count: 0,
            unlowered_node_count: 0,
            lowered_effect_ids: vec![],
            unlowered_effect_ids: vec![],
            composition_spec_summary: BTreeMap::new(),
            source_render_mode: "postEffectIr".to_string(),
            native_source_isolated: false,
            changed_cells: None,
            backend_metadata: BTreeMap::new(),
        }
    }

    /// Build a backend output from a player render IR report.
    pub fn from_ir(
        backend: &'static str,
        input: &PlayerRenderIrReport,
        rows: Vec<String>,
        styled_cells: Vec<PlayerRenderCell>,
        diagnostics: Vec<PlayerRenderBackendDiagnostic>,
        backend_metadata: BTreeMap<String, serde_json::Value>,
    ) -> Self {
        let non_default_styled_cells = styled_cells
            .iter()
            .filter(|cell| cell_has_non_default_style(cell))
            .count();
        let backend_hash = backend_hash_for(backend, &rows, &styled_cells);
        let letter_cell_evidence = letter_cell_evidence_for(&styled_cells);
        Self {
            schema_version: "v3.1.player.renderBackend.1",
            backend,
            recipe_id: input.recipe_id.clone(),
            recipe_path: input.path.clone(),
            sample: PlayerRenderBackendSample {
                phase: format!("{:?}", input.phase),
                phase_t: input.phase_t,
                loop_t: input.loop_t,
                clock_mode: input.clock.mode.clone(),
                clock_period_ms: input.clock.period_ms,
                absolute_t_ms: input.clock.absolute_t_ms,
            },
            rows,
            styled_cells,
            render_hash: input.render_hash,
            backend_hash,
            non_default_styled_cells,
            letter_cell_evidence,
            composition_mode: "irResolved".to_string(),
            fallback_used: false,
            native_lowering_attempted: false,
            native_lowering_succeeded: false,
            composition_spec_non_empty: false,
            lowered_node_count: 0,
            unlowered_node_count: 0,
            lowered_effect_ids: vec![],
            unlowered_effect_ids: vec![],
            composition_spec_summary: BTreeMap::new(),
            source_render_mode: "postEffectIr".to_string(),
            native_source_isolated: false,
            changed_cells: None,
            warnings: input.warnings.clone(),
            errors: input.errors.clone(),
            diagnostics,
            backend_metadata,
        }
    }

    /// Attach backend-native composition evidence after rendering.
    pub fn with_composition_evidence(
        mut self,
        evidence: PlayerRenderBackendCompositionEvidence,
    ) -> Self {
        self.composition_mode = evidence.composition_mode;
        self.fallback_used = evidence.fallback_used;
        self.native_lowering_attempted = evidence.native_lowering_attempted;
        self.native_lowering_succeeded = evidence.native_lowering_succeeded;
        self.composition_spec_non_empty = evidence.composition_spec_non_empty;
        self.lowered_node_count = evidence.lowered_node_count;
        self.unlowered_node_count = evidence.unlowered_node_count;
        self.lowered_effect_ids = evidence.lowered_effect_ids;
        self.unlowered_effect_ids = evidence.unlowered_effect_ids;
        self.composition_spec_summary = evidence.composition_spec_summary;
        self.source_render_mode = evidence.source_render_mode;
        self.native_source_isolated = evidence.native_source_isolated;
        self
    }

    /// Attach optional before/after changed-cell evidence.
    pub fn with_changed_cells(mut self, changed_cells: usize) -> Self {
        self.changed_cells = Some(changed_cells);
        self
    }
}

/// Deterministic color-class evidence for visible alphanumeric recipe cells.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRenderBackendLetterCellEvidence {
    /// Number of cells whose first glyph character is alphanumeric.
    pub letter_cell_count: usize,
    /// Background color counts across alphanumeric cells.
    pub background_class_counts: BTreeMap<String, usize>,
    /// Foreground/background color-pair counts across alphanumeric cells.
    pub foreground_background_class_counts: BTreeMap<String, usize>,
}

/// Backend-native composition evidence attached to render output.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlayerRenderBackendCompositionEvidence {
    /// Requested composition strategy reported by backend adapters.
    pub composition_mode: String,
    /// Whether the backend used fallback instead of the requested native path.
    pub fallback_used: bool,
    /// Whether backend-native lowering was attempted.
    pub native_lowering_attempted: bool,
    /// Whether backend-native lowering succeeded for every required node.
    pub native_lowering_succeeded: bool,
    /// Whether the backend emitted non-empty native composition instructions.
    pub composition_spec_non_empty: bool,
    /// Count of graph/effect nodes successfully lowered.
    pub lowered_node_count: usize,
    /// Count of graph/effect nodes not lowered.
    pub unlowered_node_count: usize,
    /// Stable effect ids successfully lowered.
    pub lowered_effect_ids: Vec<String>,
    /// Stable effect ids not lowered.
    pub unlowered_effect_ids: Vec<String>,
    /// Backend-owned summary of emitted composition instructions.
    pub composition_spec_summary: BTreeMap<String, serde_json::Value>,
    /// Source substrate used by the backend adapter.
    pub source_render_mode: String,
    /// Whether native backend execution used pre-effect source-only player IR.
    pub native_source_isolated: bool,
}

fn backend_hash_for(backend: &str, rows: &[String], styled_cells: &[PlayerRenderCell]) -> u64 {
    let mut parts = vec![backend.to_string()];
    parts.extend(rows.iter().cloned());
    for cell in styled_cells {
        parts.push(format!(
            "{}:{}:{}:{}:{}:{}:{}",
            cell.x,
            cell.y,
            cell.glyph,
            cell.foreground,
            cell.background,
            cell.modifiers.join("|"),
            cell.role.clone().unwrap_or_default()
        ));
    }
    render_hash(&parts)
}

fn cell_has_non_default_style(cell: &PlayerRenderCell) -> bool {
    !(cell.foreground == "transparent"
        && cell.background == "transparent"
        && cell.modifiers.is_empty()
        && cell.role.is_none())
}

fn letter_cell_evidence_for(
    styled_cells: &[PlayerRenderCell],
) -> PlayerRenderBackendLetterCellEvidence {
    let mut evidence = PlayerRenderBackendLetterCellEvidence::default();
    for cell in styled_cells {
        let Some(glyph) = cell.glyph.chars().next() else {
            continue;
        };
        if !glyph.is_alphanumeric() {
            continue;
        }
        evidence.letter_cell_count += 1;
        *evidence
            .background_class_counts
            .entry(cell.background.clone())
            .or_insert(0) += 1;
        *evidence
            .foreground_background_class_counts
            .entry(format!("fg={} bg={}", cell.foreground, cell.background))
            .or_insert(0) += 1;
    }
    evidence
}

// <FILE>crates/tui-vfx-player/src/cls_player_render_backend_output.rs</FILE> - <DESC>Player-owned render backend output DTO</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
