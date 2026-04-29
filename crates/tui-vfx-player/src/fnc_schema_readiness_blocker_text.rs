// <FILE>crates/tui-vfx-player/src/fnc_schema_readiness_blocker_text.rs</FILE> - <DESC>Describe schema-readiness blocker actions</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.11 schema readiness: keep blocker action text stable and centralized.</WCTX>
// <CLOG>0.1.0: INIT — isolate blocker decision and packet labels from grouping.</CLOG>

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
        "sourceDescriptor" => "K2.12 source/content descriptor expansion tranche",
        "descriptorPack" => "K2.12 descriptor-pack vocabulary tranche",
        "fieldCoverage" => "K2.12 primitive descriptor/field-coverage closure tranche",
        "valueSourceSemantics"
        | "bindingSemantics"
        | "lifecycleSemantics"
        | "motionTimingSemantics" => {
            "K2.12 lifecycle/signal/binding/value-source schema decision packet"
        }
        "sceneSemantics" => "K2.12 scene/source-local pipeline schema decision packet",
        "schemaModel" => "K2.12 schema/model decision packet",
        "playerAdapter" => "K2.12 player adapter support tranche",
        "backendRenderer" => "K2.12 backend renderer boundary tranche",
        "duplicateOrVariant" | "oracleOnly" => "K2.11 owner-audit signoff",
        _ => "K2.12 owner-audit normalization tranche",
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_schema_readiness_blocker_text.rs</FILE> - <DESC>Describe schema-readiness blocker actions</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
