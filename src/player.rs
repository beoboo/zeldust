use std::time::Duration;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy_tileset::prelude::TileHandleType::Animated;
use bevy_tileset::prelude::{TileIndex, Tilesets};
use crate::{GameAssets, MapBorder, MapSize, Position, Size};

#[derive(Component)]
pub struct Player {
    speed: f32,
    status: Status,
    direction: Direction,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 10.0,
            status: Status::Idle,
            direction: Direction::Down,
        }
    }
}

#[derive(Debug)]
pub enum Status {
    Idle,
    Moving,
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down
}

pub struct PlayerPositionEvent(Position);

pub fn spawn_player(
    commands: &mut Commands,
    tilesets: Tilesets,
    assets: Res<GameAssets>,
    map_size: &Res<MapSize>,
) {
    // println!("spawn player");

    let (x, y) = (map_size.width as f32 / 2., map_size.height as f32 / 2.);

    let player_assets = tilesets.get(&assets.player).unwrap();

    let index = player_assets.get_tile_index("Player up idle").unwrap();
    let index = match index {
        TileIndex::Standard(index) => index,
        TileIndex::Animated(start, _, _) => start,
    };

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(index),
        texture_atlas: player_assets.atlas().clone_weak(),
        ..Default::default()
    })
        .insert(Player::default())
        .insert(Position { x, y, layer: 1 })
        .insert(Size::default())
        .insert(Timer::from_seconds(0.1, true))
    ;
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Player, &Transform, &Size, &mut Position), Without<MapBorder>>,
    borders: Query<(&MapBorder, &Transform, &Size, &Position), Without<Player>>,
    mut position_writer: EventWriter<PlayerPositionEvent>,
) {
    // println!("move player");

    let mut vec = Vec2::default();

    let (mut player, transform, size, mut position) = players.single_mut();

    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::Left => {
                vec.x = -1.0;
                player.direction = Direction::Left;
            },
            KeyCode::Right => {
                vec.x = 1.0;
                player.direction = Direction::Right;
            },
            KeyCode::Up => {
                vec.y = -1.0;
                player.direction = Direction::Up;
            },
            KeyCode::Down => {
                vec.y = 1.0;
                player.direction = Direction::Down;
            },
            _ => ()
        }
    }


    if vec != Vec2::default() {
        let vec = vec.normalize();
        position.x += player.speed * vec.x;
        position.y += player.speed * vec.y;
        player.status = Status::Moving;

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
    }
    else {
        player.status = Status::Idle;
    }
}

pub fn move_camera(
    mut query: Query<&mut Position, With<Camera>>,
    mut position_reader: EventReader<PlayerPositionEvent>,
) {
    // println!("move camera");

    if let Some(player_position) = position_reader.iter().next() {
        let mut camera_position = query.single_mut();
        *camera_position = player_position.0;
    }
}

pub fn animate_player(
    tilesets: Tilesets,
    assets: Res<GameAssets>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {

    let (player, mut timer, mut sprite, handle) = query.single_mut();
    let assets = tilesets.get(&assets.player).unwrap();

    let index = match player.status {
        Status::Idle => {
            match player.direction {
                Direction::Left => assets.get_tile_index("Player left idle").unwrap(),
                Direction::Right => assets.get_tile_index("Player right idle").unwrap(),
                Direction::Up => assets.get_tile_index("Player up idle").unwrap(),
                Direction::Down => assets.get_tile_index("Player down idle").unwrap(),
            }
        }
        Status::Moving => {
            match player.direction {
                Direction::Left => assets.get_tile_index("Player left moving").unwrap(),
                Direction::Right => assets.get_tile_index("Player right moving").unwrap(),
                Direction::Up => assets.get_tile_index("Player up moving").unwrap(),
                Direction::Down => assets.get_tile_index("Player down moving").unwrap(),
                _ => return
            }
        }
    };

    match index {
        TileIndex::Standard(index) => {
            timer.pause();
            sprite.index = index
        },
        TileIndex::Animated(start, end, speed) => {
            if timer.paused() {
                sprite.index = start;
                timer.set_duration(Duration::from_secs_f32(speed));
                timer.reset();
                timer.unpause();
            } else {
                timer.tick(time.delta());
                if timer.finished() {
                    sprite.index += 1;
                    if sprite.index > end || sprite.index < start {
                        sprite.index = start
                    }
                }
            }
        }
    }
}