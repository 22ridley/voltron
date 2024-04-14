use std::fs;
use std::io::{Error, ErrorKind, Read};
use toml;

#[derive(Debug, Clone)]
pub struct Config {
    // User for the mySQL database
    pub db_user: String,
    // Password for the mySQL database
    pub db_password: String,
    // Directory for page templates
    pub template_dir: String,
    pub prime: bool,
}

pub(crate) fn parse(path: &str) -> Result<Config, Error> {
    let mut file = fs::File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let value = match toml::Parser::new(&buffer).parse() {
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Failed to parse the config file!",
            ))
        }
        Some(v) => v,
    };

    Ok(Config {
        db_user: value.get("db_user").unwrap().as_str().unwrap().into(),
        db_password: value.get("db_password").unwrap().as_str().unwrap().into(),
        template_dir: value.get("template_dir").unwrap().as_str().unwrap().into(),
        prime: value.get("prime").unwrap().as_bool().unwrap().into(),
    })
}
