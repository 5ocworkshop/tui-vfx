// <FILE>tui-vfx/tests/test_full_schema_dump.rs</FILE>
// <DESC>Dump complete schema for all major Spec types</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Schema investigation for GTD theme integration</WCTX>
// <CLOG>Initial implementation</CLOG>

use tui_vfx::prelude::*;

#[test]
fn dump_full_schema_to_stdout() {
    println!("\n\n=== MASK SPEC ===");
    println!("{}", MaskSpec::schema().to_json_pretty());

    println!("\n\n=== FILTER SPEC ===");
    println!("{}", FilterSpec::schema().to_json_pretty());

    println!("\n\n=== SAMPLER SPEC ===");
    println!("{}", SamplerSpec::schema().to_json_pretty());

    println!("\n\n=== STYLE EFFECT ===");
    println!("{}", StyleEffect::schema().to_json_pretty());

    println!("\n\n=== FADE SPEC ===");
    println!("{}", FadeSpec::schema().to_json_pretty());
}

#[test]
fn dump_json_schema_draft07() {
    use tui_vfx_core::schema::to_json_schema;

    println!("\n\n=== MASK SPEC (JSON Schema draft-07) ===");
    let schema = to_json_schema(&MaskSpec::schema(), "MaskSpec");
    // Schema is already a serde_json::Value, use Debug formatting
    println!("{:#?}", schema);
}

// <FILE>tui-vfx/tests/test_full_schema_dump.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
