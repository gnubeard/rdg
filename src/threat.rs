use crate::*;
use monster::Monster;
use rand::{seq::SliceRandom, thread_rng};
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Threat {
    Monster(Monster),
    Hazard(String),
    Person(String),
    Robot(String),
}

impl Threat {
    pub fn roll(config: &Config, threat_type_roll: u8) -> Result<Self, Box<dyn Error>> {
        let threat = match threat_type_roll {
            0..=1 => Threat::Monster(Monster::build(&config.monsters)?),
            2..=3 => Threat::build_person(&config.people)?,
            4..=5 => Threat::build_robot(&config.people)?,
            6..=6 => Threat::build_hazard(&config.rooms)?,
            _ => panic!("d6 rolled higher than a 6!"),
        };
        Ok(threat)
    }
    fn build_hazard(room_attrs: &RoomAttributes) -> Result<Self, Box<dyn Error>> {
        let mut rng = thread_rng();
        let h = room_attrs
            .hazards
            .choose(&mut rng)
            .ok_or("No hazards found!")?
            .to_string();
        let hazard = Threat::Hazard(h);
        Ok(hazard)
    }
    fn build_person(people_attrs: &PeopleAttributes) -> Result<Self, Box<dyn Error>> {
        let mut rng = thread_rng();
        let first_name = people_attrs
            .first_names
            .choose(&mut rng)
            .ok_or("No first names found!")?
            .to_string();
        let last_name = people_attrs
            .last_names
            .choose(&mut rng)
            .ok_or("No last names found!")?
            .to_string();
        let person = Threat::Person(format!("{} {}", first_name, last_name));
        Ok(person)
    }
    fn build_robot(robot_attrs: &PeopleAttributes) -> Result<Self, Box<dyn Error>> {
        let mut rng = thread_rng();
        let prefix = robot_attrs
            .robot_prefixes
            .choose(&mut rng)
            .ok_or("No prefixes found!")?
            .to_string();
        let suffix = robot_attrs
            .robot_suffixes
            .choose(&mut rng)
            .ok_or("No suffixes found!")?
            .to_string();
        let person = Threat::Robot(format!("{}-{}", prefix, suffix));
        Ok(person)
    }
}

impl fmt::Display for Threat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Threat::Monster(m) => write!(f, "  MONSTER:\n{}", m),
            Threat::Hazard(h) => write!(f, "  HAZARD: {}", h),
            Threat::Person(p) => write!(f, "  PERSON: {}", p),
            Threat::Robot(p) => write!(f, "  ROBOT: {}", p),
        }
    }
}
