// Key types
use crate::error::AccessKeyError;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum KeyType {
    Abia,
    Acca,
    Agpa,
    Aida,
    Aipa,
    Akia,
    Anpa,
    Anva,
    Apka,
    Aroa,
    Asca,
    Asia,
}

impl KeyType {
    pub fn description(&self) -> &str {
        match self {
            Self::Abia => "ABIA (AWS STS service bearer token)",
            Self::Acca => "ACCA (Context-specific credential)",
            Self::Agpa => "AGPA (Group)",
            Self::Aida => "AIDA (IAM user)",
            Self::Aipa => "AIPA (Amazon EC2 instance profile)",
            Self::Akia => "AKIA (Access key)",
            Self::Anpa => "ANPA (Managed policy)",
            Self::Anva => "ANVA (Version in a managed policy)",
            Self::Apka => "APKA (Public key)",
            Self::Aroa => "AROA (Role)",
            Self::Asca => "ASCA (Certificate)",
            Self::Asia => "ASIA (Temporary (AWS STS) keys)",
        }
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Self::Abia => "ABIA",
            Self::Acca => "ACCA",
            Self::Agpa => "AGPA",
            Self::Aida => "AIDA",
            Self::Aipa => "AIPA",
            Self::Akia => "AKIA",
            Self::Anpa => "ANPA",
            Self::Anva => "ANVA",
            Self::Apka => "APKA",
            Self::Aroa => "AROA",
            Self::Asca => "ASCA",
            Self::Asia => "ASIA",
        };

        write!(f, "{output}")
    }
}

impl FromStr for KeyType {
    type Err = AccessKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key_type = match &s[..4] {
            "ABIA"  => Self::Abia,
            "ACCA"  => Self::Acca,
            "AGPA"  => Self::Agpa,
            "AIDA"  => Self::Aida,
            "AIPA"  => Self::Aipa,
            "AKIA"  => Self::Akia,
            "ANPA"  => Self::Anpa,
            "ANVA"  => Self::Anva,
            "APKA"  => Self::Apka,
            "AROA"  => Self::Aroa,
            "ASCA"  => Self::Asca,
            "ASIA"  => Self::Asia,
            unknown => Err(AccessKeyError::UnknownPrefix(unknown.to_string()))?,
        };

        Ok(key_type)
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
                Ok(KeyType::Asia),
            ),
            (
                "KEKW1234567890ABCDEF",
                Err(AccessKeyError::UnknownPrefix(String::from("KEKW"))),
            ),
        ];

        for test in tests {
            let key_type = KeyType::from_str(test.0);

            assert_eq!(key_type, test.1);
        }
    }
}
