use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::*;
use lazy_static::lazy_static;

use crate::{
    constants::{HIT_DURATION, SPEED},
    entities::{Attackable, Enemy, HitTimer, Player},
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
    pub static ref ENEMY_ATTACK_COLLISION_GROUP: CollisionGroups = CollisionGroups::new(Group::GROUP_4, Group::GROUP_2);
    pub static ref ENEMY_MOVE_COLLISION_GROUP: CollisionGroups = CollisionGroups::new(Group::GROUP_4, Group::GROUP_3);
    pub static ref OBJECTS_COLLISION_GROUP: CollisionGroups = CollisionGroups::new(Group::GROUP_4, Group::GROUP_2);
}

pub fn handle_weapon_collisions(
    mut commands: Commands,
    mut contact_events: EventReader<CollisionEvent>,
    player_q: Query<(&Player, &Transform)>,
    mut weapon_q: Query<(Entity, &Weapon), With<PlayerWeapon>>,
    mut sttackable_q: Query<(&Parent, &mut Attackable)>,
    mut enemy_q: Query<(&mut Enemy, &Transform, &mut Velocity)>,
) {
    let (player, player_transform) = player_q.single();

    if let Ok((w, weapon)) = weapon_q.get_single_mut() {
        for contact_event in contact_events.iter() {
            // for (entity, pin) in query.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                let other = if &w == h1 { h2 } else { h1 };

                if let Ok((parent, mut attackable)) = sttackable_q.get_mut(*other) {
                    let remaining_health = attackable.update(player.damage() + weapon.damage());
                    // println!("Remaining health: {remaining_health}");

                    if remaining_health == 0 {
                        commands.entity(parent.get()).despawn_recursive();
                    } else {
                        // If it's an enemy, bump it back
                        if let Ok((mut enemy, enemy_transform, mut velocity)) = enemy_q.get_mut(parent.get()) {
                            let direction = player_transform.translation - enemy_transform.translation;

                            velocity.linvel = -direction.xy().normalize_or_zero() * enemy.resistance() * SPEED;

                            // println!("Velocity: {velocity:?}");

                            enemy.hit();
                            commands
                                .entity(parent.get())
                                .insert(HitTimer(Timer::new(HIT_DURATION, TimerMode::Once)));
                        }
                    }
                };
            }
        }
    }
}
