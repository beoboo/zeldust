use std::ops::Add;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    constants::SPEED,
    entities::{Animation, AttackTimer, CastSpellTimer, Direction, Player, Status},
    events::{SwitchMagic, SwitchWeapon},
    weapon::Weapon,
    StaticCollider,
};

pub fn handle_input(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Player, &mut Velocity, &mut Animation), Without<StaticCollider>>,
    mut switch_weapon: EventWriter<SwitchWeapon>,
    mut switch_magic: EventWriter<SwitchMagic>,
    weapon: Res<Weapon>,
) {
    let mut vec = Vec2::default();

    let (entity, mut player, mut velocity, mut animation) = query.single_mut();

    if player.is_attacking() || player.is_casting_spell() {
        velocity.linvel = Vec2::ZERO;
        return;
    }

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
                vec.y = 1.0;
                player.direction = Direction::Up;
            },
            KeyCode::Down => {
                vec.y = -1.0;
                player.direction = Direction::Down;
            },
            _ => (),
        }
    }

    let mut status = if vec == Vec2::ZERO {
        Status::Idle
    } else {
        Status::Move(vec)
    };

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::Space => {
                status = Status::Attack;
                commands.entity(entity).insert(AttackTimer(Timer::new(
                    player.attack_cooldown().add(weapon.cooldown()),
                    TimerMode::Once,
                )));
            },
            KeyCode::LControl => {
                status = Status::CastSpell;
                commands
                    .entity(entity)
                    .insert(CastSpellTimer(Timer::new(player.attack_cooldown(), TimerMode::Once)));
            },
            KeyCode::Q => {
                switch_weapon.send(SwitchWeapon);
            },
            KeyCode::E => {
                switch_magic.send(SwitchMagic);
            },
            _ => (),
        }
    }

    if player.status != status {
        player.status = status;
        animation.stop();

        if player.is_moving() {
            velocity.linvel = vec * player.speed.value() as f32 * SPEED;
        } else {
            velocity.linvel = Vec2::ZERO;
        }
    }
}
