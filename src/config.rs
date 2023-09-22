use serde::{self,Deserialize};
use std::error::Error;

#[derive(Deserialize)]
pub struct Config {
    pub monsters: MonsterAttributes,
}

#[derive(Deserialize)]
pub struct MonsterAttributes {
    pub sizes: Vec<String>,
    pub body_types: Vec<String>,
    pub weak_points: Vec<String>,
    pub behaviors: Vec<String>,
    pub extra_features: Vec<String>,
}

pub fn build(str: &str) -> Result<Config, Box<dyn Error>> {
    let config: Config = toml::from_str(str)?;
    Ok(config)
}
