// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_assets.rs</FILE> - <DESC>Lift the top-level assets block into canonical AssetSpec entries</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Lift author asset shorthand (kind/format/path) into AssetSpec id+kind+format+locator shape.</WCTX>
// <CLOG>0.1.0: INIT — wrap AssetKind into its internally-tagged form, lift `path:` into AssetLocator::Path.</CLOG>

use serde_json::{Map, Value, json};

use super::cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};

/// Lift the top-level `assets: { id: { kind, format, path|locator } }` block
/// into canonical [`AssetSpec`] entries. Each entry gains the map-key as `id`,
/// the bare kind string is wrapped in the internally-tagged [`AssetKind`]
/// shape, and an author-side `path:` becomes [`AssetLocator::Path`]. Format
/// passes through (it's a transparent newtype string).
///
/// [`AssetSpec`]: crate::AssetSpec
/// [`AssetKind`]: crate::AssetKind
/// [`AssetLocator::Path`]: crate::AssetLocator
pub fn lift_assets(recipe: &mut Value) -> Result<(), CanonicalizationError> {
    let recipe_obj = recipe.as_object_mut().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "object".into(),
            },
            "recipe root must be an object",
        )
    })?;

    let raw = match recipe_obj.get_mut("assets") {
        Some(Value::Object(map)) => map,
        Some(other) => {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "object".into(),
                },
                format!("assets must be an object, got {other}"),
            )
            .at(JsonPathSegment::field("assets")));
        }
        None => return Ok(()),
    };

    let mut canonical_assets = Map::new();
    let entries: Vec<(String, Value)> = raw.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    for (id, value) in entries {
        // Skip already-canonical entries (objects with `id` and `locator`).
        if let Value::Object(obj) = &value
            && obj.contains_key("id")
            && obj.contains_key("locator")
        {
            canonical_assets.insert(id, value);
            continue;
        }
        let lifted = build_asset_spec(&id, &value).map_err(|e| {
            e.at(JsonPathSegment::field(id.clone()))
                .at(JsonPathSegment::field("assets"))
        })?;
        canonical_assets.insert(id, lifted);
    }
    *raw = canonical_assets;
    Ok(())
}

fn build_asset_spec(id: &str, value: &Value) -> Result<Value, CanonicalizationError> {
    let obj = value.as_object().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "object".into(),
            },
            "asset entry must be an object",
        )
    })?;

    let kind = obj
        .get("kind")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::MissingRequired {
                    field: "kind".into(),
                },
                "asset entry must declare `kind`",
            )
        })?
        .to_string();

    let format = obj.get("format").cloned().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::MissingRequired {
                field: "format".into(),
            },
            "asset entry must declare `format`",
        )
    })?;

    let locator = if let Some(path) = obj.get("path").and_then(Value::as_str) {
        json!({ "kind": "path", "path": path })
    } else if let Some(logical) = obj.get("locator").and_then(Value::as_str) {
        json!({ "kind": "logical", "locator": logical })
    } else if let Some(canonical) = obj.get("locator") {
        canonical.clone()
    } else {
        return Err(CanonicalizationError::new(
            CanonicalizationErrorKind::MissingRequired {
                field: "path".into(),
            },
            "asset entry must declare `path` (or canonical `locator`)",
        ));
    };

    let mut spec = json!({
        "id": id,
        "kind": { "kind": kind },
        "format": format,
        "locator": locator,
    });
    if let Some(description) = obj.get("description") {
        spec["description"] = description.clone();
    }
    Ok(spec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn braille_dotfield_lifts_to_canonical_asset_spec() {
        let mut recipe = json!({
            "id": "x",
            "assets": {
                "flag": {
                    "kind": "brailleDotfield",
                    "format": "tui-vfx.braille_flag_asset.v1",
                    "path": "/path/to/flag.json"
                }
            }
        });
        lift_assets(&mut recipe).unwrap();
        let asset = &recipe["assets"]["flag"];
        assert_eq!(asset["id"], "flag");
        assert_eq!(asset["kind"], json!({ "kind": "brailleDotfield" }));
        assert_eq!(asset["format"], "tui-vfx.braille_flag_asset.v1");
        assert_eq!(
            asset["locator"],
            json!({ "kind": "path", "path": "/path/to/flag.json" })
        );
    }

    #[test]
    fn missing_assets_block_is_a_noop() {
        let mut recipe = json!({ "id": "x" });
        lift_assets(&mut recipe).unwrap();
        assert!(!recipe.as_object().unwrap().contains_key("assets"));
    }

    #[test]
    fn already_canonical_passes_through() {
        let canonical = json!({
            "id": "x",
            "assets": {
                "flag": {
                    "id": "flag",
                    "kind": { "kind": "image" },
                    "format": "image/png",
                    "locator": { "kind": "path", "path": "x.png" }
                }
            }
        });
        let mut recipe = canonical.clone();
        lift_assets(&mut recipe).unwrap();
        assert_eq!(recipe, canonical);
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_assets.rs</FILE> - <DESC>Lift the top-level assets block into canonical AssetSpec entries</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
