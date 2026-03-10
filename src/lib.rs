use std::{fs::File, io::{BufReader, BufWriter}};
use serde::de::DeserializeOwned;

pub fn save<T>(path: &str, data: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: serde::Serialize
{
    let file = File::create(&path)?;
    let buf_writer = BufWriter::new(file);
    bincode::serialize_into(buf_writer, &data)?;
    Ok(())
}

pub fn load<T>(path: &str) -> Result<T, Box<dyn std::error::Error>> 
where 
    T: DeserializeOwned
{
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let load_data: T = bincode::deserialize_from(buf_reader)?;
    Ok(load_data)
}
