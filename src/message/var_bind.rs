use simple_asn1::{ASN1Block, ASN1Class, BigInt, BigUint, FromASN1, ToASN1, OID};

use crate::message::obj_ident::{ObjectIdentifier, ObjectIdentifierError};
use std::net::IpAddr;

use super::scoped_pdu::SNMPMessageError;
pub(crate) enum BindValue {
    Unspecified,
    Value(ObjectValue),
    NoSuchObject,
    NoSuchInstance,
    EndOfMibView,
}

pub(crate) enum ObjectValue {
    //Simple syntax
    Integer(i32),
    String(String),
    ObjectIdentifier(ObjectIdentifier),
    //Application-wide syntax
    IpAddress(IpAddr),
    Counter(u32),
    TimeTicks(u64),
    Opaque(Vec<u8>),
    Counter64(u64),
    Unsigned32(u32),
}

trait ToASN1Block {
    type Error: From<SNMPMessageError>;

    fn to_asn1_block(&self) -> Result<ASN1Block, Self::Error>;
}

impl ToASN1Block for ObjectValue {
    type Error = SNMPMessageError;

    fn to_asn1_block(&self) -> Result<ASN1Block, Self::Error> {
        let asn_block = match self {
            ObjectValue::Integer(v) => ASN1Block::Integer(4, BigInt::from(*v)),
            ObjectValue::String(v) => ASN1Block::UTF8String(v.len(), v.to_string()),
            ObjectValue::ObjectIdentifier(_) => todo!(),
            ObjectValue::IpAddress(_) => todo!(),
            ObjectValue::Counter(_) => todo!(),
            ObjectValue::TimeTicks(_) => todo!(),
            ObjectValue::Opaque(_) => todo!(),
            ObjectValue::Counter64(_) => todo!(),
            ObjectValue::Unsigned32(_) => todo!(),
        };

        Ok(asn_block)
    }
}

impl ToASN1Block for BindValue {
    type Error = SNMPMessageError;

    fn to_asn1_block(&self) -> Result<ASN1Block, Self::Error> {
        let asn_block = match self {
            BindValue::Unspecified => ASN1Block::Null(0),
            BindValue::Value(ov) => ov.to_asn1_block()?,
            BindValue::NoSuchObject => todo!(),
            BindValue::NoSuchInstance => todo!(),
            BindValue::EndOfMibView => todo!(),
        };

        Ok(asn_block)
    }
}

impl TryFrom<&ObjectIdentifier> for OID {
    type Error = ObjectIdentifierError;

    fn try_from(value: &ObjectIdentifier) -> Result<Self, Self::Error> {
        if value.length() > crate::message::obj_ident::MAX_OBJECT_IDENTIFIER_LEN {
            return Err(ObjectIdentifierError::TooLong(value.length()));
        };

        let big_vec: Vec<BigUint> = value
            .get_value()
            .iter()
            .map(|&v| BigUint::from(v))
            .collect();

        Ok(OID::new(big_vec))
    }
}
// RFC 3416
pub(crate) struct VarBind {
    name: ObjectIdentifier,
    value: BindValue,
}

impl VarBind {
    pub(crate) fn new(name: ObjectIdentifier, value: BindValue) -> Self {
        Self { name, value }
    }
}

impl FromASN1 for VarBind {
    type Error = SNMPMessageError;

    fn from_asn1(v: &[ASN1Block]) -> Result<(Self, &[ASN1Block]), Self::Error> {
        todo!()
    }
}

impl ToASN1 for VarBind {
    type Error = SNMPMessageError;

    fn to_asn1_class(&self, _c: ASN1Class) -> Result<Vec<ASN1Block>, Self::Error> {
        let mut asn_vec: Vec<ASN1Block> = Vec::new();

        // encode ObjectIdentifier
        match OID::try_from(&self.name) {
            Ok(oid) => asn_vec.push(ASN1Block::ObjectIdentifier(self.name.length(), oid)),
            Err(e) => {
                return Err(SNMPMessageError::EncodeError(
                    "Error encoding OID".to_string(),
                ))
            }
        }

        let value_block = self.value.to_asn1_block()?;

        asn_vec.push(value_block);

        Ok(asn_vec)
    }
}

#[cfg(test)]
mod tests {
    use simple_asn1::oid;

    use super::*;

    #[test]
    fn test_create_var_bind() {
        let name = ObjectIdentifier::new(vec![1, 3, 5, 7, 9, 11, 13, 17]);
        let value = BindValue::Unspecified;

        let var_bind = VarBind::new(name, value);
        let asn = var_bind.to_asn1().unwrap();

        assert_eq!(
            asn,
            vec![
                ASN1Block::ObjectIdentifier(8, oid!(1, 3, 5, 7, 9, 11, 13, 17)),
                ASN1Block::Null(0)
            ]
        );
    }

    #[test]
    fn test_create_var_bind_with_int_value() {
        let name = ObjectIdentifier::new(vec![1, 3, 5, 7, 9, 11, 13, 17]);
        let value = BindValue::Value(ObjectValue::Integer(42));

        let var_bind = VarBind::new(name, value);
        let asn = var_bind.to_asn1().unwrap();

        assert_eq!(
            asn,
            vec![
                ASN1Block::ObjectIdentifier(8, oid!(1, 3, 5, 7, 9, 11, 13, 17)),
                ASN1Block::Integer(4, BigInt::from(42))
            ]
        );
    }
    #[test]
    fn test_create_var_bind_with_string_value() {
        let name = ObjectIdentifier::new(vec![1, 3, 5, 7, 9, 11, 13, 17]);
        let value = BindValue::Value(ObjectValue::String("This is a test".to_string()));

        let var_bind = VarBind::new(name, value);
        let asn = var_bind.to_asn1().unwrap();

        assert_eq!(
            asn,
            vec![
                ASN1Block::ObjectIdentifier(8, oid!(1, 3, 5, 7, 9, 11, 13, 17)),
                ASN1Block::UTF8String(14, "This is a test".to_string())
            ]
        );
    }
}
