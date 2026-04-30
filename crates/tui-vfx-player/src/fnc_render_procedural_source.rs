// <FILE>crates/tui-vfx-player/src/fnc_render_procedural_source.rs</FILE> - <DESC>Render bounded procedural source registry entries</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Source fidelity adapters: keep procedural source generation deterministic and command-free.</WCTX>
// <CLOG>0.2.0: MINOR — copy/adapt Madeira procedural generators for v3.1 player-local use.
// 0.1.0: INIT — add bounded dots-spinner and checkerboard registry rendering.</CLOG>

use serde_json::Value;

use crate::{PlayerSampleRequest, PlayerStyledGrid};

/// Render a registered deterministic procedural source, if the name is supported.
pub(crate) fn render_registered_procedural_source(
    generator: &str,
    width: usize,
    height: usize,
    seed: usize,
    request: &PlayerSampleRequest,
    params: &Value,
) -> Option<RenderedProceduralSource> {
    match generator {
        "dots_spinner" => Some(RenderedProceduralSource::from_rows(render_dots_spinner(
            width, height, seed, request,
        ))),
        "checkerboard" => Some(RenderedProceduralSource::from_rows(render_checkerboard(
            width, height, seed,
        ))),
        "progress_bar" => Some(RenderedProceduralSource::from_rows(render_progress_bar(
            width, height, request,
        ))),
        "subcell_shape_atlas" => Some(RenderedProceduralSource::from_rows(
            render_subcell_shape_atlas(width, height, seed),
        )),
        "solid_color_fade" => Some(render_solid_color_fade(width, height, params)),
        "braille_flag_field" => Some(render_braille_flag_field(width, height, request, params)),
        "ballistic_fireworks" => Some(render_ballistic_fireworks(
            width, height, seed, request, params,
        )),
        _ => None,
    }
}

/// Rendered procedural rows with styled-cell evidence when the generator owns it.
pub(crate) struct RenderedProceduralSource {
    pub(crate) rows: Vec<String>,
    pub(crate) styled_grid: PlayerStyledGrid,
}

impl RenderedProceduralSource {
    fn from_rows(rows: Vec<String>) -> Self {
        Self {
            styled_grid: PlayerStyledGrid::from_rows(&rows),
            rows,
        }
    }
}

fn render_braille_flag_field(
    width: usize,
    height: usize,
    request: &PlayerSampleRequest,
    params: &Value,
) -> RenderedProceduralSource {
    let empty_rows = || vec![" ".repeat(width); height];
    let Some(canvas) = load_flag_asset_canvas(params) else {
        return RenderedProceduralSource::from_rows(empty_rows());
    };
    if canvas.width == 0 || canvas.height == 0 || width == 0 || height == 0 {
        return RenderedProceduralSource::from_rows(empty_rows());
    }

    let flag_height_cells = u64_path(params, &["layout", "flag_height_cells"])
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or(height)
        .min(height)
        .max(1);
    let flag_top_rows = height.saturating_sub(flag_height_cells) / 2;
    let output_width_dots = width * 2;
    let output_height_dots = flag_height_cells * 4;
    let wave_speed = f64_path(params, &["wave", "speed"]).unwrap_or(1.0).max(0.0) as f32;
    let time_s = elapsed_seconds(request) as f32 * wave_speed;

    let mut rows = vec![vec![' '; width]; height];
    let mut styled_grid = PlayerStyledGrid::blank(width, height, false);
    for (y, row) in rows.iter_mut().enumerate().take(height) {
        for (x, cell) in row.iter_mut().enumerate().take(width) {
            if let Some((glyph, foreground)) = emit_braille_cell(x, y, |sample_x, sample_y| {
                displaced_flag_dot(
                    &canvas,
                    sample_x,
                    sample_y,
                    output_width_dots,
                    output_height_dots,
                    flag_top_rows * 4,
                    time_s,
                    params,
                )
            }) {
                *cell = glyph;
                styled_grid.set_cell_glyph_and_style(
                    x,
                    y,
                    &glyph.to_string(),
                    &foreground.rgba_label(),
                    "transparent",
                    vec![],
                    Some("Procedural".to_string()),
                );
            }
        }
    }
    RenderedProceduralSource {
        rows: rows_to_strings(rows),
        styled_grid,
    }
}

#[derive(Clone, Copy)]
struct DotColor {
    r: u8,
    g: u8,
    b: u8,
}

impl DotColor {
    fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn rgba_label(self) -> String {
        format!("rgba({},{},{},255)", self.r, self.g, self.b)
    }
}

struct DotCanvas {
    width: usize,
    height: usize,
    pixels: Vec<Option<DotColor>>,
}

impl DotCanvas {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![None; width * height],
        }
    }

    fn set(&mut self, x: usize, y: usize, color: DotColor) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = Some(color);
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<DotColor> {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            None
        }
    }
}

fn load_flag_asset_canvas(params: &Value) -> Option<DotCanvas> {
    let asset = load_flag_asset_value(params)?;
    let rows = asset.get("rows")?.as_array()?;
    let palette = asset.get("palette")?.as_object()?;
    let transparent = asset
        .get("transparent")
        .and_then(Value::as_str)
        .and_then(|value| value.chars().next())
        .unwrap_or('.');
    let width = asset
        .get("width_dots")
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or_else(|| {
            rows.iter()
                .filter_map(Value::as_str)
                .map(|row| row.chars().count())
                .max()
                .unwrap_or(0)
        });
    let height = asset
        .get("height_dots")
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or(rows.len());
    let mut canvas = DotCanvas::new(width, height);
    for (y, row) in rows.iter().filter_map(Value::as_str).enumerate() {
        for (x, token) in row.chars().enumerate() {
            if token == transparent {
                continue;
            }
            if let Some(color) = palette.get(&token.to_string()).and_then(color_from_value) {
                canvas.set(x, y, color);
            }
        }
    }
    Some(canvas)
}

fn load_flag_asset_value(params: &Value) -> Option<Value> {
    let asset = params.get("asset")?;
    if asset.get("rows").is_some() {
        return Some(asset.clone());
    }
    let path = asset.get("path").or_else(|| asset.get("file"))?.as_str()?;
    if path.contains("{{") {
        return None;
    }
    let contents = std::fs::read_to_string(path).ok().or_else(|| {
        std::fs::read_to_string(std::path::Path::new("/usr/projects/tui-vfx-recipes").join(path))
            .ok()
    })?;
    serde_json::from_str(&contents).ok()
}

fn displaced_flag_dot(
    canvas: &DotCanvas,
    output_dot_x: usize,
    output_dot_y: usize,
    output_width_dots: usize,
    output_height_dots: usize,
    flag_top_dots: usize,
    time_s: f32,
    params: &Value,
) -> Option<DotColor> {
    if canvas.width == 0 || canvas.height == 0 || output_height_dots == 0 {
        return None;
    }
    let normalized_x = normalized_index(output_dot_x, output_width_dots);
    let wave = wave_field(normalized_x, time_s, params);
    let amplitude = normalized_x * f32_path(params, &["wave", "max_amplitude"]).unwrap_or(0.15);
    let relative_y = output_dot_y as f32 - flag_top_dots as f32;
    let normalized_y = relative_y / output_height_dots.saturating_sub(1).max(1) as f32;
    let source_y = ((normalized_y - amplitude * wave)
        * canvas.height.saturating_sub(1).max(1) as f32)
        .round() as isize;
    let source_x = (normalized_x * canvas.width.saturating_sub(1).max(1) as f32).round() as usize;
    if source_y < 0 || source_y >= canvas.height as isize {
        return None;
    }
    let source = canvas.get(
        source_x.min(canvas.width.saturating_sub(1)),
        source_y as usize,
    )?;
    Some(scale_color(
        source,
        f64::from(shade_from_wave(wave, params)),
    ))
}

fn shade_from_wave(wave: f32, params: &Value) -> f32 {
    let base = f32_path(params, &["shading", "base"]).unwrap_or(0.75);
    let scale = f32_path(params, &["shading", "scale"]).unwrap_or(0.25);
    let min = f32_path(params, &["shading", "min"]).unwrap_or(0.65);
    let max = f32_path(params, &["shading", "max"]).unwrap_or(1.0);
    (wave * scale + base).clamp(min, max)
}

fn scale_color(color: DotColor, shade: f64) -> DotColor {
    DotColor::rgb(
        ((color.r as f64) * shade).round().clamp(0.0, 255.0) as u8,
        ((color.g as f64) * shade).round().clamp(0.0, 255.0) as u8,
        ((color.b as f64) * shade).round().clamp(0.0, 255.0) as u8,
    )
}

fn normalized_index(index: usize, extent: usize) -> f32 {
    index as f32 / extent.saturating_sub(1).max(1) as f32
}

fn wave_field(normalized_x: f32, time_s: f32, params: &Value) -> f32 {
    let primary_cycles = f32_path(params, &["wave", "primary_cycles"]).unwrap_or(8.0);
    let primary_rate = f32_path(params, &["wave", "primary_rate"]).unwrap_or(2.4);
    let secondary_cycles = f32_path(params, &["wave", "secondary_cycles"]).unwrap_or(15.0);
    let secondary_rate = f32_path(params, &["wave", "secondary_rate"]).unwrap_or(4.0);
    let secondary_scale = f32_path(params, &["wave", "secondary_scale"]).unwrap_or(0.3);
    let primary = (normalized_x * primary_cycles - time_s * primary_rate).sin();
    let secondary =
        (normalized_x * secondary_cycles - time_s * secondary_rate).sin() * secondary_scale;
    primary + secondary
}

const BRAILLE_DOT_ORDER: [(usize, usize, u8); 8] = [
    (0, 0, 0x01),
    (0, 1, 0x02),
    (0, 2, 0x04),
    (1, 0, 0x08),
    (1, 1, 0x10),
    (1, 2, 0x20),
    (0, 3, 0x40),
    (1, 3, 0x80),
];

fn emit_braille_cell(
    cell_x: usize,
    cell_y: usize,
    sample_dot: impl Fn(usize, usize) -> Option<DotColor>,
) -> Option<(char, DotColor)> {
    let mut braille_pattern: u8 = 0;
    let mut total_r: u32 = 0;
    let mut total_g: u32 = 0;
    let mut total_b: u32 = 0;
    let mut dot_count: u32 = 0;
    for (dx, dy, bit) in BRAILLE_DOT_ORDER {
        if let Some(color) = sample_dot(cell_x * 2 + dx, cell_y * 4 + dy) {
            braille_pattern |= bit;
            total_r += u32::from(color.r);
            total_g += u32::from(color.g);
            total_b += u32::from(color.b);
            dot_count += 1;
        }
    }
    if braille_pattern == 0 || dot_count == 0 {
        return None;
    }
    Some((
        char::from_u32(0x2800 + braille_pattern as u32).unwrap_or('⣿'),
        DotColor::rgb(
            (total_r / dot_count) as u8,
            (total_g / dot_count) as u8,
            (total_b / dot_count) as u8,
        ),
    ))
}

fn render_ballistic_fireworks(
    width: usize,
    height: usize,
    seed: usize,
    request: &PlayerSampleRequest,
    params: &Value,
) -> RenderedProceduralSource {
    let mut rows = vec![vec![' '; width]; height];
    let mut styled_grid = PlayerStyledGrid::blank(width, height, false);
    let slot_count = u64_key(params, "slot_count").unwrap_or(12) as usize;
    if slot_count == 0 || width == 0 || height == 0 {
        return RenderedProceduralSource {
            rows: rows_to_strings(rows),
            styled_grid,
        };
    }
    let elapsed_ms = elapsed_seconds(request) * 1000.0;
    let base_ms = f64_key(params, "cycle_base_ms").unwrap_or(2500.0).max(1.0);
    let jitter_ms = f64_key(params, "cycle_jitter_ms")
        .unwrap_or(1500.0)
        .max(0.0);
    let slot_phase_offset_ms = f64_key(params, "slot_phase_offset_ms").unwrap_or(600.0);
    let peak_fraction = f64_key(params, "peak_time_fraction")
        .unwrap_or(0.2)
        .clamp(0.05, 0.8);
    let expansion_max = f64_key(params, "expansion_max").unwrap_or(1.5).max(0.1);
    let gravity_scale = f64_key(params, "gravity_scale").unwrap_or(0.15).max(0.0);
    let seed = u64_key(params, "seed").unwrap_or(seed as u64);
    let (particle_min, particle_max) = usize_range_param(params, "particle_count_range", 10, 16);
    let (speed_min, speed_max) = f64_range_named(params, "particle_speed_range", 0.04, 0.12);
    for slot in 0..slot_count {
        let slot_seed = mix64(seed ^ (slot as u64).wrapping_mul(7_919));
        let speed_variation = (elapsed_ms * 0.0001 + slot as f64 * 0.5).sin() * 0.3 + 1.0;
        let cycle_period_ms =
            (base_ms + jitter_ms * unit_f64(slot_seed)).max(1.0) * speed_variation.max(0.25);
        let slot_elapsed_ms = elapsed_ms + slot as f64 * slot_phase_offset_ms;
        let cycle_index = (slot_elapsed_ms / cycle_period_ms).floor().max(0.0) as u64;
        let cycle_time_ms = slot_elapsed_ms.rem_euclid(cycle_period_ms);
        let cycle_seed = mix64(slot_seed ^ cycle_index.wrapping_mul(13));

        let peak_time_ms = cycle_period_ms * peak_fraction;
        let hold_ms = (cycle_period_ms * 0.08).min(200.0);
        let fade_duration_ms = (cycle_period_ms - peak_time_ms - hold_ms).max(1.0);
        let burst_time_s = ((cycle_time_ms - peak_time_ms) / 1000.0).max(0.0);
        let base_intensity = if cycle_time_ms < peak_time_ms {
            (cycle_time_ms / peak_time_ms).sqrt()
        } else if cycle_time_ms < peak_time_ms + hold_ms {
            1.0
        } else {
            let fade_progress = (cycle_time_ms - peak_time_ms - hold_ms) / fade_duration_ms;
            (1.0 - fade_progress).max(0.0).powf(0.6)
        };
        if base_intensity <= 0.01 {
            continue;
        }

        let expansion = (cycle_time_ms * 0.0015).min(expansion_max);
        let gravity = burst_time_s * burst_time_s * gravity_scale;
        let (base_x, base_y) = spawn_point(params, cycle_seed);
        let color = palette_color(params, cycle_seed as usize);
        let particle_count = ranged_count(cycle_seed, particle_min, particle_max);
        for particle in 0..particle_count {
            let particle_seed =
                mix64(cycle_seed ^ (particle as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
            let angle = (particle as f64 / particle_count as f64) * std::f64::consts::TAU
                + ranged_signed_f64(particle_seed, 0.25);
            let speed = ranged_f64(mix64(particle_seed ^ 0xA5A5), speed_min, speed_max);
            let px = base_x + angle.cos() * speed * expansion;
            let initial_py = base_y + angle.sin() * speed * expansion * 0.5;
            let py = initial_py + gravity;
            let fall_distance = (py - base_y).max(0.0);
            let intensity = base_intensity * (1.0 - fall_distance * 2.0).max(0.0);
            if intensity <= 0.01 {
                continue;
            }

            let screen_x = (px * width as f64) as i32;
            let screen_y = (py * height as f64) as i32;
            if screen_x < 0 || screen_y < 0 || screen_x >= width as i32 || screen_y >= height as i32
            {
                continue;
            }

            let x = screen_x as usize;
            let y = screen_y as usize;
            let foreground = scale_color(color, intensity);
            let candidate_strength =
                u16::from(foreground.r) + u16::from(foreground.g) + u16::from(foreground.b);
            if let Some(existing) = styled_grid.cells().get(y * width + x) {
                let existing_strength = color_strength(&existing.foreground);
                if rows[y][x] != ' ' && existing_strength > candidate_strength {
                    continue;
                }
            }

            let glyph = sparkle_char(params, particle_seed as usize);
            rows[y][x] = glyph;
            styled_grid.set_cell_glyph_and_style(
                x,
                y,
                &glyph.to_string(),
                &foreground.rgba_label(),
                "transparent",
                vec![],
                Some("Procedural".to_string()),
            );
        }
    }
    RenderedProceduralSource {
        rows: rows_to_strings(rows),
        styled_grid,
    }
}

fn render_solid_color_fade(
    width: usize,
    height: usize,
    params: &Value,
) -> RenderedProceduralSource {
    let rows = vec![" ".repeat(width); height];
    let mut styled_grid = PlayerStyledGrid::blank(width, height, false);
    let background = color_path(params, &["target_color"])
        .or_else(|| color_path(params, &["color"]))
        .unwrap_or_else(|| DotColor::rgb(0, 0, 0))
        .rgba_label();
    for y in 0..height {
        for x in 0..width {
            styled_grid.set_cell_style(
                x,
                y,
                "defaultForeground",
                &background,
                vec![],
                Some("Procedural".to_string()),
            );
        }
    }
    RenderedProceduralSource { rows, styled_grid }
}

fn rows_to_strings(rows: Vec<Vec<char>>) -> Vec<String> {
    rows.into_iter()
        .map(|row| row.into_iter().collect())
        .collect()
}

fn color_from_value(value: &Value) -> Option<DotColor> {
    Some(DotColor::rgb(
        object_u8(value, "r")?,
        object_u8(value, "g")?,
        object_u8(value, "b")?,
    ))
}

fn color_path(value: &Value, path: &[&str]) -> Option<DotColor> {
    color_from_value(lookup(value, path)?)
}

fn object_u8(value: &Value, key: &str) -> Option<u8> {
    value
        .get(key)
        .and_then(json_u64)
        .and_then(|value| u8::try_from(value).ok())
}

fn color_strength(label: &str) -> u16 {
    let Some(inner) = label
        .strip_prefix("rgba(")
        .and_then(|value| value.strip_suffix(')'))
    else {
        return 0;
    };
    inner
        .split(',')
        .take(3)
        .filter_map(|part| part.trim().parse::<u16>().ok())
        .sum()
}

fn spawn_point(params: &Value, seed: u64) -> (f64, f64) {
    let Some(zones) = params
        .get("spawn_zones")
        .and_then(|zones| zones.get("quadrants"))
        .and_then(Value::as_array)
    else {
        return (
            ranged_f64(seed, 0.1, 0.9),
            ranged_f64(seed ^ 0x11, 0.08, 0.3),
        );
    };

    let total_weight: f64 = zones
        .iter()
        .map(|zone| f64_key(zone, "weight").unwrap_or(1.0).max(0.0))
        .sum();
    if total_weight <= 0.0 {
        return (
            ranged_f64(seed, 0.1, 0.9),
            ranged_f64(seed ^ 0x11, 0.08, 0.3),
        );
    }

    let mut target = unit_f64(seed) * total_weight;
    for zone in zones {
        let weight = f64_key(zone, "weight").unwrap_or(1.0).max(0.0);
        if target > weight {
            target -= weight;
            continue;
        }
        let (min_x, max_x) = f64_range_named(zone, "x_range", 0.1, 0.9);
        let (min_y, max_y) = f64_range_named(zone, "y_range", 0.08, 0.3);
        return (
            ranged_f64(seed ^ 0x22, min_x, max_x),
            ranged_f64(seed ^ 0x33, min_y, max_y),
        );
    }

    (
        ranged_f64(seed, 0.1, 0.9),
        ranged_f64(seed ^ 0x11, 0.08, 0.3),
    )
}

fn palette_color(params: &Value, index: usize) -> DotColor {
    params
        .get("palette")
        .and_then(Value::as_array)
        .and_then(|palette| palette.get(index % palette.len().max(1)))
        .and_then(color_from_value)
        .unwrap_or_else(|| default_palette_color(index))
}

fn default_palette_color(index: usize) -> DotColor {
    match index % 8 {
        0 => DotColor::rgb(255, 215, 0),
        1 => DotColor::rgb(255, 69, 0),
        2 => DotColor::rgb(0, 255, 127),
        3 => DotColor::rgb(255, 20, 147),
        4 => DotColor::rgb(0, 191, 255),
        5 => DotColor::rgb(255, 255, 255),
        6 => DotColor::rgb(255, 140, 0),
        _ => DotColor::rgb(138, 43, 226),
    }
}

fn sparkle_char(params: &Value, index: usize) -> char {
    const DEFAULT_SPARKLE_CHARS: [char; 8] = ['✦', '✧', '★', '☆', '·', '•', '✴', '✳'];
    params
        .get("sparkle_chars")
        .and_then(Value::as_array)
        .and_then(|chars| chars.get(index % chars.len().max(1)))
        .and_then(Value::as_str)
        .and_then(|value| value.chars().next())
        .unwrap_or(DEFAULT_SPARKLE_CHARS[index % DEFAULT_SPARKLE_CHARS.len()])
}

fn ranged_count(seed: u64, min: usize, max: usize) -> usize {
    min + ((mix64(seed) as usize) % (max.saturating_sub(min) + 1).max(1))
}

fn usize_range_param(value: &Value, key: &str, min: usize, max: usize) -> (usize, usize) {
    let Some(range) = value.get(key).and_then(Value::as_array) else {
        return (min, max.max(min));
    };
    let parsed_min = range
        .first()
        .and_then(json_u64)
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or(min);
    let parsed_max = range
        .get(1)
        .and_then(json_u64)
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or(max);
    if parsed_min <= parsed_max {
        (parsed_min, parsed_max)
    } else {
        (parsed_max, parsed_min)
    }
}

fn f64_range_named(value: &Value, key: &str, min: f64, max: f64) -> (f64, f64) {
    let Some(range) = value.get(key).and_then(Value::as_array) else {
        return (min, max.max(min));
    };
    let parsed_min = range.first().and_then(Value::as_f64).unwrap_or(min);
    let parsed_max = range.get(1).and_then(Value::as_f64).unwrap_or(max);
    if parsed_min <= parsed_max {
        (parsed_min, parsed_max)
    } else {
        (parsed_max, parsed_min)
    }
}

fn lookup<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    path.iter()
        .try_fold(value, |current, key| current.get(*key))
}

fn f32_path(value: &Value, path: &[&str]) -> Option<f32> {
    lookup(value, path)
        .and_then(Value::as_f64)
        .map(|value| value as f32)
}

fn f64_path(value: &Value, path: &[&str]) -> Option<f64> {
    lookup(value, path).and_then(Value::as_f64)
}

fn u64_path(value: &Value, path: &[&str]) -> Option<u64> {
    lookup(value, path).and_then(json_u64)
}

fn f64_key(value: &Value, key: &str) -> Option<f64> {
    value.get(key).and_then(Value::as_f64)
}

fn u64_key(value: &Value, key: &str) -> Option<u64> {
    value.get(key).and_then(json_u64)
}

fn json_u64(value: &Value) -> Option<u64> {
    value.as_u64().or_else(|| {
        value
            .as_f64()
            .filter(|value| value.is_finite() && *value >= 0.0)
            .map(|value| value.round() as u64)
    })
}

fn mix64(seed: u64) -> u64 {
    let mut value = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

fn unit_f64(seed: u64) -> f64 {
    ((mix64(seed) >> 11) as f64) * (1.0 / ((1u64 << 53) as f64))
}

fn ranged_f64(seed: u64, min: f64, max: f64) -> f64 {
    min + (max - min) * unit_f64(seed)
}

fn ranged_signed_f64(seed: u64, magnitude: f64) -> f64 {
    (unit_f64(seed) * 2.0 - 1.0) * magnitude
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

fn elapsed_seconds(request: &PlayerSampleRequest) -> f64 {
    request
        .absolute_t_ms
        .map(|absolute_t_ms| absolute_t_ms.max(0.0) / 1000.0)
        .unwrap_or_else(|| request.loop_t.unwrap_or(request.phase_t).max(0.0))
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
// <VERS>END OF VERSION: 0.2.0</VERS>
