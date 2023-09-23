use crate::monster::Monster;

use std::fmt;
#[derive(Debug, PartialEq, Clone)]
pub enum Threat {
    Monster(Monster),
    Hazard(String),
    //    Robot(String, u8),
    //    Person(String),
}

impl fmt::Display for Threat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Threat::Monster(m) => write!(f, "  MONSTER:\n{}", m),
            Threat::Hazard(h) => write!(f, "  HAZARD: {}", h),
        }
    }
}
