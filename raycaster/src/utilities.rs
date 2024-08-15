use serde::de::DeserializeOwned;
use serde_json::Error;
use std::fs::File;
use std::io::BufReader;

pub fn load_json<T: DeserializeOwned>(file_path: &str) -> Result<T, Error> {
    let file = File::open(file_path).map_err(serde_json::Error::io)?;
    let reader = BufReader::new(file);
    let data: T = serde_json::from_reader(reader)?;

    Ok(data)
}