// <FILE>xtask/src/audit/fnc_audit_configschema.rs</FILE> - <DESC>Public entrypoint for `cargo xtask audit configschema`</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Packet 1.9.A — ConfigSchema justification lint</WCTX>
// <CLOG>1.0.0: initial implementation — warn-only mode per Q2 default; promote-to-fail date 2026-07-01</CLOG>

use super::fnc_find_justification::{find_justification, Justification};
use super::fnc_load_baseline::load_baseline;
use super::fnc_scan_file_for_impls::scan_file_for_impls;
use anyhow::{Context, Result};
use owo_colors::OwoColorize;
use std::path::{Path, PathBuf};

/// Path to the baseline allowlist relative to the workspace root.
const BASELINE_PATH: &str = "xtask/data/configschema_baseline.toml";

/// Directories to skip when walking the workspace.
///
/// `xtask` is excluded because it is build-tooling; the test fixtures under
/// `xtask/tests/` contain intentionally unjustified impls for lint verification
/// and must not be scanned as production code.
const SKIP_DIRS: &[&str] = &["recyclebin", "target", ".git", "xtask"];

/// Promotion-to-fail date (warn-only until this date, per Q2 default).
const PROMOTE_TO_FAIL_DATE: &str = "2026-07-01";

/// Lint result for a single impl hit.
#[derive(Debug)]
enum HitOutcome {
    /// New impl with a canonical justification comment — passes.
    JustifiedCanonical,
    /// New impl with `Other("...")` justification — passes with a warning.
    JustifiedOther { reason: String },
    /// New impl with a justification marker but an unrecognised kind — fails.
    UnrecognizedKind { kind: String },
    /// New impl with no justification comment at all — fails.
    MissingJustification,
}

/// Diagnostic record for a failing or warning impl.
#[derive(Debug)]
struct Diagnostic {
    file: PathBuf,
    line: usize,
    type_name: String,
    outcome: HitOutcome,
}

/// Run the `audit configschema` lint against the workspace rooted at
/// `workspace_root`.
///
/// # Behaviour (Q2 warn-only mode)
///
/// - Baseline impls: silently pass.
/// - New impls with a canonical `CONFIGSCHEMA-JUSTIFICATION:` comment: pass.
/// - New impls with `Other(...)` justification: pass, emit a CI warning.
/// - New impls with an unrecognised kind: **fail** (a typo in the kind is an
///   error; use `Other("...")` as the escape hatch).
/// - New impls with no justification comment: **warn** today, will **fail**
///   after `PROMOTE_TO_FAIL_DATE` (2026-07-01). The warning message names the
///   playbook.
///
/// # Promotion to hard-fail
///
/// Change the `WARN_ONLY` constant below to `false` on or after
/// `PROMOTE_TO_FAIL_DATE` to promote the lint to hard-fail mode.
pub fn audit_configschema(workspace_root: &Path) -> Result<()> {
    /// Set to `false` on or after 2026-07-01 to promote the lint to
    /// hard-fail mode per the schedule in `docs/CONFIGSCHEMA_JUSTIFICATION.md`.
    const WARN_ONLY: bool = true;

    println!("{}", "Auditing hand-written impl ConfigSchema for X blocks...".bold());

    let baseline_path = workspace_root.join(BASELINE_PATH);
    let baseline = load_baseline(&baseline_path)?;

    let rust_files = collect_rust_files(workspace_root);

    let mut failures: Vec<Diagnostic> = Vec::new();
    let mut warnings: Vec<Diagnostic> = Vec::new();

    for file_path in rust_files {
        let source =
            std::fs::read_to_string(&file_path).with_context(|| {
                format!("Failed to read {}", file_path.display())
            })?;

        let source_lines: Vec<&str> = source.lines().collect();
        let hits = scan_file_for_impls(&source);

        for hit in hits {
            if hit.is_macro_body {
                continue;
            }

            // Build the baseline key using a path relative to workspace_root.
            let rel_path = file_path
                .strip_prefix(workspace_root)
                .unwrap_or(&file_path)
                .to_string_lossy()
                .replace('\\', "/");

            let in_baseline = baseline.contains(&(rel_path.clone(), hit.type_name.clone()));

            if in_baseline {
                continue; // grandfathered
            }

            let justification = find_justification(&source_lines, hit.line_number);

            let outcome = match justification {
                None => HitOutcome::MissingJustification,
                Some(Justification::Canonical(_)) => HitOutcome::JustifiedCanonical,
                Some(Justification::Other(reason)) => HitOutcome::JustifiedOther { reason },
                Some(Justification::UnrecognizedKind(kind)) => {
                    HitOutcome::UnrecognizedKind { kind }
                }
            };

            let diag = Diagnostic {
                file: file_path.clone(),
                line: hit.line_number,
                type_name: hit.type_name,
                outcome,
            };

            match &diag.outcome {
                HitOutcome::JustifiedCanonical => {}
                HitOutcome::JustifiedOther { .. } => warnings.push(diag),
                HitOutcome::UnrecognizedKind { .. } => failures.push(diag),
                HitOutcome::MissingJustification => {
                    if WARN_ONLY {
                        warnings.push(diag);
                    } else {
                        failures.push(diag);
                    }
                }
            }
        }
    }

    // Emit warnings.
    for w in &warnings {
        match &w.outcome {
            HitOutcome::JustifiedOther { reason } => {
                println!(
                    "  {} {}:{} — `impl ConfigSchema for {}` uses Other(\"{}\") justification. \
                     Consider filing a packet to add a canonical kind.",
                    "warning:".yellow().bold(),
                    w.file.display(),
                    w.line,
                    w.type_name,
                    reason,
                );
            }
            HitOutcome::MissingJustification => {
                println!(
                    "  {} {}:{} — `impl ConfigSchema for {}` has no justification comment. \
                     Add `// CONFIGSCHEMA-JUSTIFICATION: <kind>: <reason>` above the impl, \
                     or migrate to `#[derive(ConfigSchema)]`. \
                     This warning becomes a hard failure after {}.",
                    "warning:".yellow().bold(),
                    w.file.display(),
                    w.line,
                    w.type_name,
                    PROMOTE_TO_FAIL_DATE,
                );
            }
            _ => {}
        }
    }

    // Emit failures.
    for f in &failures {
        match &f.outcome {
            HitOutcome::UnrecognizedKind { kind } => {
                println!(
                    "  {} {}:{} — `impl ConfigSchema for {}` has an unrecognised justification \
                     kind `{}`. Use a canonical kind or `Other(\"...\")` as an escape hatch. \
                     See docs/CONFIGSCHEMA_JUSTIFICATION.md for the canonical kinds list.",
                    "error:".red().bold(),
                    f.file.display(),
                    f.line,
                    f.type_name,
                    kind,
                );
            }
            HitOutcome::MissingJustification => {
                println!(
                    "  {} {}:{} — `impl ConfigSchema for {}` has no justification comment. \
                     Add `// CONFIGSCHEMA-JUSTIFICATION: <kind>: <reason>` above the impl, \
                     or migrate to `#[derive(ConfigSchema)]`. \
                     See docs/CONFIGSCHEMA_JUSTIFICATION.md.",
                    "error:".red().bold(),
                    f.file.display(),
                    f.line,
                    f.type_name,
                );
            }
            _ => {}
        }
    }

    if warnings.is_empty() && failures.is_empty() {
        println!("{}", "✓ All impl ConfigSchema for X blocks are justified or baselined.".green().bold());
    } else if !failures.is_empty() {
        println!(
            "\n{} {} unjustified or misconfigured impl(s) found.",
            "FAIL:".red().bold(),
            failures.len()
        );
        anyhow::bail!(
            "{} unjustified or misconfigured impl ConfigSchema for X block(s). \
             See above for details.",
            failures.len()
        );
    } else {
        println!(
            "\n{} {} warning(s). The lint passed (warn-only mode until {}).",
            "WARN:".yellow().bold(),
            warnings.len(),
            PROMOTE_TO_FAIL_DATE,
        );
    }

    Ok(())
}

/// Walk the workspace root and collect all `.rs` source files, excluding
/// the directories named in `SKIP_DIRS`.
fn collect_rust_files(workspace_root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(workspace_root)
        .into_iter()
        .filter_entry(|e| {
            // Skip top-level directories in SKIP_DIRS.
            let name = e.file_name().to_string_lossy();
            !SKIP_DIRS.iter().any(|skip| *skip == name.as_ref())
        })
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            files.push(path.to_path_buf());
        }
    }
    files.sort();
    files
}

// <FILE>xtask/src/audit/fnc_audit_configschema.rs</FILE> - <DESC>Public entrypoint for `cargo xtask audit configschema`</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
