mod data_type;
mod field;
mod field_name;
mod record;
mod user_defined_field;

use std::collections::HashMap;

pub use data_type::DataType;
pub use field::Field;
pub use field_name::FieldName;
pub use record::Record;
pub use user_defined_field::{UserDefinedField, ValueConstraint};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdifDocument {
    preamble: String,
    headers: HashMap<String, Field>,
    user_defined_fields: Vec<UserDefinedField>,
    records: Vec<Record>,
}

impl AdifDocument {
    pub fn new<P, H, U, R, RS>(
        preamble: P,
        headers: H,
        user_defined_fields: U,
        records: RS,
    ) -> AdifDocument
    where
        P: Into<String>,
        H: IntoIterator<Item = (String, Field)>,
        U: IntoIterator<Item = UserDefinedField>,
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
        let mut user_defined_fields: Vec<_> = user_defined_fields.into_iter().collect();
        user_defined_fields.sort_by_key(UserDefinedField::id);
        let records = records.into_iter().map(Record::new).collect();

        AdifDocument {
            preamble,
            headers,
            user_defined_fields,
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

    pub fn user_defined_fields(&self) -> &[UserDefinedField] {
        &self.user_defined_fields
    }

    pub fn records(&self) -> &[Record] {
        &self.records
    }
}

pub trait IntoAdifDocument {
    fn into_adif_document(self) -> AdifDocument;
}
