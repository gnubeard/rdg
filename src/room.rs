use crate::RoomAttributes;
use rand::{seq::SliceRandom, thread_rng};
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Room {
    room_type: String,
    descriptor: String,
    threat_count: String,
    threat_type: String,
    door_count: String,
    size: String,
    set_piece: String,
}

impl Room {
    pub fn build(attrs: RoomAttributes) -> Result<Room, Box<dyn Error>> {
        let room_types = attrs.room_types;
        let descriptors = attrs.descriptors;
        let threat_counts = attrs.threat_counts;
        let threat_types = attrs.threat_types;
        let door_counts = attrs.door_counts;
        let sizes = attrs.sizes;
        let set_pieces = attrs.set_pieces;
        let mut rng = thread_rng();
        let room_type = room_types
            .choose(&mut rng)
            .ok_or("No room types found!")?
            .to_string();
        let descriptor = descriptors
            .choose(&mut rng)
            .ok_or("No descriptors found!")?
            .to_string();
        let threat_count = threat_counts
            .choose(&mut rng)
            .ok_or("No threat counts found!")?
            .to_string();
        let threat_type = threat_types
            .choose(&mut rng)
            .ok_or("No threat types found!")?
            .to_string();
        let door_count = door_counts
            .choose(&mut rng)
            .ok_or("No door counts found!")?
            .to_string();
        let size = sizes.choose(&mut rng).ok_or("No sizes found!")?.to_string();
        let set_piece = set_pieces
            .choose(&mut rng)
            .ok_or("No set pieces found!")?
            .to_string();
        Ok(Room {
            room_type,
            descriptor,
            threat_count,
            threat_type,
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
        write!(f, "Threat Count: {}\n", self.threat_count)?;
        write!(f, "Threat Type: {}\n", self.threat_type)?;
        write!(f, "Door Count: {}\n", self.door_count)?;
        write!(f, "Size: {}\n", self.size)?;
        write!(f, "Set piece: {}\n", self.set_piece)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn room_from_attrs() {
        let attrs = RoomAttributes {
            room_types: vec!["X".to_string()],
            descriptors: vec!["Y".to_string()],
            threat_counts: vec!["Z".to_string()],
            threat_types: vec!["1".to_string()],
            door_counts: vec!["2".to_string()],
            sizes: vec!["3".to_string()],
            set_pieces: vec!["4".to_string()],
        };
        let new_room = Room::build(attrs).unwrap();
        let example_room = Room {
            room_type: "X".to_string(),
            descriptor: "Y".to_string(),
            threat_count: "Z".to_string(),
            threat_type: "1".to_string(),
            door_count: "2".to_string(),
            size: "3".to_string(),
            set_piece: "4".to_string(),
        };
        assert_eq!(example_room, new_room);
    }
}
