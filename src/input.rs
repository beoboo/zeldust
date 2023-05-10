use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::FPS;
use crate::entities::Direction;
use crate::entities::{AnimationTimer, AttackTimer, Player, Status};
use crate::events::{SwitchMagic, SwitchWeapon};
use crate::weapon::Weapon;
use crate::StaticCollider;

pub fn handle_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<
        (Entity, &mut Player, &mut Velocity, &mut AnimationTimer),
        Without<StaticCollider>,
    >,
    mut switch_weapon: EventWriter<SwitchWeapon>,
    mut switch_magic: EventWriter<SwitchMagic>,
    weapon: Res<Weapon>,
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
            _ => (),
        }
    }

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::Space => {
                player.is_attacking = true;
                commands.entity(entity).insert(AttackTimer(Timer::new(
                    Duration::from_millis(weapon.cooldown() as _),
                    TimerMode::Once,
                )));
            }
            KeyCode::LControl => {
                player.is_attacking = true;
                // commands
                //     .entity(entity)
                //     .insert(MagicTimer(Timer::new(Duration::from_millis(weapon.cooldown() as _), TimerMode::Once)));
            }
            KeyCode::Q => {
                switch_weapon.send(SwitchWeapon);
            }
            KeyCode::E => {
                switch_magic.send(SwitchMagic);
            }
            _ => (),
        }
    }

    if vec != Vec2::ZERO && !player.is_attacking {
        velocity.linvel = vec * player.speed * FPS;

        if player.status != Status::Move {
            player.status = Status::Move;
            animation_timer.pause();
        }
    } else {
        velocity.linvel = Vec2::ZERO;
        player.status = Status::Idle;
    }
}
