use crate::part_1::v3_1::reference::Reference;
use crate::part_1::v3_1::submodel_elements::SubmodelElementFields;
use crate::part_1::v3_1::submodel_elements::data_element::DataElement;
use crate::part_1::{MetamodelError, ToJsonMetamodel};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(tag = "modelType")]
pub struct RelationshipElement {
    #[serde(flatten)]
    pub submodel_element_fields: SubmodelElementFields,

    #[serde(skip_serializing_if = "Option::is_none")]
    first: Option<Reference>,

    #[serde(skip_serializing_if = "Option::is_none")]
    second: Option<Reference>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(tag = "modelType")]
pub struct AnnotatedRelationshipElement {
    // Inherited from RelationshipElement
    #[serde(flatten)]
    pub submodel_element_fields: SubmodelElementFields,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<Reference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<Reference>,
    // ----
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<DataElement>>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(tag = "modelType", rename = "RelationshipElement")]
pub struct RelationshipElementMeta {
    #[serde(flatten)]
    pub submodel_element_fields: SubmodelElementFields,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(tag = "modelType", rename = "AnnotatedRelationshipElement")]
pub struct AnnotatedRelationshipElementMeta {
    #[serde(flatten)]
    pub submodel_element_fields: SubmodelElementFields,
}

impl From<RelationshipElement> for RelationshipElementMeta {
    fn from(element: RelationshipElement) -> Self {
        Self {
            submodel_element_fields: element.submodel_element_fields,
        }
    }
}

impl From<&RelationshipElement> for RelationshipElementMeta {
    fn from(element: &RelationshipElement) -> Self {
        Self {
            submodel_element_fields: element.submodel_element_fields.clone(),
        }
    }
}

impl From<AnnotatedRelationshipElement> for AnnotatedRelationshipElementMeta {
    fn from(element: AnnotatedRelationshipElement) -> Self {
        Self {
            submodel_element_fields: element.submodel_element_fields,
        }
    }
}

impl From<&AnnotatedRelationshipElement> for AnnotatedRelationshipElementMeta {
    fn from(element: &AnnotatedRelationshipElement) -> Self {
        Self {
            submodel_element_fields: element.submodel_element_fields.clone(),
        }
    }
}

impl ToJsonMetamodel for RelationshipElement {
    type Error = MetamodelError;

    fn to_json_metamodel(&self) -> Result<String, Self::Error> {
        serde_json::to_string::<RelationshipElementMeta>(&self.into())
            .map_err(|e| MetamodelError::FailedSerialisation(e))
    }
}

impl ToJsonMetamodel for AnnotatedRelationshipElement {
    type Error = MetamodelError;

    fn to_json_metamodel(&self) -> Result<String, Self::Error> {
        serde_json::to_string::<AnnotatedRelationshipElementMeta>(&self.into())
            .map_err(|e| MetamodelError::FailedSerialisation(e))
    }
}
