use crate::{MonsterAttributes, roll_d6};
use rand::{seq::SliceRandom, thread_rng};
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Monster {
    id: u8,
    hp: u8,
    size: String,
    body_type: String,
    weak_point: String,
    behavior: String,
    extra_feature: String,
}

impl Monster {
    pub fn build(attrs: &MonsterAttributes) -> Result<Monster, Box<dyn Error>> {
        let sizes = &attrs.sizes;
        let body_types = &attrs.body_types;
        let weak_points = &attrs.weak_points;
        let behaviors = &attrs.behaviors;
        let extra_features = &attrs.extra_features;
        let mut rng = thread_rng();
        let id = roll_d6() + roll_d6() + 2;
        let hp = roll_d6();
        let size = sizes
            .choose(&mut rng)
            .ok_or("No sizes found!")?
            .to_string();
        let body_type = body_types
            .choose(&mut rng)
            .ok_or("No body types found!")?
            .to_string();
        let weak_point = weak_points
            .choose(&mut rng)
            .ok_or("No weak points found!")?
            .to_string();
        let behavior = behaviors
            .choose(&mut rng)
            .ok_or("No behaviors found!")?
            .to_string();
        let extra_feature = extra_features
            .choose(&mut rng)
            .ok_or("No extra features found!")?
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

impl fmt::Display for Monster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  ID: {} HP: {}\n", self.id, self.hp)?;
        write!(f, "  Size: {}\n", self.size)?;
        write!(f, "  Body Type: {}\n", self.body_type)?;
        write!(f, "  Weak Point: {}\n", self.weak_point)?;
        write!(f, "  Behavior: {}\n", self.behavior)?;
        write!(f, "  Extra Feature: {}", self.extra_feature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn monster_from_attrs() {
        let attrs = MonsterAttributes {
            sizes: vec!["X".to_string()],
            body_types: vec!["Y".to_string()],
            weak_points: vec!["Z".to_string()],
            behaviors: vec!["1".to_string()],
            extra_features: vec!["2".to_string()],
        };
        let new_monster = Monster::build(&attrs).unwrap();
        let example_monster = Monster {
            id: new_monster.id,
            hp: new_monster.hp,
            size: "X".to_string(),
            body_type: "Y".to_string(),
            weak_point: "Z".to_string(),
            behavior: "1".to_string(),
            extra_feature: "2".to_string(),
        };
        assert_eq!(example_monster, new_monster);
    }
}
