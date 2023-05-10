use std::time::Duration;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use crate::{from_position, from_translation, GameAssets, Layer, MapSize, Position, Size, StaticCollider};

#[derive(Component, Reflect)]
pub struct Player {
    speed: f32,
    status: Status,
    direction: Direction,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 500.0,
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
    Down,
}

#[derive(Debug)]
pub struct PlayerPositionEvent(Position);

pub fn spawn_player(
    commands: &mut Commands,
    window: &Window,
    assets: &Res<GameAssets>,
    map_size: &Res<MapSize>,
) {
    // println!("spawn player");

    let (x, y) = (map_size.width as f32 / 2., map_size.height as f32 / 2.);
    let position = Position { x, y };

    let translation = from_position(&position, window);

    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(4),
            texture_atlas: assets.player.clone(),
            transform: Transform::from_translation(translation),
            ..Default::default()
        },
        position,
        Layer(1),
        Player::default(),
        RigidBody::Dynamic,
        GravityScale(0.0),
        LockedAxes::ROTATION_LOCKED,
        ActiveEvents::COLLISION_EVENTS,
        Velocity::zero(),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Size::default(),
    )).with_children(|parent| {
        parent.spawn((
            Collider::cuboid(32.0, 16.0),
            Transform::from_xyz(0.0, -16.0, 0.0),
            ColliderDebugColor(Color::YELLOW_GREEN),
        ));
    });
}

pub fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Velocity, &mut AnimationTimer), Without<StaticCollider>>,
) {
    let mut vec = Vec2::default();

    let (mut player, mut velocity, mut animation_timer) = query.single_mut();

    for key in keyboard_input.get_pressed() {
        match key {
            KeyCode::Left => {
                vec.x = -1.0;
                player.direction = Direction::Left;
            }
            KeyCode::Right => {
                vec.x = 1.0;
                player.direction = Direction::Right;
            }
            KeyCode::Up => {
                vec.y = 1.0;
                player.direction = Direction::Up;
            }
            KeyCode::Down => {
                vec.y = -1.0;
                player.direction = Direction::Down;
            }
            _ => ()
        }
    }

    if vec != Vec2::ZERO {
        velocity.linvel = vec * player.speed;

        if player.status != Status::Moving {
            player.status = Status::Moving;
            animation_timer.pause();
        }
    } else {
        velocity.linvel = Vec2::ZERO;
        player.status = Status::Idle;
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

pub fn move_player(
    mut query: Query<(&mut Position, &Player, &Velocity), With<Player>>,
    mut position_writer: EventWriter<PlayerPositionEvent>,
) {
    let (mut position, player, velocity) = query.single_mut();

    position.x += player.speed * velocity.linvel.x;
    position.y += player.speed * velocity.linvel.y;

    position_writer.send(PlayerPositionEvent(*position));
}

pub fn update_player_position(
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Position, &mut Transform), With<Player>>,
    mut position_writer: EventWriter<PlayerPositionEvent>,
) {
    let Ok(window) = window.get_single() else { return; };

    let (mut position, mut transform) = query.single_mut();
    transform.translation.z = -transform.translation.y + 1000.0;

    *position = from_translation(transform.translation, window);

    position_writer.send(PlayerPositionEvent(*position));
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
                timer.0.set_duration(Duration::from_secs_f32(0.1));
                timer.0.reset();
                timer.0.unpause();
            } else {
                timer.0.tick(time.delta());
                if timer.0.just_finished() {
                    let mut current = sprite.index;
                    current = (current + 1) % 4;
                    sprite.index = index + current;
                }
            }
        }
    }
}