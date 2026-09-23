pub mod document;
pub mod error;
mod format;

use roxmltree::Document;

use crate::{
    document::{AdifDocument, IntoAdifDocument},
    error::AdifError,
    format::{adi::AdiDocument, adx::AdxDocument},
};

pub use format::adi::LengthMode;

pub fn read_adi(adi_text: &str, length_mode: LengthMode) -> Result<AdifDocument, AdifError> {
    let adi = AdiDocument::parse(adi_text, length_mode)?;
    Ok(adi.into_adif_document())
}

pub fn read_adx(adx_text: &str) -> Result<AdifDocument, AdifError> {
    let xml = Document::parse(adx_text)?;
    let adx = AdxDocument::parse(&xml)?;
    Ok(adx.into_adif_document())
}

#[cfg(test)]
mod tests {
    use crate::{LengthMode, document::DataType, read_adi, read_adx};

    #[test]
    fn keeps_adi_data_types() {
        let adi = read_adi(
            "h<USERDEF1:3:N>EPC<EOH><CALL:6>JL1HIS<APP_X_Y:1:b>Y<EOR>",
            LengthMode::Bytes,
        )
        .expect("must parse");
        assert_eq!(adi.header_type("USERDEF1"), Some(DataType::Number));
        assert_eq!(adi.records()[0].field_type("call"), None);
        assert_eq!(
            adi.records()[0].field_type("APP_X_Y"),
            Some(DataType::Boolean)
        );
        assert_eq!(adi.records()[0].field("APP_X_Y"), Some("Y"));
    }

    #[test]
    fn keeps_adx_data_types() {
        let adx = read_adx(include_str!("../fixtures/example.adx")).expect("must parse");
        assert_eq!(adx.header_type("USERDEF2"), Some(DataType::Enumeration));
        assert_eq!(
            adx.records()[0].field_type("APP_MONOLOG_COMPRESSION"),
            Some(DataType::String)
        );
        assert_eq!(adx.records()[0].field_type("CALL"), None);
    }
}
