use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use parse_display::Display;

use crate::frames::TexturePack;
use crate::{from_position, GameAssetType, GameAssets, Size};

pub enum AttackType {
    Slash,
    Claw,
    Thunder,
    Leaf,
}

impl AttackType {
    pub fn sound(&self) -> &str {
        match self {
            AttackType::Slash => "slash",
            AttackType::Claw => "claw",
            AttackType::Thunder => "fireball",
            AttackType::Leaf => "slash",
        }
    }
}

#[derive(Debug, Clone, Copy, Display, PartialEq, Component, Reflect)]
#[display(style = "snake_case")]
pub enum Enemy {
    Squid,
    Raccoon,
    Spirit,
    Bamboo,
}

impl Enemy {
    pub fn health(&self) -> u32 {
        match self {
            Enemy::Squid => 100,
            Enemy::Raccoon => 300,
            Enemy::Spirit => 100,
            Enemy::Bamboo => 70,
        }
    }

    pub fn exp(&self) -> u32 {
        match self {
            Enemy::Squid => 100,
            Enemy::Raccoon => 250,
            Enemy::Spirit => 110,
            Enemy::Bamboo => 120,
        }
    }

    pub fn damage(&self) -> u32 {
        match self {
            Enemy::Squid => 20,
            Enemy::Raccoon => 40,
            Enemy::Spirit => 8,
            Enemy::Bamboo => 6,
        }
    }

    pub fn speed(&self) -> u32 {
        match self {
            Enemy::Squid => 3,
            Enemy::Raccoon => 2,
            Enemy::Spirit => 4,
            Enemy::Bamboo => 3,
        }
    }

    pub fn resistance(&self) -> u32 {
        match self {
            Enemy::Squid => 3,
            Enemy::Raccoon => 3,
            Enemy::Spirit => 3,
            Enemy::Bamboo => 3,
        }
    }

    pub fn attack_radius(&self) -> u32 {
        match self {
            Enemy::Squid => 80,
            Enemy::Raccoon => 120,
            Enemy::Spirit => 60,
            Enemy::Bamboo => 50,
        }
    }

    pub fn notice_radius(&self) -> u32 {
        match self {
            Enemy::Squid => 360,
            Enemy::Raccoon => 400,
            Enemy::Spirit => 350,
            Enemy::Bamboo => 300,
        }
    }
}

impl From<i32> for Enemy {
    fn from(value: i32) -> Self {
        match value {
            390 => Enemy::Bamboo,
            391 => Enemy::Raccoon,
            392 => Enemy::Spirit,
            393 => Enemy::Squid,
            _ => unreachable!(),
        }
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    window: &Window,
    asset_server: &Res<AssetServer>,
    assets: &Res<GameAssets>,
    textures: &Res<Assets<TexturePack>>,
    cell: i32,
    x: f32,
    y: f32,
) {
    let enemy = Enemy::from(cell);

    let name = format!("monsters/{enemy}/idle/0.png");
    let handle = asset_server.load("textures/monsters.json");
    let pack = textures.get(&handle).expect("Texture pack must exist");
    let index = pack.index_of(&name);

    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(index),
                texture_atlas: assets.get(GameAssetType::Monsters).clone(),
                transform: Transform::from_translation(from_position(x, y, window)),
                ..Default::default()
            },
            RigidBody::Dynamic,
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            // AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Size::default(),
            enemy,
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(32.0, 16.0),
                Transform::from_xyz(0.0, -16.0, 0.0),
                ColliderDebugColor(Color::RED),
            ));
        });
}
