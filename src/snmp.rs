use crate::params::Params;
use std::io::Error;

use crate::params;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SNMPVersion {
    V2c,
    V3,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AuthProtocol {
    MD5,
    SHA,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PrivacyProtocol {
    DES,
    AES,
}

trait SNMPClient {
    fn get(&self, oid: &str) -> Result<String, Error>;
    fn get_next(&self, oid: &str) -> Result<String, Error>;
    fn get_bulk(&self, oid: &str, max_repetitions: u32) -> Result<String, Error>;
    fn set(&self, oid: &str, value: &str) -> Result<String, Error>;
    fn walk(&self, oid: &str) -> Result<String, Error>;
}

struct SNMPClientV2<'a> {
    params: &'a Params,
}

impl<'a> SNMPClientV2<'a> {
    pub fn new(params: &'a Params) -> Self {
        Self { params }
    }
}

impl SNMPClient for SNMPClientV2<'_> {
    fn get(&self, oid: &str) -> Result<String, Error> {
        todo!()
    }

    fn get_next(&self, oid: &str) -> Result<String, Error> {
        todo!()
    }

    fn get_bulk(&self, oid: &str, max_repetitions: u32) -> Result<String, Error> {
        todo!()
    }

    fn set(&self, oid: &str, value: &str) -> Result<String, Error> {
        todo!()
    }

    fn walk(&self, oid: &str) -> Result<String, Error> {
        todo!()
    }
}

struct SNMPClientV3<'a> {
    params: &'a Params,
}

impl<'a> SNMPClientV3<'a> {
    pub fn new(params: &'a Params) -> Self {
        Self { params }
    }
}

impl SNMPClient for SNMPClientV3<'_> {
    fn get(&self, oid: &str) -> Result<String, Error> {
        todo!()
    }

    fn get_next(&self, oid: &str) -> Result<String, Error> {
        todo!()
    }

    fn get_bulk(&self, oid: &str, max_repetitions: u32) -> Result<String, Error> {
        todo!()
    }

    fn set(&self, oid: &str, value: &str) -> Result<String, Error> {
        todo!()
    }

    fn walk(&self, oid: &str) -> Result<String, Error> {
        todo!()
    }
}

pub fn new_client<'a>(params: &'a Params) -> Result<Box<dyn SNMPClient + 'a>, Error> {
    match params.get_version() {
        SNMPVersion::V2c => Ok(Box::new(SNMPClientV2::new(params))),
        SNMPVersion::V3 => Ok(Box::new(SNMPClientV3::new(params))),
    }
}
