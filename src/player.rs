use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy_tileset::prelude::Tilesets;
use crate::{GameAssets, MapBorder, Position, Size};

#[derive(Component)]
pub struct Player {
    speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 10.0,
        }
    }
}

pub enum Direction {
    Idle,
    Left,
    Up,
    Right,
    Down
}

pub struct PlayerPositionEvent(Position);
pub struct PlayerDirectionEvent(Direction);

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &Transform, &Size, &mut Position), Without<MapBorder>>,
    borders: Query<(&MapBorder, &Transform, &Size, &Position), Without<Player>>,
    mut position_writer: EventWriter<PlayerPositionEvent>,
    mut direction_writer: EventWriter<PlayerDirectionEvent>,
) {
    let mut vec = Vec2::default();
    let mut direction = Direction::Left;

    for key in keyboard_input.get_pressed() {
        direction = match key {
            KeyCode::Left => {
                vec.x = -1.0;
                Direction::Left
            },
            KeyCode::Right => {
                vec.x = 1.0;
                Direction::Right
            },
            KeyCode::Up => {
                vec.y = -1.0;
                Direction::Up
            },
            KeyCode::Down => {
                vec.y = 1.0;
                Direction::Down
            },
            _ => Direction::Idle

        }
    }

    if vec != Vec2::default() {
        let (player, transform, size, mut position) = players.single_mut();
        let vec = vec.normalize();
        position.x += player.speed * vec.x;
        position.y += player.speed * vec.y;

        borders.iter().for_each(|(_, t, s, p)| {
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

        position_writer.send(PlayerPositionEvent(*position));
        direction_writer.send(PlayerDirectionEvent(direction));
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

pub fn animate_player(
    tilesets: Tilesets,
    assets: Res<GameAssets>,
    mut query: Query<&mut TextureAtlasSprite, With<Player>>,
    mut direction_reader: EventReader<PlayerDirectionEvent>,
) {
    if let Some(direction) = direction_reader.iter().next() {
        let assets = tilesets.get(&assets.player).unwrap();

        let (index, _) = match direction.0 {
            Direction::Left => assets.select_tile("Player left idle").unwrap(),
            Direction::Right => assets.select_tile("Player right idle").unwrap(),
            Direction::Up => assets.select_tile("Player up idle").unwrap(),
            Direction::Down => assets.select_tile("Player down idle").unwrap(),
            _ => return
        };

        let mut sprite = query.single_mut();
        sprite.index = *index.base_index();
    }
}