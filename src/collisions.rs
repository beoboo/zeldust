use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;

use crate::{
    constants::{HIT_DURATION, SPEED},
    entities::{Attackable, Enemy, HitTimer, Player},
    events::{DamageAttackable, EmitParticleEffect, KillAttackable, MagicCollision, PlayerCollision, WeaponCollision},
    magic::{Magic, PlayerMagic},
    particles::ParticleEffect,
    weapon::{PlayerWeapon, Weapon},
};
use crate::events::DamagePlayer;

// Player: GROUP_1
// Weapon: GROUP_2
// Enemies: GROUP_10
// Objects: GROUP_20

lazy_static! {
    pub static ref PLAYER_MOVE_COLLISION_GROUP: CollisionGroups =
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_10 | Group::GROUP_20);
    pub static ref MAGIC_COLLISION_GROUP: CollisionGroups =
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_10 | Group::GROUP_20);
    pub static ref WEAPON_COLLISION_GROUP: CollisionGroups =
        CollisionGroups::new(Group::GROUP_3, Group::GROUP_10 | Group::GROUP_20);
    pub static ref ENEMY_ATTACK_COLLISION_GROUP: CollisionGroups =
        CollisionGroups::new(Group::GROUP_10, Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_3);
    pub static ref ENEMY_MOVE_COLLISION_GROUP: CollisionGroups = CollisionGroups::new(Group::GROUP_11, Group::GROUP_20);
    pub static ref OBJECTS_COLLISION_GROUP: CollisionGroups =
        CollisionGroups::new(Group::GROUP_20, Group::GROUP_1 | Group::GROUP_2 | Group::GROUP_3);
}

pub fn handle_collisions(
    mut contact_events: EventReader<CollisionEvent>,
    magic_q: Query<Entity, (With<PlayerMagic>, Without<Player>)>,
    weapon_q: Query<Entity, (With<PlayerWeapon>, Without<Player>)>,
    parent_q: Query<&Parent>,
    mut magic_collision_writer: EventWriter<MagicCollision>,
    mut weapon_collision_writer: EventWriter<WeaponCollision>,
) {
    for contact_event in contact_events.iter() {
        match contact_event {
            CollisionEvent::Started(h1, h2, _event_flag) => {
                let mut with_magic = false;
                for magic in magic_q.iter() {
                    if h1 == &magic || h2 == &magic {
                        let other = if &magic == h1 { h2 } else { h1 };

                        magic_collision_writer.send(MagicCollision::new(
                            magic.clone(),
                            parent_q.get(other.clone()).expect("Parent must exist").get(),
                        ));

                        with_magic = true;
                        break;
                    }
                }

                if with_magic {
                    continue;
                }

                if let Ok(weapon) = weapon_q.get_single() {
                    if h1 == &weapon || h2 == &weapon {
                        let other = if &weapon == h1 { h2 } else { h1 };
                        weapon_collision_writer.send(WeaponCollision::new(
                            weapon.clone(),
                            parent_q.get(other.clone()).expect("Parent must exist").get(),
                        ));

                        continue;
                    }
                }
            },
            CollisionEvent::Stopped(_h1, _h2, _entity_flag) => {
                // Not handling this now
            },
        }
    }
}

pub fn handle_magic_collisions(
    player_q: Query<&Player>,
    magic_q: Query<&Magic, With<PlayerMagic>>,
    mut attackable_q: Query<(Entity, &mut Attackable)>,
    mut magic_collision_reader: EventReader<MagicCollision>,
    mut kill_attackable_writer: EventWriter<KillAttackable>,
    mut damage_attackable_writer: EventWriter<DamageAttackable>,
) {
    let player = player_q.single();

    for event in magic_collision_reader.iter() {
        let Ok(magic) = magic_q.get(event.magic) else {
            return;
        };

        attack_attackable(
            &mut attackable_q,
            &mut kill_attackable_writer,
            &mut damage_attackable_writer,
            player,
            magic.strength(),
            &event.other,
        );
    }
}

pub fn handle_weapon_collisions(
    player_q: Query<&Player, &Transform>,
    weapon_q: Query<&Weapon, With<PlayerWeapon>>,
    mut attackable_q: Query<(Entity, &mut Attackable)>,
    mut weapon_collision_reader: EventReader<WeaponCollision>,
    mut kill_attackable_writer: EventWriter<KillAttackable>,
    mut damage_attackable_writer: EventWriter<DamageAttackable>,
) {
    let player = player_q.single();

    for event in weapon_collision_reader.iter() {
        let Ok(weapon) = weapon_q.get_single() else {
            return;
        };

        attack_attackable(
            &mut attackable_q,
            &mut kill_attackable_writer,
            &mut damage_attackable_writer,
            player,
            weapon.damage(),
            &event.other,
        );
    }
}

fn attack_attackable(
    attackable_q: &mut Query<(Entity, &mut Attackable)>,
    kill_attackable_writer: &mut EventWriter<KillAttackable>,
    damage_attackable_writer: &mut EventWriter<DamageAttackable>,
    player: &Player,
    damage: u32,
    attacked: &Entity,
) {
    if let Ok((entity, mut attackable)) = attackable_q.get_mut(*attacked) {
        let remaining_health = attackable.hit(player.damage() + damage);

        if remaining_health == 0 {
            kill_attackable_writer.send(KillAttackable(entity.clone()));
        } else {
            damage_attackable_writer.send(DamageAttackable(entity.clone()));
        }
    }
}

pub fn kill_attackable(
    mut commands: Commands,
    mut player_q: Query<&mut Player>,
    mut parent_q: Query<&Transform>,
    attackable_q: Query<Entity, With<Attackable>>,
    mut enemy_q: Query<&mut Enemy>,
    mut kill_attackable_reader: EventReader<KillAttackable>,
    mut particle_effect_writer: EventWriter<EmitParticleEffect>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let mut player = player_q.single_mut();

    for event in kill_attackable_reader.iter() {
        if !attackable_q.contains(event.0) {
            continue;
        }

        let transform = parent_q.get(event.0).expect("Parent entity must exist");

        let effect = if let Ok(enemy) = enemy_q.get_mut(event.0) {
            player.add_xp(enemy.xp());
            audio.play(asset_server.load("audio/death.wav")).with_volume(0.4);
            ParticleEffect::EnemyDeath(enemy.clone())
        } else {
            ParticleEffect::Leaf
        };

        particle_effect_writer.send(EmitParticleEffect::new(effect, transform.translation));
        commands.entity(event.0).despawn_recursive();
    }
}

pub fn damage_attackable(
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    mut attackable_q: Query<Entity, With<Attackable>>,
    mut enemy_q: Query<(&mut Enemy, &Transform, &mut Velocity)>,
    mut damage_attackable_reader: EventReader<DamageAttackable>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let player_transform = player_q.single();

    for event in damage_attackable_reader.iter() {
        if !attackable_q.contains(event.0) {
            continue;
        }

        // If it's an enemy, bump it back
        if let Ok((mut enemy, enemy_transform, mut velocity)) = enemy_q.get_mut(event.0) {
            let direction = player_transform.translation - enemy_transform.translation;

            velocity.linvel = -direction.xy().normalize_or_zero() * enemy.resistance() * SPEED;

            enemy.hit();
            audio.play(asset_server.load("audio/hit.wav")).with_volume(0.4);

            commands
                .entity(event.0)
                .insert(HitTimer(Timer::new(HIT_DURATION, TimerMode::Once)));
        } else {
            info!("Unknown collision");
        }
    }
}

pub fn damage_player(
    mut commands: Commands,
    mut player_q: Query<(Entity, &mut Player, &Transform)>,
    enemy_q: Query<&Enemy>,
    mut damage_player_reader: EventReader<DamagePlayer>,
    mut particle_effect_writer: EventWriter<EmitParticleEffect>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let (player_e, mut player, transform) = player_q.single_mut();

    for event in damage_player_reader.iter() {
        let Ok(enemy) = enemy_q.get(event.0) else {
            // Not an enemy, bailing out...
            continue;
        };

        player.hit(enemy.damage());
        audio.play(asset_server.load(enemy.attack_type().sound()));

        commands
            .entity(player_e)
            .insert(HitTimer(Timer::new(HIT_DURATION, TimerMode::Once)));

        particle_effect_writer.send(EmitParticleEffect::new(
            ParticleEffect::EnemyAttack(enemy.clone()),
            transform.translation,
        ));
    }
}
