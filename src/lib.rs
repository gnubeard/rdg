mod monster;
use monster::Monster;

use serde::{self, Deserialize};
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

impl Config {
    pub fn build(str: &str) -> Result<Self, Box<dyn Error>> {
        let config: Self = toml::from_str(str)?;
        Ok(config)
    }
}

pub fn run(s: &str) -> Result<(), Box<dyn Error>> {
    let config = Config::build(s)?;
    let my_monster = Monster::build(config.monsters)?;
    println!("{}", my_monster);
    Ok(())
}
