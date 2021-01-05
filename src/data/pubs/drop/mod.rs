const EDF_NPC_DATA_SIZE: usize = 4;
const EDF_DROP_DATA_SIZE: usize = 10;

mod drop_npc_record;
pub use drop_npc_record::DropNPCRecord;
mod drop_record;
pub use drop_record::DropRecord;
mod drop_file;
pub use drop_file::DropFile;
