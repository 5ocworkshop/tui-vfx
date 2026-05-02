// <FILE>crates/tui-vfx-compost/src/render/fnc_apply_effect_stack.rs</FILE> - <DESC>Apply supported native effect stages to a source grid</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Effect application delegates per-cell style mutation to native graph execution, then writes through the role-aware element-surface merge seam.</WCTX>
// <CLOG>0.4.0: MINOR — execute graph topology before final cell writes.
// 0.3.0: MINOR — skip inactive lifecycle stages and copy sampled source roles during merge.
// 0.2.1: PATCH — remove redundant timing/effect-id reads from shader application.
// 0.2.0: MINOR — route destination mutation through cell and role write policies.
// 0.1.1: PATCH — separate supported shader dispatch from cell application.
// 0.1.0: INIT — apply ordered supported shader stages through the effect stack.</CLOG>

use tui_vfx_types::{Grid, RoleTag, SemanticScene, Style};

use crate::render::{
    EffectStack, ElementClipBounds, RenderError, SampleContext, execute_effect_graph,
    is_node_active, merge_element_surface,
};
use crate::runtime::RuntimeContext;

pub(crate) fn apply_effect_stack(
    source: &SemanticScene,
    destination: &mut SemanticScene,
    bounds: ElementClipBounds,
    sample: &SampleContext,
    stack: &EffectStack<'_>,
    runtime_context: &RuntimeContext,
) -> Result<(), RenderError> {
    if let Some(stage) = stack
        .content_stages()
        .chain(stack.style_stages())
        .chain(stack.sampler_stages())
        .chain(stack.mask_stages())
        .chain(stack.filter_stages())
        .find(|stage| is_node_active(stage.node(), sample))
    {
        return Err(unsupported_stage(stage));
    }

    let source_grid = source.grid();
    let base_context = runtime_context
        .clone()
        .with_dimensions(source_grid.width() as u16, source_grid.height() as u16);
    for visible_y in 0..bounds.height {
        for visible_x in 0..bounds.width {
            let local_x = bounds.local_x_start + visible_x;
            let local_y = bounds.local_y_start + visible_y;
            let dest_x = bounds.dest_x_start + visible_x;
            let dest_y = bounds.dest_y_start + visible_y;
            let Some(source_cell) = source_grid.get(local_x, local_y).copied() else {
                continue;
            };
            let sampled_role = source
                .role((local_x as u16, local_y as u16))
                .unwrap_or(RoleTag::Background);
            let cell_context = base_context
                .clone()
                .with_cell(local_x as u16, local_y as u16);
            let style = execute_effect_graph(
                stack,
                sample,
                Style::new(source_cell.fg, source_cell.bg, source_cell.mods),
                &cell_context,
                local_x as u16,
                local_y as u16,
                source_grid.width() as u16,
                source_grid.height() as u16,
                dest_x as u16,
                dest_y as u16,
            )?
            .style;
            let final_cell = source_cell
                .with_fg(style.fg)
                .with_bg(style.bg)
                .with_mods(style.mods);
            merge_element_surface(
                destination,
                dest_x,
                dest_y,
                sampled_role.clone(),
                final_cell,
                stack.cell_write_policy(),
                stack.role_write_policy(),
            );
        }
    }

    Ok(())
}

fn unsupported_stage(stage: crate::render::EffectStage<'_>) -> RenderError {
    RenderError::Unsupported(format!(
        "effect stage `{}` uses unsupported {} family effect `{}`",
        stage.node_id().as_str(),
        stage.family().as_str(),
        stage.node().effect.as_str()
    ))
}

// <FILE>crates/tui-vfx-compost/src/render/fnc_apply_effect_stack.rs</FILE> - <DESC>Apply supported native effect stages to a source grid</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
