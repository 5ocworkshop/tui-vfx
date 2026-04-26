// <FILE>xtask/src/main.rs</FILE> - <DESC>CLI entry point for xtask build tooling</DESC>
// <VERS>VERSION: 1.5.0</VERS>
// <WCTX>Phase α + β: signal-facade — add signals pipeline subcommands</WCTX>
// <CLOG>1.5.0: add Signals, SignalsCheck, SignalsValidate DocsAction variants; wire to docs::signals(), docs::signals_check(), docs::signals_validate()</CLOG>

mod audit;
mod docs;
mod recipes;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::Path;

/// tui-vfx build tooling
#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Build tooling for tui-vfx", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Audit gates — policy validation for the workspace.
    Audit {
        #[command(subcommand)]
        action: AuditAction,
    },
    /// Documentation generation and validation
    Docs {
        #[command(subcommand)]
        action: DocsAction,
    },
    /// Recipe validation tooling
    Recipes {
        #[command(subcommand)]
        action: RecipesAction,
    },
}

/// Subcommands for `cargo xtask audit`.
#[derive(Subcommand)]
enum AuditAction {
    /// Verify every hand-written `impl ConfigSchema for X` has a justification
    /// comment, or is in the baseline allowlist at `xtask/data/configschema_baseline.toml`.
    ///
    /// See `docs/CONFIGSCHEMA_JUSTIFICATION.md` for the format spec and
    /// canonical exception kinds.
    Configschema,
}

#[derive(Subcommand)]
enum DocsAction {
    /// Generate all documentation from rustdoc + TOML sources (CAPABILITIES.md)
    Generate,

    /// Check that generated docs are up-to-date (for CI)
    Check,

    /// Generate only the AI context prompt
    AiContext,

    /// Generate only CAPABILITIES.md
    Markdown,

    /// Validate capabilities.toml covers all code variants
    Validate,

    /// Generate TOML stubs for undocumented effects
    Scaffold {
        /// Write stubs directly to capabilities.toml instead of stdout
        #[arg(long)]
        write: bool,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // API DOCUMENTATION
    // ═══════════════════════════════════════════════════════════════════════════
    /// Generate API.md from code + api_docs.toml
    Api,

    /// Check that API.md is up-to-date (for CI)
    ApiCheck,

    /// Validate api_docs.toml covers all public types
    ApiValidate,

    /// Generate TOML stubs for undocumented API types
    ApiScaffold {
        /// Write stubs directly to api_docs.toml instead of stdout
        #[arg(long)]
        write: bool,
    },

    // ═══════════════════════════════════════════════════════════════════════════
    // SIGNAL REFERENCE DOCUMENTATION (phase α + β)
    // ═══════════════════════════════════════════════════════════════════════════
    /// Generate SIGNALS_REFERENCE.md from mixed-signals rustdoc + signals.toml overlay
    Signals,

    /// Check that SIGNALS_REFERENCE.md is up-to-date (for CI)
    SignalsCheck,

    /// Validate signals.toml: every named signal exists in mixed-signals,
    /// and every Core 12 entry is in the autogen catalog
    SignalsValidate,
}

#[derive(Subcommand)]
enum RecipesAction {
    /// Validate recipes against capabilities.json
    Validate {
        /// Directory containing recipe JSON files
        #[arg(long)]
        recipes_dir: String,
        /// Output directory for reports
        #[arg(long, default_value = "docs/generated")]
        output_dir: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Audit { action } => match action {
            AuditAction::Configschema => {
                let workspace_root = Path::new(
                    std::env::var("CARGO_MANIFEST_DIR")
                        .ok()
                        .as_deref()
                        .unwrap_or("."),
                )
                .parent()
                .unwrap_or(Path::new("."))
                .to_path_buf();
                // Resolve to canonical path so strip_prefix works correctly.
                let workspace_root = workspace_root
                    .canonicalize()
                    .unwrap_or(workspace_root);
                audit::audit_configschema(&workspace_root)
            }
        },
        Commands::Docs { action } => match action {
            // CAPABILITIES.md generation
            DocsAction::Generate => docs::generate(),
            DocsAction::Check => docs::check(),
            DocsAction::AiContext => docs::ai_context(),
            DocsAction::Markdown => docs::markdown(),
            DocsAction::Validate => docs::validate(),
            DocsAction::Scaffold { write } => docs::scaffold_toml(write),
            // API.md generation
            DocsAction::Api => docs::api(),
            DocsAction::ApiCheck => docs::api_check(),
            DocsAction::ApiValidate => docs::api_validate(),
            DocsAction::ApiScaffold { write } => docs::api_scaffold(write),
            // SIGNALS_REFERENCE.md generation (phase α + β)
            DocsAction::Signals => docs::signals(),
            DocsAction::SignalsCheck => docs::signals_check(),
            DocsAction::SignalsValidate => docs::signals_validate(),
        },
        Commands::Recipes { action } => match action {
            RecipesAction::Validate {
                recipes_dir,
                output_dir,
            } => recipes::validate(Path::new(&recipes_dir), Path::new(&output_dir)),
        },
    }
}

// <FILE>xtask/src/main.rs</FILE> - <DESC>CLI entry point for xtask build tooling</DESC>
// <VERS>END OF VERSION: 1.5.0</VERS>
