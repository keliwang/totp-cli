use std::io::{BufReader, Read};
use std::fs::File;
use oath::{totp_raw_now, HashType};
use serde::Deserialize;

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
    let key = base32::decode(base32::Alphabet::RFC4648 { padding: false }, &config.secret).unwrap();
    let totp_token = totp_raw_now(&key, config.digits, 0, config.period, &HashType::SHA1);
    print!("{}{}", config.pin, totp_token);

    Ok(())
}
