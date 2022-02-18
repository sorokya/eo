const MAP_NAME_LENGTH: usize = 24;
const NUMBER_OF_GFX_LAYERS: usize = 9;
const NPC_SPAWN_SIZE: usize = 8;
const CHEST_SPAWN_SIZE: usize = 12;
const UNKNOWN_SIZE: usize = 4;
const WARP_ROW_SIZE: usize = 1;
const WARP_SIZE: usize = 8;
const TILE_ROW_SIZE: usize = 1;
const TILE_SIZE: usize = 2;
const GFX_ROW_SIZE: usize = 1;
const GFX_SIZE: usize = 3;
const SIGN_SIZE: usize = 2;

use crate::data::EOByte;

fn encode_map_string(s: &str, length: usize) -> Vec<EOByte> {
    let mut buf = vec![0xFF; length];
    for (i, c) in s.chars().enumerate() {
        buf[i] = c as u8;
    }

    let mut flippy = buf.len() % 2 == 1;
    for i in 0..buf.len() {
        let mut c = buf[i];
        if flippy {
            if (0x22..=0x4F).contains(&c) {
                c = 0x71 - c;
            } else if (0x50..=0x7E).contains(&c) {
                c = 0xCD - c;
            }
        } else if (0x22..=0x7E).contains(&c) {
            c = 0x9F - c;
        }
        buf[i] = c;
        flippy = !flippy;
    }
    buf.reverse();

    buf
}

fn decode_map_string(mut buf: Vec<EOByte>) -> String {
    buf.reverse();

    let mut chars: Vec<EOByte> = vec![0xFF; buf.len()];
    let mut flippy = buf.len() % 2 == 1;
    for i in 0..buf.len() {
        let mut c = buf[i];
        if c == 0xFF {
            chars.truncate(i);
            break;
        }

        if flippy {
            if (0x22..=0x4F).contains(&c) {
                c = 0x71 - c;
            } else if (0x50..=0x7E).contains(&c) {
                c = 0xCD - c;
            }
        } else if (0x22..=0x7E).contains(&c) {
            c = 0x9F - c;
        }
        chars[i] = c;
        flippy = !flippy;
    }

    String::from_utf8(chars).expect("Failed to convert byte array to string")
}

mod chest_spawn;
pub use chest_spawn::ChestSpawn;
mod gfx;
pub use gfx::Gfx;
mod gfx_row;
pub use gfx_row::GfxRow;
mod map_effect;
pub use map_effect::MapEffect;
mod map_type;
pub use map_type::MapType;
mod npc_speed;
pub use npc_speed::NPCSpeed;
mod npc_spawn;
pub use npc_spawn::NPCSpawn;
mod sign;
pub use sign::Sign;
mod tile_spec;
pub use tile_spec::TileSpec;
mod tile;
pub use tile::Tile;
mod tile_row;
pub use tile_row::TileRow;
mod warp;
pub use warp::Warp;
mod warp_row;
pub use warp_row::WarpRow;
mod unknown;
pub use unknown::Unknown;
mod map_file;
pub use map_file::MapFile;

#[cfg(test)]
mod tests {
    use crate::data::{map::MAP_NAME_LENGTH, EOByte};
    #[test]
    fn decode_map_string() {
        let encoded_map_name: Vec<EOByte> = vec![
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x3D, 0x7C, 0x20, 0x31, 0x5E, 0x36,
            0x5A, 0x31, 0x6C, 0x52, 0x20, 0x2B, 0x5A, 0x30, 0x65, 0x58,
        ];
        assert_eq!(
            super::decode_map_string(encoded_map_name),
            "Ghost Mansion #4"
        );
    }

    #[test]
    fn encode_map_string() {
        assert_eq!(
            super::encode_map_string("Ghost Mansion #4", MAP_NAME_LENGTH),
            [
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x3D, 0x7C, 0x20, 0x31, 0x5E, 0x36,
                0x5A, 0x31, 0x6C, 0x52, 0x20, 0x2B, 0x5A, 0x30, 0x65, 0x58,
            ]
        );
    }
}
