//
use aws_access_key_info::{
    AccessKeyError,
    AccessKeyInfo,
};
use std::str::FromStr;

const ACCESS_KEY_ID: &str = "ASIAY34FZKBOKMUTVV7A";

fn main() -> Result<(), AccessKeyError> {
    let access_key_info = AccessKeyInfo::from_str(ACCESS_KEY_ID)?;

    println!("{access_key_info}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
