use crate::part_1::v3_1::submodel_elements::SubmodelElementFields;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, Default)]
#[serde(tag = "modelType")]
pub struct Capability {
    #[serde(flatten)]
    submodel_element_fields: SubmodelElementFields,
}

impl Capability {
    pub fn new() -> Self {
        Self {
            submodel_element_fields: SubmodelElementFields::default(),
        }
    }
}

// TODO: Test serialization and deserialization
#[cfg(test)]
mod tests {}
