// <FILE>crates/tui-vfx-probe/src/cls_probe_inspector.rs</FILE> - <DESC>Internal compositor inspector for probe runs</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Phase-1.5 probe causation support</WCTX>
// <CLOG>MINOR: Upgrade the inspector from last-touch-only tracking to full per-cell trace collection with optional before/after snapshots</CLOG>

use std::collections::HashMap;

use tui_vfx_compositor::traits::pipeline_inspector::CompositorInspector;
use tui_vfx_types::{Cell, Style};

use crate::cls_probe_last_touch::ProbeLastTouch;
use crate::cls_probe_report::ProbePoint;
use crate::cls_probe_state_snapshot::ProbeStateSnapshot;
use crate::cls_probe_trace_event::ProbeTraceEvent;

#[derive(Debug, Default)]
pub struct ProbeInspector {
    events_by_cell: HashMap<(u16, u16), Vec<ProbeTraceEvent>>,
}

impl ProbeInspector {
    pub fn last_touch_for(&self, x: u16, y: u16) -> Option<ProbeLastTouch> {
        self.events_by_cell
            .get(&(x, y))
            .and_then(|events| events.last())
            .map(|event| ProbeLastTouch {
                stage: event.stage.clone(),
                effect: event.effect.clone(),
            })
    }

    pub fn trace_for(&self, x: u16, y: u16) -> Vec<ProbeTraceEvent> {
        self.events_by_cell
            .get(&(x, y))
            .cloned()
            .unwrap_or_default()
    }

    fn remember(&mut self, x: u16, y: u16, event: ProbeTraceEvent) {
        self.events_by_cell.entry((x, y)).or_default().push(event);
    }
}

impl CompositorInspector for ProbeInspector {
    fn on_sampler_applied(
        &mut self,
        dest_x: u16,
        dest_y: u16,
        src_x: Option<u16>,
        src_y: Option<u16>,
        sampler_name: &str,
    ) {
        if sampler_name != "None" {
            self.remember(
                dest_x,
                dest_y,
                ProbeTraceEvent {
                    stage: "sampler".to_string(),
                    effect: Some(sampler_name.to_string()),
                    sampled_from: src_x.zip(src_y).map(|(x, y)| ProbePoint { x, y }),
                    visible: None,
                    before: None,
                    after: None,
                },
            );
        }
    }

    fn on_mask_checked(&mut self, x: u16, y: u16, visible: bool, mask_name: &str) {
        self.remember(
            x,
            y,
            ProbeTraceEvent {
                stage: "mask".to_string(),
                effect: Some(mask_name.to_string()),
                sampled_from: None,
                visible: Some(visible),
                before: None,
                after: None,
            },
        );
    }

    fn on_shader_applied(
        &mut self,
        x: u16,
        y: u16,
        before: Style,
        after: Style,
        shader_name: &str,
    ) {
        self.remember(
            x,
            y,
            ProbeTraceEvent {
                stage: "shader".to_string(),
                effect: Some(shader_name.to_string()),
                sampled_from: None,
                visible: None,
                before: Some(ProbeStateSnapshot::from_style(before)),
                after: Some(ProbeStateSnapshot::from_style(after)),
            },
        );
    }

    fn on_filter_applied(
        &mut self,
        x: u16,
        y: u16,
        before: &Cell,
        after: &Cell,
        filter_name: &str,
    ) {
        self.remember(
            x,
            y,
            ProbeTraceEvent {
                stage: "filter".to_string(),
                effect: Some(filter_name.to_string()),
                sampled_from: None,
                visible: None,
                before: Some(ProbeStateSnapshot::from_cell(before)),
                after: Some(ProbeStateSnapshot::from_cell(after)),
            },
        );
    }
}

// <FILE>crates/tui-vfx-probe/src/cls_probe_inspector.rs</FILE> - <DESC>Internal compositor inspector for probe runs</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
