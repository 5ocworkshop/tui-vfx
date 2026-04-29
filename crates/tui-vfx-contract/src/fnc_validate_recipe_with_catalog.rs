// <FILE>crates/tui-vfx-contract/src/fnc_validate_recipe_with_catalog.rs</FILE> - <DESC>Validate recipes after resolving descriptor packs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase J2: merge pack-provided descriptors before canonical recipe validation.</WCTX>
// <CLOG>0.1.0: INIT — add strict descriptor-pack resolution with collision diagnostics.</CLOG>

use std::collections::BTreeSet;

use crate::{
    DescriptorCatalog, DescriptorValidationError, EffectId, RecipeDocument, SourceId,
    orc_validate_recipe_document::validate_recipe_document,
};

/// Validate a recipe using externally loaded descriptor packs named by the recipe.
pub fn validate_recipe_with_catalog(
    recipe: &RecipeDocument,
    catalog: &DescriptorCatalog,
) -> Result<(), DescriptorValidationError> {
    catalog.validate()?;
    if recipe.descriptor_packs.is_empty() {
        return validate_recipe_document(recipe);
    }
    let resolved = resolved_recipe(recipe, catalog)?;
    validate_recipe_document(&resolved)
}

fn resolved_recipe(
    recipe: &RecipeDocument,
    catalog: &DescriptorCatalog,
) -> Result<RecipeDocument, DescriptorValidationError> {
    let mut resolved = recipe.clone();
    let mut pack_sources = BTreeSet::new();
    let mut pack_effects = BTreeSet::new();
    for pack_ref in &recipe.descriptor_packs {
        let pack = catalog.pack(&pack_ref.id).ok_or_else(|| {
            DescriptorValidationError::UnknownDescriptorPack {
                id: pack_ref.id.clone(),
            }
        })?;
        pack.validate()?;
        merge_pack_sources(recipe, &mut resolved, &mut pack_sources, pack)?;
        merge_pack_effects(recipe, &mut resolved, &mut pack_effects, pack)?;
    }
    Ok(resolved)
}

fn merge_pack_sources(
    recipe: &RecipeDocument,
    resolved: &mut RecipeDocument,
    pack_sources: &mut BTreeSet<SourceId>,
    pack: &crate::DescriptorPack,
) -> Result<(), DescriptorValidationError> {
    for (id, descriptor) in &pack.source_descriptors {
        if recipe.source_descriptors.contains_key(id) {
            return Err(
                DescriptorValidationError::EmbeddedSourceDescriptorCollision { id: id.clone() },
            );
        }
        if !pack_sources.insert(id.clone()) {
            return Err(DescriptorValidationError::DuplicatePackSourceDescriptor {
                id: id.clone(),
            });
        }
        resolved
            .source_descriptors
            .insert(id.clone(), descriptor.clone());
    }
    Ok(())
}

fn merge_pack_effects(
    recipe: &RecipeDocument,
    resolved: &mut RecipeDocument,
    pack_effects: &mut BTreeSet<EffectId>,
    pack: &crate::DescriptorPack,
) -> Result<(), DescriptorValidationError> {
    for (id, descriptor) in &pack.effects {
        if recipe.graph.effects.contains_key(id) {
            return Err(
                DescriptorValidationError::EmbeddedEffectDescriptorCollision { id: id.clone() },
            );
        }
        if !pack_effects.insert(id.clone()) {
            return Err(DescriptorValidationError::DuplicatePackEffectDescriptor {
                id: id.clone(),
            });
        }
        resolved
            .graph
            .effects
            .insert(id.clone(), descriptor.clone());
    }
    Ok(())
}

// <FILE>crates/tui-vfx-contract/src/fnc_validate_recipe_with_catalog.rs</FILE> - <DESC>Validate recipes after resolving descriptor packs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
