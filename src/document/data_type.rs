#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    Boolean,
    Number,
    Date,
    Time,
    String,
    IntlString,
    MultilineString,
    IntlMultilineString,
    Enumeration,
    Location,
}

impl DataType {
    pub fn from_indicator(indicator: &str) -> Option<DataType> {
        let [c] = indicator.as_bytes() else {
            return None;
        };
        match c.to_ascii_uppercase() {
            b'B' => Some(DataType::Boolean),
            b'N' => Some(DataType::Number),
            b'D' => Some(DataType::Date),
            b'T' => Some(DataType::Time),
            b'S' => Some(DataType::String),
            b'I' => Some(DataType::IntlString),
            b'M' => Some(DataType::MultilineString),
            b'G' => Some(DataType::IntlMultilineString),
            b'E' => Some(DataType::Enumeration),
            b'L' => Some(DataType::Location),
            _ => None,
        }
    }

    pub fn indicator(self) -> char {
        match self {
            DataType::Boolean => 'B',
            DataType::Number => 'N',
            DataType::Date => 'D',
            DataType::Time => 'T',
            DataType::String => 'S',
            DataType::IntlString => 'I',
            DataType::MultilineString => 'M',
            DataType::IntlMultilineString => 'G',
            DataType::Enumeration => 'E',
            DataType::Location => 'L',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DataType;

    #[test]
    fn parses_indicators_case_insensitively() {
        assert_eq!(DataType::from_indicator("N"), Some(DataType::Number));
        assert_eq!(DataType::from_indicator("s"), Some(DataType::String));
        assert_eq!(DataType::from_indicator("X"), None);
        assert_eq!(DataType::from_indicator("SS"), None);
        assert_eq!(DataType::from_indicator(""), None);
    }

    #[test]
    fn round_trips_indicators() {
        for c in "BNDTSIMGEL".chars() {
            let data_type = DataType::from_indicator(&c.to_string()).expect("must be known");
            assert_eq!(data_type.indicator(), c);
        }
    }
}
