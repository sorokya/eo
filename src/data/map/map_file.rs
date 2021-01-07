#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use num_traits::FromPrimitive;

use std::io::{
    prelude::{Read, Seek},
    SeekFrom,
};

use super::{
    ChestSpawn, Gfx, GfxRow, MapEffect, MapType, NPCSpawn, Sign, Tile, TileRow, TileSpec, Unknown,
    Warp, WarpRow,
};
use crate::data::{EOByte, EOChar, EOInt, EOShort, Serializeable, StreamReader};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MapFile {
    pub revision: EOInt,
    pub name: String,
    pub map_type: MapType,
    pub effect: MapEffect,
    pub music_id: EOChar,
    pub music_extra: EOChar,
    pub ambient_sound_id: EOShort,
    pub width: EOChar,
    pub height: EOChar,
    pub fill_tile: EOShort,
    pub map_available: bool,
    pub can_scroll: bool,
    pub relog_x: EOChar,
    pub relog_y: EOChar,
    pub npc_spawns: Vec<NPCSpawn>,
    pub unknowns: Vec<Unknown>,
    pub chest_spawns: Vec<ChestSpawn>,
    pub tile_rows: Vec<TileRow>,
    pub warp_rows: Vec<WarpRow>,
    pub signs: Vec<Sign>,
    pub gfx_rows: [Vec<GfxRow>; 9],
}

impl MapFile {
    /// creates an empty MapFile
    pub fn new() -> Self {
        Self::default()
    }

    /// reads the content of a [Read]+[Seek] implemented struct into the MapFile
    pub fn read<R: Read + Seek>(&mut self, buf: &mut R) -> std::io::Result<()> {
        let mut data_buf: Vec<EOByte> = Vec::new();
        buf.seek(SeekFrom::Start(0))?;
        buf.read_to_end(&mut data_buf)?;
        let mut reader = StreamReader::new(&data_buf);
        reader.seek(3);
        self.revision = reader.get_int();
        self.name = decode_map_string(&mut reader.get_vec(24));
        let map_type_char = reader.get_char();
        self.map_type = match MapType::from_u8(map_type_char) {
            Some(map_type) => map_type,
            None => panic!("Failed to convert char to MapType: {}", map_type_char),
        };
        let effect_char = reader.get_char();
        self.effect = match MapEffect::from_u8(effect_char) {
            Some(effect) => effect,
            None => panic!("Failed to convert char to MapEffect: {}", effect_char),
        };
        self.music_id = reader.get_char();
        self.music_extra = reader.get_char();
        self.ambient_sound_id = reader.get_short();
        self.width = reader.get_char();
        self.height = reader.get_char();
        self.fill_tile = reader.get_short();
        self.map_available = reader.get_char() == 1;
        self.can_scroll = reader.get_char() == 1;
        self.relog_x = reader.get_char();
        self.relog_y = reader.get_char();
        reader.get_char();
        self.read_npc_spawns(&mut reader);
        self.read_unknowns(&mut reader);
        self.read_chest_spawns(&mut reader);
        self.read_tiles(&mut reader);
        self.read_warps(&mut reader);
        for layer in 0..9 {
            if !reader.eof() {
                self.read_gfx_layer(layer, &mut reader);
            }
        }
        if !reader.eof() {
            self.read_signs(&mut reader);
        }
        Ok(())
    }

    fn read_npc_spawns(&mut self, reader: &mut StreamReader) {
        let npc_spawns_length = reader.get_char();
        self.npc_spawns = Vec::with_capacity(npc_spawns_length as usize);
        for _ in 0..npc_spawns_length {
            let mut npc = NPCSpawn::new();
            npc.deserialize(reader);
            self.npc_spawns.push(npc);
        }
    }

    fn read_unknowns(&mut self, reader: &mut StreamReader) {
        let unknowns_length = reader.get_char();
        self.unknowns = Vec::with_capacity(unknowns_length as usize);
        for _ in 0..unknowns_length {
            let mut unknown = Unknown::new();
            unknown.deserialize(reader);
            self.unknowns.push(unknown);
        }
    }

    fn read_chest_spawns(&mut self, reader: &mut StreamReader) {
        let chest_spawns_length = reader.get_char();
        self.chest_spawns = Vec::with_capacity(chest_spawns_length as usize);
        for _ in 0..chest_spawns_length {
            let mut chest_spawn = ChestSpawn::new();
            chest_spawn.deserialize(reader);
            self.chest_spawns.push(chest_spawn);
        }
    }

    fn read_tiles(&mut self, reader: &mut StreamReader) {
        let outer_length = reader.get_char();
        self.tile_rows = Vec::with_capacity(outer_length as usize);
        for _ in 0..outer_length {
            let y = reader.get_char();
            let inner_length = reader.get_char();
            let mut tile_row = TileRow::new(y, inner_length as usize);
            for _ in 0..inner_length {
                let x = reader.get_char();
                let spec_char = reader.get_char();
                let spec = match TileSpec::from_u8(spec_char) {
                    Some(spec) => spec,
                    None => panic!("Failed to convert char to TileSpec: {}", spec_char),
                };
                tile_row.tiles.push(Tile::new(x, spec));
            }
            self.tile_rows.push(tile_row);
        }
    }

    fn read_warps(&mut self, reader: &mut StreamReader) {
        let outer_length = reader.get_char();
        self.warp_rows = Vec::with_capacity(outer_length as usize);
        for _ in 0..outer_length {
            let y = reader.get_char();
            let inner_length = reader.get_char();
            let mut warp_row = WarpRow::new(y, inner_length as usize);
            for _ in 0..inner_length {
                let mut warp = Warp::new();
                warp.x = reader.get_char();
                warp.warp_map = reader.get_short();
                warp.warp_x = reader.get_char();
                warp.warp_y = reader.get_char();
                warp.level_req = reader.get_char();
                warp.door = reader.get_short() == 1;
                warp_row.tiles.push(warp);
            }
            self.warp_rows.push(warp_row);
        }
    }

    fn read_gfx_layer(&mut self, layer: usize, reader: &mut StreamReader) {
        let outer_length = reader.get_char();
        self.gfx_rows[layer] = Vec::with_capacity(outer_length as usize);
        for _ in 0..outer_length {
            let y = reader.get_char();
            let inner_length = reader.get_char();
            let mut gfx_row = GfxRow::new(y, inner_length as usize);
            for _ in 0..inner_length {
                let y = reader.get_char();
                let tile = reader.get_short();
                gfx_row.tiles.push(Gfx::new(y, tile));
            }
            self.gfx_rows[layer].push(gfx_row);
        }
    }

    fn read_signs(&mut self, reader: &mut StreamReader) {
        let signs_length = reader.get_char();
        self.signs = Vec::with_capacity(signs_length as usize);
        for _ in 0..signs_length {
            if reader.remaining() <= 4 {
                break;
            }
            let mut sign = Sign::new();
            sign.x = reader.get_char();
            sign.y = reader.get_char();
            let text_length = reader.get_short() - 1;
            if reader.remaining() >= text_length as usize {
                let sign_text = decode_map_string(&mut reader.get_vec(text_length as usize));
                let title_length = reader.get_char();
                sign.title = sign_text.chars().take(title_length as usize).collect();
                sign.message = sign_text.chars().skip(title_length as usize).collect();
                self.signs.push(sign);
            }
        }
    }
}

fn encode_map_string(s: &str) -> Vec<EOByte> {
    let mut buf = s.as_bytes().to_vec();
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

fn decode_map_string(buf: &mut [EOByte]) -> String {
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
