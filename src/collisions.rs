use bevy::{log, math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;

use crate::{
    constants::{HIT_DURATION, SPEED},
    entities::{Attackable, Enemy, HitTimer, Player},
    events::{EmitParticleEffect, MagicCollision, PlayerCollision, WeaponCollision},
    magic::PlayerMagic,
    particles::ParticleEffect,
    weapon::{PlayerWeapon, Weapon},
};
use crate::magic::Magic;

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
    player_q: Query<(Entity, &Children), With<Player>>,
    magic_q: Query<Entity, (With<PlayerMagic>, Without<Player>)>,
    weapon_q: Query<Entity, (With<PlayerWeapon>, Without<Player>)>,
    mut player_collision_writer: EventWriter<PlayerCollision>,
    mut magic_collision_writer: EventWriter<MagicCollision>,
    mut weapon_collision_writer: EventWriter<WeaponCollision>,
) {
    let (player, children) = player_q.single();

    for contact_event in contact_events.iter() {
        // println!("Contact: {contact_event:?}");
        if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
            let mut with_magic = false;
            for magic in magic_q.iter() {
                // log::info!("Magic: {magic:?}");
                if h1 == &magic || h2 == &magic {
                    log::info!("Magic collision: {h1:?} {h2:?}");
                    magic_collision_writer.send(MagicCollision::new(
                        magic.clone(),
                        if &magic == h1 { h2.clone() } else { h1.clone() },
                    ));

                    with_magic = true;
                    break;
                }
            }

            if with_magic {
                continue;
            }

            if let Ok(weapon) = weapon_q.get_single() {
                log::info!("Weapon: {weapon:?}");
                if h1 == &weapon || h2 == &weapon {
                    log::info!("Weapon collision: {h1:?} {h2:?}");
                    weapon_collision_writer.send(WeaponCollision::new(
                        weapon.clone(),
                        if &weapon == h1 { h2.clone() } else { h1.clone() },
                    ));

                    continue;
                }
            }

            if children.contains(&h1) || children.contains(h2) {
                log::info!("Player collision: {h1:?} {h2:?}");
                player_collision_writer.send(PlayerCollision::new(
                    player.clone(),
                    if children.contains(h1) { h2.clone() } else { h1.clone() },
                ));

                continue;
            }

            log::info!("Unknown collision ({h1:?} : {h2:?})");
        }
    }
}

pub fn handle_magic_collisions(
    mut commands: Commands,
    player_q: Query<(&Player, &Transform)>,
    mut magic_q: Query<&Magic, With<PlayerMagic>>,
    mut attackable_q: Query<(&Parent, &mut Attackable)>,
    mut parent_q: Query<&Transform>,
    mut enemy_q: Query<(&mut Enemy, &Transform, &mut Velocity)>,
    mut magic_collision_reader: EventReader<MagicCollision>,
    mut particle_effect_writer: EventWriter<EmitParticleEffect>,
) {
    let (player, player_transform) = player_q.single();

    for event in magic_collision_reader.iter() {
        let Ok(magic) = magic_q.get(event.magic) else {
            return;
        };

        // println!("handling magic collision");

        if let Ok((parent, mut attackable)) = attackable_q.get_mut(event.other) {
            let remaining_health = attackable.hit(player.damage() + magic.strength());
            // println!("Remaining health: {remaining_health}");

            if remaining_health == 0 {
                let transform = parent_q.get(parent.get()).expect("Parent must exist");

                let effect = if let Ok((enemy, _, _)) = enemy_q.get_mut(parent.get()) {
                    ParticleEffect::EnemyDeath(enemy.clone())
                } else {
                    ParticleEffect::Leaf
                };

                particle_effect_writer.send(EmitParticleEffect::new(effect, transform.translation));
                commands.entity(parent.get()).despawn_recursive();
            } else {
                // If it's an enemy, bump it back
                if let Ok((mut enemy, enemy_transform, mut velocity)) = enemy_q.get_mut(parent.get()) {
                    let direction = player_transform.translation - enemy_transform.translation;

                    velocity.linvel = -direction.xy().normalize_or_zero() * enemy.resistance() * SPEED;

                    enemy.hit();
                    commands
                        .entity(parent.get())
                        .insert(HitTimer(Timer::new(HIT_DURATION, TimerMode::Once)));
                } else {
                    log::info!("Unknown collision");
                }
            }
        };
    }
}

pub fn handle_player_collisions(
    mut commands: Commands,
    mut player_q: Query<(&mut Player, &Transform)>,
    mut attackable_q: Query<&Parent, With<Attackable>>,
    mut enemy_q: Query<&Enemy>,
    mut player_collision_reader: EventReader<PlayerCollision>,
    mut particle_effect_writer: EventWriter<EmitParticleEffect>,
) {
    let (mut player, transform) = player_q.single_mut();

    for event in player_collision_reader.iter() {
        let Ok(parent) = attackable_q.get(event.other) else {
            // Not attackable, bailing out...
            continue;
        };

        let Ok(enemy) = enemy_q.get(parent.get()) else {
            // Not an enemy, bailing out...
            continue;
        };

        player.hit(enemy.damage());
        commands
            .entity(event.player)
            .insert(HitTimer(Timer::new(HIT_DURATION, TimerMode::Once)));

        particle_effect_writer.send(EmitParticleEffect::new(
            ParticleEffect::EnemyAttack(enemy.clone()),
            transform.translation,
        ));
    }
}

pub fn handle_weapon_collisions(
    mut commands: Commands,
    player_q: Query<(&Player, &Transform)>,
    mut weapon_q: Query<&Weapon, With<PlayerWeapon>>,
    mut attackable_q: Query<(&Parent, &mut Attackable)>,
    mut parent_q: Query<&Transform>,
    mut enemy_q: Query<(&mut Enemy, &Transform, &mut Velocity)>,
    mut weapon_collision_reader: EventReader<WeaponCollision>,
    mut particle_effect_writer: EventWriter<EmitParticleEffect>,
) {
    let (player, player_transform) = player_q.single();

    for event in weapon_collision_reader.iter() {
        println!("handling weapon collision");
        let Ok(weapon) = weapon_q.get_single_mut() else {
            return;
        };

        if let Ok((parent, mut attackable)) = attackable_q.get_mut(event.other) {
            let remaining_health = attackable.hit(player.damage() + weapon.damage());
            println!("Remaining health: {remaining_health}");

            if remaining_health == 0 {
                let transform = parent_q.get(parent.get()).expect("Parent must exist");

                let effect = if let Ok((enemy, _, _)) = enemy_q.get_mut(parent.get()) {
                    ParticleEffect::EnemyDeath(enemy.clone())
                } else {
                    ParticleEffect::Leaf
                };

                particle_effect_writer.send(EmitParticleEffect::new(effect, transform.translation));
                commands.entity(parent.get()).despawn_recursive();
            } else {
                // If it's an enemy, bump it back
                if let Ok((mut enemy, enemy_transform, mut velocity)) = enemy_q.get_mut(parent.get()) {
                    let direction = player_transform.translation - enemy_transform.translation;

                    velocity.linvel = -direction.xy().normalize_or_zero() * enemy.resistance() * SPEED;

                    enemy.hit();
                    commands
                        .entity(parent.get())
                        .insert(HitTimer(Timer::new(HIT_DURATION, TimerMode::Once)));
                } else {
                    log::info!("Unknown collision");
                }
            }
        };
    }
}
