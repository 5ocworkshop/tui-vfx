// <FILE>xtask/src/docs/extract_signals_rustdoc.rs</FILE> - <DESC>Extract Signal-impl rustdoc from mixed-signals source files</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase α + β: signal-facade — parallel sibling pipeline (Q1 decision: separate extractor, not extending extract_rustdoc.rs)</WCTX>
// <CLOG>0.2.0: resolve mixed-signals root via CARGO_MANIFEST_DIR/.. so tests pass under `cargo test -p xtask` (CWD = xtask) without breaking `cargo xtask` (CWD = workspace root)</CLOG>

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Extracted catalog of every Signal primitive in mixed-signals.
#[derive(Debug, Default)]
pub struct SignalsRustdocData {
    /// Flat lookup by SignalSpec snake_case discriminant (e.g. "sine").
    /// For signals not in SignalSpec (e.g. physics primitives), keyed by
    /// their struct name lowercased (e.g. "dampedspring").
    pub by_discriminant: HashMap<String, SignalDoc>,
    /// All signals organized by family, in discovery order.
    pub by_family: HashMap<SignalFamily, Vec<SignalDoc>>,
}

/// Documentation for a single Signal primitive.
#[derive(Debug, Clone)]
pub struct SignalDoc {
    /// SignalSpec discriminant (snake_case), e.g. "sine". For non-SignalSpec
    /// signals this is the struct name lowercased.
    pub discriminant: String,
    /// Rust struct name, e.g. "Sine".
    pub struct_name: String,
    /// Family this signal belongs to.
    pub family: SignalFamily,
    /// First line of the struct-level doc comment.
    pub summary: String,
    /// Full doc comment text (all /// lines above the struct).
    pub description: String,
    /// Public fields with their doc comments.
    pub fields: Vec<SignalFieldDoc>,
    /// True when this signal is NOT reachable through SignalSpec JSON today.
    /// Set for physics primitives (the parallel motion-spec channel).
    pub is_parallel_channel: bool,
}

/// Documentation for a single field on a Signal struct.
#[derive(Debug, Clone)]
pub struct SignalFieldDoc {
    /// Field name as written in source, e.g. "frequency".
    pub name: String,
    /// Rust type as a string, e.g. "f32".
    pub ty: String,
    /// Field-level doc comment (may be empty).
    pub doc: String,
}

/// Signal family groupings for the reference sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SignalFamily {
    /// Periodic oscillators (Sine, Triangle, Square, Sawtooth, Ramp, Step, Pulse, Constant, Keyframes, PhaseSine, PhaseAccumulator).
    Oscillator,
    /// Spatial coordinate leaves (SampleNorm*, SampleCell*, SampleCentered*, SampleRadius, SampleAngle, etc.).
    Spatial,
    /// Envelope generators (Adsr, Impact, LinearEnvelope, LinearDecay, ExponentialDecay).
    Envelope,
    /// Physics-based signals (DampedSpring, BouncingDrop, FrictionDecay, SimplePendulum, CircularOrbit, BallisticTrajectory, PointAttractor).
    Physics,
    /// Noise and random generators (PerlinNoise, WhiteNoise, SpatialNoise, SeededRandom, GaussianNoise, etc.).
    Noise,
    /// Composition operators (Add, Multiply, Mix, WeightedMix, FrequencyMod, VcaCentered).
    Composition,
    /// Signal processing transforms (Clamp, Quantize, Remap, Invert, Abs, Normalized, Lowpass, SVF, Biquad).
    Processing,
}

impl SignalFamily {
    /// Human-readable section heading for the generated reference.
    pub fn heading(&self) -> &'static str {
        match self {
            SignalFamily::Oscillator => "Oscillators",
            SignalFamily::Spatial => "Spatial Coordinates",
            SignalFamily::Envelope => "Envelopes",
            SignalFamily::Physics => "Physics",
            SignalFamily::Noise => "Noise and Random",
            SignalFamily::Composition => "Composition Operators",
            SignalFamily::Processing => "Processing",
        }
    }

    /// Anchor-friendly lowercase id for the generated reference.
    pub fn anchor(&self) -> &'static str {
        match self {
            SignalFamily::Oscillator => "oscillators",
            SignalFamily::Spatial => "spatial-coordinates",
            SignalFamily::Envelope => "envelopes",
            SignalFamily::Physics => "physics",
            SignalFamily::Noise => "noise-and-random",
            SignalFamily::Composition => "composition-operators",
            SignalFamily::Processing => "processing",
        }
    }
}

/// Test-fixture types that implement Signal but are NOT recipe-author-facing.
/// Populated from the packet's §Current-state audit "Test-only / construction-helper" list.
const TEST_FIXTURE_STRUCTS: &[&str] = &[
    "RawFrequency",
    "NanSignal",
    "OverflowSignal",
    "UnderflowSignal",
    "LinearSignal",
    "StepSignal",
    "RawSignal",
    "RawValue",
    "BipolarConstant",
    "UnitConstant",
    "UnitSignal",
    "PositiveSignal",
    "ContextSignal",
    "AnalyticSlopeSignal",
    "ConstantSignal",
];

/// Directory-name → SignalFamily mapping for the walked source tree.
fn family_for_dir(dir: &str) -> Option<SignalFamily> {
    match dir {
        "generators" => Some(SignalFamily::Oscillator),
        "envelopes" => Some(SignalFamily::Envelope),
        "physics" => Some(SignalFamily::Physics),
        "composition" => Some(SignalFamily::Composition),
        "noise" => Some(SignalFamily::Noise),
        "random" => Some(SignalFamily::Noise),
        "processing" => Some(SignalFamily::Processing),
        _ => None,
    }
}

/// Struct-name → SignalSpec discriminant mapping for cases where the discriminant
/// does not match the simple snake_case of the struct name.
/// Derived from reading `mixed-signals/src/types/signal_spec.rs`.
fn discriminant_override(struct_name: &str) -> Option<&'static str> {
    match struct_name {
        "PerlinNoise" => Some("perlin"),
        "Adsr" => Some("adsr"),
        "LinearEnvelope" => Some("linear_envelope"),
        "ImpulseNoise" => Some("impulse_noise"),
        "WhiteNoise" => Some("white_noise"),
        "SeededRandom" => Some("seeded_random"),
        "SpatialNoise" => Some("spatial_noise"),
        "GaussianNoise" => Some("gaussian_noise"),
        "PoissonNoise" => Some("poisson_noise"),
        "CorrelatedNoise" => Some("correlated_noise"),
        "PinkNoise" => Some("pink_noise"),
        "PerCharacterNoise" => Some("per_character_noise"),
        "StudentTNoise" => Some("student_t_noise"),
        "FastSeededRandom" => Some("fast_seeded_random"),
        "FastCorrelatedNoise" => Some("fast_correlated_noise"),
        "FastPinkNoise" => Some("fast_pink_noise"),
        "PhaseAccumulator" => Some("phase_accumulator"),
        "PhaseSine" => Some("phase_sine"),
        "FrequencyMod" => Some("frequency_mod"),
        "VcaCentered" => Some("vca_centered"),
        "WeightedMix" => Some("weighted_mix"),
        "SampleNormX" => Some("sample_norm_x"),
        "SampleNormY" => Some("sample_norm_y"),
        "SampleCellX" => Some("sample_cell_x"),
        "SampleCellY" => Some("sample_cell_y"),
        "SampleCenteredX" => Some("sample_centered_x"),
        "SampleCenteredY" => Some("sample_centered_y"),
        "SampleRadius" => Some("sample_radius"),
        "SampleAngle" => Some("sample_angle"),
        "SampleSurfaceCenteredX" => Some("sample_surface_centered_x"),
        "SampleSurfaceCenteredY" => Some("sample_surface_centered_y"),
        "SampleSurfaceRadius" => Some("sample_surface_radius"),
        "SampleSurfaceRadiusFrom" => Some("sample_surface_radius_from"),
        "SampleCellRadiusFrom" => Some("sample_cell_radius_from"),
        "SampleSurfaceAngleFrom" => Some("sample_surface_angle_from"),
        "LinearDecay" => Some("linear_decay"),
        "ExponentialDecay" => Some("exponential_decay"),
        // Physics primitives are NOT in SignalSpec (parallel channel).
        // They get a key derived from their struct name for catalog purposes;
        // is_parallel_channel is set to true.
        _ => None,
    }
}

/// Physics struct names — these live in the parallel motion-spec channel,
/// not in SignalSpec. Documented with an is_parallel_channel callout.
const PHYSICS_STRUCTS: &[&str] = &[
    "DampedSpring",
    "BouncingDrop",
    "FrictionDecay",
    "SimplePendulum",
    "CircularOrbit",
    "BallisticTrajectory",
    "PointAttractor",
];

/// Convert a struct name to a snake_case discriminant key.
fn struct_name_to_key(struct_name: &str) -> String {
    // Convert CamelCase to snake_case
    let mut result = String::new();
    for (i, c) in struct_name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}

/// Extract all Signal-impl documentation from the mixed-signals source tree.
///
/// Walks `mixed-signals/src/{generators,envelopes,physics,composition,noise,random,processing}`,
/// parses `///` doc comments above `pub struct` declarations, and builds the catalog.
/// Test-fixture types (listed in `TEST_FIXTURE_STRUCTS`) are excluded.
/// Physics primitives are included with `is_parallel_channel = true`.
pub fn extract() -> Result<SignalsRustdocData> {
    let ms_root = find_mixed_signals_root()?;
    let mut data = SignalsRustdocData::default();

    let target_dirs = [
        "generators",
        "envelopes",
        "physics",
        "composition",
        "noise",
        "random",
        "processing",
    ];

    for dir_name in target_dirs {
        let dir_path = ms_root.join("src").join(dir_name);
        if !dir_path.exists() {
            continue;
        }

        let family = match family_for_dir(dir_name) {
            Some(f) => f,
            None => continue,
        };

        for entry in WalkDir::new(&dir_path)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            // Only process cls_*.rs files (OFPF struct files).
            // Skip test files (test_*.rs).
            let file_name = match path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n,
                None => continue,
            };
            if !file_name.starts_with("cls_") || !file_name.ends_with(".rs") {
                continue;
            }

            let source = fs::read_to_string(path)
                .with_context(|| format!("Failed to read {}", path.display()))?;

            let structs = parse_structs_from_source(&source, family)?;
            for doc in structs {
                // Skip test fixtures.
                if TEST_FIXTURE_STRUCTS.contains(&doc.struct_name.as_str()) {
                    continue;
                }

                data.by_family
                    .entry(family)
                    .or_default()
                    .push(doc.clone());
                data.by_discriminant
                    .insert(doc.discriminant.clone(), doc);
            }
        }
    }

    // Also parse the SampleNorm*/SampleCell*/SampleCentered* unit structs from signal_spec.rs
    // because they are SignalSpec variants declared inline (no cls_ file).
    let spec_path = ms_root.join("src/types/signal_spec.rs");
    if spec_path.exists() {
        let spec_source = fs::read_to_string(&spec_path)
            .with_context(|| format!("Failed to read {}", spec_path.display()))?;
        let spatial_docs = parse_spatial_unit_variants(&spec_source)?;
        for doc in spatial_docs {
            data.by_family
                .entry(SignalFamily::Spatial)
                .or_default()
                .push(doc.clone());
            data.by_discriminant
                .insert(doc.discriminant.clone(), doc);
        }
    }

    Ok(data)
}

/// Find the mixed-signals repository root.
///
/// Resolved relative to the tui-vfx workspace root (`CARGO_MANIFEST_DIR/..`)
/// so this works under both `cargo xtask` (CWD = workspace root) and
/// `cargo test -p xtask` (CWD = `xtask/`).
fn find_mixed_signals_root() -> Result<PathBuf> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR (xtask/) always has a parent (workspace root)")
        .to_path_buf();
    let candidates = [
        workspace_root.join("../mixed-signals"),
        workspace_root.join("../../mixed-signals"),
    ];
    for candidate in &candidates {
        if candidate.join("src/lib.rs").exists() {
            return Ok(candidate.clone());
        }
    }
    anyhow::bail!(
        "Could not locate mixed-signals repository. \
         Expected at ../mixed-signals or ../../mixed-signals relative to the tui-vfx workspace root."
    );
}

/// Parse all `pub struct` declarations (with their doc comments and fields)
/// from a single source file.
fn parse_structs_from_source(source: &str, family: SignalFamily) -> Result<Vec<SignalDoc>> {
    let mut results = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Look for `pub struct Name` or `pub struct Name<` declarations.
        if line.starts_with("pub struct ") {
            // Collect doc comments above this line.
            let doc_lines = collect_doc_lines_before(&lines, i);
            let struct_name = extract_struct_name(line);

            if let Some(name) = struct_name {
                // Skip test fixtures by name.
                if TEST_FIXTURE_STRUCTS.contains(&name.as_str()) {
                    i += 1;
                    continue;
                }

                // Collect fields (lines between `{` and the matching `}`).
                let fields = collect_fields(&lines, i);

                let description = doc_lines.join("\n");
                let summary = doc_lines.first().cloned().unwrap_or_default();

                let is_physics = PHYSICS_STRUCTS.contains(&name.as_str());

                let discriminant = if let Some(ov) = discriminant_override(&name) {
                    ov.to_string()
                } else {
                    struct_name_to_key(&name)
                };

                results.push(SignalDoc {
                    discriminant,
                    struct_name: name,
                    family,
                    summary,
                    description,
                    fields,
                    is_parallel_channel: is_physics,
                });
            }
        }

        i += 1;
    }

    Ok(results)
}

/// Collect `///` doc-comment lines immediately above line `target_idx`.
/// Strips the `/// ` prefix. Stops at the first non-doc-comment line scanning backward.
fn collect_doc_lines_before(lines: &[&str], target_idx: usize) -> Vec<String> {
    let mut doc = Vec::new();
    let mut j = target_idx as isize - 1;

    // Skip derive/attribute lines directly above the struct.
    while j >= 0 {
        let line = lines[j as usize].trim();
        if line.starts_with("#[") || line.starts_with('#') {
            j -= 1;
        } else {
            break;
        }
    }

    // Now collect /// lines scanning backward.
    while j >= 0 {
        let line = lines[j as usize].trim();
        if line.starts_with("///") {
            let content = line.strip_prefix("///").unwrap_or("").trim_start().to_string();
            doc.push(content);
            j -= 1;
        } else {
            break;
        }
    }

    doc.reverse();
    doc
}

/// Extract the struct name from a `pub struct Name` or `pub struct Name<...>` line.
fn extract_struct_name(line: &str) -> Option<String> {
    let after = line.strip_prefix("pub struct ")?.trim_start();
    // Take up to `<`, `{`, `(`, or whitespace.
    let name: String = after
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if name.is_empty() { None } else { Some(name) }
}

/// Collect public fields from the struct body starting at line `struct_idx`.
/// Scans forward to find the opening `{` then reads until the matching `}`.
fn collect_fields(lines: &[&str], struct_idx: usize) -> Vec<SignalFieldDoc> {
    let mut fields = Vec::new();
    let mut depth = 0i32;
    let mut in_struct = false;
    let mut pending_doc: Vec<String> = Vec::new();

    for line in &lines[struct_idx..] {
        let trimmed = line.trim();

        // Track brace depth.
        for ch in trimmed.chars() {
            match ch {
                '{' => {
                    depth += 1;
                    in_struct = true;
                }
                '}' => {
                    depth -= 1;
                }
                _ => {}
            }
        }

        if !in_struct {
            continue;
        }

        // At depth == 0 after we entered, struct is done.
        if depth == 0 {
            break;
        }

        // Only process lines at depth 1 (direct struct fields).
        if depth != 1 {
            pending_doc.clear();
            continue;
        }

        if trimmed.starts_with("///") {
            let content = trimmed.strip_prefix("///").unwrap_or("").trim_start().to_string();
            pending_doc.push(content);
            continue;
        }

        // Look for `pub field_name: Type,` pattern.
        // Also match `pub(crate) field_name: Type,`.
        let field_line = if let Some(rest) = trimmed.strip_prefix("pub ") {
            // Drop `(crate)` or similar visibility qualifiers.
            let rest = if rest.starts_with('(') {
                rest.split_once(')').map_or(rest, |(_, after)| after).trim_start()
            } else {
                rest
            };
            Some(rest)
        } else if !trimmed.starts_with("//") && !trimmed.starts_with('#') && !trimmed.is_empty() {
            // Private fields without doc still consume pending docs.
            pending_doc.clear();
            None
        } else {
            None
        };

        if let Some(field_line) = field_line {
            if let Some((name, ty)) = parse_field_name_type(field_line) {
                let doc = pending_doc.join(" ");
                fields.push(SignalFieldDoc { name, ty, doc });
            }
            pending_doc.clear();
        }
    }

    fields
}

/// Parse `field_name: Type` from a field declaration line (after `pub `).
fn parse_field_name_type(line: &str) -> Option<(String, String)> {
    let colon = line.find(':')?;
    let name = line[..colon].trim().to_string();
    if name.is_empty() {
        return None;
    }
    // Type is everything after `:` up to `,` or end.
    let ty_raw = line[colon + 1..].trim();
    let ty = ty_raw.trim_end_matches(',').trim().to_string();
    if ty.is_empty() {
        return None;
    }
    Some((name, ty))
}

/// Parse the unit-struct SignalSpec variants for spatial leaves (SampleNormX, etc.)
/// which have no corresponding cls_ file.
fn parse_spatial_unit_variants(source: &str) -> Result<Vec<SignalDoc>> {
    let mut results = Vec::new();
    let lines: Vec<&str> = source.lines().collect();

    // Spatial unit variants are `SampleNormX,` style lines inside the enum,
    // preceded by `///` doc comments.
    let spatial_unit_names = [
        "SampleNormX",
        "SampleNormY",
        "SampleCellX",
        "SampleCellY",
        "SampleCenteredX",
        "SampleCenteredY",
        "SampleRadius",
        "SampleAngle",
        "SampleSurfaceCenteredX",
        "SampleSurfaceCenteredY",
        "SampleSurfaceRadius",
    ];

    for i in 0..lines.len() {
        let trimmed = lines[i].trim().trim_end_matches(',');
        if spatial_unit_names.contains(&trimmed) {
            let doc_lines = collect_doc_lines_before(&lines, i);
            let name = trimmed.to_string();
            let discriminant = discriminant_override(&name)
                .map(|s| s.to_string())
                .unwrap_or_else(|| struct_name_to_key(&name));

            let description = doc_lines.join("\n");
            let summary = doc_lines.first().cloned().unwrap_or_default();

            results.push(SignalDoc {
                discriminant,
                struct_name: name,
                family: SignalFamily::Spatial,
                summary,
                description,
                fields: Vec::new(),
                is_parallel_channel: false,
            });
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn every_signal_primitive_has_a_catalog_entry() {
        let data = extract().expect("extraction failed");

        // Known non-test-fixture Signal-impl struct names from the packet's audit.
        let known_primitives: &[&str] = &[
            "Sine", "Triangle", "Square", "Sawtooth", "Ramp", "Step", "Pulse",
            "Constant", "Keyframes", "PhaseSine", "PhaseAccumulator",
            "Adsr", "Impact", "LinearEnvelope", "LinearDecay", "ExponentialDecay",
            "DampedSpring", "BouncingDrop", "FrictionDecay", "SimplePendulum",
            "CircularOrbit", "BallisticTrajectory", "PointAttractor",
            "PerlinNoise", "WhiteNoise", "SpatialNoise",
            "SeededRandom", "GaussianNoise", "PoissonNoise", "CorrelatedNoise",
            "PinkNoise", "PerCharacterNoise", "StudentTNoise", "ImpulseNoise",
            "Add", "Multiply", "Mix", "WeightedMix", "VcaCentered", "FrequencyMod",
            "Clamp", "Quantize", "Remap", "Invert", "Abs",
        ];

        let catalog: HashSet<&str> = data
            .by_discriminant
            .values()
            .map(|s| s.struct_name.as_str())
            .collect();

        let mut missing = Vec::new();
        for name in known_primitives {
            if !catalog.contains(name) {
                missing.push(*name);
            }
        }
        assert!(
            missing.is_empty(),
            "Missing primitives in catalog: {:?}",
            missing
        );
    }

    #[test]
    fn core_12_is_subset_of_catalog() {
        let data = extract().expect("extraction failed");
        let toml = super::super::parse_signals_toml::parse().expect("parse failed");

        for name in &toml.core_12.order {
            // spring is a special case: documented with is_parallel_channel=true
            // but must still appear in the catalog (keyed as "damped_spring").
            // We accept either the toml key or the canonical catalog entry.
            let found = data.by_discriminant.contains_key(name)
                || (name == "spring"
                    && data.by_discriminant.contains_key("damped_spring"));
            assert!(
                found,
                "Core 12 entry `{name}` not found in the autogen catalog"
            );
        }
        assert_eq!(
            toml.core_12.order.len(),
            12,
            "Core 12 must have exactly 12 entries; update signals.toml and this assertion together"
        );
    }

    #[test]
    fn extract_sine_has_doc_and_fields() {
        let data = extract().expect("extraction failed");
        let sine = data.by_discriminant.get("sine").expect("sine not found");
        assert!(!sine.summary.is_empty(), "Sine should have a summary");
        assert!(!sine.fields.is_empty(), "Sine should have documented fields");
        let has_frequency = sine.fields.iter().any(|f| f.name == "frequency");
        assert!(has_frequency, "Sine should have a `frequency` field");
    }
}

// <FILE>xtask/src/docs/extract_signals_rustdoc.rs</FILE> - <DESC>Extract Signal-impl rustdoc from mixed-signals source files</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
