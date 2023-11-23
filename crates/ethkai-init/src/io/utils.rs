use ethers_rs_types::{init::{types::{WhichAPIProvider}}, error::{RED_ERROR_STRING}};
use anyhow::{anyhow, Ok, Result};
use dotenv;
use ethers::abi::Hash;
use secrecy::SecretString;
use std::collections::HashMap;
use std::convert::AsRef;
use std::fs;
use std::path::Path;

/// Get the API Secret Key stored in a `.secrets`. file for an API Provider.
pub fn init_api_credentials(which_provider: &WhichAPIProvider) -> Result<SecretString> {
    dotenv::from_filename(".secrets").ok();

    match *which_provider {
        WhichAPIProvider::MORALIS => get_api_secret_from_file(None, "MORALIS_JWT"),
        WhichAPIProvider::INFURA => get_api_secret_from_file(None, "INFURA_API_SECRET_KEY"),
        WhichAPIProvider::QUICKNODE => get_api_secret_from_file(None, "QUICKNODE_API_SECRET_KEY"),
        WhichAPIProvider::ALCHEMY => get_api_secret_from_file(None, "ALCHEMY_API_SECRET_KEY"),
        _ => unreachable!(),
    }
}

fn get_api_secret_from_file(
    file_path: Option<&str>,
    key: impl AsRef<str> + std::fmt::Debug,
) -> Result<SecretString> {
    let file_path = file_path.unwrap_or(".secrets");
    let hm: HashMap<String, String> = parse_file_delim_eq::<&str>(file_path.as_ref())?;
    let api_secret = hm
        .get(&key.as_ref().to_string())
        .ok_or(anyhow!("Missing value for key = `{:?}`", key))?;
    Ok(SecretString::new(api_secret.clone()))
}

/// Parse file that stores `key=value` pairs delimited by `=`
pub fn parse_file_delim_eq<T: AsRef<Path>>(file_path: T) -> Result<HashMap<String, String>> {
    let contents = fs::read_to_string(file_path.as_ref())
        .map_err(|err| anyhow!("{}Failed to read the file, {err}", *RED_ERROR_STRING))?;

    let mut result: HashMap<String, String> = HashMap::new();

    for line in contents.lines() {
        if let Some(index) = line.find('=') {
            let (key, value) = line.split_at(index);
            let key = key.trim();
            let value = value.trim_start_matches('=').trim();
            result.insert(key.to_string(), value.to_string());
        }
    }

    Ok(result)
}
