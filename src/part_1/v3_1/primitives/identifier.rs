use crate::utilities::{deserialize_identifier, validate_text};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use thiserror::Error;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize, Serialize)]
pub struct Identifier(#[serde(deserialize_with = "deserialize_identifier")] String);

#[derive(Error, Debug)]
pub enum IdentifierError {
    #[error("The label type needs at lest 1 character")]
    TooShort,

    #[error("The label type can hold at most 2048 characters")]
    TooLong,

    #[error("Invalid character found")]
    InvalidCharacter,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Identifier {
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for Identifier {
    type Error = IdentifierError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(IdentifierError::TooShort);
        }

        if value.len() > 64 {
            return Err(IdentifierError::TooLong);
        }

        if !validate_text(&value) {
            return Err(IdentifierError::InvalidCharacter);
        }

        Ok(Identifier(value.to_owned()))
    }
}

impl TryFrom<&str> for Identifier {
    type Error = IdentifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(IdentifierError::TooShort);
        }

        if value.len() > 2048 {
            return Err(IdentifierError::TooLong);
        }

        if !validate_text(value) {
            return Err(IdentifierError::InvalidCharacter);
        }

        Ok(Identifier(value.to_string()))
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Identifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
