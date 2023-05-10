pub const TILE_SIZE: usize = 64;
pub const SCREEN_WIDTH: usize = 1280;
pub const SCREEN_HEIGHT: usize = 720;
pub const FPS: u32 = 60;

pub fn build_world() -> Vec<String> {
    vec![
        "xxxxxxxxxxxxxxxxxxxx".to_string(),
        "x                  x".to_string(),
        "x p                x".to_string(),
        "x  x     xxxxx     x".to_string(),
        "x  x         x     x".to_string(),
        "x  x         x     x".to_string(),
        "x  x         x     x".to_string(),
        "x  x         x     x".to_string(),
        "x  x         x     x".to_string(),
        "x  x         x     x".to_string(),
        "x  x         x     x".to_string(),
        "x  x         xxx   x".to_string(),
        "x      x x         x".to_string(),
        "x     xxxxx        x".to_string(),
        "x      xxx         x".to_string(),
        "x       x          x".to_string(),
        "x                  x".to_string(),
        "x                  x".to_string(),
        "x                  x".to_string(),
        "xxxxxxxxxxxxxxxxxxxx".to_string(),
    ]
}
