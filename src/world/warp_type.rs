use crate::data::{EOShort, EOThree};

use super::WarpAnimation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WarpRequestType {
    Local,
    Remote {
        map_rid: [EOShort; 2],
        map_filesize: EOThree,
    },
}

impl Default for WarpRequestType {
    fn default() -> Self {
        Self::Local
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WarpAgreeType {
    Local,
    Remote {
        map_id: EOShort,
        warp_animation: Option<WarpAnimation>,
    },
}

impl Default for WarpAgreeType {
    fn default() -> Self {
        Self::Local
    }
}
