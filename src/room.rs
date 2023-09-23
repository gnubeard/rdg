use crate::*;
use monster::Monster;
use rand::{seq::SliceRandom, thread_rng};
use std::error::Error;
use std::fmt;

//struct Threat

//pub enum Threat {
//    Hazard(Hazard),
//    Robot(String, u8),
//    Person(String),
//    Monster(Monster),
//}

#[derive(Debug, PartialEq)]
pub struct Room {
    room_type: String,
    descriptor: String,
    threats: Vec<Monster>, // TODO: update to Vec<Threat>
    door_count: u8,
    size: String,
    set_piece: String,
}

impl Room {
    pub fn build(config: Config) -> Result<Room, Box<dyn Error>> {
        let attrs: RoomAttributes = config.rooms;
        let mut rng = thread_rng();
        let room_type = attrs
            .room_types
            .choose(&mut rng)
            .ok_or("No room types found!")?
            .to_string();
        let descriptor = attrs
            .descriptors
            .choose(&mut rng)
            .ok_or("No descriptors found!")?
            .to_string();
        let threat_count = attrs
            .threat_counts
            .choose(&mut rng)
            .ok_or("No threat counts found!")?
            .clone();
        let door_count = attrs
            .door_counts
            .choose(&mut rng)
            .ok_or("No door counts found!")?
            .clone();
        let size = attrs
            .sizes
            .choose(&mut rng)
            .ok_or("No sizes found!")?
            .to_string();
        let set_piece = attrs
            .set_pieces
            .choose(&mut rng)
            .ok_or("No set pieces found!")?
            .to_string();
        let mut threats = Vec::new();
        // TODO let threat_type = <Roll against Threat table>;
        for _ in 0..threat_count {
            let my_monster = Monster::build(&config.monsters)?;
            threats.push(my_monster);
        }
        Ok(Room {
            room_type,
            descriptor,
            threats,
            door_count,
            size,
            set_piece,
        })
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Room Type: {}\n", self.room_type)?;
        write!(f, "Descriptor: {}\n", self.descriptor)?;
        for (i, threat) in self.threats.iter().enumerate() {
            write!(f, "Threat {}:\n{}\n", i + 1, threat)?;
        }
        write!(f, "Door Count: {}\n", self.door_count)?;
        write!(f, "Size: {}\n", self.size)?;
        write!(f, "Set piece: {}", self.set_piece)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn room_from_config() {
        let room_attrs = RoomAttributes {
            room_types: vec!["X".to_string()],
            descriptors: vec!["Y".to_string()],
            threat_counts: vec![2],
            door_counts: vec![3],
            sizes: vec!["3".to_string()],
            set_pieces: vec!["4".to_string()],
        };
        let monster_attrs = MonsterAttributes {
            sizes: vec!["X".to_string()],
            body_types: vec!["Y".to_string()],
            weak_points: vec!["Z".to_string()],
            behaviors: vec!["1".to_string()],
            extra_features: vec!["2".to_string()],
        };
        let config = Config {
            monsters: monster_attrs,
            rooms: room_attrs,
        };
        let new_room = Room::build(config).unwrap();
        // a hole in testing, for now
        let threats = new_room.threats.clone();
        let example_room = Room {
            room_type: "X".to_string(),
            descriptor: "Y".to_string(),
            threats, // TODO: threat enum
            door_count: 3,
            size: "3".to_string(),
            set_piece: "4".to_string(),
        };
        assert_eq!(example_room, new_room);
    }
}
