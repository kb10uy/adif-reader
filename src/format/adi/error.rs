use std::num::ParseIntError;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError, PartialEq, Eq)]
pub enum AdiError {
    #[error("no data found")]
    NoData,

    #[error("no <eoh> found")]
    NoEoh,

    #[error("no <eor> found after index {0}")]
    NoEor(usize),

    #[error("tag error at index {0}: {1}")]
    Tag(usize, TagError),

    #[error("invalid character boundary found: {0}")]
    CharacterBoundary(usize),

    #[error("field value too short; expected {expected}, available {available}")]
    ValueTooShort { expected: usize, available: usize },

    #[error("invalid user-defined field definition: {0}")]
    InvalidUserDefinedField(String),
}

impl AdiError {
    pub(super) fn offset_by(self, offset: usize) -> AdiError {
        match self {
            AdiError::NoEor(p) => AdiError::NoEor(p + offset),
            AdiError::Tag(p, e) => AdiError::Tag(p + offset, e),
            AdiError::CharacterBoundary(p) => AdiError::CharacterBoundary(p + offset),
            e => e,
        }
    }
}

#[derive(Debug, ThisError, PartialEq, Eq)]
pub enum TagError {
    #[error("no valid tag found")]
    NotValidTag,

    #[error("invalid length: {0}")]
    ParseInt(#[from] ParseIntError),

    #[error("unknown data type indicator: {0}")]
    UnknownDataType(String),
}
