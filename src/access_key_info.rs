// Access Key Info
use crate::account_id::AccountId;
use crate::error::AccessKeyError;
use crate::key_type::KeyType;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct AccessKeyInfo {
    account_id: AccountId,
    key_type: KeyType,
}

impl fmt::Display for AccessKeyInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AccountID: {}, Key Desc: {}",
            self.account_id,
            self.key_type.description(),
        )
    }
}

impl FromStr for AccessKeyInfo {
    type Err = AccessKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 20 {
            return Err(AccessKeyError::InvalidLength);
        }

        let account_id = AccountId::from_str(s)?;
        let key_type = KeyType::from_str(s)?;

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
}
