// Account ID
use crate::error::AccessKeyError;
use data_encoding::BASE32;
use std::fmt;
use std::str::FromStr;

const MASK: u64 = 0x7fff_ffff_ff80;

#[derive(Debug, Eq, PartialEq)]
pub struct AccountId {
    id: u64,
}

impl AccountId {
    #[must_use]
    pub fn id(&self) -> u64 {
        self.id
    }
}

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:012}", self.id)
    }
}

impl FromStr for AccountId {
    type Err = AccessKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded = BASE32.decode(s[4..].as_bytes())?;
        let mut buf = [0; 8];
        buf[2..].copy_from_slice(&decoded[0..6]);
        let id = (u64::from_be_bytes(buf) & MASK) >> 7;

        let account_id = Self {
            id,
        };

        Ok(account_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
