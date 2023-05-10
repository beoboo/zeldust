use bevy::prelude::*;

use crate::{constants::CAMERA_SCALE, entities::Player};

pub fn spawn_camera(mut commands: Commands) {
    // println!("spawn cameras");
    let camera = Camera2dBundle {
        projection: OrthographicProjection {
            scale: CAMERA_SCALE,
            near: -10000.0,
            far: 10000.0,
            ..default()
        },
        ..default()
    };

    commands.spawn(camera);
}

pub fn move_camera(
    mut camera_q: Query<&mut Transform, With<Camera>>,
    player_q: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let player_transform = player_q.single();
    let mut camera_transform = camera_q.single_mut();
    camera_transform.translation = player_transform.translation;
}
