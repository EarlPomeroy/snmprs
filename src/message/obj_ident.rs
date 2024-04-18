use std::{error::Error, fmt::Display, str::FromStr, u32};

use simple_asn1::OID;

pub const MAX_OBJECT_IDENTIFIER_LEN: usize = 128;

#[derive(Debug)]
pub(crate) enum ObjectIdentifierError {
    ParseError(String),
    DecodeError(String),
    TooLong(usize),
}

impl Display for ObjectIdentifierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectIdentifierError::ParseError(e) => {
                write!(f, "Parse error: {} is not the right format", e)
            }
            ObjectIdentifierError::DecodeError(e) => write!(f, "Decode error: {}", e),
            ObjectIdentifierError::TooLong(size) => write!(
                f,
                "OID of length {} exceeds the maximum: {}",
                size, MAX_OBJECT_IDENTIFIER_LEN
            ),
        }
    }
}

impl Error for ObjectIdentifierError {}

#[derive(Debug, Clone)]
pub(crate) struct ObjectIdentifier {
    value: Vec<u32>,
}

impl ObjectIdentifier {
    pub(crate) fn new(oid: Vec<u32>) -> Self {
        Self { value: oid }
    }

    pub(crate) fn get_value(&self) -> Vec<u32> {
        self.value.clone()
    }

    pub(crate) fn length(&self) -> usize {
        self.value.len()
    }
}

impl FromStr for ObjectIdentifier {
    type Err = ObjectIdentifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut oid_vec = Vec::new();

        for num_str in s.split('.') {
            if let Ok(num) = num_str.parse::<u32>() {
                oid_vec.push(num)
            } else {
                return Err(ObjectIdentifierError::ParseError(s.to_string()));
            }
        }

        Ok(ObjectIdentifier::new(oid_vec))
    }
}

impl TryFrom<&OID> for ObjectIdentifier {
    type Error = ObjectIdentifierError;

    fn try_from(value: &OID) -> Result<Self, Self::Error> {
        match value.as_vec::<u32>() {
            Ok(oid_vec) => Ok(ObjectIdentifier::new(oid_vec)),
            Err(e) => Err(ObjectIdentifierError::DecodeError(e.to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_obj_ident_from_str() {
        let oid_str = "1.3.4.105.99.0.1";

        let oid = ObjectIdentifier::from_str(oid_str).unwrap();
        assert_eq!(oid.get_value(), vec![1, 3, 4, 105, 99, 0, 1]);
        assert_eq!(oid.length(), 7);
    }

    #[test]
    fn test_create_obj_ident_from_vec() {
        let oid = ObjectIdentifier::new(vec![1, 3, 4, 105, 99, 0, 1]);
        assert_eq!(oid.get_value(), vec![1, 3, 4, 105, 99, 0, 1]);
        assert_eq!(oid.length(), 7);
    }

    #[test]
    fn test_create_obj_ident_error() {
        let oid_str_arr = vec![
            (
                "1.2.3.4.5.a.6",
                "Parse error: 1.2.3.4.5.a.6 is not the right format",
            ),
            (
                "1,2,3,4,5",
                "Parse error: 1,2,3,4,5 is not the right format",
            ),
        ];
        for (oid, expected) in oid_str_arr {
            let oid = ObjectIdentifier::from_str(oid);
            assert!(oid.is_err());
            assert_eq!(oid.unwrap_err().to_string(), expected);
        }
    }

    #[test]
    fn test_create_obj_really_bad_str() {
        let oid = ObjectIdentifier::from_str("1.3.4.105084028402984092840239840.99.0.1");
        assert!(oid.is_err());
    }

    #[test]
    fn test_ans1_encoding() {
        let oid = ObjectIdentifier::from_str("1.3.5.7.11.13.17").unwrap();
        let asn1_oid = OID::try_from(&oid).unwrap();
        assert_eq!(
            asn1_oid.as_vec::<u32>().unwrap(),
            vec![1, 3, 5, 7, 11, 13, 17]
        );
    }
}
