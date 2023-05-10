use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::player::Player;

pub fn handle_collisions(
    mut contact_events: EventReader<CollisionEvent>,
    mut player_q: Query<(Entity, &mut Transform), With<Player>>,
    mut object_q: Query<&Transform, Without<Player>>,
) {
    let (player, mut player_transform) = player_q.single_mut();
    for contact_event in contact_events.iter() {
        // for (entity, pin) in query.iter() {
            if let CollisionEvent::Started(h1, h2, _event_flag) = contact_event {
                info!("h1: {h1:?}");
                info!("h2: {h2:?}");
                info!("player: {player:?}");
                if h2 == &player {
                    let object_transform = object_q.get_component::<Transform>(*h1).expect("Object without transform");
                    info!("{object_transform:?}");
                }
            }
        // }
    }}

