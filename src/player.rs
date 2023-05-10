use std::time::Duration;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use parse_display::Display;

use crate::{from_position, from_translation, GameAssets, MapSize, Position, Size, StaticCollider};
use crate::constants::ATTACK_COOLDOWN;
use crate::frames::TexturePack;

#[derive(Component, Deref)]
pub struct AttackTimer(Timer);

#[derive(Component, Reflect)]
pub struct Player {
    speed: f32,
    status: Status,
    direction: Direction,
    is_attacking: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 500.0,
            status: Status::Idle,
            direction: Direction::Down,
            is_attacking: false,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


#[derive(Debug, Clone, Copy, Display, PartialEq, Reflect)]
#[display(style = "snake_case")]
pub enum Status {
    Idle,
    Moving,
}

#[derive(Debug, Clone, Copy, Display, PartialEq, Reflect)]
#[display(style = "snake_case")]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug)]
pub struct PlayerPositionEvent(Position);

pub fn spawn_player(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    assets: Res<GameAssets>,
    map_size: Res<MapSize>,
) {
    // println!("spawn player");
    let Ok(window) = window.get_single() else { return; };

    let (x, y) = (map_size.width as f32 / 2., map_size.height as f32 / 2.);
    // let (x, y) = (0., 0.);
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
            ColliderDebugColor(Color::RED),
        ));
    });
}

pub fn handle_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Player, &mut Velocity, &mut AnimationTimer), Without<StaticCollider>>,
) {
    let mut vec = Vec2::default();

    let (entity, mut player, mut velocity, mut animation_timer) = query.single_mut();

    if player.is_attacking {
        return;
    }

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

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::Space | KeyCode::LControl => {
                player.is_attacking = true;
                commands.entity(entity).insert(AttackTimer(Timer::new(ATTACK_COOLDOWN, TimerMode::Once)));
            }
            _ => ()
        }
    }

    if vec != Vec2::ZERO && !player.is_attacking {
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

pub fn render_player(
    time: Res<Time>,
    mut query: Query<(&Player, &mut AnimationTimer, &mut TextureAtlasSprite)>,
    asset_server: Res<AssetServer>,
    textures: Res<Assets<TexturePack>>,
) {
    let (player, mut timer, mut sprite) = query.single_mut();

    let direction = player.direction;
    let mut status = player.status.to_string();
    if player.status == Status::Moving {
        status = String::from("0");
    }

    if player.is_attacking {
        status = String::from("attack");
    }

    let name = format!("{direction}_{status}.png");
    let handle = asset_server.load("textures/player.json");
    let pack = textures.get(&handle).expect("Texture pack must exist");
    let index = pack.index_of(&name);

    if !player.is_attacking && player.status == Status::Moving {
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
    } else {
        sprite.index = index;
    }
}

pub fn end_attack(
    mut commands: Commands, time: Res<Time>,
    mut query: Query<(Entity, &mut Player, &mut AttackTimer)>,
) {
    for (entity, mut player, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            player.is_attacking = false;
            commands.entity(entity).remove::<AttackTimer>();
        }
    }
}