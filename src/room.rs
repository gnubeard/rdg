use crate::*;
use rand::{seq::SliceRandom, thread_rng};
use std::error::Error;
use std::fmt;
use threat::Threat;

#[derive(Debug, PartialEq)]
pub struct Room {
    room_type: String,
    descriptor: String,
    threats: Vec<Threat>,
    door_count: u8,
    size: String,
    set_piece: Option<String>,
}

impl Room {
    pub fn build(config: &Config) -> Result<Room, Box<dyn Error>> {
        let attrs = &config.rooms;
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
        let threat_count = roll_d6() / 2;
        let door_count = roll_d6();
        let size = attrs
            .sizes
            .choose(&mut rng)
            .ok_or("No sizes found!")?
            .to_string();
        let set_piece_roll = roll_d6() + roll_d6();
        let set_piece_happening = set_piece_roll > 10;
        let set_piece = match set_piece_happening {
            false => None,
            true => {
                let set_piece = attrs
                    .set_pieces
                    .choose(&mut rng)
                    .ok_or("No set pieces found!")?
                    .to_string();
                Some(set_piece)
            }
        };
        let mut threats = Vec::new();
        let threat_type_roll = roll_d6();
        for _ in 0..threat_count {
            let my_threat = Threat::roll(&config, threat_type_roll)?;
            threats.push(my_threat);
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
        match &self.set_piece {
            Some(s) => write!(f, "SET PIECE: {}\n", s)?,
            None => (),
        }
        write!(f, "ROOM TYPE: {}\n", self.room_type)?;
        write!(f, "DESCRIPTOR: {}\n", self.descriptor)?;
        write!(f, "DOOR COUNT: {}\n", self.door_count)?;
        write!(f, "SIZE: {}\n", self.size)?;
        for (i, threat) in self.threats.iter().enumerate() {
            write!(f, "THREAT {}:\n{}\n", i + 1, threat)?;
        }
        fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn room_from_config() {
        let room_attrs = RoomAttributes {
            room_types: vec!["Basic".to_string()],
            descriptors: vec!["Blue".to_string()],
            sizes: vec!["Large".to_string()],
            set_pieces: vec!["Skeletons".to_string()],
            hazards: vec!["Chemical Spill".to_string()],
        };
        let monster_attrs = MonsterAttributes {
            sizes: vec!["X".to_string()],
            body_types: vec!["Y".to_string()],
            weak_points: vec!["Z".to_string()],
            behaviors: vec!["1".to_string()],
            extra_features: vec!["2".to_string()],
        };
        let people_attrs = PeopleAttributes {
            first_names: vec!["Dave".to_string()],
            last_names: vec!["Davidson".to_string()],
        };
        let robot_attrs = RobotAttributes {
            designations: vec!["R2-D2".to_string()],
        };
        let config = Config {
            monsters: monster_attrs,
            rooms: room_attrs,
            people: people_attrs,
            robots: robot_attrs,
        };
        let new_room = Room::build(&config).unwrap();
        // holes in testing, for now
        let door_count = new_room.door_count.clone();
        let threats = new_room.threats.clone();
        let set_piece = new_room.set_piece.clone();
        let example_room = Room {
            room_type: "Basic".to_string(),
            descriptor: "Blue".to_string(),
            threats,
            door_count,
            size: "Large".to_string(),
            set_piece,
        };
        assert_eq!(example_room, new_room);
    }
}
