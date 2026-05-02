// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_resolve_scope.rs</FILE> - <DESC>Pattern-match author-side scope shorthand into canonical ScopeSpec, splitting out writeChannels lift</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2 of canonicalize: scope shape map + Gap 1 lift of channel keys out of scope into NodeSpec.writeChannels.</WCTX>
// <CLOG>0.1.0: INIT — handle role/content/rect/cell/rowRange/columnRange/rows/columns/moduloRows/moduloColumns/string scopes plus channel(s) extraction.</CLOG>

use serde_json::{Map, Value, json};

use super::cls_canonicalization_error::{CanonicalizationError, CanonicalizationErrorKind};

/// Result of resolving an author-side scope shorthand: a canonical `ScopeSpec`
/// (or `None` if the author wrote no remaining scope keys after channel extraction)
/// plus any cell channels the author wrote inside the scope object that should
/// lift to `NodeSpec.writeChannels`.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedScope {
    pub scope: Option<Value>,
    pub write_channels: Vec<String>,
}

impl ResolvedScope {
    pub fn empty() -> Self {
        Self {
            scope: None,
            write_channels: Vec::new(),
        }
    }
}

/// Resolve any of the accepted author-side scope shorthand shapes.
///
/// Author-side `{ channel: "foreground" }` lifts the channel out of scope and
/// into the returned `write_channels` list. Mixed shapes like
/// `{ channel: "foreground", rect: [0,0,4,4] }` produce both a writeChannels
/// list and a canonical `ScopeSpec::Rect` — the structural rule is that
/// channel keys belong with the node, not with cell selection.
pub fn resolve_scope(value: &Value) -> Result<ResolvedScope, CanonicalizationError> {
    match value {
        Value::String(s) => Ok(ResolvedScope {
            scope: Some(scope_from_string(s)?),
            write_channels: Vec::new(),
        }),
        Value::Object(obj) => resolve_object_scope(obj),
        _ => Err(CanonicalizationError::new(
            CanonicalizationErrorKind::InvalidScopeShape,
            format!("scope must be a string or object, got {value}"),
        )),
    }
}

fn scope_from_string(s: &str) -> Result<Value, CanonicalizationError> {
    match s {
        "all" => Ok(json!({ "kind": "all" })),
        "nonEmpty" => Ok(json!({ "kind": "nonEmpty" })),
        "outerBand" => Ok(json!({ "kind": "outerBand" })),
        "inner" => Ok(json!({ "kind": "inner" })),
        _ => Err(CanonicalizationError::new(
            CanonicalizationErrorKind::InvalidScopeShape,
            format!("unknown string-form scope: {s}"),
        )),
    }
}

fn resolve_object_scope(obj: &Map<String, Value>) -> Result<ResolvedScope, CanonicalizationError> {
    if let Some(kind) = obj.get("kind").and_then(Value::as_str)
        && matches!(
            kind,
            "all"
                | "role"
                | "content"
                | "rect"
                | "cell"
                | "rowRange"
                | "columnRange"
                | "rows"
                | "columns"
                | "moduloRows"
                | "moduloColumns"
                | "nonEmpty"
                | "outerBand"
                | "inner"
        )
    {
        return Ok(ResolvedScope {
            scope: Some(Value::Object(obj.clone())),
            write_channels: Vec::new(),
        });
    }

    let mut write_channels = Vec::new();
    if let Some(channel) = obj.get("channel").and_then(Value::as_str) {
        write_channels.push(channel.to_string());
    }
    if let Some(channels) = obj.get("channels").and_then(Value::as_array) {
        for entry in channels {
            let s = entry.as_str().ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::InvalidScopeShape,
                    "scope.channels entries must be strings",
                )
            })?;
            write_channels.push(s.into());
        }
    }

    let mut scope = None;
    for (key, value) in obj {
        if matches!(key.as_str(), "channel" | "channels" | "kind") {
            continue;
        }
        if scope.is_some() {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::InvalidScopeShape,
                format!("scope object has more than one cell-selection key (found {key})"),
            ));
        }
        scope = Some(scope_from_pair(key, value)?);
    }

    Ok(ResolvedScope {
        scope,
        write_channels,
    })
}

fn scope_from_pair(key: &str, value: &Value) -> Result<Value, CanonicalizationError> {
    match key {
        "role" => Ok(json!({ "kind": "role", "role": value })),
        "content" => Ok(json!({ "kind": "content", "value": value })),
        "rect" => {
            let arr = value.as_array().ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::InvalidScopeShape,
                    "scope.rect must be a [x, y, w, h] tuple",
                )
            })?;
            if arr.len() != 4 {
                return Err(CanonicalizationError::new(
                    CanonicalizationErrorKind::InvalidScopeShape,
                    "scope.rect tuple must have 4 elements",
                ));
            }
            Ok(json!({
                "kind": "rect",
                "rect": {
                    "x": arr[0],
                    "y": arr[1],
                    "width": arr[2],
                    "height": arr[3],
                }
            }))
        }
        "cell" => {
            let arr = value.as_array().ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::InvalidScopeShape,
                    "scope.cell must be a [x, y] tuple",
                )
            })?;
            if arr.len() != 2 {
                return Err(CanonicalizationError::new(
                    CanonicalizationErrorKind::InvalidScopeShape,
                    "scope.cell tuple must have 2 elements",
                ));
            }
            Ok(json!({ "kind": "cell", "x": arr[0], "y": arr[1] }))
        }
        "rowRange" => {
            let (start, end) = pair_tuple(value, "rowRange")?;
            Ok(json!({ "kind": "rowRange", "start": start, "end": end }))
        }
        "columnRange" => {
            let (start, end) = pair_tuple(value, "columnRange")?;
            Ok(json!({ "kind": "columnRange", "start": start, "end": end }))
        }
        "rows" => {
            let arr = value.as_array().ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::InvalidScopeShape,
                    "scope.rows must be an array of indices",
                )
            })?;
            Ok(json!({ "kind": "rows", "indices": arr }))
        }
        "columns" => {
            let arr = value.as_array().ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::InvalidScopeShape,
                    "scope.columns must be an array of indices",
                )
            })?;
            Ok(json!({ "kind": "columns", "indices": arr }))
        }
        "moduloRows" => clone_kinded(value, "moduloRows"),
        "moduloColumns" => clone_kinded(value, "moduloColumns"),
        _ => Err(CanonicalizationError::new(
            CanonicalizationErrorKind::InvalidScopeShape,
            format!("unknown scope key: {key}"),
        )),
    }
}

fn pair_tuple(value: &Value, key: &str) -> Result<(Value, Value), CanonicalizationError> {
    let arr = value.as_array().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::InvalidScopeShape,
            format!("scope.{key} must be a [start, end] tuple"),
        )
    })?;
    if arr.len() != 2 {
        return Err(CanonicalizationError::new(
            CanonicalizationErrorKind::InvalidScopeShape,
            format!("scope.{key} tuple must have 2 elements"),
        ));
    }
    Ok((arr[0].clone(), arr[1].clone()))
}

fn clone_kinded(value: &Value, kind: &str) -> Result<Value, CanonicalizationError> {
    let mut obj = value.as_object().cloned().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::InvalidScopeShape,
            format!("scope.{kind} must be an object"),
        )
    })?;
    obj.insert("kind".into(), Value::String(kind.into()));
    Ok(Value::Object(obj))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_extracts_to_write_channels_and_leaves_no_scope() {
        let resolved = resolve_scope(&json!({ "channel": "foreground" })).unwrap();
        assert_eq!(resolved.scope, None);
        assert_eq!(resolved.write_channels, vec!["foreground"]);
    }

    #[test]
    fn channels_array_extracts_each() {
        let resolved = resolve_scope(&json!({ "channels": ["foreground", "background"] })).unwrap();
        assert_eq!(resolved.scope, None);
        assert_eq!(resolved.write_channels, vec!["foreground", "background"]);
    }

    #[test]
    fn channel_plus_rect_yields_both() {
        let resolved =
            resolve_scope(&json!({ "channel": "foreground", "rect": [0, 0, 4, 4] })).unwrap();
        assert_eq!(resolved.write_channels, vec!["foreground"]);
        assert_eq!(
            resolved.scope,
            Some(json!({
                "kind": "rect",
                "rect": { "x": 0, "y": 0, "width": 4, "height": 4 }
            }))
        );
    }

    #[test]
    fn rows_indices() {
        let resolved = resolve_scope(&json!({ "rows": [0, 10] })).unwrap();
        assert_eq!(
            resolved.scope,
            Some(json!({ "kind": "rows", "indices": [0, 10] }))
        );
    }

    #[test]
    fn role_passthrough_string() {
        let resolved = resolve_scope(&json!({ "role": "border" })).unwrap();
        assert_eq!(
            resolved.scope,
            Some(json!({ "kind": "role", "role": "border" }))
        );
    }

    #[test]
    fn content_passthrough() {
        let resolved = resolve_scope(&json!({ "content": "text" })).unwrap();
        assert_eq!(
            resolved.scope,
            Some(json!({ "kind": "content", "value": "text" }))
        );
    }

    #[test]
    fn string_shorthand_all() {
        let resolved = resolve_scope(&json!("all")).unwrap();
        assert_eq!(resolved.scope, Some(json!({ "kind": "all" })));
    }

    #[test]
    fn already_canonical_scope_passes_through() {
        let canonical = json!({ "kind": "all" });
        let resolved = resolve_scope(&canonical).unwrap();
        assert_eq!(resolved.scope, Some(canonical));
    }

    #[test]
    fn cell_tuple() {
        let resolved = resolve_scope(&json!({ "cell": [3, 7] })).unwrap();
        assert_eq!(
            resolved.scope,
            Some(json!({ "kind": "cell", "x": 3, "y": 7 }))
        );
    }

    #[test]
    fn row_range_tuple() {
        let resolved = resolve_scope(&json!({ "rowRange": [1, 5] })).unwrap();
        assert_eq!(
            resolved.scope,
            Some(json!({ "kind": "rowRange", "start": 1, "end": 5 }))
        );
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_resolve_scope.rs</FILE> - <DESC>Pattern-match author-side scope shorthand into canonical ScopeSpec, splitting out writeChannels lift</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
