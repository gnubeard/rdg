mod monster;
use monster::Monster;

mod room;
use room::Room;

use serde::{self, Deserialize};
use std::error::Error;
use rand::Rng;

#[derive(Deserialize)]
pub struct Config {
    pub monsters: MonsterAttributes,
    pub rooms: RoomAttributes,
}

#[derive(Deserialize)]
pub struct MonsterAttributes {
    pub sizes: Vec<String>,
    pub body_types: Vec<String>,
    pub weak_points: Vec<String>,
    pub behaviors: Vec<String>,
    pub extra_features: Vec<String>,
}

#[derive(Deserialize)]
pub struct RoomAttributes {
    pub room_types: Vec<String>,
    pub descriptors: Vec<String>,
    pub threat_counts: Vec<String>,
    pub threat_types: Vec<String>,
    pub door_counts: Vec<String>,
    pub sizes: Vec<String>,
    pub set_pieces: Vec<String>,
}

impl Config {
    pub fn build(str: &str) -> Result<Self, Box<dyn Error>> {
        let config: Self = toml::from_str(str)?;
        Ok(config)
    }
}

pub fn roll_d6() -> u8 {
    rand::thread_rng().gen_range(0..6)
}

pub fn run(s: &str) -> Result<(), Box<dyn Error>> {
    let config = Config::build(s)?;
    let my_monster = Monster::build(config.monsters)?;
    let my_room = Room::build(config.rooms)?;
    println!("{}", my_monster);
    println!("{}", my_room);
    Ok(())
}
