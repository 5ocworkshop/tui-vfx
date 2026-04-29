// <FILE>crates/tui-vfx-contract/src/orc_validate_recipe_document.rs</FILE> - <DESC>Validate canonical recipe document contracts</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase H1: validate cross-document recipe references.</WCTX>
// <CLOG>0.1.0: INIT — validate recipe assets, source instances, graph, scenes, and element pipelines.</CLOG>

use std::collections::BTreeSet;

use crate::{
    DescriptorValidationError, GraphStep, NodeId, RecipeDocument, RecipeElementPipeline,
    RecipeScene, RecipeSceneElement,
};

pub(crate) fn validate_recipe_document(
    recipe: &RecipeDocument,
) -> Result<(), DescriptorValidationError> {
    if !recipe.id.is_valid() {
        return Err(DescriptorValidationError::InvalidRecipeId {
            id: recipe.id.clone(),
        });
    }

    recipe.graph.validate()?;
    validate_assets(recipe)?;
    validate_source_descriptors(recipe)?;
    validate_sources(recipe)?;
    validate_scenes(recipe)?;
    Ok(())
}

fn validate_assets(recipe: &RecipeDocument) -> Result<(), DescriptorValidationError> {
    for (id, asset) in &recipe.assets {
        if !id.is_valid() {
            return Err(DescriptorValidationError::InvalidAssetId { id: id.clone() });
        }
        if &asset.id != id {
            return Err(DescriptorValidationError::AssetIdMismatch {
                key: id.clone(),
                asset: asset.id.clone(),
            });
        }
        asset.validate()?;
    }
    Ok(())
}

fn validate_source_descriptors(recipe: &RecipeDocument) -> Result<(), DescriptorValidationError> {
    for (id, descriptor) in &recipe.source_descriptors {
        if !id.is_valid() {
            return Err(DescriptorValidationError::InvalidSourceId { id: id.clone() });
        }
        if &descriptor.id != id {
            return Err(DescriptorValidationError::SourceDescriptorIdMismatch {
                key: id.clone(),
                source: descriptor.id.clone(),
            });
        }
        descriptor.validate_contract()?;
    }
    Ok(())
}

fn validate_sources(recipe: &RecipeDocument) -> Result<(), DescriptorValidationError> {
    for (id, source) in &recipe.sources {
        if !id.is_valid() {
            return Err(DescriptorValidationError::InvalidSourceInstanceId { id: id.clone() });
        }
        source.validate(
            &recipe.source_descriptors,
            &recipe.assets,
            &recipe.graph.parameters,
            &recipe.graph.signals,
            None,
        )?;
    }
    Ok(())
}

fn validate_scenes(recipe: &RecipeDocument) -> Result<(), DescriptorValidationError> {
    for scene in &recipe.scenes {
        validate_scene(recipe, scene)?;
    }
    Ok(())
}

fn validate_scene(
    recipe: &RecipeDocument,
    scene: &RecipeScene,
) -> Result<(), DescriptorValidationError> {
    if !scene.id.is_valid() {
        return Err(DescriptorValidationError::InvalidSceneId {
            id: scene.id.clone(),
        });
    }
    for element in &scene.elements {
        validate_scene_element(recipe, scene, element)?;
    }
    Ok(())
}

fn validate_scene_element(
    recipe: &RecipeDocument,
    scene: &RecipeScene,
    element: &RecipeSceneElement,
) -> Result<(), DescriptorValidationError> {
    if !recipe.sources.contains_key(&element.source) {
        return Err(DescriptorValidationError::UnknownSceneElementSource {
            scene: scene.id.clone(),
            element: element.id.clone(),
            source: element.source.clone(),
        });
    }
    if let Some(pipeline) = &element.pipeline {
        validate_element_pipeline(recipe, scene, element, pipeline)?;
    }
    Ok(())
}

fn validate_element_pipeline(
    recipe: &RecipeDocument,
    scene: &RecipeScene,
    element: &RecipeSceneElement,
    pipeline: &RecipeElementPipeline,
) -> Result<(), DescriptorValidationError> {
    if pipeline.graph != recipe.graph.id {
        return Err(DescriptorValidationError::UnknownElementPipelineGraph {
            scene: scene.id.clone(),
            element: element.id.clone(),
            graph: pipeline.graph.clone(),
        });
    }
    let Some(topology) = &pipeline.topology else {
        return Ok(());
    };
    let mut seen = BTreeSet::new();
    validate_pipeline_topology(recipe, scene, element, topology, &mut seen)
}

fn validate_pipeline_topology(
    recipe: &RecipeDocument,
    scene: &RecipeScene,
    element: &RecipeSceneElement,
    step: &GraphStep,
    seen: &mut BTreeSet<NodeId>,
) -> Result<(), DescriptorValidationError> {
    match step {
        GraphStep::Node { node } => validate_pipeline_node(recipe, scene, element, node, seen),
        GraphStep::Sequence { children } | GraphStep::Parallel { children, .. } => {
            for child in children {
                validate_pipeline_topology(recipe, scene, element, child, seen)?;
            }
            Ok(())
        }
    }
}

fn validate_pipeline_node(
    recipe: &RecipeDocument,
    scene: &RecipeScene,
    element: &RecipeSceneElement,
    node: &NodeId,
    seen: &mut BTreeSet<NodeId>,
) -> Result<(), DescriptorValidationError> {
    if !recipe.graph.nodes.contains_key(node) {
        return Err(DescriptorValidationError::UnknownElementPipelineNode {
            scene: scene.id.clone(),
            element: element.id.clone(),
            node: node.clone(),
        });
    }
    if !seen.insert(node.clone()) {
        return Err(DescriptorValidationError::DuplicateElementPipelineNode {
            scene: scene.id.clone(),
            element: element.id.clone(),
            node: node.clone(),
        });
    }
    Ok(())
}

// <FILE>crates/tui-vfx-contract/src/orc_validate_recipe_document.rs</FILE> - <DESC>Validate canonical recipe document contracts</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
