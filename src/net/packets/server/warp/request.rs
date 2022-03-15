use crate::{
    data::{EOByte, EOShort, EOThree, Serializeable, StreamBuilder, StreamReader},
    world::WarpRequestType,
};

const REQUEST_SIZE: usize = 5;

#[derive(Debug, Default)]
pub struct Request {
    pub warp_type: WarpRequestType,
    pub map_id: EOShort,
    pub session_id: EOShort,
}

impl Request {
    pub fn local(map_id: EOShort, session_id: EOShort) -> Self {
        Self {
            warp_type: WarpRequestType::Local,
            map_id,
            session_id,
        }
    }

    pub fn remote(
        map_id: EOShort,
        session_id: EOShort,
        map_rid: [EOShort; 2],
        map_filesize: EOThree,
    ) -> Self {
        Self {
            warp_type: WarpRequestType::Remote {
                map_rid,
                map_filesize,
            },
            map_id,
            session_id,
        }
    }
}

impl Serializeable for Request {
    fn deserialize(&mut self, reader: &StreamReader) {
        let warp_type_char = reader.get_char();
        self.map_id = reader.get_short();

        match warp_type_char {
            1 => {
                self.warp_type = WarpRequestType::Local;
            }
            2 => {
                self.warp_type = WarpRequestType::Remote {
                    map_rid: [reader.get_short(), reader.get_short()],
                    map_filesize: reader.get_three(),
                };
            }
            _ => panic!("Invalid warp type: {}", warp_type_char),
        }

        self.session_id = reader.get_short();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            REQUEST_SIZE
                + if self.warp_type == WarpRequestType::Local {
                    0
                } else {
                    7
                },
        );
        builder.add_char(if self.warp_type == WarpRequestType::Local {
            1
        } else {
            2
        });
        builder.add_short(self.map_id);
        if let WarpRequestType::Remote {
            map_rid,
            map_filesize,
        } = self.warp_type
        {
            builder.add_short(map_rid[0]);
            builder.add_short(map_rid[1]);
            builder.add_three(map_filesize);
        }
        builder.add_short(self.session_id);
        builder.get()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_local() {
        let buf = vec![2, 15, 254, 168, 89];
        let reader = StreamReader::new(&buf);
        let mut request = Request::default();
        request.deserialize(&reader);
        assert_eq!(request.warp_type, WarpRequestType::Local);
        assert_eq!(request.map_id, 14);
        assert_eq!(request.session_id, 22431);
    }

    #[test]
    fn serialize_local() {
        let request = Request::local(14, 22431);
        assert_eq!(request.serialize(), vec![2, 15, 254, 168, 89]);
    }

    #[test]
    fn deserialize_remote() {
        let buf = vec![3, 15, 254, 240, 115, 129, 8, 66, 8, 254, 168, 89];
        let reader = StreamReader::new(&buf);
        let mut request = Request::default();
        request.deserialize(&reader);
        assert_eq!(
            request.warp_type,
            WarpRequestType::Remote {
                map_rid: [29081, 1899],
                map_filesize: 1836,
            }
        );
        assert_eq!(request.map_id, 14);
        assert_eq!(request.session_id, 22431);
    }

    #[test]
    fn serialize_remote() {
        let request = Request::remote(14, 22431, [29081, 1899], 1836);
        assert_eq!(
            request.serialize(),
            vec![3, 15, 254, 240, 115, 129, 8, 66, 8, 254, 168, 89]
        );
    }
}
