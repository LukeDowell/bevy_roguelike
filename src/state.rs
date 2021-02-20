pub use std::time::Instant;

pub struct Position { pub x: f32, pub y: f32 }
pub struct Player;

pub struct InputTimer {
    pub last_moved: Instant,
    pub move_cooldown: f64
}

impl Default for InputTimer {
    fn default() -> Self {
        InputTimer { last_moved: Instant::now(), move_cooldown: 0.5 }
    }
}

pub struct GameSettings {
    pub tile_size: f32,
    pub map_width: i32,
    pub map_height: i32
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, Floor
}
