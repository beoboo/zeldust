use bevy::{log, math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;

use crate::{
    constants::{HIT_DURATION, SPEED},
    entities::{Attackable, Enemy, HitTimer, Player},
    events::{PlayerCollision, WeaponCollision},
    weapon::{PlayerWeapon, Weapon},
};

// Player: GROUP_1
// Weapon: GROUP_2
// Objects: GROUP_3
// Enemies: GROUP_4

lazy_static! {
    pub static ref PLAYER_MOVE_COLLISION_GROUP: CollisionGroups =
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_3 | Group::GROUP_4);
    pub static ref WEAPON_COLLISION_GROUP: CollisionGroups =
        CollisionGroups::new(Group::GROUP_2, Group::GROUP_3 | Group::GROUP_4);
    pub static ref OBJECTS_COLLISION_GROUP: CollisionGroups =
        CollisionGroups::new(Group::GROUP_3, Group::GROUP_1 | Group::GROUP_2);
    pub static ref ENEMY_ATTACK_COLLISION_GROUP: CollisionGroups = CollisionGroups::new(Group::GROUP_4, Group::GROUP_1);
    pub static ref ENEMY_MOVE_COLLISION_GROUP: CollisionGroups = CollisionGroups::new(Group::GROUP_4, Group::GROUP_3);
}

pub fn handle_collisions(
    mut contact_events: EventReader<CollisionEvent>,
    player_q: Query<(Entity, &Children), With<Player>>,
    weapon_q: Query<Entity, (With<PlayerWeapon>, Without<Player>)>,
    mut player_collision_writer: EventWriter<PlayerCollision>,
    mut weapon_collision_writer: EventWriter<WeaponCollision>,
) {
    let (player, children) = player_q.single();

    for contact_event in contact_events.iter() {
        if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
            if children.contains(&h1) || children.contains(h2) {
                log::info!("Player collision");
                player_collision_writer.send(PlayerCollision::new(
                    player.clone(),
                    if children.contains(h1) { h2.clone() } else { h1.clone() },
                ));
            } else if let Ok(weapon) = weapon_q.get_single() {
                log::info!("Weapon collision");
                if h1 == &weapon || h2 == &weapon {
                    weapon_collision_writer.send(WeaponCollision::new(
                        weapon.clone(),
                        if &weapon == h1 { h2.clone() } else { h1.clone() },
                    ));
                }
            } else {
                log::info!("Unknown collision ({h1:?} : {h2:?})");
            }
        }
    }
}

pub fn handle_player_collisions(
    mut commands: Commands,
    mut player_q: Query<&mut Player>,
    mut enemy_q: Query<&Enemy>,
    mut player_collision_reader: EventReader<PlayerCollision>,
) {
    let mut player = player_q.single_mut();

    for event in player_collision_reader.iter() {
        let Ok(enemy) = enemy_q.get(event.other) else {
            continue;
        };
        player.hit(enemy.damage());
        commands
            .entity(event.player)
            .insert(HitTimer(Timer::new(HIT_DURATION, TimerMode::Once)));
    }
}

pub fn handle_weapon_collisions(
    mut commands: Commands,
    player_q: Query<(&Player, &Transform)>,
    mut weapon_q: Query<&Weapon, With<PlayerWeapon>>,
    mut attackable_q: Query<(&Parent, &mut Attackable)>,
    mut enemy_q: Query<(&mut Enemy, &Transform, &mut Velocity)>,
    mut weapon_collision_reader: EventReader<WeaponCollision>,
) {
    let (player, player_transform) = player_q.single();
    let Ok(weapon) = weapon_q.get_single_mut() else {
        return;
    };

    for event in weapon_collision_reader.iter() {
        if let Ok((parent, mut attackable)) = attackable_q.get_mut(event.other) {
            let remaining_health = attackable.hit(player.damage() + weapon.damage());
            // println!("Remaining health: {remaining_health}");

            if remaining_health == 0 {
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
