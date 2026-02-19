// <FILE>tui-vfx-debug/src/config.rs</FILE> - <DESC>Module registry configuration for centralized debug logger</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>WG3: Debug Logger Integration</WCTX>
// <CLOG>Initial creation with comprehensive module registry for mixed-signals and tui-compositor</CLOG>

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

#[derive(Debug, Clone)]
pub struct ModuleConfig {
    pub level: LogLevel,
    pub description: String,
}

pub fn module_registry() -> HashMap<String, ModuleConfig> {
    let mut registry = HashMap::new();

    // ===== mixed-signals modules =====

    // Oscillators
    registry.insert(
        "mixed_signals::oscillators::sine".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "Sine wave oscillator for smooth periodic values".to_string(),
        },
    );
    registry.insert(
        "mixed_signals::oscillators::triangle".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "Triangle wave oscillator for linear periodic movement".to_string(),
        },
    );
    registry.insert(
        "mixed_signals::oscillators::square".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "Square wave oscillator for on/off toggling".to_string(),
        },
    );
    registry.insert(
        "mixed_signals::oscillators::sawtooth".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "Sawtooth wave oscillator for ramping values".to_string(),
        },
    );

    // Transitions
    registry.insert(
        "mixed_signals::transitions::interpolate_position".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "Position interpolation with easing for smooth movement".to_string(),
        },
    );

    // ===== tui-compositor modules =====

    // Pipeline
    registry.insert(
        "tui_vfx_compositor::pipeline::render_pipeline".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "Core rendering pipeline with mask/sampler/filter application".to_string(),
        },
    );

    // Masks
    registry.insert(
        "tui_vfx_compositor::masks::rect".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "Rectangular mask for clipping regions".to_string(),
        },
    );
    registry.insert(
        "tui_vfx_compositor::masks::linear".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "Linear gradient mask for fades".to_string(),
        },
    );
    registry.insert(
        "tui_vfx_compositor::masks::circular".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "Circular/radial mask for spotlight effects".to_string(),
        },
    );

    // Samplers
    registry.insert(
        "tui_vfx_compositor::samplers::ripple".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "Ripple distortion sampler for wave effects".to_string(),
        },
    );
    registry.insert(
        "tui_vfx_compositor::samplers::crt_jitter".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "CRT jitter sampler for glitch effects".to_string(),
        },
    );

    // Filters
    registry.insert(
        "tui_vfx_compositor::filters::tint".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "Color tint filter with spatial targeting".to_string(),
        },
    );
    registry.insert(
        "tui_vfx_compositor::filters::brighten".to_string(),
        ModuleConfig {
            level: LogLevel::Off,
            description: "Brightness adjustment filter".to_string(),
        },
    );

    registry
}

// <FILE>tui-vfx-debug/src/config.rs</FILE> - <DESC>Module registry configuration for centralized debug logger</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
