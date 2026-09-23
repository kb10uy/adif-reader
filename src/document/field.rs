use crate::document::DataType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    value: String,
    data_type: Option<DataType>,
}

impl Field {
    pub fn new<V: Into<String>>(value: V, data_type: Option<DataType>) -> Field {
        Field {
            value: value.into(),
            data_type,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn data_type(&self) -> Option<DataType> {
        self.data_type
    }
}
