use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{self,Deserialize};
use std::error::Error;
use std::fmt;

pub struct Monster {
    id: u8,
    hp: u8,
    size: String,
    body_type: String,
    weak_point: String,
    behavior: String,
    extra_feature: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub monsters: MonsterAttributes,
}

#[derive(Deserialize)]
pub struct MonsterAttributes {
    sizes: Vec<String>,
    body_types: Vec<String>,
    weak_points: Vec<String>,
    behaviors: Vec<String>,
    extra_features: Vec<String>,
}

impl Monster {
    pub fn build(config: MonsterAttributes) -> Result<Monster, Box<dyn Error>> {
        let sizes = config.sizes;
        let body_types = config.body_types;
        let weak_points = config.weak_points;
        let behaviors = config.behaviors;
        let extra_features = config.extra_features;
        let mut rng = thread_rng();
        let id = roll_d6() + roll_d6() + 2;
        let hp = roll_d6();
        let size = sizes
            .choose(&mut rng)
            .ok_or("No monster sizes found!")?
            .to_string();
        let body_type = body_types
            .choose(&mut rng)
            .ok_or("No monster body types found!")?
            .to_string();
        let weak_point = weak_points
            .choose(&mut rng)
            .ok_or("No monster weak points found!")?
            .to_string();
        let behavior = behaviors
            .choose(&mut rng)
            .ok_or("No monster behaviors found!")?
            .to_string();
        let extra_feature = extra_features
            .choose(&mut rng)
            .ok_or("No monster extra features found!")?
            .to_string();
        Ok(Monster {
            id,
            hp,
            size,
            body_type,
            weak_point,
            behavior,
            extra_feature,
        })
    }
}

fn roll_d6() -> u8 {
    rand::thread_rng().gen_range(0..6)
}

impl fmt::Display for Monster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ID: {} HP: {}\n", self.id, self.hp)?;
        write!(f, "Size: {}\n", self.size)?;
        write!(f, "Body Type: {}\n", self.body_type)?;
        write!(f, "Weak Point: {}\n", self.weak_point)?;
        write!(f, "Behavior: {}\n", self.behavior)?;
        write!(f, "Extra Feature: {}\n", self.extra_feature)
    }
}
