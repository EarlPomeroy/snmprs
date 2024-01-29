use crate::params::Params;
use std::io::Error;

use crate::params;

#[derive(PartialEq)]
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
    fn get(&self, oid: &str) -> Result<String,  Error>;
    fn get_next(&self, oid: &str) -> Result<String,  Error>;
    fn get_bulk(&self, oid: &str, max_repetitions: u32) -> Result<String,  Error>;
    fn set(&self, oid: &str, value: &str) -> Result<String,  Error>;
    fn walk(&self, oid: &str) -> Result<String,  Error>;
}

// pub fn new_client(params: Box<dyn Params>, version: SNMPVersion) -> Result< SNMPClient,  Error> {
//     // let mut client = Client::new(host, community);
//     // client.set_port(port);
//     // client.set_version(version);
//     // Ok(client)
//     todo!()
// }

