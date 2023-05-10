use std::time::Duration;

use bevy_kira_audio::{Audio, AudioControl};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use parse_display::Display;

use crate::{
    collisions::WEAPON_COLLISION_GROUP,
    constants::TILE_SIZE,
    entities::{Direction, Player},
    events::SwitchWeapon,
    frames::TexturePack,
    GameAssetType,
    GameAssets,
};

#[derive(Component)]
pub struct PlayerWeapon;

#[derive(Clone, Copy, Debug, Default, Display, Component, Resource, Reflect)]
#[display(style = "snake_case")]
#[reflect(Resource)]
pub enum Weapon {
    Axe = 0,
    Lance = 1,
    Rapier = 2,
    Sai = 3,
    #[default]
    Sword = 4,
}

impl Weapon {
    pub fn cooldown(&self) -> Duration {
        let ms = match self {
            Weapon::Axe => 300,
            Weapon::Lance => 400,
            Weapon::Rapier => 50,
            Weapon::Sai => 80,
            Weapon::Sword => 100,
        };

        Duration::from_millis(ms)
    }

    pub fn damage(&self) -> u32 {
        match self {
            Weapon::Axe => 20,
            Weapon::Lance => 30,
            Weapon::Rapier => 8,
            Weapon::Sai => 10,
            Weapon::Sword => 15,
        }
    }

    pub fn next(&self) -> Self {
        let index = *self as u8;
        let next = (index + 1) % 5;

        Self::from(next)
    }
}

impl From<u8> for Weapon {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Axe,
            1 => Self::Lance,
            2 => Self::Rapier,
            3 => Self::Sai,
            _ => Self::Sword,
        }
    }
}

pub fn spawn_weapon(
    mut commands: Commands,
    current_weapon: Res<Weapon>,
    player_q: Query<(Entity, &Player)>,
    weapon_q: Query<Entity, With<PlayerWeapon>>,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    textures: Res<Assets<TexturePack>>,
    audio: Res<Audio>,
) {
    let Err(_) = weapon_q.get_single() else {
        return;
    };

    let (entity, player) = player_q.single();

    if !player.is_attacking() {
        return;
    }

    let direction = player.direction;
    let weapon = *current_weapon;

    let name = format!("weapons/{weapon}/{direction}.png");
    let handle = asset_server.load("textures/weapons.json");
    let pack = textures.get(&handle).expect("Texture pack must exist");
    let index = pack.index_of(&name);
    let frame = &pack.frames[&name];

    let y_offset = TILE_SIZE - (TILE_SIZE - frame.frame.h) / 2.0 - 4.0;
    let x_offset = (TILE_SIZE + frame.frame.w) / 2.0;

    let translation = match direction {
        Direction::Down => Vec2::new(0.0, -y_offset),
        Direction::Left => Vec2::new(-x_offset, -TILE_SIZE / 4.0),
        Direction::Right => Vec2::new(x_offset, -TILE_SIZE / 4.0),
        Direction::Up => Vec2::new(0.0, y_offset),
    };

    commands.entity(entity).with_children(|parent| {
        parent.spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(index),
                texture_atlas: assets.get(GameAssetType::Weapons).clone(),
                transform: Transform::from_translation(translation.extend(0.0)),
                ..Default::default()
            },
            *current_weapon,
            PlayerWeapon,
            ActiveEvents::COLLISION_EVENTS,
            Collider::cuboid(frame.frame.w / 2.0, frame.frame.h / 2.0),
            WEAPON_COLLISION_GROUP.clone(),
            ColliderDebugColor(Color::GOLD),
        ));
    });

    audio.play(asset_server.load("audio/sword.wav")).with_volume(0.4);
}

pub fn switch_weapon(mut current_weapon: ResMut<Weapon>, mut reader: EventReader<SwitchWeapon>) {
    for _ in reader.iter() {
        *current_weapon = current_weapon.next();
    }
}
