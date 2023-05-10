use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;
use parse_display::Display;

pub use enemies::*;
pub use player::*;

mod enemies;
mod player;

#[derive(Debug, Clone, Copy, Display, PartialEq, Reflect)]
#[display(style = "snake_case")]
pub enum Status {
    Idle,
    Move,
}

#[derive(Debug, Clone, Copy, Display, PartialEq, Reflect)]
#[display(style = "snake_case")]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl From<Vec2> for Direction {
    fn from(value: Vec2) -> Self {
        let angle = value.angle_between(Vec2::Y);

        if angle >= -FRAC_PI_4 && angle <= FRAC_PI_4 {
            Direction::Up
        } else if angle > -3.0 * FRAC_PI_4 {
            Direction::Left
        } else if angle < 3.0 * FRAC_PI_4 {
            Direction::Right
        } else {
            Direction::Down
        }
    }
}
