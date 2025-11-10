use crate::part_1::v3_1::core::asset_administration_shell::SpecificAssetId;
use crate::part_1::v3_1::primitives::Identifier;
use crate::part_1::v3_1::submodel_elements::SubmodelElement;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

/// The entity submodel element is designed to be used in submodels defining the relationship between the parts of the composite asset
/// it is composed of (e.g. bill of material).
/// These parts are called entities. Not all entities have a global asset ID.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Display, EnumString)]
pub enum Entity {
    /// There is no separate Asset Administration Shell for co-managed entities.
    /// Co-managed entities need to be part of a self-managed entity.
    CoManagedEntity(EntityInner),

    /// Self-managed entities have their own Asset Administration Shell but can be part of another composite self-managed entity.
    /// The asset represented by an Asset Administration Shell is a self-managed entity per definition
    SelfManagedEntity(EntityInner),
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]
pub struct EntityInner {
    /// Statement applicable to the entity,
    /// each statement described by submodel element - typically with a qualified value
    statement: Option<Vec<SubmodelElement>>,

    #[serde(rename = "globalAssetId")]
    global_asset_id: Option<Identifier>,

    #[serde(rename = "specificAssetId")]
    specific_asset_id: Option<Vec<SpecificAssetId>>,
}
