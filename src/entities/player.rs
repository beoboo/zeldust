use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;
use std::time::Duration;

use crate::{
    collisions::PLAYER_MOVE_COLLISION_GROUP,
    constants::{ANIMATION_DURATION, ATTACK_DURATION, TILE_SIZE},
    entities::{render_animation, AnimatedEntity, Animation, AttackTimer, Direction, Status},
    frames::TexturePack,
    from_position,
    weapon::PlayerWeapon,
    GameAssetType,
    GameAssets,
};

#[derive(Component, Reflect)]
pub struct Player {
    pub speed: f32,
    pub status: Status,
    pub direction: Direction,
    pub frame: usize,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 5.0,
            status: Status::Idle,
            direction: Direction::Down,
            frame: 0,
        }
    }
}

impl Player {
    pub fn attack_cooldown(&self) -> Duration {
        ATTACK_DURATION
    }

    pub fn damage(&self) -> u32 {
        10
    }

    pub fn is_moving(&self) -> bool {
        matches!(self.status, Status::Move(_))
    }

    pub fn is_attacking(&self) -> bool {
        self.status == Status::Attack
    }
}

impl AnimatedEntity for Player {
    fn asset_name(&self) -> String {
        "player".to_string()
    }

    fn texture_name(&self) -> String {
        let postfix = if self.is_moving() { "_0" } else { "" };

        format!("player/{}/{}{postfix}.png", self.status, self.direction)
    }

    fn num_frames(&self) -> usize {
        match self.status {
            Status::Attack => 1,
            Status::Idle => 1,
            Status::Move(_) => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StatType {
    Health,
    Energy,
    Attack,
    Magic,
    Speed,
}

pub struct Stat {
    pub value: u32,
    pub max: u32,
}

impl Stat {
    pub fn new(max: u32) -> Self {
        Self { value: max / 2, max }
    }

    pub fn ratio(&self) -> f32 {
        self.value as f32 / self.max as f32
    }
}

#[derive(Component)]
pub struct Stats {
    stats: HashMap<StatType, Stat>,
}

impl Default for Stats {
    fn default() -> Self {
        let mut stats = HashMap::default();

        stats.insert(StatType::Attack, Stat::new(10));
        stats.insert(StatType::Energy, Stat::new(60));
        stats.insert(StatType::Health, Stat::new(100));
        stats.insert(StatType::Magic, Stat::new(4));
        stats.insert(StatType::Speed, Stat::new(6));

        Self { stats }
    }
}

impl Stats {
    pub fn ratio(&self, ty: StatType) -> f32 {
        self.stats[&ty].ratio()
    }
}

pub fn spawn_player(commands: &mut Commands, window: &Window, assets: &Res<GameAssets>, x: f32, y: f32) {
    commands
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: assets.get(GameAssetType::Player).clone(),
                transform: Transform::from_translation(from_position(x, y, window)),
                ..Default::default()
            },
            Player::default(),
            RigidBody::Dynamic,
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            Stats::default(),
            Animation::new(ANIMATION_DURATION),
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 4.0),
                Transform::from_xyz(0.0, -TILE_SIZE / 4.0, 0.0),
                ColliderDebugColor(Color::RED),
                PLAYER_MOVE_COLLISION_GROUP.clone(),
            ));
        });
}

pub fn render_player(
    mut query: Query<(&Player, &mut Animation, &mut TextureAtlasSprite)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    textures: Res<Assets<TexturePack>>,
) {
    let (player, mut animation, mut sprite) = query.single_mut();

    render_animation(player, &mut animation, &mut sprite, &time, &asset_server, &textures);
}

pub fn end_player_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut player_q: Query<(Entity, &mut Player, &mut AttackTimer)>,
    weapon_q: Query<Entity, With<PlayerWeapon>>,
) {
    if let Ok((entity, mut player, mut timer)) = player_q.get_single_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            player.status = Status::Idle;
            commands.entity(entity).remove::<AttackTimer>();

            if let Ok(weapon) = weapon_q.get_single() {
                commands.entity(weapon).despawn();
            };
        }
    }
}
