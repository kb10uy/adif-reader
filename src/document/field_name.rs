use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldName<'a>(Cow<'a, str>);

impl<'a> FieldName<'a> {
    pub fn new(name: &'a str) -> FieldName<'a> {
        if name.bytes().any(|b| b.is_ascii_lowercase()) {
            FieldName(Cow::Owned(name.to_ascii_uppercase()))
        } else {
            FieldName(Cow::Borrowed(name))
        }
    }

    pub fn new_owned(mut name: String) -> FieldName<'a> {
        name.make_ascii_uppercase();
        FieldName(Cow::Owned(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'a> From<&'a str> for FieldName<'a> {
    fn from(value: &'a str) -> Self {
        FieldName::new(value)
    }
}

impl<'a> From<String> for FieldName<'a> {
    fn from(value: String) -> Self {
        FieldName::new_owned(value)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::FieldName;

    #[test]
    fn borrows_names_without_lowercase_letters() {
        assert!(matches!(
            FieldName::new("ADIF_VER").0,
            Cow::Borrowed("ADIF_VER")
        ));
        assert!(matches!(
            FieldName::new("USERDEF1").0,
            Cow::Borrowed("USERDEF1")
        ));
    }

    #[test]
    fn uppercases_only_ascii_letters() {
        assert_eq!(FieldName::new("call").as_str(), "CALL");
        assert_eq!(FieldName::new("straße").as_str(), "STRAßE");
        assert_eq!(
            FieldName::new_owned("straße".to_string()).as_str(),
            "STRAßE"
        );
    }
}
