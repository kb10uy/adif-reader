use std::collections::HashMap;

use roxmltree::{Node, NodeType};

use crate::{
    document::{DataType, UserDefinedField, ValueConstraint},
    format::adx::{element_text, error::AdxError, field_name::FieldName, parse_data_type},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    pub fields: HashMap<FieldName<'a>, (String, Option<DataType>)>,
    pub user_defined_fields: Vec<UserDefinedField>,
}

impl<'a, 'i: 'a> Header<'a> {
    pub fn new(header_element: Node<'a, 'i>) -> Result<Header<'a>, AdxError> {
        let fields: Result<_, AdxError> = header_element
            .children()
            .map(|c| {
                let NodeType::Element = c.node_type() else {
                    return Ok(None);
                };

                let tag_name = c.tag_name().name();
                let value = (element_text(c), parse_data_type(c)?);
                if tag_name == "USERDEF" {
                    let id = c
                        .attribute("FIELDID")
                        .ok_or(AdxError::MissingAttribute {
                            element: "USERDEF",
                            attribute: "FIELDID",
                        })?
                        .parse()?;
                    Ok(Some((FieldName::UserdefHeader(id), value)))
                } else {
                    Ok(Some((FieldName::Defined(tag_name), value)))
                }
            })
            .flat_map(|ro| ro.transpose())
            .collect();

        let user_defined_fields = header_element
            .children()
            .filter(|c| c.is_element() && c.tag_name().name() == "USERDEF")
            .map(parse_user_defined_field)
            .collect::<Result<_, _>>()?;

        Ok(Header {
            fields: fields?,
            user_defined_fields,
        })
    }
}

fn parse_user_defined_field(element: Node) -> Result<UserDefinedField, AdxError> {
    let id = element
        .attribute("FIELDID")
        .ok_or(AdxError::MissingAttribute {
            element: "USERDEF",
            attribute: "FIELDID",
        })?
        .parse()?;
    let constraint = match (element.attribute("ENUM"), element.attribute("RANGE")) {
        (None, None) => None,
        (Some(e), None) => Some(ValueConstraint::parse_enumeration(e).ok_or(
            AdxError::InvalidAttribute {
                element: "USERDEF",
                attribute: "ENUM",
            },
        )?),
        (None, Some(r)) => Some(ValueConstraint::parse_range(r).ok_or(
            AdxError::InvalidAttribute {
                element: "USERDEF",
                attribute: "RANGE",
            },
        )?),
        (Some(_), Some(_)) => return Err(AdxError::EnumAndRange),
    };

    Ok(UserDefinedField::new(
        id,
        element_text(element).to_ascii_uppercase(),
        parse_data_type(element)?,
        constraint,
    ))
}

#[cfg(test)]
mod tests {
    use roxmltree::{Document, Node};

    use crate::{
        document::{DataType, UserDefinedField, ValueConstraint},
        format::adx::{error::AdxError, field_name::FieldName},
    };

    use super::Header;

    fn example_adx() -> Document<'static> {
        Document::parse(include_str!("../../../fixtures/example.adx")).unwrap()
    }

    fn find_header<'a>(document: &'a Document<'static>) -> Node<'a, 'static> {
        document
            .root_element()
            .children()
            .find(|n| n.tag_name().name().to_uppercase() == "HEADER")
            .expect("example must have header")
    }

    #[test]
    fn parses_header() {
        let adx = example_adx();
        let header_element = find_header(&adx);
        let header = Header::new(header_element);
        assert_eq!(
            header,
            Ok(Header {
                fields: vec![
                    (FieldName::Defined("ADIF_VER"), ("3.0.5".to_string(), None)),
                    (
                        FieldName::Defined("PROGRAMID"),
                        ("monolog".to_string(), None)
                    ),
                    (
                        FieldName::UserdefHeader(1),
                        ("EPC".to_string(), Some(DataType::Number))
                    ),
                    (
                        FieldName::UserdefHeader(2),
                        ("SWEATERSIZE".to_string(), Some(DataType::Enumeration))
                    ),
                    (
                        FieldName::UserdefHeader(3),
                        ("SHOESIZE".to_string(), Some(DataType::Number))
                    ),
                ]
                .into_iter()
                .collect(),
                user_defined_fields: vec![
                    UserDefinedField::new(1, "EPC", Some(DataType::Number), None),
                    UserDefinedField::new(
                        2,
                        "SWEATERSIZE",
                        Some(DataType::Enumeration),
                        Some(ValueConstraint::Enumeration(vec![
                            "S".to_string(),
                            "M".to_string(),
                            "L".to_string(),
                        ])),
                    ),
                    UserDefinedField::new(
                        3,
                        "SHOESIZE",
                        Some(DataType::Number),
                        Some(ValueConstraint::Range {
                            lower: "5".to_string(),
                            upper: "20".to_string(),
                        }),
                    ),
                ],
            })
        )
    }

    #[test]
    fn rejects_both_enum_and_range() {
        let adx = Document::parse(
            r#"<HEADER><USERDEF FIELDID="1" ENUM="{A}" RANGE="{1:2}">X</USERDEF></HEADER>"#,
        )
        .unwrap();
        assert_eq!(Header::new(adx.root_element()), Err(AdxError::EnumAndRange));
    }

    #[test]
    fn rejects_invalid_range() {
        let adx =
            Document::parse(r#"<HEADER><USERDEF FIELDID="1" RANGE="5-20">X</USERDEF></HEADER>"#)
                .unwrap();
        assert_eq!(
            Header::new(adx.root_element()),
            Err(AdxError::InvalidAttribute {
                element: "USERDEF",
                attribute: "RANGE",
            })
        );
    }
}
