use crate::{
    data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader},
    net::NearbyInfo,
    world::{WarpAgreeType, WarpAnimation},
};

const REQUEST_SIZE: usize = 1;

#[derive(Debug, Default)]
pub struct Agree {
    pub warp_type: WarpAgreeType,
    pub nearby_info: NearbyInfo,
}

impl Agree {
    pub fn local(nearby_info: NearbyInfo) -> Self {
        Self {
            warp_type: WarpAgreeType::Local,
            nearby_info,
        }
    }

    pub fn remote(
        map_id: EOShort,
        warp_animation: Option<WarpAnimation>,
        nearby_info: NearbyInfo,
    ) -> Self {
        Self {
            warp_type: WarpAgreeType::Remote {
                map_id,
                warp_animation,
            },
            nearby_info,
        }
    }
}

impl Serializeable for Agree {
    fn deserialize(&mut self, reader: &StreamReader) {
        let warp_type_char = reader.get_char();
        match warp_type_char {
            1 => {
                self.warp_type = WarpAgreeType::Local;
            }
            2 => {
                self.warp_type = {
                    let map_id = reader.get_short();
                    let warp_animation_char = reader.get_char();
                    WarpAgreeType::Remote {
                        map_id,
                        warp_animation: WarpAnimation::from_char(warp_animation_char),
                    }
                };
            }
            _ => panic!("Invalid warp type: {}", warp_type_char),
        }
        self.nearby_info.deserialize(reader);
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            REQUEST_SIZE
                + if self.warp_type == WarpAgreeType::Local {
                    0
                } else {
                    3
                }
                + self.nearby_info.get_size(),
        );
        builder.add_char(if self.warp_type == WarpAgreeType::Local {
            1
        } else {
            2
        });
        if let WarpAgreeType::Remote {
            map_id,
            warp_animation,
        } = self.warp_type
        {
            builder.add_short(map_id);
            if let Some(warp_animation) = warp_animation {
                builder.add_char(warp_animation as EOChar);
            } else {
                builder.add_char(0);
            }
        }
        builder.append(&mut self.nearby_info.serialize());
        builder.get()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_local() {
        let buf = vec![
            2, 2, 255, 103, 111, 114, 111, 110, 255, 72, 2, 15, 254, 6, 254, 2, 254, 2, 7, 32, 32,
            32, 11, 2, 12, 4, 1, 223, 2, 223, 2, 55, 3, 55, 3, 43, 254, 1, 254, 1, 254, 1, 254, 25,
            254, 1, 254, 1, 254, 1, 254, 64, 254, 1, 1, 255, 2, 26, 254, 18, 3, 4, 3, 29, 254, 6,
            2, 3, 255,
        ];
        let reader = StreamReader::new(&buf);
        let mut agree = Agree::default();
        agree.deserialize(&reader);
        assert_eq!(agree.warp_type, WarpAgreeType::Local);
        assert_eq!(agree.nearby_info.characters.len(), 1);
    }

    #[test]
    fn serialize_local() {
        let buf = vec![
            2, 2, 255, 103, 111, 114, 111, 110, 255, 72, 2, 15, 254, 6, 254, 2, 254, 2, 7, 32, 32,
            32, 11, 2, 12, 4, 1, 223, 2, 223, 2, 55, 3, 55, 3, 43, 254, 1, 254, 1, 254, 1, 254, 25,
            254, 1, 254, 1, 254, 1, 254, 64, 254, 1, 1, 255, 2, 26, 254, 18, 3, 4, 3, 29, 254, 6,
            2, 3, 255,
        ];
        let reader = StreamReader::new(&buf);
        let mut agree = Agree::default();
        agree.deserialize(&reader);
        assert_eq!(agree.serialize(), buf);
    }

    #[test]
    fn deserialize_remote() {
        let buf = vec![
            3, 15, 254, 1, 2, 255, 103, 111, 114, 111, 110, 255, 72, 2, 15, 254, 6, 254, 2, 254, 2,
            7, 32, 32, 32, 11, 2, 12, 4, 1, 223, 2, 223, 2, 55, 3, 55, 3, 43, 254, 1, 254, 1, 254,
            1, 254, 25, 254, 1, 254, 1, 254, 1, 254, 64, 254, 1, 1, 255, 2, 26, 254, 18, 3, 4, 3,
            29, 254, 6, 2, 3, 255,
        ];
        let reader = StreamReader::new(&buf);
        let mut agree = Agree::default();
        agree.deserialize(&reader);
        assert_eq!(
            agree.warp_type,
            WarpAgreeType::Remote {
                map_id: 14,
                warp_animation: None
            }
        );
        assert_eq!(agree.nearby_info.characters.len(), 1);
    }

    #[test]
    fn serialize_remote() {
        let buf = vec![
            3, 15, 254, 1, 2, 255, 103, 111, 114, 111, 110, 255, 72, 2, 15, 254, 6, 254, 2, 254, 2,
            7, 32, 32, 32, 11, 2, 12, 4, 1, 223, 2, 223, 2, 55, 3, 55, 3, 43, 254, 1, 254, 1, 254,
            1, 254, 25, 254, 1, 254, 1, 254, 1, 254, 64, 254, 1, 1, 255, 2, 26, 254, 18, 3, 4, 3,
            29, 254, 6, 2, 3, 255,
        ];
        let reader = StreamReader::new(&buf);
        let mut agree = Agree::default();
        agree.deserialize(&reader);
        assert_eq!(agree.serialize(), buf);
    }
}
