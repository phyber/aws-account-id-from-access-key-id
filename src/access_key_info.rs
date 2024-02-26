// Access Key Info
use crate::account_id::AccountId;
use crate::error::AccessKeyError;
use crate::key_type::KeyType;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct AccessKeyInfo {
    account_id: AccountId,
    key_type: KeyType,
}

impl AccessKeyInfo {
    /// # Errors
    ///
    /// Will return an `AccessKeyError` if:
    ///   - The access key length is incorrect (not 20 characters long)
    ///   - The Account ID cannot be decoded from the base32
    ///   - The key type is unknown
    pub fn new(access_key_id: &str) -> Result<Self, AccessKeyError> {
        Self::from_str(access_key_id)
    }
}

impl fmt::Display for AccessKeyInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AccountID: {account_id}, Key Desc: {description}",
            account_id = self.account_id,
            description = self.key_type.description(),
        )
    }
}

impl FromStr for AccessKeyInfo {
    type Err = AccessKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 20 {
            return Err(AccessKeyError::InvalidLength);
        }

        // Get the key type first, we error fast if we don't recognise the
        // type.
        let key_type = KeyType::from_str(s)?;
        let account_id = AccountId::from_str(s)?;

        let access_key_info = Self {
            account_id,
            key_type,
        };

        Ok(access_key_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_key_info_from_str_ok() {
        let info = AccessKeyInfo::from_str("ASIAY34FZKBOKMUTVV7A").unwrap();

        assert_eq!(info.account_id.id(), 609629065308);
        assert_eq!(info.key_type, KeyType::ASIA);
    }

    #[test]
    fn test_access_key_info_from_str_err() {
        let tests = vec![
            (
                "FOO",
                Err(AccessKeyError::InvalidLength),
            ),
            (
                "BOOP1234567890ABCDEF",
                Err(AccessKeyError::UnknownPrefix(String::from("BOOP"))),
            ),
            (
                "ASIA1234567890ABCDEF",
                Err(
                    AccessKeyError::Base32DecodeError(
                        data_encoding::DecodeError {
                            kind: data_encoding::DecodeKind::Symbol,
                            position: 0,
                        },
                    ),
                ),
            ),
        ];

        for test in tests {
            let info = AccessKeyInfo::new(test.0);

            assert_eq!(info, test.1);
        }
    }
}
