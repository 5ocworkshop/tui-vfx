// <FILE>crates/tui-vfx-player/src/cls_legacy_migration_mapping_evidence_collector.rs</FILE> - <DESC>Legacy migration mapping evidence collector</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>K2.13 field coverage closure: stop treating accepted gradient and position fields as undecided.</WCTX>
// <CLOG>0.4.0: MINOR — accept gradient and position value-source fields for migration evidence.</CLOG>

use std::collections::BTreeSet;

use serde_json::Value;

use crate::{
    LegacyMigrationMappingEvidence,
    fnc_build_legacy_migration_mapping_summary::build_legacy_migration_mapping_summary,
    fnc_legacy_migration_mapping_names::{
        canonical_legacy_field, content_descriptor_id_for_content_effect, legacy_descriptor_id,
        lower_camel,
    },
};

/// Mutable collector for conservative evidence extracted from legacy recipe JSON.
#[derive(Default)]
pub(crate) struct LegacyMigrationMappingEvidenceCollector {
    descriptors: BTreeSet<String>,
    sources: BTreeSet<String>,
    fields: BTreeSet<String>,
    signals: BTreeSet<String>,
    bindings: BTreeSet<String>,
    value_source_decision_fields: BTreeSet<String>,
    source_kinds: BTreeSet<String>,
    effect_families: BTreeSet<String>,
}

impl LegacyMigrationMappingEvidenceCollector {
    /// Visit a legacy JSON value recursively.
    pub(crate) fn visit(&mut self, value: &Value) {
        match value {
            Value::Object(object) => {
                self.collect_key_evidence(object);
                self.collect_effect_evidence(object);
                for child in object.values() {
                    self.visit(child);
                }
            }
            Value::Array(items) => {
                for item in items {
                    self.visit(item);
                }
            }
            _ => {}
        }
    }

    /// Add family-derived source candidates after JSON traversal.
    pub(crate) fn add_family_source_candidates(&mut self, family: &str) {
        match family {
            "content" if self.sources.is_empty() => self.add_source("source.text", "text"),
            "fixtures" => {
                self.add_source("source.commandCaptureArtifact", "commandCaptureArtifact")
            }
            "scene" => self.add_source("source.scene", "scene"),
            _ => self.add_source("source.card", "card"),
        }
    }

    /// Finish collection into an immutable evidence DTO.
    pub(crate) fn finish(self) -> LegacyMigrationMappingEvidence {
        let required_descriptor_ids = sorted(self.descriptors);
        let required_source_ids = sorted(self.sources);
        let legacy_effect_families = sorted(self.effect_families);
        LegacyMigrationMappingEvidence {
            summary: build_legacy_migration_mapping_summary(
                &required_descriptor_ids,
                &required_source_ids,
                &legacy_effect_families,
            ),
            required_descriptor_ids,
            required_source_ids,
            required_input_fields: sorted(self.fields),
            value_source_decision_fields: sorted(self.value_source_decision_fields),
            legacy_signals: sorted(self.signals),
            legacy_bindings: sorted(self.bindings),
            legacy_source_kinds: sorted(self.source_kinds),
            legacy_effect_families,
        }
    }

    fn collect_key_evidence(&mut self, object: &serde_json::Map<String, Value>) {
        for key in object.keys() {
            self.collect_signal_or_binding_key(key);
        }
        if let Some(Value::Object(content)) = object.get("content")
            && let Some(Value::Object(effect)) = content.get("effect")
            && let Some(content_type) = effect.get("type").and_then(Value::as_str)
        {
            self.descriptors
                .insert(content_descriptor_id_for_content_effect(content_type));
            self.add_source("source.text", &lower_camel(content_type));
        }
        if let Some(Value::Object(content)) = object.get("content")
            && content.contains_key("glyph_emitters")
        {
            self.descriptors
                .insert(content_descriptor_id_for_content_effect("glyph_particles"));
            self.add_source("source.text", "glyphParticles");
        }
    }

    fn collect_signal_or_binding_key(&mut self, key: &str) {
        if key.contains("signal") {
            self.signals.insert(canonical_legacy_field(key));
        }
        if key.contains("binding") || key.contains("bindable") || key == "bindings" {
            self.bindings.insert(canonical_legacy_field(key));
        }
    }

    fn collect_effect_evidence(&mut self, object: &serde_json::Map<String, Value>) {
        let Some(kind) = object.get("kind").and_then(Value::as_str) else {
            return;
        };
        let Some(payload) = object.get("payload").and_then(Value::as_object) else {
            return;
        };
        let Some(effect_type) = payload.get("type").and_then(Value::as_str) else {
            return;
        };
        self.effect_families.insert(kind.to_string());
        if let Some(descriptor_id) = legacy_descriptor_id(kind, effect_type) {
            self.descriptors.insert(descriptor_id);
        }
        for (key, value) in payload.iter().filter(|(key, _)| key.as_str() != "type") {
            let field = canonical_legacy_field(key);
            if needs_value_source_decision(&field, value) {
                self.value_source_decision_fields.insert(field.clone());
            }
            self.fields.insert(field);
        }
    }

    fn add_source(&mut self, source: &str, kind: &str) {
        self.sources.insert(source.to_string());
        self.source_kinds.insert(kind.to_string());
    }
}

fn needs_value_source_decision(field: &str, value: &Value) -> bool {
    if matches!(field, "gradient" | "position") {
        return false;
    }
    match value {
        Value::Object(object) => {
            object.contains_key("type")
                || object.contains_key("signal")
                || object.contains_key("binding")
                || object.contains_key("remap")
                || object
                    .values()
                    .any(|child| needs_value_source_decision(field, child))
        }
        Value::Array(items) => items
            .iter()
            .any(|child| needs_value_source_decision(field, child)),
        _ => false,
    }
}

fn sorted(values: BTreeSet<String>) -> Vec<String> {
    values.into_iter().collect()
}

// <FILE>crates/tui-vfx-player/src/cls_legacy_migration_mapping_evidence_collector.rs</FILE> - <DESC>Legacy migration mapping evidence collector</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
