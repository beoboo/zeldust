use std::time::Duration;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use parse_display::Display;

use crate::constants::{FPS, TILE_SIZE};
use crate::entities::Status;
use crate::entities::{AnimationTimer, Player};
use crate::frames::TexturePack;
use crate::{from_position, GameAssetType, GameAssets};

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
pub enum EnemyType {
    Squid,
    Raccoon,
    Spirit,
    Bamboo,
}

impl From<i32> for EnemyType {
    fn from(value: i32) -> Self {
        match value {
            390 => EnemyType::Bamboo,
            391 => EnemyType::Spirit,
            392 => EnemyType::Raccoon,
            393 => EnemyType::Squid,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Component, Reflect)]
pub struct Enemy {
    ty: EnemyType,
    status: Status,
    is_attacking: bool,
}

impl Enemy {
    pub fn new(ty: EnemyType) -> Self {
        Self {
            ty,
            status: Status::Idle,
            is_attacking: false,
        }
    }

    pub fn health(&self) -> u32 {
        match self.ty {
            EnemyType::Squid => 100,
            EnemyType::Raccoon => 300,
            EnemyType::Spirit => 100,
            EnemyType::Bamboo => 70,
        }
    }

    pub fn exp(&self) -> u32 {
        match self.ty {
            EnemyType::Squid => 100,
            EnemyType::Raccoon => 250,
            EnemyType::Spirit => 110,
            EnemyType::Bamboo => 120,
        }
    }

    pub fn damage(&self) -> u32 {
        match self.ty {
            EnemyType::Squid => 20,
            EnemyType::Raccoon => 40,
            EnemyType::Spirit => 8,
            EnemyType::Bamboo => 6,
        }
    }

    pub fn speed(&self) -> f32 {
        match self.ty {
            EnemyType::Squid => 3.0,
            EnemyType::Raccoon => 2.0,
            EnemyType::Spirit => 4.0,
            EnemyType::Bamboo => 3.0,
        }
    }

    pub fn resistance(&self) -> u32 {
        match self.ty {
            EnemyType::Squid => 3,
            EnemyType::Raccoon => 3,
            EnemyType::Spirit => 3,
            EnemyType::Bamboo => 3,
        }
    }

    pub fn attack_radius(&self) -> f32 {
        match self.ty {
            EnemyType::Squid => 80.0,
            EnemyType::Raccoon => 120.0,
            EnemyType::Spirit => 60.0,
            EnemyType::Bamboo => 50.0,
        }
    }

    pub fn notice_radius(&self) -> f32 {
        match self.ty {
            EnemyType::Squid => 360.0,
            EnemyType::Raccoon => 400.0,
            EnemyType::Spirit => 350.0,
            EnemyType::Bamboo => 300.0,
        }
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    window: &Window,
    asset_server: &Res<AssetServer>,
    assets: &Res<GameAssets>,
    atlases: &Res<Assets<TextureAtlas>>,
    textures: &Res<Assets<TexturePack>>,
    cell: i32,
    x: f32,
    y: f32,
) {
    let ty = EnemyType::from(cell);

    let name = format!("monsters/{ty}/idle/0.png");
    let handle = asset_server.load("textures/monsters.json");

    let pack = textures.get(&handle).expect("Texture pack must exist");
    let index = pack.index_of(&name);

    let atlas_handle = assets.get(GameAssetType::Monsters);
    let atlas = atlases.get(atlas_handle).unwrap();
    let rect = atlas.textures[index];
    let offset = (rect.height() - TILE_SIZE) / 2.0;

    let y = y - offset;

    let collider_height = TILE_SIZE / 2.0;

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
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Enemy::new(ty),
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(rect.width() / 2.0, collider_height / 2.0),
                Transform::from_xyz(0.0, -offset, 0.0),
                ColliderDebugColor(Color::RED),
            ));
        });
}

pub fn move_enemy(
    player_q: Query<&Transform, With<Player>>,
    mut enemy_q: Query<(&Transform, &mut Velocity, &mut Enemy)>,
) {
    let player_transform = player_q.single();

    for (transform, mut velocity, mut enemy) in enemy_q.iter_mut() {
        let diff = player_transform.translation - transform.translation;
        let distance = diff.length();
        let direction = diff.xy().normalize_or_zero() * enemy.speed() * FPS;

        if distance < enemy.attack_radius() {
            velocity.linvel = direction.into();
            enemy.is_attacking = true;
        } else if distance < enemy.notice_radius() {
            velocity.linvel = direction.into();
            enemy.is_attacking = false;
        } else {
            velocity.linvel = Vec2::ZERO;
            enemy.is_attacking = false;
        }
    }
}

pub fn render_enemy(
    time: Res<Time>,
    mut query: Query<(&Enemy, &mut AnimationTimer, &mut TextureAtlasSprite)>,
    asset_server: Res<AssetServer>,
    textures: Res<Assets<TexturePack>>,
) {
    for (enemy, mut timer, mut sprite) in query.iter_mut() {
        let mut status = enemy.status.to_string();

        let name = format!("monsters/{}/{status}/0.png", enemy.ty);
        let handle = asset_server.load("textures/monsters.json");
        let pack = textures.get(&handle).expect("Texture pack must exist");
        let index = pack.index_of(&name);

        if !enemy.is_attacking {
            if timer.0.paused() {
                sprite.index = index;
                timer.0.set_duration(Duration::from_secs_f32(0.1));
                timer.0.reset();
                timer.0.unpause();
            } else {
                timer.0.tick(time.delta());
                if timer.0.just_finished() {
                    let mut current = sprite.index;
                    current = (current + 1) % 4;
                    sprite.index = index + current;
                }
            }
        } else {
            sprite.index = index;
        }
    }
}
