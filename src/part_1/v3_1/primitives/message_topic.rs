use crate::utilities::{deserialize_message_topic_type, validate_text};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use thiserror::Error;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize, Serialize)]
pub struct MessageTopic(#[serde(deserialize_with = "deserialize_message_topic_type")] String);

#[derive(Error, Debug)]
pub enum MessageTopicError {
    #[error("The label type needs at lest 1 character")]
    TooShort,

    #[error("The label type can hold at most 255 characters")]
    TooLong,

    #[error("Invalid character found")]
    InvalidCharacter,
}

impl Display for MessageTopic {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl MessageTopic {
    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for MessageTopic {
    type Error = MessageTopicError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(MessageTopicError::TooShort);
        }

        if value.chars().count()  > 64 {
            return Err(MessageTopicError::TooLong);
        }

        if !validate_text(&value) {
            return Err(MessageTopicError::InvalidCharacter);
        }

        Ok(MessageTopic(value))
    }
}

impl TryFrom<&str> for MessageTopic {
    type Error = MessageTopicError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_string())
    }
}

impl AsRef<str> for MessageTopic {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for MessageTopic {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
