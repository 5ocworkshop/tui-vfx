// <FILE>crates/tui-vfx-player/src/fnc_schema_readiness_blocker_text.rs</FILE> - <DESC>Describe schema-readiness blocker actions</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>K2.12 schema lock: keep blocker action labels forward-looking after the decision sprint.</WCTX>
// <CLOG>0.2.0: MINOR — update next-packet labels to durable decision lanes.
// 0.1.0: INIT — isolate blocker decision and packet labels from grouping.</CLOG>

pub(crate) fn schema_readiness_blocking_decision(kind: &str) -> &'static str {
    match kind {
        "sourceDescriptor" => "decide source descriptor and adapter coverage",
        "descriptorPack" => "decide descriptor-pack vocabulary expansion",
        "fieldCoverage" => "decide whether authored fields are descriptor or schema blockers",
        "schemaModel" => "settle missing schema/model semantics",
        "motionTimingSemantics" => "settle easing and motion-route timing semantics",
        "valueSourceSemantics" => "settle value-source and signal semantics",
        "bindingSemantics" => "settle binding execution semantics",
        "lifecycleSemantics" => "settle lifecycle, trigger, and timing semantics",
        "sceneSemantics" => "settle scene/source-local pipeline semantics",
        "playerAdapter" => "add or defer player adapter support",
        "ownerAudit" => "classify owner-audit records into explicit blocker kinds",
        "backendRenderer" => "decide backend renderer or compositor support boundary",
        "duplicateOrVariant" => "confirm duplicate or variant disposition",
        "oracleOnly" => "confirm oracle-only disposition",
        _ => "perform owner audit and classify blocker",
    }
}

pub(crate) fn schema_readiness_next_packet(kind: &str) -> &'static str {
    match kind {
        "sourceDescriptor" => "source/content descriptor decision packet",
        "descriptorPack" => "descriptor-pack vocabulary expansion packet",
        "fieldCoverage" => "primitive descriptor/field-coverage closure packet",
        "valueSourceSemantics"
        | "bindingSemantics"
        | "lifecycleSemantics"
        | "motionTimingSemantics" => "runtime dynamism schema decision packet",
        "sceneSemantics" => "scene/source-local pipeline schema decision packet",
        "schemaModel" => "schema/model decision packet",
        "playerAdapter" => "player adapter support tranche",
        "backendRenderer" => "backend renderer boundary signoff",
        "guiHumanReview" => "GUI/human-review holdback signoff",
        "duplicateOrVariant" | "oracleOnly" => "owner-audit holdback signoff",
        _ => "owner-audit normalization tranche",
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_schema_readiness_blocker_text.rs</FILE> - <DESC>Describe schema-readiness blocker actions</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
