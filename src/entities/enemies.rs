use bevy::{log, math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::*;
use parse_display::Display;

use crate::{
    constants::{ANIMATION_DURATION, ATTACK_DURATION, SPEED, TILE_SIZE},
    entities::{AnimationTimer, AttackTimer, Player, Status},
    frames::TexturePack,
    from_position,
    GameAssetType,
    GameAssets,
};

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
    can_attack: bool,
    frame: usize,
}

impl Enemy {
    pub fn new(ty: EnemyType) -> Self {
        Self {
            ty,
            status: Status::Idle,
            can_attack: true,
            frame: 0,
        }
    }

    pub fn is_attacking(&self) -> bool {
        self.status == Status::Attack
    }

    pub fn num_frames(&self) -> usize {
        let frames = match self.ty {
            EnemyType::Bamboo => [1, 4, 4],
            EnemyType::Raccoon => [4, 6, 5],
            EnemyType::Spirit => [1, 4, 4],
            EnemyType::Squid => [1, 4, 4],
        };

        match self.status {
            Status::Attack => frames[0],
            Status::Idle => frames[1],
            Status::Move(_) => frames[2],
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

    // let collider_height = TILE_SIZE / 2.0;

    let mut timer = AnimationTimer(Timer::new(ANIMATION_DURATION, TimerMode::Repeating));
    timer.0.pause();

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
            timer,
            Enemy::new(ty),
        ))
        .with_children(|parent| {
            parent.spawn((
                // Collider::cuboid(rect.width() / 2.0, collider_height / 2.0),
                // Transform::from_xyz(0.0, -offset, 0.0),
                // ColliderDebugColor(Color::RED),
            ));
        });
}

pub fn move_enemy(
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    mut enemy_q: Query<(Entity, &mut Enemy, &Transform, &mut Velocity, &mut AnimationTimer)>,
) {
    let player_transform = player_q.single();

    for (entity, mut enemy, transform, mut velocity, mut timer) in enemy_q.iter_mut() {
        let diff = player_transform.translation - transform.translation;
        let distance = diff.length();
        let direction = diff.xy().normalize_or_zero() * enemy.speed() * SPEED;

        if enemy.is_attacking() {
            continue;
        }

        let mut status = Status::Idle;

        if distance < enemy.attack_radius() && enemy.can_attack {
            velocity.linvel = Vec2::ZERO;
            enemy.can_attack = false;
            status = Status::Attack;

            commands
                .entity(entity)
                .insert(AttackTimer(Timer::new(ATTACK_DURATION, TimerMode::Once)));
        } else if distance < enemy.notice_radius() {
            velocity.linvel = direction.into();
            status = Status::Move(direction);
        } else {
            velocity.linvel = Vec2::ZERO;
        }

        if status != enemy.status {
            timer.pause();

            enemy.status = status;
        }
    }
}

pub fn render_enemy(
    time: Res<Time>,
    mut query: Query<(&mut Enemy, &mut AnimationTimer, &mut TextureAtlasSprite)>,
    asset_server: Res<AssetServer>,
    textures: Res<Assets<TexturePack>>,
) {
    for (mut enemy, mut timer, mut sprite) in query.iter_mut() {
        let mut status = enemy.status.to_string();

        let name = format!("monsters/{}/{status}/0.png", enemy.ty);
        // log::info!("{name}");
        let handle = asset_server.load("textures/monsters.json");
        let pack = textures.get(&handle).expect("Texture pack must exist");
        let mut index = pack.index_of(&name);

        if timer.0.paused() {
            // log::info!("Unpausing enemy {enemy:?}");
            timer.0.reset();
            timer.0.unpause();
            sprite.index = index;
            enemy.frame = 0;
        } else {
            timer.0.tick(time.delta());

            if timer.0.just_finished() {
                enemy.frame = (enemy.frame + 1) % enemy.num_frames();
                sprite.index = index + enemy.frame;
            }
        }
    }
}

pub fn end_enemy_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut entity_q: Query<(Entity, &mut Enemy, &mut AttackTimer)>,
) {
    for (entity, mut enemy, mut timer) in entity_q.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            enemy.status = Status::Idle;
            enemy.can_attack = true;
            commands.entity(entity).remove::<AttackTimer>();
        }
    }
}
