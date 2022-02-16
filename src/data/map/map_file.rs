#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use num_traits::FromPrimitive;

use std::io::{
    prelude::{Read, Seek},
    SeekFrom,
};

use super::{
    decode_map_string, ChestSpawn, GfxRow, MapEffect, MapType, NPCSpawn, Sign, TileRow, Unknown,
    WarpRow, MAP_NAME_LENGTH, NUMBER_OF_GFX_LAYERS,
};
use crate::data::{EOByte, EOChar, EOShort, Serializeable, StreamReader};

/// represents emf (map) files
///
/// The map file contains all of the properties, tiles, npcs, signs, and
/// chest spawns associated with a single map in the game world.
///
/// The file layout is:
///```text
/// "EMF" (fixed string)
/// hash (4 bytes)
/// name (fixed string 24 bytes long)
/// type (1 byte)
/// effect (1 byte)
/// music_id (1 byte)
/// music_extra (1 byte)
/// ambient_sound_id (2 bytes)
/// width (1 byte)
/// height (1 byte)
/// fill_tile (2 bytes)
/// map_available (1 byte)
/// can_scroll (1 byte)
/// relog_x (1 byte)
/// relog_y (1 byte)
/// unknown (1 byte)
/// npc_spawns_length (1 byte)
/// NPCSpawn*npc_spawns_length
/// {
///     x (1 byte)
///     y (1 byte)
///     id (2 bytes)
///     type (1 byte)
///     time (2 bytes)
///     amount (1 byte)
/// }
/// unknowns_length (1 byte)
/// Unknown*unknowns_length
/// {
///     unknown (1 byte)
///     unknown (1 byte)
///     unknown (1 byte)
///     unknown (1 byte)
/// }
/// chest_spawns_length (1 byte)
/// ChestSpawn*chest_spawns_length
/// {
///     x (1 byte)
///     y (1 byte)
///     key (2 bytes)
///     slot (1 byte)
///     item_id (2 bytes)
///     respawn_time (2 bytes)
///     amount (3 bytes)
/// }
/// tiles_outer_length (1 byte)
/// TileRow*tiles_outer_length
/// {
///     y (1 byte)
///     tiles_inner_length (1 byte)
///     Tile*tiles_inner_length
///     {
///         x (1 byte)
///         tile_spec (1 byte)
///     }
/// }
/// warps_outer_length (1 byte)
/// WarpRow*warps_outer_length
/// {
///     y (1 byte)
///     warps_inner_length (1 byte)
///     Warp*warps_inner_length
///     {
///         x (1 byte)
///         warp_map (2 bytes)
///         warp_x (1 byte)
///         warp_y (1 byte)
///         level_req (1 byte)
///         door (2 bytes)
///     }
/// }
/// GfxLayer*9
/// {
///     gfx_outer_length (1 byte)
///     GfxRow*gfx_outer_length
///     {
///         y (1 byte)
///         gfx_inner_length (1 byte)
///         Gfx*gfx_inner_length
///         {
///             x (1 byte)
///             tile (1 byte)
///         }
///     }
/// }
/// signs_length (1 byte)
/// Sign*signs_length
/// {
///     x (1 byte)
///     y (1 byte)
///     text_length (2 bytes)
///     text (fixed string with above length - 1)
///     title_length (1 byte)
/// }
///```
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MapFile {
    pub hash: [EOByte; 4],
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
    pub gfx_rows: [Vec<GfxRow>; NUMBER_OF_GFX_LAYERS],
    pub signs: Vec<Sign>,
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
        let reader = StreamReader::new(&data_buf);
        reader.seek(3);
        self.hash = [
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
        ];
        self.name = decode_map_string(reader.get_vec(MAP_NAME_LENGTH));
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
        self.read_npc_spawns(&reader);
        self.read_unknowns(&reader);
        self.read_chest_spawns(&reader);
        self.read_tiles(&reader);
        self.read_warps(&reader);
        for layer in 0..NUMBER_OF_GFX_LAYERS {
            if !reader.eof() {
                self.read_gfx_layer(layer, &reader);
            }
        }
        if !reader.eof() {
            self.read_signs(&reader);
        }
        Ok(())
    }

    fn read_npc_spawns(&mut self, reader: &StreamReader) {
        let npc_spawns_length = reader.get_char();
        self.npc_spawns = Vec::with_capacity(npc_spawns_length as usize);
        for _ in 0..npc_spawns_length {
            let mut npc = NPCSpawn::new();
            npc.deserialize(reader);
            self.npc_spawns.push(npc);
        }
    }

    fn read_unknowns(&mut self, reader: &StreamReader) {
        let unknowns_length = reader.get_char();
        self.unknowns = Vec::with_capacity(unknowns_length as usize);
        for _ in 0..unknowns_length {
            let mut unknown = Unknown::new();
            unknown.deserialize(reader);
            self.unknowns.push(unknown);
        }
    }

    fn read_chest_spawns(&mut self, reader: &StreamReader) {
        let chest_spawns_length = reader.get_char();
        self.chest_spawns = Vec::with_capacity(chest_spawns_length as usize);
        for _ in 0..chest_spawns_length {
            let mut chest_spawn = ChestSpawn::new();
            chest_spawn.deserialize(reader);
            self.chest_spawns.push(chest_spawn);
        }
    }

    fn read_tiles(&mut self, reader: &StreamReader) {
        let outer_length = reader.get_char();
        self.tile_rows = Vec::with_capacity(outer_length as usize);
        for _ in 0..outer_length {
            let mut tile_row = TileRow::new();
            tile_row.deserialize(reader);
            self.tile_rows.push(tile_row);
        }
    }

    fn read_warps(&mut self, reader: &StreamReader) {
        let outer_length = reader.get_char();
        self.warp_rows = Vec::with_capacity(outer_length as usize);
        for _ in 0..outer_length {
            let mut warp_row = WarpRow::new();
            warp_row.deserialize(reader);
            self.warp_rows.push(warp_row);
        }
    }

    fn read_gfx_layer(&mut self, layer: usize, reader: &StreamReader) {
        let outer_length = reader.get_char();
        self.gfx_rows[layer] = Vec::with_capacity(outer_length as usize);
        for _ in 0..outer_length {
            let mut gfx_row = GfxRow::new();
            gfx_row.deserialize(reader);
            self.gfx_rows[layer].push(gfx_row);
        }
    }

    fn read_signs(&mut self, reader: &StreamReader) {
        let signs_length = reader.get_char();
        self.signs = Vec::with_capacity(signs_length as usize);
        for _ in 0..signs_length {
            if reader.remaining() <= 4 {
                break;
            }
            let mut sign = Sign::new();
            sign.deserialize(reader);
            self.signs.push(sign);
        }
    }
}
