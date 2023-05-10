use std::{collections::HashMap, time::Duration};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use enum_iterator::{all, Sequence};
use parse_display::Display;

use crate::{
    collisions::PLAYER_MOVE_COLLISION_GROUP,
    constants::{ANIMATION_DURATION, ATTACK_DURATION, ENERGY_RECOVERY_DURATION, STARTING_XP, TILE_SIZE},
    entities::{
        from_position,
        render_animation,
        AnimatedEntity,
        Animation,
        AttackTimer,
        CastSpellTimer,
        Direction,
        EnergyRecoveryTimer,
        HitTimer,
        Status,
    },
    frames::TexturePack,
    stats::Stat,
    weapon::PlayerWeapon,
    GameAssetType,
    GameAssets,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Display, Sequence, Reflect, FromReflect, Component)]
pub enum PlayerStat {
    Damage,
    Energy,
    Health,
    Magic,
    Speed,
}

impl PlayerStat {
    pub fn start(&self) -> u32 {
        match self {
            Self::Damage => 10,
            Self::Energy => 48,
            Self::Health => 50,
            Self::Magic => 4,
            Self::Speed => 5,
        }
    }

    pub fn limit(&self) -> u32 {
        match self {
            Self::Damage => 10,
            Self::Energy => 60,
            Self::Health => 100,
            Self::Magic => 4,
            Self::Speed => 5,
        }
    }

    pub fn max(&self) -> u32 {
        match self {
            Self::Damage => 20,
            Self::Energy => 140,
            Self::Health => 300,
            Self::Magic => 10,
            Self::Speed => 10,
        }
    }

    pub fn cost(&self) -> u32 {
        match self {
            Self::Damage => 100,
            Self::Energy => 100,
            Self::Health => 100,
            Self::Magic => 100,
            Self::Speed => 100,
        }
    }
}

impl From<u32> for PlayerStat {
    fn from(value: u32) -> Self {
        match value {
            0 => PlayerStat::Damage,
            1 => PlayerStat::Energy,
            2 => PlayerStat::Health,
            3 => PlayerStat::Magic,
            _ => PlayerStat::Speed,
        }
    }
}

#[derive(Reflect)]
pub struct PlayerStats(HashMap<PlayerStat, Stat>);

impl Default for PlayerStats {
    fn default() -> Self {
        let stats = all::<PlayerStat>()
            .map(|s| (s, Stat::new(s.start(), s.limit(), s.max(), s.cost())))
            .collect::<HashMap<_, _>>();

        Self(stats)
    }
}

impl PlayerStats {
    pub fn get(&self, ty: PlayerStat) -> &Stat {
        &self.0[&ty]
    }

    pub fn get_mut(&mut self, ty: PlayerStat) -> &mut Stat {
        self.0.get_mut(&ty).unwrap()
    }

    pub fn limit(&self, ty: PlayerStat) -> u32 {
        self.get(ty).limit()
    }

    pub fn max(&self, ty: PlayerStat) -> u32 {
        self.get(ty).max()
    }

    pub fn value(&self, ty: PlayerStat) -> u32 {
        self.get(ty).value()
    }

    pub fn cost(&self, ty: PlayerStat) -> u32 {
        self.get(ty).cost()
    }

    pub fn ratio_by_limit_of(&self, ty: PlayerStat) -> f32 {
        self.0[&ty].ratio_by_limit()
    }

    pub fn set(&mut self, ty: PlayerStat, val: u32) {
        self.get_mut(ty).set(val)
    }

    pub fn upgrade(&mut self, ty: PlayerStat) {
        self.get_mut(ty).upgrade();
    }
}

#[derive(Component, Reflect)]
pub struct Player {
    pub xp: u32,
    pub status: Status,
    pub direction: Direction,
    pub frame: usize,
    pub can_cast_spell: bool,
    pub stats: PlayerStats,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            xp: STARTING_XP,
            status: Status::Idle,
            direction: Direction::Down,
            frame: 0,
            can_cast_spell: true,
            stats: PlayerStats::default(),
        }
    }
}

impl Player {
    pub fn attack_cooldown(&self) -> Duration {
        ATTACK_DURATION
    }

    pub fn damage(&self) -> u32 {
        self.stats.limit(PlayerStat::Damage)
    }

    pub fn is_moving(&self) -> bool {
        matches!(self.status, Status::Move(_))
    }

    pub fn is_attacking(&self) -> bool {
        matches!(self.status, Status::Attack)
    }

    pub fn is_casting_spell(&self) -> bool {
        matches!(self.status, Status::CastSpell)
    }

    pub fn hit(&mut self, damage: u32) {
        let health = self.stats.value(PlayerStat::Health);
        self.stats.set(PlayerStat::Health, health - damage);
    }

    pub fn cast_spell(&mut self, cost: u32) -> bool {
        let energy = self.stats.value(PlayerStat::Energy);

        if energy >= cost {
            self.stats.set(PlayerStat::Energy, energy - cost);
            true
        } else {
            false
        }
    }

    pub fn heal(&mut self, strength: u32) {
        let health = self.stats.value(PlayerStat::Health);
        self.stats.set(PlayerStat::Health, health + strength);
    }

    pub fn recover_energy(&mut self, amount: u32) {
        let energy = self.stats.value(PlayerStat::Energy);
        self.stats.set(PlayerStat::Energy, energy + amount);
    }

    pub fn value_by(&self, stat: PlayerStat) -> u32 {
        self.stats.value(stat)
    }

    pub fn cost_by(&self, stat: PlayerStat) -> u32 {
        self.stats.cost(stat)
    }

    pub fn limit_by(&self, stat: PlayerStat) -> u32 {
        self.stats.limit(stat)
    }

    pub fn max_by(&self, stat: PlayerStat) -> u32 {
        self.stats.max(stat)
    }

    pub fn upgrade(&mut self, stat: PlayerStat) {
        self.stats.upgrade(stat);
    }

    pub fn add_xp(&mut self, xp: u32) {
        self.xp += xp;
    }
}

impl AnimatedEntity for Player {
    fn asset_name(&self) -> String {
        "player".to_string()
    }

    fn texture_name(&self) -> String {
        let postfix = if self.is_moving() { "_0" } else { "" };

        let status_name = match self.status {
            Status::Idle => "idle",
            Status::Move(_) => "move",
            _ => "attack",
        };

        format!("player/{status_name}/{}{postfix}.png", self.direction)
    }

    fn num_frames(&self) -> usize {
        match self.status {
            Status::Attack => 1,
            Status::CastSpell => 1,
            Status::Idle => 1,
            Status::Move(_) => 4,
        }
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
            Velocity::zero(),
            Animation::new(ANIMATION_DURATION),
            EnergyRecoveryTimer(Timer::new(ENERGY_RECOVERY_DURATION, TimerMode::Repeating)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 4.0),
                Transform::from_xyz(0.0, -TILE_SIZE / 4.0, 0.0),
                ColliderDebugColor(Color::RED),
                PLAYER_MOVE_COLLISION_GROUP.clone(),
                ActiveEvents::COLLISION_EVENTS,
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

pub fn end_player_spell_cast(
    mut commands: Commands,
    time: Res<Time>,
    mut player_q: Query<(Entity, &mut Player, &mut CastSpellTimer)>,
) {
    if let Ok((entity, mut player, mut timer)) = player_q.get_single_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            player.status = Status::Idle;
            player.can_cast_spell = true;
            commands.entity(entity).remove::<CastSpellTimer>();
        }
    }
}

pub fn handle_player_hit(
    mut commands: Commands,
    time: Res<Time>,
    mut player_q: Query<(Entity, &mut HitTimer, &mut TextureAtlasSprite), With<Player>>,
) {
    let Ok((entity, mut timer, mut sprite)) = player_q.get_single_mut() else {
        return;
    };

    let delta = time.delta();
    let elapsed = time.elapsed();

    timer.0.tick(delta);

    if timer.0.finished() {
        sprite.color.set_a(1.0);
        commands.entity(entity).remove::<HitTimer>();
    } else {
        let alpha = elapsed.as_micros() as f32;
        let alpha = alpha.sin();

        sprite.color.set_a(alpha);
    }
}
