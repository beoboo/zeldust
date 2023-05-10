use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::Position;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 5.0,
        }
    }
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Position)>
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
    }
}