use crate::document::DataType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueConstraint {
    Enumeration(Vec<String>),
    Range { lower: String, upper: String },
}

impl ValueConstraint {
    pub(crate) fn parse(braced: &str) -> Option<ValueConstraint> {
        let inner = unbrace(braced)?;
        if inner.contains(':') && !inner.contains(',') {
            range(inner)
        } else {
            enumeration(inner)
        }
    }

    pub(crate) fn parse_enumeration(braced: &str) -> Option<ValueConstraint> {
        enumeration(unbrace(braced)?)
    }

    pub(crate) fn parse_range(braced: &str) -> Option<ValueConstraint> {
        range(unbrace(braced)?)
    }
}

fn unbrace(braced: &str) -> Option<&str> {
    braced.trim().strip_prefix('{')?.strip_suffix('}')
}

fn enumeration(inner: &str) -> Option<ValueConstraint> {
    if inner.trim().is_empty() {
        return None;
    }
    let values = inner.split(',').map(|v| v.trim().to_string()).collect();
    Some(ValueConstraint::Enumeration(values))
}

fn range(inner: &str) -> Option<ValueConstraint> {
    let (lower, upper) = inner.split_once(':')?;
    Some(ValueConstraint::Range {
        lower: lower.trim().to_string(),
        upper: upper.trim().to_string(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserDefinedField {
    id: usize,
    name: String,
    data_type: Option<DataType>,
    constraint: Option<ValueConstraint>,
}

impl UserDefinedField {
    pub fn new(
        id: usize,
        name: impl Into<String>,
        data_type: Option<DataType>,
        constraint: Option<ValueConstraint>,
    ) -> UserDefinedField {
        UserDefinedField {
            id,
            name: name.into(),
            data_type,
            constraint,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data_type(&self) -> Option<DataType> {
        self.data_type
    }

    pub fn constraint(&self) -> Option<&ValueConstraint> {
        self.constraint.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::ValueConstraint;

    fn enumeration(values: &[&str]) -> ValueConstraint {
        ValueConstraint::Enumeration(values.iter().map(|v| v.to_string()).collect())
    }

    fn range(lower: &str, upper: &str) -> ValueConstraint {
        ValueConstraint::Range {
            lower: lower.to_string(),
            upper: upper.to_string(),
        }
    }

    #[test]
    fn parses_constraints() {
        assert_eq!(
            ValueConstraint::parse("{S,M,L}"),
            Some(enumeration(&["S", "M", "L"]))
        );
        assert_eq!(
            ValueConstraint::parse("{A, B, C, D}"),
            Some(enumeration(&["A", "B", "C", "D"]))
        );
        assert_eq!(ValueConstraint::parse("{5:20}"), Some(range("5", "20")));
        assert_eq!(ValueConstraint::parse("{}"), None);
        assert_eq!(ValueConstraint::parse("S,M,L"), None);
    }

    #[test]
    fn parses_explicit_constraint_kinds() {
        assert_eq!(
            ValueConstraint::parse_enumeration("{A:B}"),
            Some(enumeration(&["A:B"]))
        );
        assert_eq!(
            ValueConstraint::parse_range("{-1.5:2}"),
            Some(range("-1.5", "2"))
        );
        assert_eq!(ValueConstraint::parse_range("{5}"), None);
    }
}
