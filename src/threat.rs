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
        let mut rng = thread_rng();
        let threat = match threat_type_roll {
            0..=1 => Threat::Monster(Monster::build(&config.monsters)?),
            2..=3 => Threat::Hazard(
                config
                    .rooms
                    .hazards
                    .choose(&mut rng)
                    .ok_or("No hazards found!")?
                    .to_string(),
            ),
            2..=6 => Threat::build_person(&config.people)?,
            _ => panic!("d6 rolled higher than a 6!"),
        };
        Ok(threat)
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
        let threat = Threat::Person(format!("{} {}", first_name, last_name));
        Ok(threat)
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
