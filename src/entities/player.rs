use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::*;

use crate::constants::TILE_SIZE;
use crate::entities::{Direction, Status};
use crate::frames::TexturePack;
use crate::weapon::PlayerWeapon;
use crate::{from_position, GameAssetType, GameAssets};

#[derive(Component, Deref)]
pub struct AttackTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Reflect)]
pub struct Player {
    pub speed: f32,
    pub status: Status,
    pub direction: Direction,
    pub is_attacking: bool,
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
        Self {
            value: max / 2,
            max,
        }
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

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 5.0,
            status: Status::Idle,
            direction: Direction::Down,
            is_attacking: false,
        }
    }
}

pub fn spawn_player(
    commands: &mut Commands,
    window: &Window,
    assets: &Res<GameAssets>,
    x: f32,
    y: f32,
) {
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
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 4.0),
                Transform::from_xyz(0.0, -TILE_SIZE / 4.0, 0.0),
                ColliderDebugColor(Color::RED),
            ));
        });
}

pub fn update_player_position(mut query: Query<(&mut Transform, Ref<Velocity>), With<Player>>) {
    let (mut transform, previous) = query.single_mut();

    if previous.is_changed() {
        transform.translation.z = -transform.translation.y + 1000.0;
    }
}

pub fn move_camera(
    mut camera_q: Query<&mut Transform, With<Camera>>,
    player_q: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let player_transform = player_q.single();
    let mut camera_transform = camera_q.single_mut();
    camera_transform.translation = player_transform.translation;
}

pub fn render_player(
    time: Res<Time>,
    mut query: Query<(&Player, &mut AnimationTimer, &mut TextureAtlasSprite)>,
    asset_server: Res<AssetServer>,
    textures: Res<Assets<TexturePack>>,
) {
    let (player, mut timer, mut sprite) = query.single_mut();

    let direction = player.direction;
    let mut status = player.status.to_string();

    if player.is_attacking {
        status = String::from("attack");
    }

    let postfix = if player.status == Status::Move {
        "_0"
    } else {
        ""
    };

    let name = format!("player/{status}/{direction}{postfix}.png");
    let handle = asset_server.load("textures/player.json");
    let pack = textures.get(&handle).expect("Texture pack must exist");
    let index = pack.index_of(&name);

    if !player.is_attacking && player.status == Status::Move {
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

pub fn end_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut player_q: Query<(Entity, &mut Player, &mut AttackTimer)>,
    weapon_q: Query<Entity, With<PlayerWeapon>>,
) {
    for (entity, mut player, mut timer) in player_q.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            player.is_attacking = false;
            commands.entity(entity).remove::<AttackTimer>();

            if let Ok(weapon) = weapon_q.get_single() {
                commands.entity(weapon).despawn();
            };
        }
    }
}
