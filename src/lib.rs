mod monster;
mod room;
mod threat;
use room::Room;

use rand::Rng;
use serde::{self, Deserialize};
use std::error::Error;

#[derive(Deserialize)]
pub struct Config {
    monsters: MonsterAttributes,
    rooms: RoomAttributes,
}

#[derive(Deserialize, Clone)]
pub struct MonsterAttributes {
    sizes: Vec<String>,
    body_types: Vec<String>,
    weak_points: Vec<String>,
    behaviors: Vec<String>,
    extra_features: Vec<String>,
}

#[derive(Deserialize)]
pub struct RoomAttributes {
    room_types: Vec<String>,
    descriptors: Vec<String>,
    threat_counts: Vec<u8>,
    door_counts: Vec<u8>,
    sizes: Vec<String>,
    set_pieces: Vec<String>,
    hazards: Vec<String>,
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
    let my_room = Room::build(config)?;
    println!("{}", my_room);
    Ok(())
}
