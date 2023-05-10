use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    collisions::MAGIC_COLLISION_GROUP,
    constants::{ANIMATION_DURATION, TILE_SIZE},
    entities::{AttackType, Enemy, EnemyType},
    events::EmitParticleEffect,
    frames::TexturePack,
    magic::{Magic, PlayerMagic},
    GameAssetType,
    GameAssets,
};

#[derive(Component)]
pub struct ParticleEffectAnimation {
    name: String,
    current_frame: usize,
    num_frames: usize,
    timer: Timer,
    finished: bool,
    offset: Vec3,
}

impl ParticleEffectAnimation {
    pub fn new(name: String, num_frames: usize, duration: Duration, offset: Vec3) -> Self {
        let timer = Timer::new(duration, TimerMode::Repeating);

        Self {
            name,
            current_frame: 0,
            num_frames,
            timer,
            finished: false,
            offset,
        }
    }

    pub fn next_frame(&mut self, delta: Duration) -> usize {
        self.timer.tick(delta);

        if self.timer.just_finished() {
            self.current_frame += 1;
            if self.current_frame == self.num_frames {
                self.finished = true;
            }
        }

        self.current_frame
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }
}

pub enum ParticleEffect {
    Aura,
    EnemyAttack(Enemy),
    EnemyDeath(Enemy),
    Flame,
    Heal,
    Leaf,
}

impl ParticleEffect {
    pub fn texture_name(&self) -> String {
        match self {
            Self::Aura => "aura".to_string(),
            Self::Leaf => {
                let mut rng = rand::thread_rng();
                format!("leaf{}", rng.gen_range(1..7))
            },
            Self::EnemyAttack(enemy) => format!("{}_attack", enemy.attack_type()),
            Self::EnemyDeath(enemy) => format!("{}_death", enemy.ty),
            Self::Flame => "flame".to_string(),
            Self::Heal => "heal".to_string(),
        }
    }

    pub fn is_flippable(&self) -> bool {
        matches!(self, Self::Leaf)
    }

    pub fn num_frames(&self, name: &str) -> usize {
        match self {
            Self::Aura => 4,
            Self::Leaf => match name {
                "leaf1" => 11,
                "leaf2" => 12,
                "leaf3" => 9,
                "leaf4" => 10,
                "leaf5" => 9,
                "leaf6" => 11,
                s => panic!("Unknown {s} particle"),
            },
            Self::EnemyAttack(enemy) => match enemy.attack_type() {
                AttackType::Claw => 4,
                AttackType::Leaf => 7,
                AttackType::Slash => 4,
                AttackType::Thunder => 8,
            },
            Self::EnemyDeath(enemy) => match enemy.ty {
                EnemyType::Bamboo => 2,
                EnemyType::Raccoon => 6,
                EnemyType::Spirit => 6,
                EnemyType::Squid => 6,
            },
            Self::Flame => 12,
            Self::Heal => 5,
        }
    }

    pub fn pos(&self) -> Vec3 {
        match self {
            Self::Leaf => Vec3::new(0., TILE_SIZE, 1.),
            _ => Vec3::Z,
        }
    }
}

pub fn spawn_particles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    textures: Res<Assets<TexturePack>>,
    mut particle_effect_reader: EventReader<EmitParticleEffect>,
) {
    for event in particle_effect_reader.iter() {
        let handle = asset_server.load(format!("textures/particles.json"));
        let pack = textures.get(&handle).expect("Texture pack must exist");
        let name = event.ty.texture_name();
        let particle = &event.ty;
        let num_frames = particle.num_frames(&name);

        let asset_name = format!("particles/{name}/00.png");
        let index = pack.index_of(&asset_name);
        let frame = &pack.frames[&asset_name];

        let atlas_handle = assets.get(GameAssetType::Particles);

        let flip_x = if particle.is_flippable() {
            let mut rng = rand::thread_rng();

            rng.gen_range(0..=1) == 1
        } else {
            false
        };

        let mut sprite = TextureAtlasSprite::new(index);
        sprite.flip_x = flip_x;

        let mut cmd = commands.spawn((
            SpriteSheetBundle {
                sprite,
                texture_atlas: atlas_handle.clone(),
                transform: Transform::from_translation(event.pos + particle.pos()),
                ..Default::default()
            },
            RigidBody::Dynamic,
            LockedAxes::all(),
            ParticleEffectAnimation::new(name, num_frames, ANIMATION_DURATION, event.offset),
        ));

        match particle {
            ParticleEffect::Flame => {
                cmd.insert((
                    Magic::Flame,
                    PlayerMagic,
                    Collider::cuboid(frame.frame.w / 2.0, frame.frame.h / 2.0),
                    MAGIC_COLLISION_GROUP.clone(),
                    ActiveEvents::COLLISION_EVENTS,
                    ColliderDebugColor(Color::INDIGO),
                ));
            },
            _ => {},
        }
    }
}

pub fn animate_particles(
    mut commands: Commands,
    mut particle_q: Query<(
        Entity,
        &mut TextureAtlasSprite,
        &mut ParticleEffectAnimation,
        &mut Transform,
    )>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    textures: Res<Assets<TexturePack>>,
) {
    let handle = asset_server.load(format!("textures/particles.json"));
    let pack = textures.get(&handle).expect("Texture pack must exist");

    let delta = time.delta();
    for (entity, mut sprite, mut animation, mut transform) in particle_q.iter_mut() {
        let index = animation.next_frame(delta);
        let name = &animation.name;

        if animation.is_finished() {
            commands.entity(entity).despawn_recursive();
        } else {
            let index = pack.index_of(&format!("particles/{name}/{index:02}.png"));
            if index != sprite.index {
                transform.translation += animation.offset * TILE_SIZE;

                sprite.index = index;
            }
        }
    }
}
