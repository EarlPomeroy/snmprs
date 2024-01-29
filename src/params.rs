use std::io::{Error, ErrorKind};
use crate::snmp::{SNMPVersion, AuthProtocol, PrivacyProtocol};

pub(crate) struct Params {
    host: String,
    username: String,
    auth_protocol: Option<AuthProtocol>,
    auth_password: Option<String>,
    privacy_protocol: Option<PrivacyProtocol>,
    privacy_password: Option<String>,
    community: Option<String>,
    version: SNMPVersion,
}

fn new_params_v3(host: &str, username: &str, auth_protocol: AuthProtocol, auth_password: &str, privacy_protocol: PrivacyProtocol, privacy_password: &str) -> Params {
    Params {
        host: host.to_string(),
        username: username.to_string(),
        auth_protocol: Some(auth_protocol),
        auth_password: Some(auth_password.to_string()),
        privacy_protocol: Some(privacy_protocol),
        privacy_password: Some(privacy_password.to_string()),
        community: None,
        version: SNMPVersion::V3,
    }
}

fn new_params_v2c(host: &str, username: &str, community: &str) -> Params {
    Params {
        host: host.to_string(),
        username: username.to_string(),
        auth_protocol: None,
        auth_password: None,
        privacy_protocol: None,
        privacy_password: None,
        community: Some(community.to_string()),
        version: SNMPVersion::V2c,
    }
}

impl Params {
    pub fn get_host(&self) -> &str {
        self.host.as_str()
    }

    pub fn get_username(&self) -> &str {
        self.username.as_str()
    }

    pub fn get_auth_protocol(&self) -> Result<AuthProtocol, Error> {
        if self.version == SNMPVersion::V2c {
            return Err(Error::new(ErrorKind::Other, "Wrong version"));
        }

        Ok(self.auth_protocol.unwrap())
    }

    pub fn get_auth_password(&self) -> Result<String, Error> {
        if self.version == SNMPVersion::V2c {
            return Err(Error::new(ErrorKind::Other, "Wrong version"));
        }

        Ok(self.auth_password.clone().unwrap())
    }

    pub fn get_privacy_protocol(&self) -> Result<PrivacyProtocol, Error> {
        if self.version == SNMPVersion::V2c {
            return Err(Error::new(ErrorKind::Other, "Wrong version"));
        }

        Ok(self.privacy_protocol.unwrap())
    }

    pub fn get_privacy_password(&self) -> Result<String, Error> {
        if self.version == SNMPVersion::V2c {
            return Err(Error::new(ErrorKind::Other, "Wrong version"));
        }

        Ok(self.privacy_password.clone().unwrap())
    }

    pub fn get_community(&self) -> Result<String, Error> {
        if self.version == SNMPVersion::V3 {
            return Err(Error::new(ErrorKind::Other, "Wrong version"));
        }


        Ok(self.community.clone().unwrap())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // V2 tests
    #[test]
    fn test_v2_config_get_host() {
        let v2 = new_params_v2c("test", "admin", "community");
        assert_eq!(v2.get_host(), "test");
    }

    #[test]
    fn test_v2_config_get_username() {
        let v2 = new_params_v2c("test", "admin", "community");
        assert_eq!(v2.get_username(), "admin");
    }

    #[test]
    fn test_v2_config_get_community() {
        let v2 = new_params_v2c("test", "admin", "community");

        let result = v2.get_community();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "community");
    }

    #[test]
    fn test_v2_config_get_auth_password() {
        let v2 = new_params_v2c("test", "admin", "community");

        let result = v2.get_auth_password();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(err.to_string(), "Wrong version");
    }

    #[test]
    fn test_v2_config_get_auth_protocol() {
        let v2 = new_params_v2c("test", "admin", "community");

        let result = v2.get_auth_protocol();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(err.to_string(), "Wrong version");
    }

    #[test]
    fn test_v2_config_get_privacy_password() {
        let v2 = new_params_v2c("test", "admin", "community");

        let result = v2.get_privacy_password();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(err.to_string(), "Wrong version");
    }

    #[test]
    fn test_v2_config_get_privacy_protocol() {
        let v2 = new_params_v2c("test", "admin", "community");

        let result = v2.get_privacy_protocol();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(err.to_string(), "Wrong version");
    }

    // V3 tests
    #[test]
    fn test_v3_config_get_host() {
        let v3 = new_params_v3("test", "admin", AuthProtocol::SHA, "auth_password", PrivacyProtocol::AES, "privacy_password");
        assert_eq!(v3.get_host(), "test");
    }

    #[test]
    fn test_v3_config_get_username() {
        let v3 = new_params_v3("test", "admin", AuthProtocol::SHA, "auth_password", PrivacyProtocol::AES, "privacy_password");
        assert_eq!(v3.get_username(), "admin");
    }

    #[test]
    fn test_v3_config_get_community() {
        let v3 = new_params_v3("test", "admin", AuthProtocol::SHA, "auth_password", PrivacyProtocol::AES, "privacy_password");

        let result = v3.get_community();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Other);
        assert_eq!(err.to_string(), "Wrong version");
    }

    #[test]
    fn test_v3_config_get_auth_password() {
        let v3 = new_params_v3("test", "admin", AuthProtocol::SHA, "auth_password", PrivacyProtocol::AES, "privacy_password");

        let result = v3.get_auth_password();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "auth_password");
    }

    #[test]
    fn test_v3_config_get_auth_protocol() {
        let v3 = new_params_v3("test", "admin", AuthProtocol::SHA, "auth_password", PrivacyProtocol::AES, "privacy_password");

        let result = v3.get_auth_protocol();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), AuthProtocol::SHA);
    }

    #[test]
    fn test_v3_config_get_privacy_password() {
        let v3 = new_params_v3("test", "admin", AuthProtocol::SHA, "auth_password", PrivacyProtocol::AES, "privacy_password");

        let result = v3.get_privacy_password();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "privacy_password");
    }

    #[test]
    fn test_v3_config_get_privacy_protocol() {
        let v3 = new_params_v3("test", "admin", AuthProtocol::SHA, "auth_password", PrivacyProtocol::AES, "privacy_password");

        let result = v3.get_privacy_protocol();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), PrivacyProtocol::AES);
    }

    #[test]
    fn test_create_snmpv3_config() {
        let v3 = new_params_v3("test", "admin", AuthProtocol::SHA, "auth_password", PrivacyProtocol::AES, "privacy_password");
        assert_eq!(v3.get_host(), "test");
        assert_eq!(v3.get_username(), "admin");
        assert_eq!(v3.get_auth_protocol().unwrap(), AuthProtocol::SHA);
        assert_eq!(v3.get_auth_password().unwrap(), "auth_password");
        assert_eq!(v3.get_privacy_protocol().unwrap(), PrivacyProtocol::AES);
        assert_eq!(v3.get_privacy_password().unwrap(), "privacy_password");
    }
}