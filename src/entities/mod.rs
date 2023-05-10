use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use parse_display::Display;

pub use enemies::*;
pub use player::*;

mod enemies;
mod player;

#[derive(Component, Deref, DerefMut)]
pub struct AttackTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Debug, Clone, Copy, Display, PartialEq, Reflect)]
#[display(style = "snake_case")]
pub enum Status {
    Idle,
    Attack,
    #[display("move")]
    Move(Vec2),
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

pub fn update_depth(mut query: Query<(&mut Transform, Ref<Velocity>)>) {
    for (mut transform, previous) in query.iter_mut() {
        if previous.is_changed() {
            transform.translation.z = -transform.translation.y + 1000.0;
        }
    }
}
