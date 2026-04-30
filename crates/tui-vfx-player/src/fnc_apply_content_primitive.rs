// <FILE>crates/tui-vfx-player/src/fnc_apply_content_primitive.rs</FILE> - <DESC>Apply bounded content-effect primitives to text-grid rows</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 descriptor/adapter migration: provide honest content-effect evidence for first canonical adapter coverage set.</WCTX>
// <CLOG>0.1.0: INIT — add typewriter, marquee, split-flap, wrap-indicator, scramble, and morph adapters.</CLOG>

use tui_vfx_contract::NodeSpec;

use crate::{
    PlayerSampleRequest,
    fnc_resolve_effect_input::{
        resolve_effect_enum, resolve_effect_integer, resolve_effect_number,
    },
};

/// Apply a supported content primitive to text-grid rows.
pub(crate) fn apply_content_primitive(
    node: &NodeSpec,
    request: &PlayerSampleRequest,
    rows: &mut [String],
) -> bool {
    match node.effect.as_str() {
        "content.typewriter" => apply_typewriter(node, request, rows),
        "content.marquee" => apply_marquee(node, request, rows),
        "content.splitFlap" => apply_split_flap(node, request, rows),
        "content.wrapIndicator" => apply_wrap_indicator(node, request, rows),
        "content.scramble" => apply_scramble(node, request, rows),
        "content.morph" => apply_morph(node, request, rows),
        "content.redact" => apply_redact(node, request, rows),
        "content.mirror" => apply_mirror(node, request, rows),
        "content.numeric" => apply_numeric(node, request, rows),
        "content.dissolve" => apply_content_dissolve(node, request, rows),
        "content.odometer" => apply_odometer(node, request, rows),
        "content.cellMotion" => apply_cell_motion(node, request, rows),
        "content.slideShift" => apply_slide_shift(node, request, rows),
        "content.glitchShift" => apply_glitch_shift(node, request, rows),
        "content.scrambleGlitchShift" => {
            apply_scramble(node, request, rows);
            apply_glitch_shift(node, request, rows);
        }
        _ => return false,
    }
    true
}

fn apply_typewriter(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let speed = resolve_effect_number(node, request, "speed", 1.0).max(0.0);
    let speed_variance = resolve_effect_number(node, request, "speedVariance", 0.0).clamp(0.0, 1.0);
    let visible_fraction =
        (request.phase_t * (speed + speed_variance * request.phase_t)).clamp(0.0, 1.0);
    let total = rows.iter().map(|row| row.chars().count()).sum::<usize>();
    let visible = (total as f64 * visible_fraction).round() as usize;
    let mut seen = 0usize;
    let cursor = resolve_effect_enum(node, request, "cursorCharacter", "▌")
        .chars()
        .next()
        .unwrap_or('▌');
    let wake_mode = resolve_effect_enum(node, request, "cursorWake", "off");
    let wake_cells = resolve_effect_integer(node, request, "wakeCells", 1).max(0) as usize;
    for row in rows {
        let mut wrote_cursor = false;
        let mut cursor_index = None;
        let mut chars = Vec::new();
        for (index, glyph) in row.chars().enumerate() {
            seen += 1;
            if seen <= visible || glyph == ' ' {
                chars.push(glyph);
            } else if !wrote_cursor {
                wrote_cursor = true;
                cursor_index = Some(index);
                chars.push(cursor);
            } else {
                chars.push(' ');
            }
        }
        if (wake_mode == "ghost" || wake_mode == "tint")
            && let Some(cursor_index) = cursor_index
        {
            let start = cursor_index.saturating_sub(wake_cells);
            for glyph in &mut chars[start..cursor_index] {
                if *glyph != ' ' {
                    *glyph = if wake_mode == "ghost" { '░' } else { '·' };
                }
            }
        }
        *row = chars.into_iter().collect();
    }
}

fn apply_marquee(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let direction = resolve_effect_enum(node, request, "direction", "left");
    let speed = resolve_effect_number(node, request, "speed", 1.0).max(0.0);
    let authored_width = resolve_effect_integer(node, request, "width", 0).max(0) as usize;
    for row in rows {
        let width = authored_width.max(row.chars().count());
        if width == 0 {
            continue;
        }
        let offset = ((request.phase_t * speed * width as f64).round() as usize) % width;
        *row = rotate_row(
            row,
            if direction == "right" {
                width - offset
            } else {
                offset
            },
        );
    }
}

fn apply_split_flap(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let settle = resolve_effect_number(node, request, "settle", 1.0).clamp(0.0, 1.0);
    let cascade = resolve_effect_number(node, request, "cascade", 0.0).clamp(0.0, 1.0);
    let speed = resolve_effect_number(node, request, "speed", 1.0).max(0.0);
    let cycles = resolve_effect_number(node, request, "cycles", 1.0).max(0.0);
    let charset_name = resolve_effect_enum(node, request, "charset", "blocks");
    let tile_width = resolve_effect_integer(node, request, "tileWidth", 1).max(1) as usize;
    let tile_height = resolve_effect_integer(node, request, "tileHeight", 1).max(1) as usize;
    let jitter = resolve_effect_number(node, request, "jitter", 0.0).clamp(0.0, 1.0);
    let threshold = (request.phase_t * settle * speed + cascade * 0.1).clamp(0.0, 1.0);
    let glyphs = split_flap_charset(&charset_name);
    for (y, row) in rows.iter_mut().enumerate() {
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                let tile_offset = ((x / tile_width) + (y / tile_height)) as f64 * cascade * 0.08;
                let cell_progress = (threshold - tile_offset
                    + jitter * cell_threshold(x, y) * 0.05)
                    .clamp(0.0, 1.0);
                if glyph == ' ' || cell_progress >= 1.0 {
                    glyph
                } else if cycles > 0.0 {
                    let index =
                        ((cell_progress * cycles * glyphs.len() as f64).floor() as usize + x + y)
                            % glyphs.len();
                    glyphs[index]
                } else {
                    '▣'
                }
            })
            .collect();
    }
}

fn apply_redact(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let symbol = resolve_effect_enum(node, request, "symbol", "█")
        .chars()
        .next()
        .unwrap_or('█');
    let reveal = resolve_effect_number(node, request, "reveal", request.phase_t).clamp(0.0, 1.0);
    for (y, row) in rows.iter_mut().enumerate() {
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                if glyph == ' ' || cell_threshold(x, y) < reveal {
                    glyph
                } else {
                    symbol
                }
            })
            .collect();
    }
}

fn apply_mirror(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let axis = resolve_effect_enum(node, request, "axis", "horizontal");
    if axis == "vertical" {
        rows.reverse();
        return;
    }
    for row in rows {
        *row = row.chars().rev().collect();
    }
}

fn apply_numeric(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let value = resolve_effect_number(node, request, "value", request.phase_t);
    let decimals = resolve_effect_integer(node, request, "decimals", 0).clamp(0, 9) as usize;
    let prefix = resolve_effect_enum(node, request, "prefix", "");
    let suffix = resolve_effect_enum(node, request, "suffix", "");
    let formatted = format!("{prefix}{value:.decimals$}{suffix}");
    if let Some(row) = rows.first_mut() {
        *row = formatted;
    }
    for row in rows.iter_mut().skip(1) {
        row.clear();
    }
}

fn apply_content_dissolve(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let replacement = resolve_effect_enum(node, request, "replacement", "space");
    let direction = resolve_effect_enum(node, request, "direction", "random");
    let seed = resolve_effect_integer(node, request, "seed", 0).max(0) as usize;
    let progress = request.phase_t.clamp(0.0, 1.0);
    let replacement = match replacement.as_str() {
        "dot" => '·',
        "block" => '█',
        value => value.chars().next().unwrap_or(' '),
    };
    for (y, row) in rows.iter_mut().enumerate() {
        let width = row.chars().count().max(1);
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                if glyph == ' ' || dissolve_threshold(x, y, width, seed, &direction) > progress {
                    replacement
                } else {
                    glyph
                }
            })
            .collect();
    }
}

fn apply_odometer(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let direction = resolve_effect_enum(node, request, "direction", "up");
    let travel = resolve_effect_enum(node, request, "travel", "axis");
    let from_message = resolve_effect_enum(node, request, "fromMessage", "");
    let tile_width = resolve_effect_integer(node, request, "tileWidth", 1).max(1) as usize;
    let tile_height = resolve_effect_integer(node, request, "tileHeight", 1).max(1) as usize;
    let progress = request.phase_t.clamp(0.0, 1.0);
    if from_message.is_empty() {
        apply_glitch_shift(node, request, rows);
        return;
    }
    let from_rows = normalized_rows(&from_message, rows.len());
    let row_count = rows.len();
    for (y, row) in rows.iter_mut().enumerate() {
        let target = row.chars().collect::<Vec<_>>();
        let source = from_rows
            .get(y)
            .map(|value| value.chars().collect::<Vec<_>>())
            .unwrap_or_default();
        *row = target
            .iter()
            .enumerate()
            .map(|(x, target_glyph)| {
                let source_glyph = source.get(x).copied().unwrap_or(' ');
                let travel_span =
                    odometer_travel_span(&travel, target.len().max(source.len()), row_count);
                let tile_delay = ((x / tile_width) + (y / tile_height)) as f64 * 0.04 * travel_span;
                let cell_progress = (progress - tile_delay).clamp(0.0, 1.0);
                let reveal_threshold = odometer_reveal_threshold(&travel);
                if cell_progress >= reveal_threshold {
                    *target_glyph
                } else if direction == "down" || direction == "left" {
                    previous_digit(source_glyph)
                } else {
                    source_glyph
                }
            })
            .collect();
    }
}

fn odometer_travel_span(travel: &str, width: usize, height: usize) -> f64 {
    match travel {
        "fullClear" | "full_clear" => width.max(height).max(1) as f64,
        "cells" => 2.0,
        _ => 1.0,
    }
}

fn odometer_reveal_threshold(travel: &str) -> f64 {
    match travel {
        "fullClear" | "full_clear" => 0.75,
        "cells" => 0.6,
        _ => 0.5,
    }
}

fn apply_cell_motion(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let route = resolve_effect_enum(node, request, "route", "fromTop");
    let stagger = resolve_effect_integer(node, request, "stagger", 0).max(0) as usize;
    let affect = resolve_effect_enum(node, request, "affect", "all");
    let progress = request.phase_t.clamp(0.0, 1.0);
    for (y, row) in rows.iter_mut().enumerate() {
        let width = row.chars().count().max(1);
        let line_threshold = ((y * stagger) % 10) as f64 / 20.0;
        if progress < line_threshold {
            row.clear();
            continue;
        }
        if route == "fromLeft" || route == "left" {
            let visible = (width as f64 * progress).round() as usize;
            *row = row
                .chars()
                .enumerate()
                .map(|(x, glyph)| {
                    if x < visible && (affect != "nonEmpty" || glyph != ' ') {
                        glyph
                    } else {
                        ' '
                    }
                })
                .collect();
        } else if progress <= 0.0 {
            row.clear();
        }
    }
}

fn apply_slide_shift(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let start_col = resolve_effect_integer(node, request, "startCol", -4);
    let end_col = resolve_effect_integer(node, request, "endCol", 0);
    let progress = request.phase_t.clamp(0.0, 1.0);
    let offset = (start_col as f64 + (end_col - start_col) as f64 * progress).round() as isize;
    for row in rows {
        *row = shift_row(row, offset);
    }
}

fn apply_glitch_shift(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let amount = resolve_effect_integer(node, request, "amount", 1).max(0) as usize;
    let seed = resolve_effect_integer(node, request, "seed", 3).max(0) as usize;
    for (y, row) in rows.iter_mut().enumerate() {
        if row.is_empty() || !(y + seed).is_multiple_of(2) {
            continue;
        }
        *row = rotate_row(row, amount.min(row.chars().count().saturating_sub(1)));
    }
}

fn apply_wrap_indicator(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let every = resolve_effect_integer(node, request, "every", 1).max(1);
    for (index, row) in rows.iter_mut().enumerate() {
        if index as i64 % every == 0 && !row.is_empty() {
            let mut chars = row.chars().collect::<Vec<_>>();
            if let Some(last) = chars.last_mut() {
                *last = '↵';
            }
            *row = chars.into_iter().collect();
        }
    }
}

fn apply_scramble(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let seed = resolve_effect_integer(node, request, "seed", 7).max(0) as usize;
    let charset = resolve_effect_enum(node, request, "charset", "#%&?+*");
    let charset = charset.chars().collect::<Vec<_>>();
    let resolved = request.phase_t.clamp(0.0, 1.0);
    for (y, row) in rows.iter_mut().enumerate() {
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                if glyph == ' ' || cell_threshold(x + seed, y) <= resolved {
                    glyph
                } else {
                    scramble_glyph(x + y + seed, &charset)
                }
            })
            .collect();
    }
}

fn apply_morph(node: &NodeSpec, request: &PlayerSampleRequest, rows: &mut [String]) {
    let target = resolve_effect_enum(node, request, "target", "blocks");
    let progress = request.phase_t.clamp(0.0, 1.0);
    for row in rows {
        *row = row
            .chars()
            .enumerate()
            .map(|(index, glyph)| {
                if glyph == ' ' || (index as f64 / row.len().max(1) as f64) > progress {
                    glyph
                } else if target == "dots" {
                    '·'
                } else {
                    '█'
                }
            })
            .collect();
    }
}

fn rotate_row(row: &str, offset: usize) -> String {
    let chars = row.chars().collect::<Vec<_>>();
    let width = chars.len();
    if width == 0 {
        return String::new();
    }
    chars[offset..]
        .iter()
        .chain(chars[..offset].iter())
        .collect::<String>()
}

fn shift_row(row: &str, offset: isize) -> String {
    let chars = row.chars().collect::<Vec<_>>();
    if chars.is_empty() {
        return String::new();
    }
    let width = chars.len() as isize;
    (0..width)
        .map(|x| {
            let source = x - offset;
            if (0..width).contains(&source) {
                chars[source as usize]
            } else {
                ' '
            }
        })
        .collect()
}

fn cell_threshold(x: usize, y: usize) -> f64 {
    ((x * 37 + y * 17) % 100) as f64 / 99.0
}

fn dissolve_threshold(x: usize, y: usize, width: usize, seed: usize, direction: &str) -> f64 {
    match direction {
        "leftToRight" | "left_to_right" => x as f64 / width.max(1) as f64,
        "rightToLeft" | "right_to_left" => width.saturating_sub(x + 1) as f64 / width.max(1) as f64,
        _ => cell_threshold(x + seed, y),
    }
}

fn split_flap_charset(charset_name: &str) -> Vec<char> {
    match charset_name {
        "digits" => "0123456789".chars().collect(),
        "binary" => "01".chars().collect(),
        "alphanumeric" => "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect(),
        _ => "▣▤▥▦▧▨".chars().collect(),
    }
}

fn normalized_rows(text: &str, expected: usize) -> Vec<String> {
    let mut rows = text.lines().map(str::to_string).collect::<Vec<_>>();
    rows.resize(expected, String::new());
    rows
}

fn previous_digit(glyph: char) -> char {
    match glyph {
        '0' => '9',
        '1'..='9' => char::from_u32(glyph as u32 - 1).unwrap_or(glyph),
        _ => glyph,
    }
}

fn scramble_glyph(index: usize, charset: &[char]) -> char {
    const FALLBACK_GLYPHS: &[char] = &['#', '%', '&', '?', '+', '*'];
    let glyphs = if charset.is_empty() {
        FALLBACK_GLYPHS
    } else {
        charset
    };
    glyphs[index % glyphs.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_redact_mirror_numeric_and_dissolve_adapters_consume_authored_fields() {
        let request = PlayerSampleRequest::default();

        let mut redacted = rows(["AB 12", "CD 34"]);
        assert!(apply_content_primitive(
            &node(
                "content.redact",
                serde_json::json!({
                    "symbol": string_source("▓"),
                    "reveal": number_source(0.0)
                }),
            ),
            &request,
            &mut redacted,
        ));
        assert_eq!(redacted[0], "▓▓ ▓▓");

        let mut mirrored = rows(["AB 12", "CD 34"]);
        assert!(apply_content_primitive(
            &node(
                "content.mirror",
                serde_json::json!({ "axis": enum_source("horizontal") }),
            ),
            &request,
            &mut mirrored,
        ));
        assert_eq!(mirrored[0], "21 BA");

        let mut numeric = rows(["AB 12", "CD 34"]);
        assert!(apply_content_primitive(
            &node(
                "content.numeric",
                serde_json::json!({
                    "value": number_source(42.126),
                    "decimals": integer_source(2),
                    "prefix": string_source("COUNT=")
                }),
            ),
            &request,
            &mut numeric,
        ));
        assert!(
            numeric[0].starts_with("COUNT=42.1"),
            "numeric row was {}",
            numeric[0]
        );

        let mut dissolved = rows(["AB 12", "CD 34"]);
        assert!(apply_content_primitive(
            &node(
                "content.dissolve",
                serde_json::json!({
                    "replacement": enum_source("dot"),
                    "direction": enum_source("leftToRight"),
                    "seed": integer_source(11)
                }),
            ),
            &PlayerSampleRequest {
                phase_t: 0.0,
                ..PlayerSampleRequest::default()
            },
            &mut dissolved,
        ));
        assert!(dissolved[0].contains('·'));
        assert_ne!(dissolved, rows(["AB 12", "CD 34"]));
    }

    #[test]
    fn content_motion_adapters_consume_roll_split_flap_and_cursor_fields() {
        let mut odometer_start = rows(["4502", "READY"]);
        let odometer_node = node(
            "content.odometer",
            serde_json::json!({
                "direction": enum_source("up"),
                "travel": enum_source("axis"),
                "tileWidth": integer_source(1),
                "tileHeight": integer_source(1),
                "fromMessage": string_source("4498")
            }),
        );
        assert!(apply_content_primitive(
            &odometer_node,
            &PlayerSampleRequest {
                phase_t: 0.0,
                ..PlayerSampleRequest::default()
            },
            &mut odometer_start,
        ));
        assert_eq!(odometer_start[0], "4498");

        let full_clear_node = node(
            "content.odometer",
            serde_json::json!({
                "direction": enum_source("up"),
                "travel": enum_source("fullClear"),
                "tileWidth": integer_source(1),
                "tileHeight": integer_source(1),
                "fromMessage": string_source("4498")
            }),
        );
        let mut full_clear_midpoint = rows(["4502", "READY"]);
        assert!(apply_content_primitive(
            &full_clear_node,
            &PlayerSampleRequest {
                phase_t: 0.6,
                ..PlayerSampleRequest::default()
            },
            &mut full_clear_midpoint,
        ));
        assert_ne!(full_clear_midpoint[0], "4502");

        let mut odometer_end = rows(["4502", "READY"]);
        assert!(apply_content_primitive(
            &odometer_node,
            &PlayerSampleRequest::default(),
            &mut odometer_end,
        ));
        assert_eq!(odometer_end[0], "4502");

        let mut split_flap = rows(["4502", "READY"]);
        assert!(apply_content_primitive(
            &node(
                "content.splitFlap",
                serde_json::json!({
                    "settle": number_source(1.0),
                    "cascade": number_source(0.1),
                    "cycles": number_source(2.0),
                    "charset": enum_source("digits")
                }),
            ),
            &PlayerSampleRequest {
                phase_t: 0.25,
                ..PlayerSampleRequest::default()
            },
            &mut split_flap,
        ));
        assert!(split_flap[0].chars().any(|glyph| glyph.is_ascii_digit()));
        assert_ne!(split_flap, rows(["4502", "READY"]));

        let mut typed = rows(["4502", "READY"]);
        assert!(apply_content_primitive(
            &node(
                "content.typewriter",
                serde_json::json!({
                    "speed": number_source(0.2),
                    "cursorCharacter": string_source("^"),
                    "cursorWake": enum_source("ghost")
                }),
            ),
            &PlayerSampleRequest {
                phase_t: 0.1,
                ..PlayerSampleRequest::default()
            },
            &mut typed,
        ));
        assert!(typed.iter().any(|row| row.contains('^')));

        let mut cell_motion = rows(["4502", "READY"]);
        assert!(apply_content_primitive(
            &node(
                "content.cellMotion",
                serde_json::json!({
                    "route": enum_source("fromTop"),
                    "stagger": integer_source(3),
                    "affect": enum_source("nonEmpty")
                }),
            ),
            &PlayerSampleRequest {
                phase_t: 0.0,
                ..PlayerSampleRequest::default()
            },
            &mut cell_motion,
        ));
        assert_ne!(cell_motion, rows(["4502", "READY"]));
    }

    fn node(effect: &str, inputs: serde_json::Value) -> NodeSpec {
        serde_json::from_value(serde_json::json!({
            "id": "contentNode",
            "effect": effect,
            "inputs": inputs,
            "outputs": {},
            "scope": { "kind": "all" },
            "cellWritePolicy": "writeCell",
            "roleWritePolicy": { "kind": "preserveDestination" }
        }))
        .expect("node")
    }

    fn rows<const N: usize>(values: [&str; N]) -> Vec<String> {
        values.into_iter().map(str::to_string).collect()
    }

    fn number_source(value: f64) -> serde_json::Value {
        serde_json::json!({ "kind": "literal", "value": { "kind": "number", "value": value } })
    }

    fn integer_source(value: i64) -> serde_json::Value {
        serde_json::json!({ "kind": "literal", "value": { "kind": "integer", "value": value } })
    }

    fn string_source(value: &str) -> serde_json::Value {
        serde_json::json!({ "kind": "literal", "value": { "kind": "string", "value": value } })
    }

    fn enum_source(value: &str) -> serde_json::Value {
        serde_json::json!({ "kind": "literal", "value": { "kind": "enum", "value": value } })
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_apply_content_primitive.rs</FILE> - <DESC>Apply bounded content-effect primitives to text-grid rows</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
