// <FILE>xtask/src/audit/fnc_find_justification.rs</FILE> - <DESC>Find and parse a CONFIGSCHEMA-JUSTIFICATION comment above an impl line</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Packet 1.9.A — ConfigSchema justification lint</WCTX>
// <CLOG>1.0.0: initial implementation — supports // and /// comment forms</CLOG>

/// The result of inspecting the lines above an `impl ConfigSchema for X`.
#[derive(Debug, Clone, PartialEq)]
pub enum Justification {
    /// A recognised canonical kind, e.g. `DeriveCannnotHandleForeignType`.
    Canonical(CanonicalKind),
    /// The `Other("...")` escape hatch — passes but emits a CI warning.
    Other(String),
    /// A justification marker was found but the kind string is not recognised.
    UnrecognizedKind(String),
}

/// Canonical exception kinds for hand-written `impl ConfigSchema for X`.
///
/// Adding a new kind is a one-line patch here plus a docs update.
#[derive(Debug, Clone, PartialEq)]
pub enum CanonicalKind {
    /// Type lives in another crate; orphan rules forbid `#[derive]`.
    DeriveCannnotHandleForeignType,
    /// Generic type with `ConfigSchema` bounds the macro does not currently emit.
    DeriveCannotHandleGenericT,
    /// Enum uses `serde(untagged)` and the macro does not emit untagged-walk schemas.
    DeriveCannotHandleUntaggedEnum,
    /// The type's variants reference another crate's types in a way that breaks
    /// the derive's import resolution.
    DeriveCannotHandleCrossCreateTraitDep,
    /// The hand-written impl produces a schema that intentionally differs from
    /// what derive would emit.
    IntentionalDivergenceFromDeriveOutput,
    /// The type IS a primitive or thin wrapper; the schema is a `SchemaNode::Primitive` literal.
    PrimitiveBridge,
    /// The type has no constructable values; schema is an empty `SchemaNode::Enum`.
    UninhabitedType,
}

const MARKER: &str = "CONFIGSCHEMA-JUSTIFICATION:";

/// Inspect lines above `impl_line` (1-based) in `source_lines` for a
/// `CONFIGSCHEMA-JUSTIFICATION:` comment.
///
/// Accepted forms:
/// 1. `// CONFIGSCHEMA-JUSTIFICATION: <kind>` on the line immediately
///    preceding the `impl` line (no blank line allowed between).
/// 2. A `///` doc-comment block immediately preceding the `impl` line where
///    at least one line contains `CONFIGSCHEMA-JUSTIFICATION:`.
///
/// Returns `None` when no marker is found.
pub fn find_justification(source_lines: &[&str], impl_line: usize) -> Option<Justification> {
    // `impl_line` is 1-based; the line immediately above is index `impl_line - 2`.
    if impl_line < 2 {
        return None;
    }

    // Walk backwards from the line above the impl, collecting comment lines.
    let mut kind_str: Option<String> = None;
    let mut idx = impl_line - 2; // 0-based index of the line above

    loop {
        let line = source_lines[idx].trim();

        // Accept both `//` and `///` comment forms.
        let content = if let Some(rest) = line.strip_prefix("///") {
            rest.trim()
        } else if let Some(rest) = line.strip_prefix("//") {
            rest.trim()
        } else {
            // Non-comment line encountered — stop scanning.
            break;
        };

        if let Some(after_marker) = content.strip_prefix(MARKER) {
            kind_str = Some(after_marker.trim().to_string());
            break;
        }

        // Continue walking up through adjacent comment lines.
        if idx == 0 {
            break;
        }
        idx -= 1;
    }

    let raw = kind_str?;
    Some(parse_kind(&raw))
}

fn parse_kind(raw: &str) -> Justification {
    // Strip an optional `:<prose>` suffix so callers can write
    // `derive-cannot-handle-foreign-type: SignalOrFloat lives in...`
    let kind_token = raw.split(':').next().unwrap_or(raw).trim();

    match kind_token {
        "derive-cannot-handle-foreign-type" => {
            Justification::Canonical(CanonicalKind::DeriveCannnotHandleForeignType)
        }
        "derive-cannot-handle-generic-T" => {
            Justification::Canonical(CanonicalKind::DeriveCannotHandleGenericT)
        }
        "derive-cannot-handle-untagged-enum" => {
            Justification::Canonical(CanonicalKind::DeriveCannotHandleUntaggedEnum)
        }
        "derive-cannot-handle-cross-crate-trait-dep" => {
            Justification::Canonical(CanonicalKind::DeriveCannotHandleCrossCreateTraitDep)
        }
        "intentional-divergence-from-derive-output" => {
            Justification::Canonical(CanonicalKind::IntentionalDivergenceFromDeriveOutput)
        }
        "primitive-bridge" => Justification::Canonical(CanonicalKind::PrimitiveBridge),
        "uninhabited-type" => Justification::Canonical(CanonicalKind::UninhabitedType),
        other if other.starts_with("Other(") && other.ends_with(')') => {
            let inner = other
                .strip_prefix("Other(")
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .trim_matches('"')
                .to_string();
            Justification::Other(inner)
        }
        _ => Justification::UnrecognizedKind(kind_token.to_string()),
    }
}

// <FILE>xtask/src/audit/fnc_find_justification.rs</FILE> - <DESC>Find and parse a CONFIGSCHEMA-JUSTIFICATION comment above an impl line</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
