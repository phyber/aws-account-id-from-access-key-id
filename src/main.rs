//
use aws_access_key_info::{
    AccessKeyError,
    AccessKeyInfo,
};
use std::env;
use std::path::Path;
use std::str::FromStr;

fn usage(name: &str) {
    let path = Path::new(name);

    let filename = match path.file_name() {
        Some(path) => path.to_str().expect("lossy string"),
        None => "?",
    };

    println!("Usage: {filename} <account id>");
}

fn main() -> Result<(), AccessKeyError> {
    let args: Vec<String> = env::args().collect();

    let account_id = if args.len() > 1 {
        &args[1]
    }
    else {
        usage(&args[0]);
        ::std::process::exit(1);
    };

    let access_key_info = match AccessKeyInfo::from_str(account_id) {
        Ok(access_key_info) => access_key_info,
        Err(err) => {
            eprintln!("{err}");
            ::std::process::exit(1);
        },
    };

    println!("{access_key_info}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
