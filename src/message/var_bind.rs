use crate::message::obj_ident::ObjectIdentifier;
use std::net::IpAddr;
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
    String(Vec<u8>),
    ObjectIdentifier(ObjectIdentifier),
    //Application-wide syntax
    IpAddress(IpAddr),
    Counter(u32),
    TimeTicks(u64),
    Opaque(Vec<u8>),
    Counter64(u64),
    Unsigned32(u32),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::obj_ident::ObjectIdentifier;

    // #[test]
    // fn test_var_bind() {
    //     let oid = ObjectIdentifier::new(vec![1, 3, 6, 1, 2, 1, 1, 1, 0]);
    //     let value = BindValue::Value(ObjectValue::String(b"test".to_vec()));
    //     let var_bind = VarBind::new(oid, value);
    //     assert_eq!(
    //         var_bind.name,
    //         ObjectIdentifier::new(vec![1, 3, 6, 1, 2, 1, 1, 1, 0])
    //     );
    //     if let BindValue::Value(ObjectValue::String(value)) = var_bind.value {
    //         assert_eq!(value, b"test");
    //     } else {
    //         panic!("Unexpected value");
    //     }
    // }
}
