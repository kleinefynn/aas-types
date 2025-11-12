use crate::utilities::{deserialize_label_type, validate_text};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use thiserror::Error;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize, Serialize)]
pub struct Label(#[serde(deserialize_with = "deserialize_label_type")] String);

#[derive(Error, Debug)]
pub enum LabelError {
    #[error("The label type needs at lest 1 character")]
    TooShort,

    #[error("The label type can hold at most 64 characters")]
    TooLong,

    #[error("Invalid character found")]
    InvalidCharacter,
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Label {
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for Label {
    type Error = LabelError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(LabelError::TooShort);
        }

        if value.chars().count() > 64 {
            return Err(LabelError::TooLong);
        }

        if !validate_text(&value) {
            return Err(LabelError::InvalidCharacter);
        }

        Ok(Label(value))
    }
}

impl TryFrom<&str> for Label {
    type Error = LabelError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl AsRef<str> for Label {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Label {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
