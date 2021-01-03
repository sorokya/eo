const ECF_DATA_SIZE: usize = 14;

mod class_type;
pub use class_type::ClassType;

mod record;
pub use record::*;

mod file;
pub use file::ClassFile;
