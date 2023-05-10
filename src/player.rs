use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use crate::{MapBorder, Position, Size};

#[derive(Component)]
pub struct Player {
    speed: f32,
}

pub struct PlayerPositionEvent(Position);

enum Axis {
    Horizontal,
    Vertical,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 10.0,
        }
    }
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &Transform, &Size, &mut Position), Without<MapBorder>>,
    borders: Query<(&MapBorder, &Transform, &Size, &Position), Without<Player>>,
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
        let (player, transform, size, mut position) = players.single_mut();
        position.x += player.speed * direction.x;
        position.y += player.speed * direction.y;

        borders.iter().for_each(|(b, t, s, p)| {
            let v1 = Vec2::new(s.width, s.height);
            let v2 = Vec2::new(size.width, size.height);

            if let Some(collision) = collide(transform.translation, v2, t.translation, v1) {
                match collision {
                    Collision::Left => position.x = p.x - s.width,
                    Collision::Right => position.x = p.x + s.width,
                    Collision::Top => position.y = p.y - s.height,
                    Collision::Bottom => position.y = p.y + s.height,
                }
            }
        });

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