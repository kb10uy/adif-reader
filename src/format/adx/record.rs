use std::collections::HashMap;

use roxmltree::{Node, NodeType};

use crate::{
    document::DataType,
    format::adx::{error::AdxError, field_name::FieldName, parse_data_type},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record<'a> {
    pub fields: HashMap<FieldName<'a>, (String, Option<DataType>)>,
}

impl<'a, 'i: 'a> Record<'a> {
    pub fn new(record_element: Node<'a, 'i>) -> Result<Record<'a>, AdxError> {
        let fields: Result<_, AdxError> = record_element
            .children()
            .map(|c| {
                let NodeType::Element = c.node_type() else {
                    return Ok(None);
                };

                let tag_name = c.tag_name().name();
                let text: String = c
                    .children()
                    .filter(|n| n.is_text())
                    .flat_map(|n| n.text())
                    .collect();
                let value = (text, parse_data_type(c)?);
                match tag_name {
                    "USERDEF" => {
                        let name = c.attribute("FIELDNAME").ok_or(AdxError::MissingAttribute {
                            element: "USERDEF",
                            attribute: "FIELDNAME",
                        })?;
                        Ok(Some((
                            FieldName::UserdefRecord(name.to_ascii_uppercase()),
                            value,
                        )))
                    }
                    "APP" => {
                        let program_id =
                            c.attribute("PROGRAMID").ok_or(AdxError::MissingAttribute {
                                element: "APP",
                                attribute: "PROGRAMID",
                            })?;
                        let field_name =
                            c.attribute("FIELDNAME").ok_or(AdxError::MissingAttribute {
                                element: "APP",
                                attribute: "FIELDNAME",
                            })?;
                        Ok(Some((
                            FieldName::AppRecord {
                                program_id,
                                field_name: field_name.to_ascii_uppercase(),
                            },
                            value,
                        )))
                    }
                    _ => Ok(Some((FieldName::Defined(tag_name), value))),
                }
            })
            .flat_map(|ro| ro.transpose())
            .collect();

        Ok(Record { fields: fields? })
    }
}

#[cfg(test)]
mod tests {
    use roxmltree::{Document, Node};

    use crate::{
        document::DataType,
        format::adx::{error::AdxError, field_name::FieldName},
    };

    use super::Record;

    fn example_adx() -> Document<'static> {
        Document::parse(include_str!("../../../fixtures/example.adx")).unwrap()
    }

    fn find_records<'a>(document: &'a Document<'static>) -> Node<'a, 'static> {
        document
            .root_element()
            .children()
            .find(|n| n.tag_name().name().to_uppercase() == "RECORDS")
            .expect("example must have records")
    }

    #[test]
    fn parses_records() {
        let adx = example_adx();
        let records_element = find_records(&adx);
        let record_element = records_element.first_element_child().unwrap();
        let record = Record::new(record_element);
        assert_eq!(
            record,
            Ok(Record {
                fields: vec![
                    (
                        FieldName::Defined("QSO_DATE"),
                        ("19900620".to_string(), None)
                    ),
                    (FieldName::Defined("TIME_ON"), ("1523".to_string(), None)),
                    (FieldName::Defined("CALL"), ("VK9NS".to_string(), None)),
                    (FieldName::Defined("BAND"), ("20M".to_string(), None)),
                    (FieldName::Defined("MODE"), ("RTTY".to_string(), None)),
                    (
                        FieldName::UserdefRecord("SWEATERSIZE".to_string()),
                        ("M".to_string(), None),
                    ),
                    (
                        FieldName::UserdefRecord("SHOESIZE".to_string()),
                        ("11".to_string(), None),
                    ),
                    (
                        FieldName::AppRecord {
                            program_id: "MONOLOG",
                            field_name: "COMPRESSION".to_string(),
                        },
                        ("off".to_string(), Some(DataType::String)),
                    ),
                ]
                .into_iter()
                .collect()
            })
        )
    }

    #[test]
    fn ignores_comments_in_values() {
        let adx = Document::parse("<RECORD><CALL>JL1<!--comment-->HIS</CALL></RECORD>").unwrap();
        let record = Record::new(adx.root_element());
        assert_eq!(
            record,
            Ok(Record {
                fields: vec![(FieldName::Defined("CALL"), ("JL1HIS".to_string(), None))]
                    .into_iter()
                    .collect()
            })
        );
    }

    #[test]
    fn rejects_unknown_data_type() {
        let adx = Document::parse(r#"<RECORD><CALL TYPE="X">JL1HIS</CALL></RECORD>"#).unwrap();
        assert_eq!(
            Record::new(adx.root_element()),
            Err(AdxError::UnknownDataType("X".to_string()))
        );
    }

    #[test]
    fn reports_missing_attribute() {
        let adx = Document::parse(r#"<RECORD><APP FIELDNAME="X">1</APP></RECORD>"#).unwrap();
        assert_eq!(
            Record::new(adx.root_element()),
            Err(AdxError::MissingAttribute {
                element: "APP",
                attribute: "PROGRAMID",
            })
        );
    }
}
