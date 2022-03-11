use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader},
    world::WarpAnimation,
};

const REMOVE_SIZE: usize = 2;

#[derive(Debug, Default)]
pub struct Remove {
    pub player_id: EOShort,
    pub warp_animation: Option<WarpAnimation>,
}

impl Remove {
    pub fn new(player_id: EOShort) -> Self {
        Self {
            player_id,
            warp_animation: None,
        }
    }
    pub fn with_animation(player_id: EOShort, warp_animation: WarpAnimation) -> Self {
        Self {
            player_id,
            warp_animation: Some(warp_animation),
        }
    }
}

impl Serializeable for Remove {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_id = reader.get_short();
        if reader.remaining() > 0 {
            let warp_animation_char = reader.get_char();
            self.warp_animation = match WarpAnimation::from_u8(warp_animation_char) {
                Some(warp_animation) => Some(warp_animation),
                None => panic!("Invalid warp animation: {}", warp_animation_char),
            };
        } else {
            self.warp_animation = None;
        }
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            REMOVE_SIZE + if self.warp_animation.is_some() { 1 } else { 0 },
        );
        builder.add_short(self.player_id);
        if self.warp_animation.is_some() {
            builder.add_char(self.warp_animation.unwrap() as EOChar);
        }
        builder.get()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize() {
        let buf: Vec<EOByte> = vec![2, 254];
        let reader = StreamReader::new(&buf);
        let mut remove = Remove::default();
        remove.deserialize(&reader);
        assert_eq!(remove.player_id, 1);
        assert!(remove.warp_animation.is_none());

        let buf: Vec<EOByte> = vec![2, 254, 2];
        let reader = StreamReader::new(&buf);
        remove.deserialize(&reader);
        assert_eq!(remove.player_id, 1);
        assert_eq!(remove.warp_animation, Some(WarpAnimation::Scroll));
    }
    #[test]
    fn serialize() {
        let remove = Remove::new(1);
        assert_eq!(remove.serialize(), [2, 254]);
        let remove = Remove::with_animation(1, WarpAnimation::Admin);
        assert_eq!(remove.serialize(), [2, 254, 3]);
    }
}
