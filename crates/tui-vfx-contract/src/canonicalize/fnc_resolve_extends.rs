// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_resolve_extends.rs</FILE> - <DESC>Deep-merge author-side `extends: "..."` template chains before canonicalization</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Canonicalize template chain: deep-merge templates into the recipe so canonicalize sees a single flat tree.</WCTX>
// <CLOG>0.1.0: INIT — child-wins deep-merge with cycle detection over a caller-supplied template map.</CLOG>

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use serde_json::Value;

use super::cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};
use super::cls_recipe_intent::ExtendsChainEntry;

/// Caller-supplied lookup from template path to its parsed JSON value. Keeping
/// this as a borrowed map keeps `canonicalize` filesystem-agnostic — the
/// corpus harness, xtask, or any other caller decides where templates come
/// from.
pub type TemplateMap<'a> = &'a BTreeMap<String, Value>;

/// Resolve the recipe's `extends:` chain in place. Each step deep-merges the
/// referenced template under the current tree (child wins per top-level key,
/// objects merge recursively, arrays and primitives replace). Returns the
/// chain of templates resolved for `RecipeIntent.extends_chain` provenance.
pub fn resolve_extends(
    recipe: &mut Value,
    templates: TemplateMap<'_>,
) -> Result<Vec<ExtendsChainEntry>, CanonicalizationError> {
    let mut chain: Vec<ExtendsChainEntry> = Vec::new();
    let mut visited: BTreeSet<String> = BTreeSet::new();

    loop {
        let path = match recipe.as_object_mut().and_then(|obj| obj.remove("extends")) {
            Some(Value::String(p)) => p,
            Some(other) => {
                return Err(CanonicalizationError::new(
                    CanonicalizationErrorKind::UnexpectedJsonShape {
                        expected: "string template path".into(),
                    },
                    format!("`extends` must be a string template path, got {other}"),
                )
                .at(JsonPathSegment::field("extends")));
            }
            None => break,
        };

        if !visited.insert(path.clone()) {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::ExtendsChainCycle {
                    chain: chain.iter().map(|e| e.path.clone()).collect(),
                },
                format!(
                    "extends chain has a cycle revisiting `{path}` after {:?}",
                    chain.iter().map(|e| &e.path).collect::<Vec<_>>()
                ),
            )
            .at(JsonPathSegment::field("extends")));
        }

        let template = templates.get(&path).ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::ExtendsTargetNotFound { path: path.clone() },
                format!(
                    "extends references `{path}` but the caller did not provide it. Available templates: {:?}",
                    templates.keys().collect::<Vec<_>>()
                ),
            )
            .at(JsonPathSegment::field("extends"))
        })?;

        let template_keys: Vec<String> = template
            .as_object()
            .map(|obj| obj.keys().cloned().collect())
            .unwrap_or_default();

        deep_merge(recipe, template);
        chain.push(ExtendsChainEntry {
            path,
            merged_keys: template_keys,
        });
    }

    Ok(chain)
}

/// Child-wins deep-merge. The `child` (recipe) is the receiver and keeps any
/// keys it already declares; missing keys come from the `parent` (template).
/// Objects merge recursively; arrays and scalars replace.
fn deep_merge(child: &mut Value, parent: &Value) {
    if let (Value::Object(child_obj), Value::Object(parent_obj)) = (child, parent) {
        for (key, parent_value) in parent_obj {
            match child_obj.get_mut(key) {
                Some(child_value) => deep_merge(child_value, parent_value),
                None => {
                    child_obj.insert(key.clone(), parent_value.clone());
                }
            }
        }
    }
    // Arrays and scalars: child already won by being present; do nothing.
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_templates(entries: &[(&str, Value)]) -> BTreeMap<String, Value> {
        entries
            .iter()
            .map(|(k, v)| ((*k).to_string(), v.clone()))
            .collect()
    }

    #[test]
    fn missing_extends_is_a_noop() {
        let mut recipe = json!({ "id": "x", "title": "no template" });
        let templates: BTreeMap<String, Value> = BTreeMap::new();
        let chain = resolve_extends(&mut recipe, &templates).unwrap();
        assert!(chain.is_empty());
        assert_eq!(recipe["title"], "no template");
    }

    #[test]
    fn child_wins_per_top_level_key() {
        let templates = make_templates(&[(
            "themes/base",
            json!({ "title": "BASE", "card": { "fg": "white", "border": "rounded" } }),
        )]);
        let mut recipe = json!({
            "id": "x",
            "extends": "themes/base",
            "title": "CHILD",
            "card": { "fg": "yellow" }
        });
        resolve_extends(&mut recipe, &templates).unwrap();
        assert_eq!(recipe["title"], "CHILD");
        assert_eq!(recipe["card"]["fg"], "yellow");
        assert_eq!(recipe["card"]["border"], "rounded");
    }

    #[test]
    fn extends_chain_resolves_in_order() {
        let templates = make_templates(&[
            (
                "themes/leaf",
                json!({ "extends": "themes/root", "card": { "border": "double" } }),
            ),
            (
                "themes/root",
                json!({ "title": "ROOT", "card": { "fg": "white", "bg": "black" } }),
            ),
        ]);
        let mut recipe = json!({ "id": "x", "extends": "themes/leaf" });
        let chain = resolve_extends(&mut recipe, &templates).unwrap();
        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].path, "themes/leaf");
        assert_eq!(chain[1].path, "themes/root");
        assert_eq!(recipe["title"], "ROOT");
        assert_eq!(recipe["card"]["border"], "double");
        assert_eq!(recipe["card"]["fg"], "white");
    }

    #[test]
    fn cycle_detection() {
        let templates = make_templates(&[
            ("themes/a", json!({ "extends": "themes/b" })),
            ("themes/b", json!({ "extends": "themes/a" })),
        ]);
        let mut recipe = json!({ "id": "x", "extends": "themes/a" });
        let err = resolve_extends(&mut recipe, &templates).unwrap_err();
        assert!(matches!(
            err.kind,
            CanonicalizationErrorKind::ExtendsChainCycle { .. }
        ));
    }

    #[test]
    fn unknown_template_path_is_a_clean_error() {
        let templates: BTreeMap<String, Value> = BTreeMap::new();
        let mut recipe = json!({ "id": "x", "extends": "themes/missing" });
        let err = resolve_extends(&mut recipe, &templates).unwrap_err();
        assert!(matches!(
            err.kind,
            CanonicalizationErrorKind::ExtendsTargetNotFound { .. }
        ));
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_resolve_extends.rs</FILE> - <DESC>Deep-merge author-side `extends: "..."` template chains before canonicalization</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
