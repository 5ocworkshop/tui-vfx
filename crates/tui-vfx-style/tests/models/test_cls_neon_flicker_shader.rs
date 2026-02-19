// <FILE>tui-vfx-style/tests/models/test_cls_neon_flicker_shader.rs</FILE> - <DESC>Tests for NeonFlickerShader</DESC>
// <VERS>VERSION: 1.1.1</VERS>
// <WCTX>Consolidate style test helpers</WCTX>
// <CLOG>Use shared ShaderContext helper</CLOG>
use crate::common::make_ctx;

use tui_vfx_style::models::{NeonFlickerShader, NoiseType, SegmentMode};
use tui_vfx_style::traits::StyleShader;
use tui_vfx_style::utils::darken;
use tui_vfx_types::{Color, Style};

fn expected_dim(shader: &NeonFlickerShader, segment_id: u32, t: f64) -> f32 {
    let time_component = (t * 5000.0 * shader.speed as f64).floor() as u32;
    let input = (shader.seed as u32)
        .wrapping_mul(segment_id.wrapping_add(1))
        .wrapping_add(time_component);
    let noise = shader.noise_type.sample(input as u64);
    if noise > shader.stability {
        let overage = (noise - shader.stability) / (1.0 - shader.stability + 0.001);
        overage * shader.dim_amount
    } else {
        0.0
    }
}

#[test]
fn test_cell_segment_id_pairing() {
    let shader = NeonFlickerShader {
        stability: 0.0,
        seed: 7,
        segment: SegmentMode::Cell,
        dim_amount: 0.8,
        speed: 1.0,
        flash_chance: 0.0,
        decay_rate: None,
        noise_type: NoiseType::Uniform,
    };
    let base_color = Color::rgb(200, 100, 50);
    let base = Style::fg(base_color);
    let t = 0.0;
    let x = 500;
    let y = 1;
    let segment_id = ((y as u32) << 16) | (x as u32);
    let dim = expected_dim(&shader, segment_id, t);
    let styled = shader.style_at(&make_ctx(x, y, 2000, 2000, t), base);
    let expected_fg = darken(base_color, dim);
    assert_eq!(styled.fg, expected_fg);
}

// <FILE>tui-vfx-style/tests/models/test_cls_neon_flicker_shader.rs</FILE> - <DESC>Tests for NeonFlickerShader</DESC>
// <VERS>END OF VERSION: 1.1.1</VERS>
