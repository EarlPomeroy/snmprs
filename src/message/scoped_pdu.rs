use core::marker::PhantomData;
use simple_asn1::{ASN1Block, ASN1DecodeErr, ASN1EncodeErr, BigInt, FromASN1, ToASN1};

use crate::message::var_bind::VarBind;

use super::var_bind;

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

fn generate_request_id() -> i32 {
    rand::random::<i32>()
}

pub(crate) enum PDUError {
    NoError = 0,
    TooBig = 1,
    NoSuchName = 2, // for proxy compatibility
    BadValue = 3,   // for proxy compatibility
    ReadOnly = 4,   // for proxy compatibility
    GenErr = 5,
    NoAccess = 6,
    WrongType = 7,
    WrongLength = 8,
    WrongEncoding = 9,
    WrongValue = 10,
    NoCreation = 11,
    InconsistentValue = 12,
    ResourceUnavailable = 13,
    CommitFailed = 14,
    UndoFailed = 15,
    AuthorizationError = 16,
    NotWritable = 17,
    InconsistentName = 18,
}

impl From<ASN1DecodeErr> for PDUError {
    fn from(value: ASN1DecodeErr) -> Self {
        todo!()
    }
}

impl From<ASN1EncodeErr> for PDUError {
    fn from(value: ASN1EncodeErr) -> Self {
        todo!()
    }
}

enum PDUType {
    GetRequest = 0,
    GetNextRequest = 1,
    Response = 2,
    SetRequest = 3,
    GetBulkRequest = 5,
    InformRequest = 6,
    Trap = 7,
    Report = 8,
}

pub(crate) struct PDU {
    request_id: i32,
    error_status: i32,
    error_index: u32,
    var_bindings: Vec<VarBind>,
}

impl PDU {
    pub(crate) fn new(v: Vec<VarBind>) -> Self {
        Self {
            request_id: generate_request_id(),
            error_status: 0,
            error_index: 0,
            var_bindings: v,
        }
    }
}
impl FromASN1 for PDU {
    type Error = PDUError;

    fn from_asn1(
        v: &[simple_asn1::ASN1Block],
    ) -> Result<(Self, &[simple_asn1::ASN1Block]), Self::Error> {
        todo!()
    }
}

impl ToASN1 for PDU {
    type Error = SNMPMessageError;

    fn to_asn1_class(&self, c: simple_asn1::ASN1Class) -> Result<Vec<ASN1Block>, Self::Error> {
        let mut asn_vec = Vec::<ASN1Block>::new();

        let req_id_block = ASN1Block::Integer(0, BigInt::from(self.request_id));
        let err_status_block = ASN1Block::Integer(0, BigInt::from(self.error_status));
        let err_index_block = ASN1Block::Integer(0, BigInt::from(self.error_index));

        asn_vec.push(req_id_block);
        asn_vec.push(err_status_block);
        asn_vec.push(err_index_block);

        for var_bind in &self.var_bindings {
            let mut var_bind_block = var_bind.to_asn1()?;
            asn_vec.append(&mut var_bind_block);
        }

        let asn_seq = ASN1Block::Sequence(0, asn_vec);

        Ok(vec![asn_seq])
    }
}

pub(crate) struct BulkPDU {
    request_id: i32,
    non_repeaters: i32,
    max_repetions: i32,
    var_bindings: Vec<VarBind>,
}

impl BulkPDU {
    pub(crate) fn new(v: Vec<VarBind>) -> Self {
        Self {
            request_id: generate_request_id(),
            non_repeaters: 0,
            max_repetions: 0,
            var_bindings: v,
        }
    }
}

impl FromASN1 for BulkPDU {
    type Error = PDUError;

    fn from_asn1(
        v: &[simple_asn1::ASN1Block],
    ) -> Result<(Self, &[simple_asn1::ASN1Block]), Self::Error> {
        todo!()
    }
}

impl ToASN1 for BulkPDU {
    type Error = PDUError;

    fn to_asn1_class(
        &self,
        c: simple_asn1::ASN1Class,
    ) -> Result<Vec<simple_asn1::ASN1Block>, Self::Error> {
        todo!()
    }
}
