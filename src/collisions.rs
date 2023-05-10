use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::Attackable;
use crate::weapon::Weapon;

pub fn handle_weapon_collisions(
    mut contact_events: EventReader<CollisionEvent>,
    mut weapon_q: Query<Entity, With<Weapon>>,
    object_q: Query<&Transform, With<Attackable>>,
) {
    if let Ok(weapon) = weapon_q.get_single_mut() {
        for contact_event in contact_events.iter() {
            // for (entity, pin) in query.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                info!("h1: {h1:?}");
                info!("h2: {h2:?}");
                info!("weapon: {weapon:?}");
                // info!("player: {player:?}");
                // if h2 == &player {
                //     let object_transform = object_q
                //         .get_component::<Transform>(*h1)
                //         .expect("Object without transform");
                //     info!("{object_transform:?}");
                // }
            }
            // }
        }
    }
}
