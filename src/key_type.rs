// Key types
use crate::error::AccessKeyError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum KeyType {
    ABIA,
    ACCA,
    AGPA,
    AIDA,
    AIPA,
    AKIA,
    ANPA,
    ANVA,
    APKA,
    AROA,
    ASCA,
    ASIA,
}

impl KeyType {
    #[must_use]
    pub fn description(&self) -> &str {
        match self {
            Self::ABIA => "ABIA (AWS STS service bearer token)",
            Self::ACCA => "ACCA (Context-specific credential)",
            Self::AGPA => "AGPA (Group)",
            Self::AIDA => "AIDA (IAM user)",
            Self::AIPA => "AIPA (Amazon EC2 instance profile)",
            Self::AKIA => "AKIA (Access key)",
            Self::ANPA => "ANPA (Managed policy)",
            Self::ANVA => "ANVA (Version in a managed policy)",
            Self::APKA => "APKA (Public key)",
            Self::AROA => "AROA (Role)",
            Self::ASCA => "ASCA (Certificate)",
            Self::ASIA => "ASIA (Temporary (AWS STS) keys)",
        }
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Self::ABIA => "ABIA",
            Self::ACCA => "ACCA",
            Self::AGPA => "AGPA",
            Self::AIDA => "AIDA",
            Self::AIPA => "AIPA",
            Self::AKIA => "AKIA",
            Self::ANPA => "ANPA",
            Self::ANVA => "ANVA",
            Self::APKA => "APKA",
            Self::AROA => "AROA",
            Self::ASCA => "ASCA",
            Self::ASIA => "ASIA",
        };

        write!(f, "{output}")
    }
}

impl FromStr for KeyType {
    type Err = AccessKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..4] {
            "ABIA"  => Ok(Self::ABIA),
            "ACCA"  => Ok(Self::ACCA),
            "AGPA"  => Ok(Self::AGPA),
            "AIDA"  => Ok(Self::AIDA),
            "AIPA"  => Ok(Self::AIPA),
            "AKIA"  => Ok(Self::AKIA),
            "ANPA"  => Ok(Self::ANPA),
            "ANVA"  => Ok(Self::ANVA),
            "APKA"  => Ok(Self::APKA),
            "AROA"  => Ok(Self::AROA),
            "ASCA"  => Ok(Self::ASCA),
            "ASIA"  => Ok(Self::ASIA),
            unknown => Err(AccessKeyError::UnknownPrefix(unknown.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_type_from_str() {
        let tests = vec![
            (
                "ASIA1234567890ABCDEF",
                Ok(KeyType::ASIA),
            ),
            (
                "WHAT1234567890ABCDEF",
                Err(AccessKeyError::UnknownPrefix(String::from("WHAT"))),
            ),
        ];

        for test in tests {
            let key_type = KeyType::from_str(test.0);

            assert_eq!(key_type, test.1);
        }
    }
}
