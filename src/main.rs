use google_authenticator::GoogleAuthenticator;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug, Deserialize)]
struct Config {
    secret: String,
    period: u64,
    digits: u32,
    pin: String,
}

fn main() -> std::io::Result<()> {
    let conf_file = File::open(dirs::home_dir().unwrap().join(".config/totp.toml"))?;
    let mut reader = BufReader::new(conf_file);

    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    let config: Config = toml::from_str(&data)?;
    let auth = GoogleAuthenticator::new();
    let code = auth.get_code(&config.secret, 0).unwrap();
    print!("{}{}", config.pin, code);

    Ok(())
}
