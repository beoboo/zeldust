use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    clamped::ClampedU32,
    collisions::PLAYER_MOVE_COLLISION_GROUP,
    constants::{ANIMATION_DURATION, ATTACK_DURATION, ENERGY_RECOVERY_DURATION, TILE_SIZE},
    entities::{
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
    from_position,
    weapon::PlayerWeapon,
    GameAssetType,
    GameAssets,
};

#[derive(Component, Reflect)]
pub struct Player {
    pub health: ClampedU32,
    pub energy: ClampedU32,
    pub speed: ClampedU32,
    pub damage: ClampedU32,
    pub magic: ClampedU32,
    pub status: Status,
    pub direction: Direction,
    pub frame: usize,
    pub can_cast_spell: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            health: ClampedU32::new(50, 100),
            energy: ClampedU32::new(48, 60),
            speed: ClampedU32::new(5, 5),
            damage: ClampedU32::new(10, 10),
            magic: ClampedU32::new(4, 4),
            status: Status::Idle,
            direction: Direction::Down,
            frame: 0,
            can_cast_spell: true,
        }
    }
}

impl Player {
    pub fn attack_cooldown(&self) -> Duration {
        ATTACK_DURATION
    }

    pub fn damage(&self) -> u32 {
        self.damage.value()
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
        self.health = self.health - damage;
    }

    pub fn cast_spell(&mut self, cost: u32) -> bool {
        if self.energy.value() >= cost {
            self.energy -= cost;
            true
        } else {
            false
        }
    }

    pub fn heal(&mut self, strength: u32) {
        self.health += strength;
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
