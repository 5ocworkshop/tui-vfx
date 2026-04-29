// <FILE>crates/tui-vfx-contract/src/orc_validate_source_spec.rs</FILE> - <DESC>Validate source specs against descriptors and assets</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>K2.13 schema decision burn-down: support optional source inputs and sampled-field external source validation.</WCTX>
// <CLOG>0.3.0: MINOR — honor optional source inputs and classify sampledField as external.
// 0.2.0: MINOR — allow graph-local value sources when validating source specs inside graph context.</CLOG>

use std::collections::BTreeMap;

use crate::{
    AssetId, AssetSpec, DescriptorValidationError, GraphValueKinds, ParameterId, ParameterSpec,
    SignalId, SignalSpec, SourceDescriptor, SourceId, SourceSpec, ValueSource,
};

pub(crate) fn validate_source_spec(
    spec: &SourceSpec,
    sources: &BTreeMap<SourceId, SourceDescriptor>,
    assets: &BTreeMap<AssetId, AssetSpec>,
    parameters: &BTreeMap<ParameterId, ParameterSpec>,
    signals: &BTreeMap<SignalId, SignalSpec>,
    graph_values: Option<&GraphValueKinds>,
) -> Result<(), DescriptorValidationError> {
    let descriptor =
        sources
            .get(&spec.source)
            .ok_or_else(|| DescriptorValidationError::UnknownSource {
                id: spec.source.clone(),
            })?;
    descriptor.validate_contract()?;
    validate_inputs(spec, descriptor, parameters, signals, graph_values)?;
    validate_assets(spec, descriptor, assets)?;
    Ok(())
}

fn validate_inputs(
    spec: &SourceSpec,
    descriptor: &SourceDescriptor,
    parameters: &BTreeMap<ParameterId, ParameterSpec>,
    signals: &BTreeMap<SignalId, SignalSpec>,
    graph_values: Option<&GraphValueKinds>,
) -> Result<(), DescriptorValidationError> {
    for (id, source) in &spec.inputs {
        if !id.is_valid() {
            return Err(DescriptorValidationError::InvalidSourceInputId { id: id.clone() });
        }
        let input = descriptor.inputs.get(id).ok_or_else(|| {
            DescriptorValidationError::UnknownSourceInput {
                source: spec.source.clone(),
                input: id.clone(),
            }
        })?;
        if !input.bindable && uses_external_source(source) {
            return Err(DescriptorValidationError::SourceInputNotBindable {
                source: spec.source.clone(),
                input: id.clone(),
            });
        }
        source.validate_kind_with_graph_values(
            input.value.kind,
            parameters,
            signals,
            graph_values,
        )?;
    }

    for (id, input) in &descriptor.inputs {
        if !spec.inputs.contains_key(id) && input.value.default.is_none() && !input.optional {
            return Err(DescriptorValidationError::MissingRequiredSourceInput {
                source: spec.source.clone(),
                input: id.clone(),
            });
        }
    }
    Ok(())
}

fn validate_assets(
    spec: &SourceSpec,
    descriptor: &SourceDescriptor,
    assets: &BTreeMap<AssetId, AssetSpec>,
) -> Result<(), DescriptorValidationError> {
    for (slot, asset_ref) in &spec.assets {
        if !slot.is_valid() {
            return Err(DescriptorValidationError::InvalidAssetId { id: slot.clone() });
        }
        if !asset_ref.id.is_valid() {
            return Err(DescriptorValidationError::InvalidAssetId {
                id: asset_ref.id.clone(),
            });
        }
        let requirement = descriptor.assets.get(slot).ok_or_else(|| {
            DescriptorValidationError::UnknownSourceAssetSlot {
                source: spec.source.clone(),
                asset: slot.clone(),
            }
        })?;
        let asset = assets.get(&asset_ref.id).ok_or_else(|| {
            DescriptorValidationError::UnknownAssetRef {
                id: asset_ref.id.clone(),
            }
        })?;
        asset.validate()?;
        if asset.kind != requirement.kind {
            return Err(DescriptorValidationError::AssetKindMismatch {
                asset: asset_ref.id.clone(),
                expected: requirement.kind.clone(),
                actual: asset.kind.clone(),
            });
        }
        if asset.format != requirement.format {
            return Err(DescriptorValidationError::AssetFormatMismatch {
                asset: asset_ref.id.clone(),
                expected: requirement.format.clone(),
                actual: asset.format.clone(),
            });
        }
    }

    for (slot, requirement) in &descriptor.assets {
        if requirement.required && !spec.assets.contains_key(slot) {
            return Err(DescriptorValidationError::MissingRequiredAsset {
                source: spec.source.clone(),
                asset: slot.clone(),
            });
        }
    }
    Ok(())
}

fn uses_external_source(source: &ValueSource) -> bool {
    match source {
        ValueSource::Literal { .. } => false,
        ValueSource::Parameter { .. }
        | ValueSource::Signal { .. }
        | ValueSource::GraphValue { .. }
        | ValueSource::SampledField { .. } => true,
        ValueSource::Map { from, .. } => uses_external_source(from),
    }
}

// <FILE>crates/tui-vfx-contract/src/orc_validate_source_spec.rs</FILE> - <DESC>Validate source specs against descriptors and assets</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
