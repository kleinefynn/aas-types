use crate::utilities::{deserialize_identifier, validate_text};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use thiserror::Error;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize, Serialize)]
pub struct Identifier(#[serde(deserialize_with = "deserialize_identifier")] String);

#[derive(Error, Debug, PartialEq)]
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

        if value.chars().count() > 2048 {
            return Err(IdentifierError::TooLong);
        }

        if !validate_text(&value) {
            return Err(IdentifierError::InvalidCharacter);
        }

        Ok(Identifier(value))
    }
}

impl TryFrom<&str> for Identifier {
    type Error = IdentifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
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


#[cfg(test)]
mod tests {
    use crate::part_1::v3_1::primitives::{Identifier, IdentifierError};

    #[test]
    fn test_try_from_max_length() {
        let test_value = std::iter::repeat('0').take(2048).collect::<String>();
        let id = Identifier::try_from(test_value.to_string());

        assert!(id.is_ok());
        let id = id.unwrap();
        assert_eq!(id.as_ref(), test_value);
    }

    #[test]
    fn test_try_from_happy() {
        for value in ["https://cust/123456", "0173-1#02-BAA120#008"] {
            let id = Identifier::try_from(value.to_string());

            assert!(id.is_ok());
            let id = id.unwrap();
            assert_eq!(id.as_ref(), value);
        }
    }

    #[test]
    fn test_try_from_too_short() {
        let id = Identifier::try_from("".to_string());

        assert_eq!(id, Err(IdentifierError::TooShort));
    }

    #[test]
    fn test_try_from_too_long() {
        let test_value = std::iter::repeat("t").take(2049).collect::<String>();
        let id = Identifier::try_from(test_value);

        assert_eq!(id, Err(IdentifierError::TooLong));
    }

    #[test]
    fn test_try_from_invalid() {
        let test_value = std::iter::repeat('\0').take(2048).collect::<String>();
        let id = Identifier::try_from(test_value);

        assert_eq!(id, Err(IdentifierError::InvalidCharacter));
    }
}
