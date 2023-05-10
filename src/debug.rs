use lazy_static::lazy_static;

use crate::LayerType;

pub const DEBUG_PHYSICS: bool = true;
pub const MAX_ENEMIES: i32 = -1;
pub const MAX_TILES: i32 = -1;

lazy_static! {
    pub static ref VALID_LAYERS: Vec<LayerType> = vec![
        LayerType::Blocks,
        LayerType::Grass,
        LayerType::Objects,
        LayerType::Entities,
    ];
}

pub fn can_spawn(current: usize, max: i32) -> bool {
    if max == -1 {
        true
    } else {
        current <= max as usize
    }
}
