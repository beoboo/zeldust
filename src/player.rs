use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::Position;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

pub struct PlayerPositionEvent(Position);

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 5.0,
        }
    }
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Position)>,
    mut position_writer: EventWriter<PlayerPositionEvent>,
) {
    let mut direction = Vec2::default();
    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::Left => direction.x = -1.0,
            KeyCode::Right => direction.x = 1.0,
            KeyCode::Up => direction.y = -1.0,
            KeyCode::Down => direction.y = 1.0,
            _ => ()
        }
    }

    if direction != Vec2::default() {
        let direction = direction.normalize();
        let (player, mut position) = query.single_mut();
        position.x += player.speed * direction.x;
        position.y += player.speed * direction.y;
        position_writer.send(PlayerPositionEvent(*position))
    }
}

pub fn move_camera(
    mut query: Query<&mut Position, With<Camera>>,
    mut position_reader: EventReader<PlayerPositionEvent>,
) {
    if let Some(player_position) = position_reader.iter().next() {
        let mut camera_position = query.single_mut();
        *camera_position = player_position.0;
    }
}