use std::time::Duration;
use bevy::log;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy_tileset::prelude::{TileIndex, Tilesets};
use crate::{GameAssets, MapBorder, MapSize, Position, Size};

#[derive(Component, Reflect)]
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

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


#[derive(Debug, PartialEq, Reflect)]
pub enum Status {
    Idle,
    Moving,
}

#[derive(Debug, Reflect)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down
}

#[derive(Debug)]
pub struct PlayerPositionEvent(Position);

pub fn spawn_player(
    commands: &mut Commands,
    assets: Res<GameAssets>,
    map_size: &Res<MapSize>,
) {
    // println!("spawn player");

    let (x, y) = (map_size.width as f32 / 2., map_size.height as f32 / 2.);

    commands.spawn(SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(4),
        texture_atlas: assets.player.clone(),
        ..Default::default()
    })
        .insert(Player::default())
        .insert(Position { x, y, layer: 1 })
        .insert(Size::default())
        .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
    ;
}

pub fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut players: Query<(&mut Player, &Transform, &Size, &mut Position, &mut AnimationTimer), Without<MapBorder>>,
    borders: Query<(&MapBorder, &Transform, &Size, &Position), Without<Player>>,
    mut position_writer: EventWriter<PlayerPositionEvent>,
) {
    let mut vec = Vec2::default();

    let (mut player, transform, size, mut position, mut animation_timer) = players.single_mut();

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

    if vec != Vec2::ZERO {
        let vec = vec.normalize();
        position.x += player.speed * vec.x;
        position.y += player.speed * vec.y;

        if player.status != Status::Moving {
            player.status = Status::Moving;
            animation_timer.pause();
        }

        borders.iter().for_each(|(_, t, s, p)| {
            let v1 = Vec2::new(s.width, s.height);
            let v2 = Vec2::new(size.width, size.height);

            if let Some(collision) = collide(transform.translation, v2, t.translation, v1) {
                match collision {
                    Collision::Left => position.x = p.x - s.width,
                    Collision::Right => position.x = p.x + s.width,
                    Collision::Top => position.y = p.y - s.height,
                    Collision::Bottom => position.y = p.y + s.height,
                    Collision::Inside => {
                        unimplemented!()
                    }
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
    time: Res<Time>,
    mut query: Query<(&Player, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    let (player, mut timer, mut sprite) = query.single_mut();

    let index = match player.status {
        Status::Idle => {
            match player.direction {
                Direction::Down => 4,
                Direction::Left => 5,
                Direction::Right => 6,
                Direction::Up => 7,
            }
        }
        Status::Moving => {
            match player.direction {
                Direction::Down => 8,
                Direction::Left => 12,
                Direction::Right => 16,
                Direction::Up => 20,
            }
        }
    };

    match player.status {
        Status::Idle => sprite.index = index,
        Status::Moving => {
            if timer.0.paused() {
                sprite.index = index;
                log::info!("Unpausing to {}", index);
                timer.0.set_duration(Duration::from_secs_f32(0.1));
                timer.0.reset();
                timer.0.unpause();
            } else {
                timer.0.tick(time.delta());
                if timer.0.just_finished() {
                    let mut current = sprite.index;
                    current = (current + 1) % 4;
                    sprite.index = index + current;
                    log::info!("Moving to {}", sprite.index);
                }
            }
        }
    }
}