// <FILE>xtask/src/docs/merge_signals.rs</FILE> - <DESC>Merge Signal-impl rustdoc with signals.toml editorial overlay</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase α + β: signal-facade — signals pipeline merge step</WCTX>
// <CLOG>0.1.0: initial implementation — combines SignalsRustdocData + SignalsManifest into MergedSignals; builds Core 12 list; organizes full catalog by family</CLOG>

use super::extract_signals_rustdoc::{SignalDoc, SignalFamily, SignalsRustdocData};
use super::parse_signals_toml::{SignalEntry, SignalsManifest};
use anyhow::Result;
use std::collections::BTreeMap;

/// Merged output ready for markdown generation.
#[derive(Debug)]
pub struct MergedSignals {
    /// Version string from signals.toml [meta].
    pub version: String,
    /// The Core 12 signals in `[core_12].order` sequence.
    pub core_12: Vec<MergedSignal>,
    /// All signals organized by family, sorted by family then by struct name.
    pub by_family: BTreeMap<SignalFamily, Vec<MergedSignal>>,
}

/// A single signal with merged rustdoc + editorial data.
#[derive(Debug, Clone)]
pub struct MergedSignal {
    /// Rustdoc-extracted information.
    pub doc: SignalDoc,
    /// Editorial overlay (may be absent for non-Core-12 signals per Q2 only-overrides).
    pub editorial: Option<SignalEntry>,
    /// Representative JSON snippet built from SignalSpec serde shape.
    pub example_json: String,
    /// Display name from signals.toml (may differ from doc.discriminant for aliases
    /// like "spring" which maps to the "damped_spring" catalog entry).
    pub display_discriminant: String,
}

/// Merge rustdoc catalog with editorial overlay.
///
/// The Core 12 list is ordered by `[core_12].order` from signals.toml.
/// The full catalog is organized by `SignalFamily`.
pub fn merge(rustdoc: SignalsRustdocData, toml: SignalsManifest) -> Result<MergedSignals> {
    // Build Core 12 list in the declared order.
    let core_12 = toml
        .core_12
        .order
        .iter()
        .filter_map(|name| {
            // Resolve "spring" → "damped_spring" for catalog lookup.
            // The display name stays "spring" per the Core 12 cheatsheet.
            let lookup_key = if name == "spring" { "damped_spring" } else { name.as_str() };
            let doc = rustdoc.by_discriminant.get(lookup_key).cloned()?;
            let editorial = toml.signals.get(name).cloned();
            let example_json = build_example_json(&doc);
            Some(MergedSignal {
                display_discriminant: name.clone(),
                doc,
                editorial,
                example_json,
            })
        })
        .collect();

    // Organize full catalog by family.
    let mut by_family: BTreeMap<SignalFamily, Vec<MergedSignal>> = BTreeMap::new();
    for doc in rustdoc.by_discriminant.values() {
        let editorial = toml.signals.get(&doc.discriminant).cloned();
        let example_json = build_example_json(doc);
        let display_discriminant = doc.discriminant.clone();
        by_family
            .entry(doc.family)
            .or_default()
            .push(MergedSignal {
                display_discriminant,
                doc: doc.clone(),
                editorial,
                example_json,
            });
    }

    // Sort each family's signals alphabetically by discriminant for deterministic output.
    for signals in by_family.values_mut() {
        signals.sort_by(|a, b| a.doc.discriminant.cmp(&b.doc.discriminant));
    }

    Ok(MergedSignals {
        version: toml.meta.version,
        core_12,
        by_family,
    })
}

/// Build a representative JSON snippet for a signal.
///
/// For signals in SignalSpec, uses the known default values from signal_spec.rs.
/// For physics primitives (is_parallel_channel=true), uses a descriptive placeholder.
/// Composition signals that take sub-signals use `{...}` as sub-signal placeholders.
fn build_example_json(doc: &SignalDoc) -> String {
    if doc.is_parallel_channel {
        // Physics primitives are not JSON-deserializable via SignalSpec today.
        return format!(
            "// {} is in the parallel physics channel (not SignalSpec).\n// See DampedSpring struct docs for field shape.",
            doc.struct_name
        );
    }

    let discriminant = &doc.discriminant;

    // Build the JSON from the known SignalSpec field defaults.
    // This avoids runtime SignalSpec instantiation while still producing accurate examples.
    match discriminant.as_str() {
        "sine" => r#"{"type": "sine", "frequency": 1.0, "amplitude": 1.0, "offset": 0.0, "phase": 0.0}"#.to_string(),
        "triangle" => r#"{"type": "triangle", "frequency": 1.0, "amplitude": 1.0, "offset": 0.0, "phase": 0.0}"#.to_string(),
        "square" => r#"{"type": "square", "frequency": 1.0, "amplitude": 1.0, "offset": 0.0, "phase": 0.0, "duty": 0.5}"#.to_string(),
        "sawtooth" => r#"{"type": "sawtooth", "frequency": 1.0, "amplitude": 1.0, "offset": 0.0, "phase": 0.0, "inverted": false}"#.to_string(),
        "constant" => r#"{"type": "constant", "value": 1.0}"#.to_string(),
        "ramp" => r#"{"type": "ramp", "start": 0.0, "end": 1.0, "duration": 1.0}"#.to_string(),
        "step" => r#"{"type": "step", "before": 0.0, "after": 1.0, "threshold": 0.5}"#.to_string(),
        "pulse" => r#"{"type": "pulse", "low": 0.0, "high": 1.0, "start": 0.0, "end": 0.5}"#.to_string(),
        "white_noise" => r#"{"type": "white_noise", "seed": 0, "amplitude": 1.0, "sample_rate": 60.0}"#.to_string(),
        "perlin" => r#"{"type": "perlin", "seed": 0, "scale": 1.0, "amplitude": 1.0, "octaves": 1, "persistence": 0.5}"#.to_string(),
        "seeded_random" => r#"{"type": "seeded_random", "seed": 0, "amplitude": 1.0, "offset": 0.0}"#.to_string(),
        "spatial_noise" => r#"{"type": "spatial_noise", "seed": 0, "frequency": 1.0, "amplitude": 1.0}"#.to_string(),
        "gaussian_noise" => r#"{"type": "gaussian_noise", "seed": 0, "std_dev": 0.3, "amplitude": 1.0, "offset": 0.0}"#.to_string(),
        "poisson_noise" => r#"{"type": "poisson_noise", "seed": 0, "lambda": 1.0, "amplitude": 1.0, "offset": 0.0}"#.to_string(),
        "correlated_noise" => r#"{"type": "correlated_noise", "seed": 0, "correlation": 0.9, "amplitude": 1.0, "offset": 0.0}"#.to_string(),
        "pink_noise" => r#"{"type": "pink_noise", "seed": 0, "amplitude": 1.0, "offset": 0.0}"#.to_string(),
        "per_character_noise" => r#"{"type": "per_character_noise", "base_seed": 0, "amplitude": 1.0, "offset": 0.0}"#.to_string(),
        "student_t_noise" => r#"{"type": "student_t_noise", "seed": 0, "degrees_of_freedom": 3.0, "scale": 1.0, "amplitude": 1.0, "offset": 0.0}"#.to_string(),
        "impulse_noise" => r#"{"type": "impulse_noise", "seed": 0, "rate_hz": 4.0, "impulse_width": 0.05}"#.to_string(),
        "sample_norm_x" => r#"{"type": "sample_norm_x"}"#.to_string(),
        "sample_norm_y" => r#"{"type": "sample_norm_y"}"#.to_string(),
        "sample_cell_x" => r#"{"type": "sample_cell_x"}"#.to_string(),
        "sample_cell_y" => r#"{"type": "sample_cell_y"}"#.to_string(),
        "sample_centered_x" => r#"{"type": "sample_centered_x"}"#.to_string(),
        "sample_centered_y" => r#"{"type": "sample_centered_y"}"#.to_string(),
        "sample_radius" => r#"{"type": "sample_radius"}"#.to_string(),
        "sample_angle" => r#"{"type": "sample_angle"}"#.to_string(),
        "sample_surface_centered_x" => r#"{"type": "sample_surface_centered_x"}"#.to_string(),
        "sample_surface_centered_y" => r#"{"type": "sample_surface_centered_y"}"#.to_string(),
        "sample_surface_radius" => r#"{"type": "sample_surface_radius"}"#.to_string(),
        "sample_surface_radius_from" => r#"{"type": "sample_surface_radius_from", "x": 0.5, "y": 0.5}"#.to_string(),
        "sample_cell_radius_from" => r#"{"type": "sample_cell_radius_from", "x": 0.5, "y": 0.5}"#.to_string(),
        "sample_surface_angle_from" => r#"{"type": "sample_surface_angle_from", "x": 0.5, "y": 0.5}"#.to_string(),
        "adsr" => r#"{"type": "adsr", "attack": 0.1, "decay": 0.1, "sustain": 0.7, "release": 0.2, "peak": 1.0}"#.to_string(),
        "impact" => r#"{"type": "impact", "intensity": 1.0, "decay": 4.0}"#.to_string(),
        "linear_envelope" => r#"{"type": "linear_envelope", "attack": 0.1, "release": 0.2, "peak": 1.0}"#.to_string(),
        "add" => r#"{"type": "add", "a": {"type": "sine", "frequency": 1.0}, "b": {"type": "constant", "value": 0.5}}"#.to_string(),
        "multiply" => r#"{"type": "multiply", "a": {"type": "sine", "frequency": 1.0}, "b": {"type": "constant", "value": 0.5}}"#.to_string(),
        "mix" => r#"{"type": "mix", "a": {"type": "sine", "frequency": 1.0}, "b": {"type": "triangle", "frequency": 2.0}, "mix": 0.5}"#.to_string(),
        "weighted_mix" => r#"{"type": "weighted_mix", "a": {"type": "sine", "frequency": 1.0}, "b": {"type": "triangle", "frequency": 2.0}, "weight_a": 0.7, "weight_b": 0.3}"#.to_string(),
        "frequency_mod" => r#"{"type": "frequency_mod", "carrier": {"type": "sine", "frequency": 1.0}, "modulator": {"type": "sine", "frequency": 4.0}, "depth": 1.0, "carrier_freq": 1.0}"#.to_string(),
        "vca_centered" => r#"{"type": "vca_centered", "carrier": {"type": "sine", "frequency": 1.0}, "amplitude": {"type": "adsr", "attack": 0.1, "decay": 0.1, "sustain": 0.7, "release": 0.2}}"#.to_string(),
        "phase_accumulator" => r#"{"type": "phase_accumulator", "frequency": {"type": "constant", "value": 1.0}, "initial_phase": 0.0}"#.to_string(),
        "phase_sine" => r#"{"type": "phase_sine", "phase": {"type": "phase_accumulator", "frequency": {"type": "constant", "value": 1.0}}}"#.to_string(),
        "keyframes" => r#"{"type": "keyframes", "keyframes": [[0.0, 0.0], [0.5, 1.0], [1.0, 0.0]]}"#.to_string(),
        "clamp" => r#"{"type": "clamp", "signal": {"type": "sine", "frequency": 1.0}, "min": 0.0, "max": 1.0}"#.to_string(),
        "quantize" => r#"{"type": "quantize", "signal": {"type": "sine", "frequency": 1.0}, "levels": 8}"#.to_string(),
        "remap" => r#"{"type": "remap", "signal": {"type": "sine", "frequency": 1.0}, "in_min": -1.0, "in_max": 1.0, "out_min": 0.0, "out_max": 1.0}"#.to_string(),
        "invert" => r#"{"type": "invert", "signal": {"type": "sine", "frequency": 1.0}}"#.to_string(),
        "abs" => r#"{"type": "abs", "signal": {"type": "sine", "frequency": 1.0}}"#.to_string(),
        // Deprecated variants — still in catalog, note the replacement.
        "scale" => r#"{"type": "scale", "a": {"type": "sine", "frequency": 1.0}, "b": {"type": "constant", "value": 0.5}}"#.to_string(),
        "sum" => r#"{"type": "sum", "a": {"type": "sine", "frequency": 1.0}, "b": {"type": "constant", "value": 0.5}}"#.to_string(),
        // Linear/exponential decay envelopes.
        "linear_decay" => r#"{"type": "linear_decay", "duration": 1.0}"#.to_string(),
        "exponential_decay" => r#"{"type": "exponential_decay", "rate": 4.0}"#.to_string(),
        _ => format!(r#"{{"type": "{}"}}"#, discriminant),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{extract_signals_rustdoc, parse_signals_toml};

    #[test]
    fn merge_produces_core_12_in_order() {
        let rustdoc = extract_signals_rustdoc::extract().expect("extraction failed");
        let toml = parse_signals_toml::parse().expect("parse failed");
        let merged = merge(rustdoc, toml).expect("merge failed");

        assert_eq!(merged.core_12.len(), 12, "Core 12 should have 12 entries");
        assert_eq!(
            merged.core_12[0].doc.struct_name, "Sine",
            "First Core 12 entry should be Sine"
        );
    }

    #[test]
    fn merge_all_families_present() {
        let rustdoc = extract_signals_rustdoc::extract().expect("extraction failed");
        let toml = parse_signals_toml::parse().expect("parse failed");
        let merged = merge(rustdoc, toml).expect("merge failed");

        let families: Vec<SignalFamily> = merged.by_family.keys().copied().collect();
        assert!(families.contains(&SignalFamily::Oscillator), "Oscillator family missing");
        assert!(families.contains(&SignalFamily::Envelope), "Envelope family missing");
        assert!(families.contains(&SignalFamily::Noise), "Noise family missing");
        assert!(families.contains(&SignalFamily::Composition), "Composition family missing");
        assert!(families.contains(&SignalFamily::Processing), "Processing family missing");
    }
}

// <FILE>xtask/src/docs/merge_signals.rs</FILE> - <DESC>Merge Signal-impl rustdoc with signals.toml editorial overlay</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
