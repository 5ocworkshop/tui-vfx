// <FILE>tui-vfx-style/tests/test_models.rs</FILE> - <DESC>Linker file for models integration tests</DESC>
// <VERS>VERSION: 0.8.0</VERS>
// <WCTX>Sub-plan A Phase A.2 — register three new test modules for the role-aware StyleRegion refactor (fnc_should_style, fnc_bounding_rect, legacy_parse_round_trip)</WCTX>
// <CLOG>0.8.0: MINOR — register test_fnc_style_region_should_style, test_fnc_style_region_bounding_rect, test_style_region_legacy_parse_round_trip
// 0.7.0: Register test_cls_cursor_shader module</CLOG>

#[path = "common/mod.rs"]
mod common;

#[path = "models/test_cls_affordance_wake_shader.rs"]
mod test_cls_affordance_wake_shader;
#[path = "models/test_cls_color_space.rs"]
mod test_cls_color_space;
#[path = "models/test_cls_concealed_light_shader.rs"]
mod test_cls_concealed_light_shader;
#[path = "models/test_cls_cursor_shader.rs"]
mod test_cls_cursor_shader;
#[path = "models/test_cls_diffusion_shader.rs"]
mod test_cls_diffusion_shader;
#[path = "models/test_cls_easing_type.rs"]
mod test_cls_easing_type;
#[path = "models/test_cls_fade_to_black.rs"]
mod test_cls_fade_to_black;
#[path = "models/test_cls_focus_field_shader.rs"]
mod test_cls_focus_field_shader;
#[path = "models/test_cls_focused_row_gradient_shader.rs"]
mod test_cls_focused_row_gradient_shader;
#[path = "models/test_cls_glisten_band_shader.rs"]
mod test_cls_glisten_band_shader;
#[path = "models/test_cls_gradient.rs"]
mod test_cls_gradient;
#[path = "models/test_cls_gradient_lut.rs"]
mod test_cls_gradient_lut;
#[path = "models/test_cls_linear_gradient_shader.rs"]
mod test_cls_linear_gradient_shader;
#[path = "models/test_cls_neon_flicker_shader.rs"]
mod test_cls_neon_flicker_shader;
#[path = "models/test_cls_spatial_shader_type.rs"]
mod test_cls_spatial_shader_type;
#[path = "models/test_terminal_fire_recipes.rs"]
mod test_terminal_fire_recipes;
#[path = "models/test_cls_style_effect.rs"]
mod test_cls_style_effect;
#[path = "models/test_cls_style_layer.rs"]
mod test_cls_style_layer;
#[path = "models/test_cls_style_region.rs"]
mod test_cls_style_region;
#[path = "models/test_cls_style_transition.rs"]
mod test_cls_style_transition;
#[path = "models/test_cls_wayfinding_node_shader.rs"]
mod test_cls_wayfinding_node_shader;
#[path = "models/test_effect_descriptions.rs"]
mod test_effect_descriptions;
#[path = "models/test_fnc_apply_style_effects_to_scene.rs"]
mod test_fnc_apply_style_effects_to_scene;
#[path = "models/test_fnc_style_region_bounding_rect.rs"]
mod test_fnc_style_region_bounding_rect;
#[path = "models/test_fnc_style_region_should_style.rs"]
mod test_fnc_style_region_should_style;
#[path = "models/test_serde_roundtrip.rs"]
mod test_serde_roundtrip;
#[path = "models/test_style_region_legacy_parse_round_trip.rs"]
mod test_style_region_legacy_parse_round_trip;

// <FILE>tui-vfx-style/tests/test_models.rs</FILE> - <DESC>Linker file for models integration tests</DESC>
// <VERS>END OF VERSION: 0.8.0</VERS>
