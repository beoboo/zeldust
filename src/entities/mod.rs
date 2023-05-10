use std::{f32::consts::FRAC_PI_4, time::Duration};

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use parse_display::Display;

pub use enemies::*;
pub use player::*;

use crate::frames::TexturePack;

mod enemies;
mod player;

#[derive(Component, Deref, DerefMut)]
pub struct AttackTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct HitTimer(pub Timer);

#[derive(Component)]
pub struct Attackable {
    health: u32,
}

impl Attackable {
    pub fn new(health: u32) -> Self {
        Self { health }
    }

    pub fn update(&mut self, damage: u32) -> u32 {
        if self.health > damage {
            self.health -= damage;
        } else {
            self.health = 0;
        }

        self.health
    }
}

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

#[derive(Component)]
pub struct Animation {
    current_frame: usize,
    num_frames: usize,
    timer: Timer,
}

impl Animation {
    pub fn new(duration: Duration) -> Self {
        let mut timer = Timer::new(duration, TimerMode::Repeating);
        timer.pause();

        Self {
            current_frame: 0,
            num_frames: 0,
            timer,
        }
    }

    pub fn next_frame(&mut self, delta: Duration) -> usize {
        self.timer.tick(delta);

        if self.timer.just_finished() {
            self.current_frame = (self.current_frame + 1) % self.num_frames;
        }

        self.current_frame
    }

    pub fn is_paused(&self) -> bool {
        self.timer.paused()
    }

    pub fn play(&mut self, num_frames: usize) {
        self.num_frames = num_frames;
        self.current_frame = 0;
        self.timer.reset();
        self.timer.unpause()
    }

    pub fn stop(&mut self) {
        self.timer.pause()
    }
}

pub trait AnimatedEntity {
    fn asset_name(&self) -> String;
    fn texture_name(&self) -> String;
    fn num_frames(&self) -> usize;
}

pub fn update_depth(mut query: Query<(&mut Transform, Ref<Velocity>)>) {
    for (mut transform, previous) in query.iter_mut() {
        if previous.is_changed() {
            transform.translation.z = -transform.translation.y + 1000.0;
        }
    }
}

pub fn render_animation(
    game_entity: &dyn AnimatedEntity,
    animation: &mut Animation,
    sprite: &mut TextureAtlasSprite,
    time: &Res<Time>,
    asset_server: &Res<AssetServer>,
    textures: &Res<Assets<TexturePack>>,
) {
    let handle = asset_server.load(format!("textures/{}.json", game_entity.asset_name()));
    let pack = textures.get(&handle).expect("Texture pack must exist");
    let index = pack.index_of(&game_entity.texture_name());

    if animation.is_paused() {
        animation.play(game_entity.num_frames());
        sprite.index = index;
    } else {
        let frame = animation.next_frame(time.delta());
        sprite.index = index + frame;
    }
}
