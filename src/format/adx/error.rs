use std::num::ParseIntError;

use thiserror::Error as ThisError;

#[derive(Debug, ThisError, PartialEq, Eq)]
pub enum AdxError {
    #[error("no <ADX> element found")]
    NoAdx,

    #[error("no <HEADER> element found")]
    NoHeader,

    #[error("no <RECORDS> element found")]
    NoRecords,

    #[error("<{element}> requires {attribute} attribute")]
    MissingAttribute {
        element: &'static str,
        attribute: &'static str,
    },

    #[error("invalid length: {0}")]
    ParseInt(#[from] ParseIntError),
}
