// <FILE>xtask/src/docs/validate_signals.rs</FILE> - <DESC>Validate signals.toml entries against the autogen catalog</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase α + β: signal-facade — signals pipeline validation gate</WCTX>
// <CLOG>0.1.0: initial implementation — three checks: (1) editorial entries name real signals, (2) Core 12 entries are in the catalog, (3) warning when Core 12 entries lack a recipe_hint</CLOG>

use super::extract_signals_rustdoc::SignalsRustdocData;
use super::parse_signals_toml::SignalsManifest;
use anyhow::Result;

/// Validate `signals.toml` against the autogen catalog.
///
/// Three checks:
/// 1. Every `[signals.<name>]` entry names a signal that exists in the autogen catalog.
///    Failure is an error; message names the unknown discriminant and suggests checking
///    mixed-signals source for the real name.
/// 2. Every name in `[core_12].order` is a valid discriminant in the catalog.
///    `spring` (DampedSpring) is accepted even though it maps to the parallel channel
///    because it is documented with an is_parallel_channel callout.
/// 3. (Warning, not error) Every Core 12 entry should have a `[signals.<name>]` block
///    with at least a `recipe_hint`. Missing hints are logged to stderr but do not fail.
pub fn validate(data: &SignalsRustdocData, toml: &SignalsManifest) -> Result<()> {
    // Check 1 — every editorial entry names a real signal.
    for name in toml.signals.keys() {
        if !is_known(data, name) {
            anyhow::bail!(
                "signals.toml [signals.{name}] names a signal that does not exist in \
                 the mixed-signals autogen catalog.\n\
                 Did you mean one of: {}?\n\
                 Check mixed-signals source for the correct SignalSpec discriminant.",
                suggest_similar(data, name)
            );
        }
    }

    // Check 2 — every Core 12 entry is a valid discriminant (or the spring exception).
    for name in &toml.core_12.order {
        if !is_known(data, name) {
            anyhow::bail!(
                "signals.toml [core_12].order contains `{name}` which is not found in \
                 the autogen catalog.\n\
                 Valid discriminants include: {}",
                sample_discriminants(data, 10)
            );
        }
    }

    // Check 3 — warn (not fail) when Core 12 entries lack recipe_hint.
    for name in &toml.core_12.order {
        if let Some(entry) = toml.signals.get(name) {
            if entry.recipe_hint.is_none() {
                eprintln!(
                    "Warning: Core 12 signal `{name}` has a [signals] entry but no \
                     recipe_hint. Consider adding one for richer cheatsheet output."
                );
            }
        } else {
            eprintln!(
                "Warning: Core 12 signal `{name}` has no editorial entry in signals.toml. \
                 The cheatsheet will use rustdoc-only output for this entry."
            );
        }
    }

    Ok(())
}

/// Check whether `name` is a known discriminant in the catalog.
///
/// Special case: "spring" is accepted even though DampedSpring's catalog key
/// is "damped_spring" — it is one of the Core 12 and documented with a
/// parallel-channel callout. The editorial overlay uses "spring" as a
/// shorthand; the generator resolves it to the DampedSpring entry.
fn is_known(data: &SignalsRustdocData, name: &str) -> bool {
    if data.by_discriminant.contains_key(name) {
        return true;
    }
    // Accept "spring" as an alias for "damped_spring" (parallel channel).
    if name == "spring" && data.by_discriminant.contains_key("damped_spring") {
        return true;
    }
    false
}

/// Build a comma-separated suggestion list of discriminants similar to `name`.
/// Falls back to a sample of known discriminants when no prefix match exists,
/// so error messages always carry actionable hints.
fn suggest_similar(data: &SignalsRustdocData, name: &str) -> String {
    let suggestions: Vec<String> = data
        .by_discriminant
        .keys()
        .filter(|k| k.starts_with(&name[..name.len().min(3)]) || name.contains(k.as_str()))
        .take(5)
        .cloned()
        .collect();
    if suggestions.is_empty() {
        return sample_discriminants(data, 5);
    }
    suggestions.join(", ")
}

/// Return a sample of discriminant names for error messages.
fn sample_discriminants(data: &SignalsRustdocData, n: usize) -> String {
    data.by_discriminant
        .keys()
        .take(n)
        .cloned()
        .collect::<Vec<_>>()
        .join(", ")
}

// <FILE>xtask/src/docs/validate_signals.rs</FILE> - <DESC>Validate signals.toml entries against the autogen catalog</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
