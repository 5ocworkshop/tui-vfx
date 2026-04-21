// <FILE>src/v2/functions/fnc_deep_merge_json.rs</FILE> - <DESC>Deep recursive merge of JSON objects for template inheritance</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Implementing Recipe Template Inheritance per PRD v1.1.0</WCTX>
// <CLOG>Initial creation - JSON-level merge preserves user intent vs serde defaults</CLOG>

use serde_json::Value;

/// Perform deep recursive merge of two JSON Value objects where overlay (recipe) fields
/// override base (template) fields.
///
/// # Merge Strategy
/// - **Objects**: Recursive merge field-by-field
/// - **Arrays**: Overlay replaces base entirely (no element-wise merge)
/// - **Primitives**: Overlay replaces base
/// - **Null**: Overlay null removes base field
///
/// # Critical Design Decision
/// This function operates at the JSON level (serde_json::Value) BEFORE deserializing
/// to Rust structs. This is essential because the V2 schema uses #[serde(default)]
/// extensively, which makes it impossible to distinguish "user set width=30" from
/// "user omitted width, serde applied default 30" after deserialization.
///
/// # Arguments
/// * `base` - Template JSON (provides defaults)
/// * `overlay` - Recipe JSON (provides overrides)
///
/// # Returns
/// Merged JSON Value with overlay fields taking precedence
///
/// # Examples
/// ```ignore
/// use serde_json::json;
///
/// let template = json!({
///     "layout": { "width": 40, "height": 6, "anchor": "TopLeft" },
///     "border": { "type": "Rounded" }
/// });
///
/// let recipe = json!({
///     "layout": { "anchor": "MiddleCenter" },
///     "message": "Hello"
/// });
///
/// let merged = deep_merge_json(template, recipe);
/// // Result: {
/// //     "layout": { "width": 40, "height": 6, "anchor": "MiddleCenter" },
/// //     "border": { "type": "Rounded" },
/// //     "message": "Hello"
/// // }
/// ```
pub fn deep_merge_json(base: Value, overlay: Value) -> Value {
    match (base, overlay) {
        (Value::Object(mut base_obj), Value::Object(overlay_obj)) => {
            for (key, value) in overlay_obj {
                // Skip "extends" field - don't copy it into merged result
                if key == "extends" {
                    continue;
                }
                if value.is_null() {
                    base_obj.remove(&key);
                    continue;
                }

                if let Some(base_value) = base_obj.get_mut(&key) {
                    // Key exists in both base and overlay - recurse if both are objects
                    if base_value.is_object() && value.is_object() {
                        let merged = deep_merge_json(std::mem::take(base_value), value);
                        *base_value = merged;
                    } else {
                        // Non-object or type mismatch: overlay replaces base
                        *base_value = value;
                    }
                } else {
                    // Key doesn't exist in base - add from overlay
                    base_obj.insert(key, value);
                }
            }
            Value::Object(base_obj)
        }
        (_, overlay) => overlay,
    }
}

// <FILE>src/v2/functions/fnc_deep_merge_json.rs</FILE> - <DESC>Deep recursive merge of JSON objects for template inheritance</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
