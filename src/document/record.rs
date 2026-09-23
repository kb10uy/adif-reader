use std::collections::HashMap;

use crate::document::{DataType, Field, FieldName};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    fields: HashMap<String, Field>,
}

impl Record {
    pub(super) fn new<R: IntoIterator<Item = (String, Field)>>(fields: R) -> Record {
        let fields = fields
            .into_iter()
            .map(|(mut k, v)| {
                k.make_ascii_uppercase();
                (k, v)
            })
            .collect();

        Record { fields }
    }

    pub fn field<'a, F: Into<FieldName<'a>>>(&self, name: F) -> Option<&str> {
        let field_name = name.into();
        self.fields.get(field_name.as_str()).map(Field::value)
    }

    pub fn field_type<'a, F: Into<FieldName<'a>>>(&self, name: F) -> Option<DataType> {
        let field_name = name.into();
        self.fields
            .get(field_name.as_str())
            .and_then(Field::data_type)
    }

    pub fn fields(&self) -> &HashMap<String, Field> {
        &self.fields
    }
}
