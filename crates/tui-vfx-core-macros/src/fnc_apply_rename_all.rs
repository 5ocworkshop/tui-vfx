// <FILE>tui-vfx-core-macros/src/fnc_apply_rename_all.rs</FILE> - <DESC>Apply a `#[serde(rename_all = "...")]` style transformation to a Rust identifier. Supports snake_case, camelCase, PascalCase, SCREAMING_SNAKE_CASE; falls back to the unchanged name for unknown styles or when rename_all is None.</DESC>
// <VERS>VERSION: 1.0.0 - 2026-04-28</VERS>
// <WCTX>Macro crate hygiene cleanup US-012 — relocate apply_rename_all out of inline lib.rs.</WCTX>
// <CLOG>1.0.0: initial — body lifted from lib.rs:235-275 verbatim.</CLOG>

use crate::col_to_snake_case::to_snake_case;

/// Apply rename_all transformation to a field/variant name.
pub(crate) fn apply_rename_all(name: &str, rename_all: Option<&str>) -> String {
    match rename_all {
        Some("snake_case") => to_snake_case(name),
        Some("camelCase") => {
            let snake = to_snake_case(name);
            let parts: Vec<&str> = snake.split('_').collect();
            if parts.is_empty() {
                return String::new();
            }
            let mut result = parts[0].to_string();
            for part in &parts[1..] {
                if !part.is_empty() {
                    let mut chars = part.chars();
                    if let Some(first) = chars.next() {
                        result.push(first.to_uppercase().next().unwrap());
                        result.push_str(chars.as_str());
                    }
                }
            }
            result
        }
        Some("PascalCase") => {
            let snake = to_snake_case(name);
            snake
                .split('_')
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let mut chars = s.chars();
                    match chars.next() {
                        Some(first) => {
                            format!("{}{}", first.to_uppercase(), chars.as_str())
                        }
                        None => String::new(),
                    }
                })
                .collect()
        }
        Some("SCREAMING_SNAKE_CASE") => to_snake_case(name).to_uppercase(),
        _ => name.to_string(),
    }
}

// <FILE>tui-vfx-core-macros/src/fnc_apply_rename_all.rs</FILE> - <DESC>Apply rename_all transformation to identifier</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2026-04-28</VERS>
