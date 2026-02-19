// <FILE>xtask/src/main.rs</FILE> - <DESC>CLI entry point for xtask build tooling</DESC>
// <VERS>VERSION: 1.3.0</VERS>
// <WCTX>Recipe validation tooling</WCTX>
// <CLOG>Add recipes validation subcommand</CLOG>

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
// <VERS>END OF VERSION: 1.3.0</VERS>
