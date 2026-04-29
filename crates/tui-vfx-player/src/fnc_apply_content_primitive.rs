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
    for row in rows {
        let mut wrote_cursor = false;
        *row = row
            .chars()
            .map(|glyph| {
                seen += 1;
                if seen <= visible || glyph == ' ' {
                    glyph
                } else if !wrote_cursor {
                    wrote_cursor = true;
                    cursor
                } else {
                    ' '
                }
            })
            .collect();
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
    let threshold = (request.phase_t * settle + cascade * 0.1).clamp(0.0, 1.0);
    for (y, row) in rows.iter_mut().enumerate() {
        *row = row
            .chars()
            .enumerate()
            .map(|(x, glyph)| {
                if glyph == ' ' || cell_threshold(x, y) <= threshold {
                    glyph
                } else {
                    '▣'
                }
            })
            .collect();
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

fn cell_threshold(x: usize, y: usize) -> f64 {
    ((x * 37 + y * 17) % 100) as f64 / 99.0
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

// <FILE>crates/tui-vfx-player/src/fnc_apply_content_primitive.rs</FILE> - <DESC>Apply bounded content-effect primitives to text-grid rows</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
