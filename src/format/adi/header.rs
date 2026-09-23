use std::collections::HashMap;

use crate::{
    document::FieldName,
    format::adi::{
        data::{FieldValue, LengthMode, get_field_value},
        error::AdiError,
        tag::Tag,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    pub preamble: &'a str,
    pub fields: HashMap<FieldName<'a>, &'a str>,
}

impl<'a> Header<'a> {
    pub fn parse(
        text: &'a str,
        length_mode: LengthMode,
    ) -> Result<(Option<Header<'a>>, usize), AdiError> {
        // > If the first character in an ADI file is <, it contains no Header.
        // https://adif.org.uk/316/ADIF_316.htm#ADI_File_Format
        if text.starts_with('<') {
            return Ok((None, 0));
        }
        let Some(header_start) = Tag::find(text) else {
            return Err(AdiError::NoData);
        };
        let preamble = &text[..header_start];

        let mut fields = HashMap::new();
        let mut consumed = header_start;
        loop {
            match Tag::parse(&text[consumed..]) {
                Ok((
                    Tag::Specifier {
                        name, value_length, ..
                    },
                    c,
                )) => {
                    consumed += c;
                    let value = match get_field_value(&text[consumed..], length_mode, value_length)
                    {
                        FieldValue::Found(v) => v,
                        FieldValue::InvalidBoundary => {
                            return Err(AdiError::CharacterBoundary(consumed));
                        }
                        FieldValue::NotEnough => {
                            return Err(AdiError::ValueTooShort {
                                expected: value_length,
                                maximum: text.len() - consumed,
                            });
                        }
                    };
                    fields.insert(FieldName::new(name), value);
                    consumed += value.len();
                }
                Ok((Tag::EndOfHeader, c)) => {
                    consumed += c;
                    break;
                }

                Ok(_) => return Err(AdiError::NoEoh),
                Err(e) => return Err(AdiError::Tag(consumed, e)),
            }
        }

        Ok((Some(Header { preamble, fields }), consumed))
    }
}

#[cfg(test)]
mod tests {
    use crate::format::adi::data::LengthMode;

    use super::Header;

    #[test]
    fn parses_header() {
        let adi_text = include_str!("../../../fixtures/basic-header.adi");

        let expected = Header {
            preamble: "Fixture ADI File\n",
            fields: vec![
                ("ADIF_VER".into(), "3.1.6"),
                ("CREATED_TIMESTAMP".into(), "20260120 000000"),
                ("PROGRAMID".into(), "jelgen"),
                ("PROGRAMVERSION".into(), "0.1.0"),
            ]
            .into_iter()
            .collect(),
        };
        assert_eq!(
            Header::parse(adi_text, LengthMode::Bytes),
            Ok((Some(expected), 122))
        );
    }

    #[test]
    fn keeps_last_of_case_insensitive_duplicates() {
        let (header, _) = Header::parse("x<adif_ver:1>1<ADIF_VER:1>2<EOH>", LengthMode::Bytes)
            .expect("must parse");
        let header = header.expect("must have header");
        assert_eq!(header.fields.len(), 1);
        assert_eq!(header.fields.get(&"ADIF_VER".into()), Some(&"2"));
    }

    #[test]
    fn keeps_non_tag_brackets_in_preamble() {
        let (header, _) =
            Header::parse("Log by <JL1HIS>\n<EOH>", LengthMode::Bytes).expect("must parse");
        assert_eq!(
            header.expect("must have header").preamble,
            "Log by <JL1HIS>\n"
        );
    }
}
