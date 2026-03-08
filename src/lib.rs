use serde::de::DeserializeOwned;

pub fn save<T>(path: &str, data: T) -> Result<(), Box<dyn std::error::Error>>
where
    T: serde::Serialize
{
    let serialized = bincode::serialize(&data)?;
    std::fs::write(path, &serialized)?;
    Ok(())
}

pub fn load<T>(path: &str) -> Result<T, Box<dyn std::error::Error>> 
where 
    T: DeserializeOwned
{
    let data = std::fs::read(path)?;
    let load_data: T = bincode::deserialize(&data)?;
    Ok(load_data)
}
