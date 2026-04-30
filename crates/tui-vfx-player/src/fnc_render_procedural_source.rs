// <FILE>crates/tui-vfx-player/src/fnc_render_procedural_source.rs</FILE> - <DESC>Render bounded procedural source registry entries</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Source fidelity adapters: keep procedural source generation deterministic and command-free.</WCTX>
// <CLOG>0.1.0: INIT — add bounded dots-spinner and checkerboard registry rendering.</CLOG>

use crate::PlayerSampleRequest;

/// Render a registered deterministic procedural source, if the name is supported.
pub(crate) fn render_registered_procedural_source(
    generator: &str,
    width: usize,
    height: usize,
    seed: usize,
    request: &PlayerSampleRequest,
) -> Option<Vec<String>> {
    match generator {
        "dots_spinner" => Some(render_dots_spinner(width, height, seed, request)),
        "checkerboard" => Some(render_checkerboard(width, height, seed)),
        "progress_bar" => Some(render_progress_bar(width, height, request)),
        "subcell_shape_atlas" => Some(render_subcell_shape_atlas(width, height, seed)),
        _ => None,
    }
}

fn render_dots_spinner(
    width: usize,
    height: usize,
    seed: usize,
    request: &PlayerSampleRequest,
) -> Vec<String> {
    let glyphs = ["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];
    let frame = (((request.loop_t.unwrap_or(request.phase_t).clamp(0.0, 1.0) * glyphs.len() as f64)
        .floor() as usize)
        + seed)
        % glyphs.len();
    text_rows(&format!("{} dots spinner", glyphs[frame]), width, height)
}

fn render_checkerboard(width: usize, height: usize, seed: usize) -> Vec<String> {
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    if (x + y + seed).is_multiple_of(2) {
                        '░'
                    } else {
                        '█'
                    }
                })
                .collect()
        })
        .collect()
}

fn render_progress_bar(width: usize, height: usize, request: &PlayerSampleRequest) -> Vec<String> {
    let progress = request.loop_t.unwrap_or(request.phase_t).clamp(0.0, 1.0);
    let filled = (width as f64 * progress).round() as usize;
    let row = (0..width)
        .map(|x| if x < filled { '█' } else { '░' })
        .collect::<String>();
    vec![row; height]
}

fn render_subcell_shape_atlas(width: usize, height: usize, seed: usize) -> Vec<String> {
    let glyphs = ['▘', '▝', '▖', '▗', '▀', '▄', '▌', '▐'];
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| glyphs[(x + y * width + seed) % glyphs.len()])
                .collect()
        })
        .collect()
}

fn text_rows(text: &str, width: usize, height: usize) -> Vec<String> {
    let mut rows = vec![" ".repeat(width); height];
    for (index, line) in text.lines().take(height).enumerate() {
        rows[index] = clip_or_pad(line, width);
    }
    rows
}

fn clip_or_pad(value: &str, width: usize) -> String {
    let mut clipped = value.chars().take(width).collect::<String>();
    let clipped_width = clipped.chars().count();
    clipped.extend(std::iter::repeat_n(
        ' ',
        width.saturating_sub(clipped_width),
    ));
    clipped
}

// <FILE>crates/tui-vfx-player/src/fnc_render_procedural_source.rs</FILE> - <DESC>Render bounded procedural source registry entries</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
