mod data_type;
mod field;
mod field_name;
mod record;

use std::collections::HashMap;

pub use data_type::DataType;
pub use field::Field;
pub use field_name::FieldName;
pub use record::Record;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdifDocument {
    preamble: String,
    headers: HashMap<String, Field>,
    records: Vec<Record>,
}

impl AdifDocument {
    pub fn new<P, H, R, RS>(preamble: P, headers: H, records: RS) -> AdifDocument
    where
        P: Into<String>,
        H: IntoIterator<Item = (String, Field)>,
        R: IntoIterator<Item = (String, Field)>,
        RS: IntoIterator<Item = R>,
    {
        let preamble = preamble.into();
        let headers = headers
            .into_iter()
            .map(|(mut k, v)| {
                k.make_ascii_uppercase();
                (k, v)
            })
            .collect();
        let records = records.into_iter().map(Record::new).collect();

        AdifDocument {
            preamble,
            headers,
            records,
        }
    }

    pub fn preamble(&self) -> &str {
        &self.preamble
    }

    pub fn header<'a, F: Into<FieldName<'a>>>(&self, name: F) -> Option<&str> {
        let field_name = name.into();
        self.headers.get(field_name.as_str()).map(Field::value)
    }

    pub fn header_type<'a, F: Into<FieldName<'a>>>(&self, name: F) -> Option<DataType> {
        let field_name = name.into();
        self.headers
            .get(field_name.as_str())
            .and_then(Field::data_type)
    }

    pub fn headers(&self) -> &HashMap<String, Field> {
        &self.headers
    }

    pub fn records(&self) -> &[Record] {
        &self.records
    }
}

pub trait IntoAdifDocument {
    fn into_adif_document(self) -> AdifDocument;
}
