use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::*;
use parse_display::Display;
use std::time::Duration;

use crate::{
    collisions::{ENEMY_ATTACK_COLLISION_GROUP, ENEMY_MOVE_COLLISION_GROUP},
    constants::{ANIMATION_DURATION, ATTACK_DURATION, SPEED, TILE_SIZE},
    entities::{render_animation, AnimatedEntity, Animation, AttackTimer, Attackable, HitTimer, Player, Status},
    frames::TexturePack,
    from_position,
    GameAssetType,
    GameAssets,
};

#[derive(Debug, Clone, Copy, Display, PartialEq)]
#[display(style = "snake_case")]
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
    can_move: bool,
    frame: usize,
}

impl Enemy {
    pub fn new(ty: EnemyType) -> Self {
        Self {
            ty,
            status: Status::Idle,
            can_attack: true,
            can_move: true,
            frame: 0,
        }
    }

    pub fn attack_cooldown(&self) -> Duration {
        ATTACK_DURATION
    }

    pub fn is_attacking(&self) -> bool {
        self.status == Status::Attack
    }

    pub fn can_move(&self) -> bool {
        self.can_move
    }

    pub fn hit(&mut self) {
        self.can_move = false;
    }

    pub fn attack_type(&self) -> AttackType {
        match self.ty {
            EnemyType::Squid => AttackType::Slash,
            EnemyType::Raccoon => AttackType::Claw,
            EnemyType::Spirit => AttackType::Thunder,
            EnemyType::Bamboo => AttackType::Leaf,
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

    pub fn resistance(&self) -> f32 {
        match self.ty {
            EnemyType::Squid => 3.0,
            EnemyType::Raccoon => 3.0,
            EnemyType::Spirit => 3.0,
            EnemyType::Bamboo => 3.0,
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

impl AnimatedEntity for Enemy {
    fn asset_name(&self) -> String {
        "monsters".to_string()
    }

    fn texture_name(&self) -> String {
        format!("monsters/{}/{}/00.png", self.ty, self.status)
    }

    fn num_frames(&self) -> usize {
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

    let name = format!("monsters/{ty}/idle/00.png");
    let handle = asset_server.load("textures/monsters.json");

    let pack = textures.get(&handle).expect("Texture pack must exist");
    let index = pack.index_of(&name);

    let atlas_handle = assets.get(GameAssetType::Monsters);
    let atlas = atlases.get(atlas_handle).unwrap();
    let rect = atlas.textures[index];
    let offset = (rect.height() - TILE_SIZE) / 2.0;
    let y = y - offset;
    let collider_height = (rect.height() - TILE_SIZE / 4.0) / 2.0;
    let collider_width = (rect.width() - TILE_SIZE / 4.0) / 2.0;

    let enemy = Enemy::new(ty);
    let health = enemy.health();

    let transform = Transform::from_translation(from_position(x, y, window));

    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(index),
                texture_atlas: assets.get(GameAssetType::Monsters).clone(),
                transform,
                ..Default::default()
            },
            RigidBody::Dynamic,
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Animation::new(ANIMATION_DURATION),
            enemy,
        ))
        .with_children(|parent| {
            // Collider for attacks
            parent.spawn((
                Attackable::new(health),
                Collider::cuboid(rect.width() / 2.0, rect.height() / 2.0),
                ENEMY_ATTACK_COLLISION_GROUP.clone(),
                Sensor,
                ColliderDebugColor(Color::RED),
            ));

            // Collider for movements
            parent.spawn((
                Collider::cuboid(collider_width, collider_height),
                ENEMY_MOVE_COLLISION_GROUP.clone(),
                ColliderDebugColor(Color::DARK_GRAY),
            ));
        });
}

pub fn move_enemy(
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    mut enemy_q: Query<(Entity, &mut Enemy, &Transform, &mut Velocity, &mut Animation)>,
) {
    let player_transform = player_q.single();

    for (entity, mut enemy, transform, mut velocity, mut animation) in enemy_q.iter_mut() {
        if enemy.is_attacking() || !enemy.can_move() {
            continue;
        }

        let diff = player_transform.translation - transform.translation;
        let distance = diff.length();
        let direction = diff.xy().normalize_or_zero() * enemy.speed() * SPEED;

        let mut status = Status::Idle;

        if distance < enemy.attack_radius() && enemy.can_attack {
            velocity.linvel = Vec2::ZERO;
            enemy.can_attack = false;
            status = Status::Attack;

            commands
                .entity(entity)
                .insert(AttackTimer(Timer::new(enemy.attack_cooldown(), TimerMode::Once)));
        } else if distance < enemy.notice_radius() {
            velocity.linvel = direction.into();
            status = Status::Move(direction);
        } else {
            velocity.linvel = Vec2::ZERO;
        }

        if status != enemy.status {
            animation.stop();

            enemy.status = status;
        }
    }
}

pub fn render_enemy(
    time: Res<Time>,
    mut query: Query<(&Enemy, &mut Animation, &mut TextureAtlasSprite)>,
    asset_server: Res<AssetServer>,
    textures: Res<Assets<TexturePack>>,
) {
    for (enemy, mut animation, mut sprite) in query.iter_mut() {
        render_animation(enemy, &mut animation, &mut sprite, &time, &asset_server, &textures);
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

pub fn handle_enemy_hit(
    mut commands: Commands,
    time: Res<Time>,
    mut entity_q: Query<(Entity, &mut Enemy, &mut HitTimer, &mut TextureAtlasSprite)>,
) {
    let delta = time.delta();
    let elapsed = time.elapsed();

    for (entity, mut enemy, mut timer, mut sprite) in entity_q.iter_mut() {
        timer.0.tick(delta);

        if timer.0.finished() {
            enemy.can_move = true;
            sprite.color.set_a(1.0);
            commands.entity(entity).remove::<HitTimer>();
        } else {
            let alpha = elapsed.as_micros() as f32;
            let alpha = alpha.sin();

            sprite.color.set_a(alpha);
        }
    }
}
