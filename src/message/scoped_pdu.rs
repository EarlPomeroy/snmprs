use simple_asn1::{ASN1DecodeErr, ASN1EncodeErr};

#[derive(Debug)]
pub(crate) enum SNMPMessageError {
    DecodeError(String),
    EncodeError(String),
    ParseError(String),
    TooLong(usize),
}

// impl Display for SNMPMessageError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

impl From<ASN1DecodeErr> for SNMPMessageError {
    fn from(value: ASN1DecodeErr) -> Self {
        SNMPMessageError::DecodeError(value.to_string())
    }
}

impl From<ASN1EncodeErr> for SNMPMessageError {
    fn from(value: ASN1EncodeErr) -> Self {
        SNMPMessageError::EncodeError(value.to_string())
    }
}
