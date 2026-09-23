use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldName<'a> {
    Defined(&'a str),
    UserdefHeader(usize),
    UserdefRecord(String),
    AppRecord {
        program_id: &'a str,
        field_name: String,
    },
}

impl<'a> Display for FieldName<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FieldName::Defined(n) => write!(f, "{n}"),
            FieldName::UserdefHeader(i) => write!(f, "USERDEF{i}"),
            FieldName::UserdefRecord(n) => write!(f, "{n}"),
            FieldName::AppRecord {
                program_id,
                field_name,
            } => write!(f, "APP_{}_{field_name}", program_id.to_uppercase()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FieldName;

    #[test]
    fn formats_app_record_with_program_id() {
        let name = FieldName::AppRecord {
            program_id: "MonoLog",
            field_name: "COMPRESSION".to_string(),
        };
        assert_eq!(name.to_string(), "APP_MONOLOG_COMPRESSION");
    }
}
