// <FILE>xtask/src/docs/effect_metadata.rs</FILE> - <DESC>Effect metadata extraction from runtime introspection</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Keep generated metadata aligned with shadow style variants</WCTX>
// <CLOG>Add MediumShade to shadow metadata extraction variants</CLOG>

use std::collections::HashMap;

use tui_vfx_compositor::types::{FilterSpec, MaskSpec, SamplerSpec};
use tui_vfx_content::types::ContentEffect;
use tui_vfx_shadow::types::ShadowStyle;
use tui_vfx_style::models::{ColorConfig, SpatialShaderType, StyleEffect};

/// Metadata for a single effect variant.
#[derive(Debug, Clone)]
pub struct EffectMetadata {
    /// The variant name (e.g., "Wipe", "Dissolve")
    pub name: String,
    /// Brief description of the effect
    pub description: String,
    /// Key parameters with their types/default values
    pub parameters: Vec<(String, String)>,
}

/// Collected metadata for all effect categories.
#[derive(Debug, Default)]
pub struct AllEffectMetadata {
    pub masks: HashMap<String, EffectMetadata>,
    pub filters: HashMap<String, EffectMetadata>,
    pub samplers: HashMap<String, EffectMetadata>,
    pub shaders: HashMap<String, EffectMetadata>,
    pub styles: HashMap<String, EffectMetadata>,
    pub content: HashMap<String, EffectMetadata>,
    pub shadows: HashMap<String, EffectMetadata>,
}

/// Extract metadata from all effect types using runtime introspection.
pub fn extract_all_metadata() -> AllEffectMetadata {
    AllEffectMetadata {
        masks: extract_mask_metadata(),
        filters: extract_filter_metadata(),
        samplers: extract_sampler_metadata(),
        shaders: extract_shader_metadata(),
        styles: extract_style_metadata(),
        content: extract_content_metadata(),
        shadows: extract_shadow_metadata(),
    }
}

fn extract_mask_metadata() -> HashMap<String, EffectMetadata> {
    let variants: Vec<MaskSpec> = vec![
        MaskSpec::None,
        MaskSpec::Wipe {
            reveal: None,
            hide: None,
            direction: None,
            soft_edge: false,
        },
        MaskSpec::Dissolve {
            seed: 0,
            chunk_size: 1,
        },
        MaskSpec::Checkers { cell_size: 2 },
        MaskSpec::Blinds {
            orientation: Default::default(),
            count: 8,
        },
        MaskSpec::Iris {
            shape: Default::default(),
            soft_edge: true,
        },
        MaskSpec::Diamond { soft_edge: true },
        MaskSpec::NoiseDither {
            seed: 0,
            matrix: Default::default(),
        },
        MaskSpec::PathReveal {
            path: Default::default(),
            soft_edge: true,
        },
        MaskSpec::Radial {
            origin: Default::default(),
            soft_edge: true,
        },
        MaskSpec::Cellular {
            pattern: Default::default(),
            seed: 0,
            cell_count: 16,
        },
    ];

    variants
        .into_iter()
        .map(|v| {
            let name = v.name().to_string();
            let meta = EffectMetadata {
                name: name.clone(),
                description: v.terse_description().to_string(),
                parameters: v
                    .key_parameters()
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect(),
            };
            (name, meta)
        })
        .collect()
}

fn extract_filter_metadata() -> HashMap<String, EffectMetadata> {
    let variants: Vec<FilterSpec> = vec![
        FilterSpec::None,
        FilterSpec::Dim {
            factor: Default::default(),
            apply_to: Default::default(),
        },
        FilterSpec::Invert {
            apply_to: Default::default(),
        },
        FilterSpec::Tint {
            color: ColorConfig::White,
            strength: Default::default(),
            apply_to: Default::default(),
        },
        FilterSpec::Vignette {
            strength: Default::default(),
            radius: Default::default(),
        },
        FilterSpec::Crt {
            scanline_strength: Default::default(),
            glow: Default::default(),
        },
        FilterSpec::PatternFill {
            pattern: Default::default(),
            color: None,
            only_empty: false,
        },
        FilterSpec::Greyscale {
            strength: Default::default(),
            apply_to: Default::default(),
        },
        FilterSpec::BrailleDust {
            density: 0.03,
            hz: 8.0,
            seed: 42,
            pattern: Default::default(),
            color: None,
        },
        FilterSpec::InterlaceCurtain {
            density: 1.0,
            dim_factor: 0.3,
            scroll_speed: 0.0,
        },
        FilterSpec::MotionBlur {
            trail_length: 0.5,
            opacity_decay: 1.5,
            direction: Default::default(),
        },
        FilterSpec::ColorBridgedShade {
            opacity: 0.5,
            fg_color: ColorConfig::White,
            bg_color: ColorConfig::Black,
        },
        FilterSpec::SubPixelBar {
            progress: 0.5,
            direction: Default::default(),
            filled_color: ColorConfig::Green,
            unfilled_color: ColorConfig::Gray,
            animated: false,
        },
        FilterSpec::SubCellShake {
            amplitude: 2,
            frequency: 8.0,
            seed: 42,
            edge_only: false,
            filled_color: ColorConfig::Blue,
            bg_color: ColorConfig::Black,
        },
        FilterSpec::RigidShake {
            shake_period: 0.29,
            num_shakes: 4,
            pause_duration: 0.52,
            max_eighths: 12,
            base_eighths: 3,
            damping: vec![1.0],
            element_color: ColorConfig::Gray,
            bg_color: ColorConfig::Black,
            inner_width: 10,
            margin_width: 2,
        },
        FilterSpec::HoverBar {
            base_eighths: 4,
            max_eighths: 12,
            position: Default::default(),
            bar_color: ColorConfig::Blue,
            bg_color: ColorConfig::Black,
            progress: 0.0,
            margin_width: 2,
        },
        FilterSpec::UnderlineWipe {
            direction: Default::default(),
            color: ColorConfig::Blue,
            bg_color: ColorConfig::Black,
            line_char: '—',
            row_offset: 0,
            progress: 0.0,
            gradient: true,
            glisten: true,
        },
        FilterSpec::BracketEmphasis {
            left: '[',
            right: ']',
            color: ColorConfig::Blue,
            bg_color: ColorConfig::Black,
            progress: 0.0,
        },
        FilterSpec::DotIndicator {
            indicator_char: '•',
            position: Default::default(),
            color: ColorConfig::Blue,
            bg_color: ColorConfig::Black,
            progress: 0.0,
        },
        FilterSpec::PillButton {
            button_color: ColorConfig::Blue,
            bg_color: ColorConfig::Black,
            edge_width: 3,
            glisten: true,
            progress: 0.0,
        },
        FilterSpec::GlistenSweep {
            boost: 40,
            band_width: 0.2,
            speed: 0.5,
            progress: 0.0,
            powerline_mode: false,
            boost_separator_bg: false,
        },
        FilterSpec::KittScanner {
            boost: 50,
            band_width: 0.15,
            bps: 1.0,
            progress: 0.0,
            apply_to: Default::default(),
            powerline_mode: false,
            boost_separator_bg: false,
        },
        FilterSpec::ShadeScanner {
            shade_color: ColorConfig::Gray,
            bps: 1.0,
            progress: 0.0,
        },
    ];

    variants
        .into_iter()
        .map(|v| {
            let name = v.name().to_string();
            let meta = EffectMetadata {
                name: name.clone(),
                description: v.terse_description().to_string(),
                parameters: v
                    .key_parameters()
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect(),
            };
            (name, meta)
        })
        .collect()
}

fn extract_sampler_metadata() -> HashMap<String, EffectMetadata> {
    let variants: Vec<SamplerSpec> = vec![
        SamplerSpec::None,
        SamplerSpec::SineWave {
            axis: Default::default(),
            amplitude: Default::default(),
            frequency: Default::default(),
            speed: Default::default(),
            phase: Default::default(),
        },
        SamplerSpec::Ripple {
            amplitude: Default::default(),
            wavelength: Default::default(),
            speed: Default::default(),
            center: Default::default(),
        },
        SamplerSpec::Shredder {
            stripe_width: 4,
            odd_speed: Default::default(),
            even_speed: Default::default(),
        },
        SamplerSpec::FaultLine {
            seed: 0,
            intensity: Default::default(),
            split_bias: 0.0,
        },
        SamplerSpec::Crt {
            scanline_strength: Default::default(),
            jitter: Default::default(),
            curvature: Default::default(),
        },
        SamplerSpec::CrtJitter {
            intensity: Default::default(),
            speed_hz: Default::default(),
            decay_ms: 1000,
        },
    ];

    variants
        .into_iter()
        .map(|v| {
            let name = v.name().to_string();
            let meta = EffectMetadata {
                name: name.clone(),
                description: v.terse_description().to_string(),
                parameters: v
                    .key_parameters()
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect(),
            };
            (name, meta)
        })
        .collect()
}

fn extract_shader_metadata() -> HashMap<String, EffectMetadata> {
    use tui_vfx_style::models::{
        Gradient, LinearGradientShader, cls_ambient_occlusion_shader::AmbientOcclusionShader,
        cls_barber_pole_shader::BarberPoleShader, cls_bevel_shader::BevelShader,
        cls_border_sweep_shader::BorderSweepShader, cls_chromatic_edge_shader::ChromaticEdgeShader,
        cls_focused_row_gradient_shader::FocusedRowGradientShader,
        cls_glisten_band_shader::GlistenBandShader, cls_glitch_lines_shader::GlitchLinesShader,
        cls_glow_shader::GlowShader, cls_highlighter_shader::HighlighterShader,
        cls_neon_flicker_shader::NeonFlickerShader, cls_pulse_wave_shader::PulseWaveShader,
        cls_radar_shader::RadarShader, cls_reflect_shader::ReflectShader,
        cls_reveal_wipe_shader::RevealWipeShader,
        cls_stochastic_sparkle_shader::StochasticSparkleShader,
        cls_sub_cell_shake_shader::SubCellShakeShader,
    };
    use tui_vfx_types::Color;

    let variants: Vec<SpatialShaderType> = vec![
        SpatialShaderType::LinearGradient(LinearGradientShader::new(Gradient::new(vec![
            (0.0, Color::BLACK),
            (1.0, Color::WHITE),
        ]))),
        SpatialShaderType::BarberPole(BarberPoleShader {
            speed: 1.0,
            stripe_width: 2,
            gap_width: 2,
            color: ColorConfig::Blue,
        }),
        SpatialShaderType::Radar(RadarShader {
            speed: 1.0,
            tail_length: 1.0,
            color: ColorConfig::Green,
        }),
        SpatialShaderType::BorderSweep(BorderSweepShader {
            speed: 1.0,
            length: 5,
            color: ColorConfig::Cyan,
        }),
        SpatialShaderType::Highlighter(HighlighterShader {
            color: ColorConfig::Yellow,
        }),
        SpatialShaderType::Reflect(ReflectShader {
            speed: 2.0,
            color: ColorConfig::White,
        }),
        SpatialShaderType::GlistenBand(GlistenBandShader::default()),
        SpatialShaderType::GlitchLines(GlitchLinesShader::default()),
        SpatialShaderType::NeonFlicker(NeonFlickerShader::default()),
        SpatialShaderType::PulseWave(PulseWaveShader::default()),
        SpatialShaderType::FocusedRowGradient(FocusedRowGradientShader::default()),
        SpatialShaderType::RevealWipe(RevealWipeShader::default()),
        SpatialShaderType::StochasticSparkle(StochasticSparkleShader::default()),
        SpatialShaderType::AmbientOcclusion(AmbientOcclusionShader::default()),
        SpatialShaderType::Bevel(BevelShader::default()),
        SpatialShaderType::Glow(GlowShader::default()),
        SpatialShaderType::SubCellShake(SubCellShakeShader::default()),
        SpatialShaderType::ChromaticEdge(ChromaticEdgeShader::default()),
    ];

    variants
        .into_iter()
        .map(|v| {
            let name = v.name().to_string();
            let meta = EffectMetadata {
                name: name.clone(),
                description: v.terse_description().to_string(),
                parameters: v
                    .key_parameters()
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect(),
            };
            (name, meta)
        })
        .collect()
}

fn extract_style_metadata() -> HashMap<String, EffectMetadata> {
    use tui_vfx_style::models::{ColorSpace, FadeApplyTo};
    use tui_vfx_types::Color;

    let variants: Vec<StyleEffect> = vec![
        StyleEffect::FadeIn {
            apply_to: FadeApplyTo::Both,
            ease: Default::default(),
        },
        StyleEffect::FadeOut {
            apply_to: FadeApplyTo::Both,
            ease: Default::default(),
        },
        StyleEffect::Pulse {
            frequency: 1.0,
            color: Color::WHITE,
        },
        StyleEffect::Rainbow { speed: 1.0 },
        StyleEffect::Glitch {
            seed: 42,
            intensity: 0.3,
            italic_start: None,
            italic_end: None,
        },
        StyleEffect::NeonFlicker { stability: 0.8 },
        StyleEffect::ItalicWindow {
            start: 0.0,
            end: 1.0,
        },
        StyleEffect::ColorShift {
            hue_shift: 0.0,
            saturation_shift: 0.0,
            lightness_shift: 0.0,
        },
        StyleEffect::ColorFade {
            target: Color::BLACK,
            color_space: ColorSpace::Rgb,
        },
        StyleEffect::RigidShakeStyle {
            shake_period: 0.29,
            num_shakes: 4,
            pause_duration: 0.52,
        },
        StyleEffect::Spatial {
            shader: SpatialShaderType::LinearGradient(
                tui_vfx_style::models::LinearGradientShader::new(
                    tui_vfx_style::models::Gradient::new(vec![
                        (0.0, Color::BLACK),
                        (1.0, Color::WHITE),
                    ]),
                ),
            ),
        },
    ];

    variants
        .into_iter()
        .map(|v| {
            let name = v.effect_type_name().to_string();
            let meta = EffectMetadata {
                name: name.clone(),
                description: v.terse_description().to_string(),
                parameters: v
                    .key_parameters()
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect(),
            };
            (name, meta)
        })
        .collect()
}

fn extract_content_metadata() -> HashMap<String, EffectMetadata> {
    use tui_vfx_content::types::{
        cls_dissolve_config::{DissolveDirection, DissolvePattern, DissolveReplacement},
        cls_mirror_axis::MirrorAxis,
        cls_morph_config::{MorphDirection, MorphProgression},
        cls_scramble_charset::ScrambleCharset,
        cls_slide_shift_flow_mode::SlideShiftFlowMode,
        cls_slide_shift_line_mode::SlideShiftLineMode,
    };

    let variants: Vec<ContentEffect> = vec![
        ContentEffect::Typewriter {
            speed_variance: Default::default(),
            cursor: None,
        },
        ContentEffect::Scramble {
            resolve_pace: Default::default(),
            charset: ScrambleCharset::Alphanumeric,
            seed: 42,
        },
        ContentEffect::GlitchShift {
            shift_amount: 4,
            glitch_start: Default::default(),
            glitch_end: Default::default(),
            seed: 42,
        },
        ContentEffect::ScrambleGlitchShift {
            resolve_pace: Default::default(),
            charset: ScrambleCharset::Alphanumeric,
            scramble_seed: 42,
            shift_amount: 4,
            glitch_start: Default::default(),
            glitch_end: Default::default(),
        },
        ContentEffect::SplitFlap {
            speed: Default::default(),
            cascade: Default::default(),
        },
        ContentEffect::Odometer,
        ContentEffect::Redact { symbol: '█' },
        ContentEffect::Numeric {
            format: "{}".to_string(),
        },
        ContentEffect::Marquee {
            speed: Default::default(),
            width: 20,
        },
        ContentEffect::SlideShift {
            start_col: 0,
            end_col: 0,
            start_row: 0,
            shift_col: 0,
            shift_width: 1,
            row_shift: 0,
            line_mode: SlideShiftLineMode::Block,
            flow_mode: SlideShiftFlowMode::StayShifted,
        },
        ContentEffect::Mirror {
            axis: MirrorAxis::Horizontal,
        },
        ContentEffect::Dissolve {
            replacement: DissolveReplacement::Space,
            pattern: DissolvePattern::Random,
            direction: DissolveDirection::LeftToRight,
            seed: 0,
        },
        ContentEffect::Morph {
            source: String::new(),
            progression: MorphProgression::Linear,
            direction: MorphDirection::LeftToRight,
            seed: 0,
        },
        ContentEffect::WrapIndicator {
            prefix: "» ".to_string(),
            suffix: " «".to_string(),
        },
    ];

    variants
        .into_iter()
        .map(|v| {
            let name = v.name().to_string();
            let meta = EffectMetadata {
                name: name.clone(),
                description: v.terse_description().to_string(),
                parameters: v
                    .key_parameters()
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect(),
            };
            (name, meta)
        })
        .collect()
}

fn extract_shadow_metadata() -> HashMap<String, EffectMetadata> {
    let variants: Vec<ShadowStyle> = vec![
        ShadowStyle::HalfBlock,
        ShadowStyle::Braille { density: 0.5 },
        ShadowStyle::MediumShade,
        ShadowStyle::Solid,
        ShadowStyle::Gradient { layers: 2 },
    ];

    variants
        .into_iter()
        .map(|v| {
            let name = v.name().to_string();
            let meta = EffectMetadata {
                name: name.clone(),
                description: v.terse_description().to_string(),
                parameters: v
                    .key_parameters()
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect(),
            };
            (name, meta)
        })
        .collect()
}

// <FILE>xtask/src/docs/effect_metadata.rs</FILE> - <DESC>Effect metadata extraction from runtime introspection</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
