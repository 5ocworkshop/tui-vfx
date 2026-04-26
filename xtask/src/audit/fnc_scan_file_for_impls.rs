// <FILE>xtask/src/audit/fnc_scan_file_for_impls.rs</FILE> - <DESC>Scan a single Rust source file for hand-written impl ConfigSchema for X lines</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Packet 1.9.A — ConfigSchema justification lint</WCTX>
// <CLOG>1.0.0: initial implementation with macro-body skip heuristic</CLOG>

/// A detected `impl ConfigSchema for X` match within a source file.
#[derive(Debug, Clone)]
pub struct ImplHit {
    /// 1-based line number of the `impl ConfigSchema for X` line.
    pub line_number: usize,
    /// The type name extracted from the impl line (e.g. `Color`, `&str`).
    pub type_name: String,
    /// `true` when the match is inside a `macro_rules!` body and must be
    /// skipped.  Detected by the `$` sigil in the type-name position.
    pub is_macro_body: bool,
}

/// Scan `source` for `impl ConfigSchema for X` patterns and return all hits.
///
/// Rules:
/// - Matches `impl ConfigSchema for <type>` at any indentation level.
/// - Skips matches where the type name contains `$` — these are inside
///   `macro_rules!` bodies (the `impl_primitive_schema!` / `impl_int_schema!`
///   pattern uses `$t` as the type parameter).
/// - Does NOT parse the full AST; uses a line-by-line pattern. This is
///   intentional: the lint is a grep-and-baseline gate (per Intention 25 rule
///   5). A full `syn` parse is overkill for this class of drift.
pub fn scan_file_for_impls(source: &str) -> Vec<ImplHit> {
    let mut hits = Vec::new();

    for (idx, line) in source.lines().enumerate() {
        let trimmed = line.trim();

        // Match the pattern:  impl[<generics>]  ConfigSchema  for  <type>
        // The regex equivalent is:  ^\s*impl(?:<[^>]+>)?\s+ConfigSchema\s+for\s+(\S+)
        // We avoid pulling in the `regex` crate; a manual parse is sufficient.
        let Some(after_impl) = strip_impl_prefix(trimmed) else {
            continue;
        };

        let Some(after_configschema) = strip_configschema(after_impl) else {
            continue;
        };

        let Some(type_name) = strip_for_prefix(after_configschema) else {
            continue;
        };

        let type_name = extract_type_name(type_name);
        let is_macro_body = type_name.contains('$');

        hits.push(ImplHit {
            line_number: idx + 1,
            type_name,
            is_macro_body,
        });
    }

    hits
}

/// Strip the `impl` keyword plus any optional generic parameter list from the
/// start of the trimmed line. Returns the remainder or `None` if the line
/// does not start with `impl `.
fn strip_impl_prefix(s: &str) -> Option<&str> {
    let rest = s.strip_prefix("impl")?;
    // After `impl` there may be a generic list `<...>` followed by whitespace,
    // or immediately whitespace.
    let rest = rest.trim_start();

    if rest.starts_with('<') {
        // Find the matching `>`.  This is a depth-counting scan; it handles
        // nested angle brackets like `impl<T: Trait<U>>`.
        let mut depth = 0usize;
        let mut end = 0;
        for (i, ch) in rest.char_indices() {
            match ch {
                '<' => depth += 1,
                '>' => {
                    if depth == 0 {
                        return None; // malformed
                    }
                    depth -= 1;
                    if depth == 0 {
                        end = i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        if end == 0 {
            return None;
        }
        Some(rest[end..].trim_start())
    } else {
        Some(rest)
    }
}

/// Consume `ConfigSchema` followed by whitespace from the start of `s`.
fn strip_configschema(s: &str) -> Option<&str> {
    let rest = s.strip_prefix("ConfigSchema")?;
    Some(rest.trim_start())
}

/// Consume `for` followed by whitespace from the start of `s`.
fn strip_for_prefix(s: &str) -> Option<&str> {
    let rest = s.strip_prefix("for")?;
    Some(rest.trim_start())
}

/// Extract the type name token from the remainder of the `impl ... for <here>` line.
///
/// Handles:
/// - Simple names: `Color` → `Color`
/// - Generic names: `Option<T>` → `Option<T>`
/// - Multi-param generics: `VfxBindable<T, S>` → `VfxBindable<T, S>`
/// - Names followed by `where` clause: `Foo<T> where ...` → `Foo<T>`
/// - Names followed by `{` on the same line: `Foo {` → `Foo`
fn extract_type_name(s: &str) -> String {
    // If the type name contains `<`, we need to find the matching `>` to
    // capture the full generic argument list including spaces.
    if let Some(lt_pos) = s.find('<') {
        // Walk forward tracking depth, capturing until the matching `>`.
        let before = &s[..lt_pos];
        let from_lt = &s[lt_pos..];
        let mut depth = 0usize;
        let mut end = lt_pos;
        for (i, ch) in from_lt.char_indices() {
            match ch {
                '<' => depth += 1,
                '>' if depth > 0 => {
                    depth -= 1;
                    if depth == 0 {
                        end = lt_pos + i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        let candidate = s[..end].trim();
        // Verify the `before` part has no whitespace (i.e. `Foo` not `Foo bar`).
        let base = before.trim();
        if !base.is_empty() && !base.contains(' ') {
            return candidate.to_string();
        }
    }

    // No generics: take the first whitespace-delimited token, strip trailing `{`.
    let end = s
        .char_indices()
        .find(|&(_, ch)| ch == '{' || ch == '\n' || ch == ' ' || ch == '\t')
        .map(|(i, _)| i)
        .unwrap_or(s.len());

    s[..end].trim_end_matches('{').trim().to_string()
}

// <FILE>xtask/src/audit/fnc_scan_file_for_impls.rs</FILE> - <DESC>Scan a single Rust source file for hand-written impl ConfigSchema for X lines</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
