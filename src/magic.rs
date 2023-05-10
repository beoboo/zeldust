use bevy::prelude::*;
use parse_display::Display;
use rand::Rng;
use std::thread::current;

use crate::{
    collisions::{MAGIC_COLLISION_GROUP, OBJECTS_COLLISION_GROUP, PLAYER_MOVE_COLLISION_GROUP},
    constants::TILE_SIZE,
    entities::{EnergyRecoveryTimer, Player},
    events::{EmitParticleEffect, SwitchMagic},
    frames::TexturePack,
    particles::{spawn_particles, ParticleEffect},
    GameAssets,
};

#[derive(Component)]
pub struct PlayerMagic;

#[derive(Clone, Copy, Debug, Default, Display, Component, Resource, Reflect)]
#[display(style = "snake_case")]
pub enum Magic {
    #[default]
    Flame = 0,
    Heal = 1,
}

impl Magic {
    pub fn strength(&self) -> u32 {
        match self {
            Magic::Flame => 5,
            Magic::Heal => 20,
        }
    }

    pub fn cost(&self) -> u32 {
        match self {
            Magic::Flame => 20,
            Magic::Heal => 10,
        }
    }

    pub fn next(&self) -> Self {
        let index = *self as u8;
        let next = (index + 1) % 2;

        Self::from(next)
    }
}

impl From<u8> for Magic {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Flame,
            _ => Self::Heal,
        }
    }
}

pub fn cast_spell(
    current_magic: Res<Magic>,
    mut player_q: Query<(&mut Player, &Transform)>,
    mut particle_effect_writer: EventWriter<EmitParticleEffect>,
) {
    let (mut player, transform) = player_q.single_mut();

    if !player.is_casting_spell() || !player.can_cast_spell {
        return;
    }

    player.can_cast_spell = false;

    println!("Casting {}", *current_magic);
    //
    // println!("{} {}",
    //          MAGIC_COLLISION_GROUP.memberships.bits() & PLAYER_MOVE_COLLISION_GROUP.filters.bits() != 0,
    //          MAGIC_COLLISION_GROUP.filters.bits() & PLAYER_MOVE_COLLISION_GROUP.memberships.bits() != 0,
    // );

    if player.cast_spell(current_magic.cost()) {
        match *current_magic {
            Magic::Heal => {
                player.heal(current_magic.strength());
                particle_effect_writer.send(EmitParticleEffect::new(ParticleEffect::Aura, transform.translation));
                particle_effect_writer.send(EmitParticleEffect::new(ParticleEffect::Heal, transform.translation));
            },
            Magic::Flame => {
                let offset = player.direction.as_vec2().extend(0.);
                for i in 1..6 {
                    let mut offset = offset * i as f32 * TILE_SIZE;
                    let mut rng = rand::thread_rng();
                    offset.x += rng.gen_range(-TILE_SIZE / 3.0..TILE_SIZE / 3.0);
                    offset.y += rng.gen_range(-TILE_SIZE / 3.0..TILE_SIZE / 3.0);
                    particle_effect_writer.send(EmitParticleEffect::new(
                        ParticleEffect::Flame,
                        transform.translation + offset,
                    ));
                }
            },
        }
    }

    // let direction = player.direction;
    // let magic = *current_magic;
    //
    // let name = format!("{magic}_{direction}.png");
    // let handle = asset_server.load("textures/magics.json");
    // let pack = textures.get(&handle).expect("Texture pack must exist");
    // let index = pack.index_of(&name);
    // let frame = &pack.frames[&name];
    //
    // let y_offset = TILE_SIZE - (TILE_SIZE - frame.frame.h) / 2.0 - 4.0;
    // let x_offset = (TILE_SIZE + frame.frame.w) / 2.0;
    //
    // let translation = match direction {
    //     Direction::Down => Vec2::new(0.0, -y_offset),
    //     Direction::Left => Vec2::new(-x_offset, -TILE_SIZE / 4.0),
    //     Direction::Right => Vec2::new(x_offset, -TILE_SIZE / 4.0),
    //     Direction::Up => Vec2::new(0.0, y_offset),
    // };
    //
    // commands.entity(entity).with_children(|parent| {
    //     parent.spawn((
    //         SpriteSheetBundle {
    //             sprite: TextureAtlasSprite::new(index),
    //             texture_atlas: assets.magics.clone(),
    //             transform: Transform::from_translation(translation.extend(0.0)),
    //             ..Default::default()
    //         },
    //         *current_magic,
    //         PlayerMagic,
    //     ));
    // });
}

pub fn switch_magic(mut current_magic: ResMut<Magic>, mut reader: EventReader<SwitchMagic>) {
    for _ in reader.iter() {
        *current_magic = current_magic.next();
    }
}

pub fn recover_energy(time: Res<Time>, mut player_q: Query<(&mut Player, &mut EnergyRecoveryTimer)>) {
    let (mut player, mut timer) = player_q.single_mut();

    timer.0.tick(time.delta());

    if timer.0.finished() {
        player.energy += 1;
    }
}
