//
use data_encoding::BASE32;

const ACCESS_KEY_ID: &str = "ASIAY34FZKBOKMUTVV7A";
const MASK: u64 = 0x7fff_ffff_ff80;

fn account_id_from_access_key_id(access_key_id: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let decoded = BASE32.decode(access_key_id[4..].as_bytes())?;
    let mut buf = [0; 8];

    buf[2..].copy_from_slice(&decoded[0..6]);

    Ok((u64::from_be_bytes(buf) & MASK) >> 7)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account_id = account_id_from_access_key_id(ACCESS_KEY_ID)?;

    println!("Account ID: {account_id:012}");

    Ok(())
}
