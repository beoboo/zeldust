use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::{Attackable, Player};
use crate::weapon::{PlayerWeapon, Weapon};

pub fn handle_weapon_collisions(
    mut commands: Commands,
    mut contact_events: EventReader<CollisionEvent>,
    mut player_q: Query<&Player>,
    mut weapon_q: Query<(Entity, &Weapon), With<PlayerWeapon>>,
    mut object_q: Query<(&Parent, &mut Attackable)>,
) {
    let player = player_q.single();

    if let Ok((w, weapon)) = weapon_q.get_single_mut() {
        for contact_event in contact_events.iter() {
            // for (entity, pin) in query.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                let other = if &w == h1 { h2 } else { h1};

                if let Ok((parent, mut attackable)) = object_q.get_mut(*other) {
                    let remaining_health = attackable.update(player.damage() + weapon.damage());
                    println!("Remaining health: {remaining_health}");

                    if remaining_health == 0 {
                        commands.entity(parent.get()).despawn_recursive();
                    }
                };
            }
        }
    }
}
