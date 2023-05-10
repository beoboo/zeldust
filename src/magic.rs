use bevy::prelude::*;
use parse_display::Display;

use crate::{entities::Player, events::SwitchMagic, frames::TexturePack, GameAssets};

#[derive(Component)]
pub struct PlayerMagic;

#[derive(Clone, Copy, Debug, Default, Display, Component, Resource, Reflect)]
#[display(style = "snake_case")]
pub enum Magic {
    #[default]
    Flame = 0,
    Heal = 1,
}

impl Magic {
    pub fn strength(&self) -> u32 {
        match self {
            Magic::Flame => 5,
            Magic::Heal => 20,
        }
    }

    pub fn cost(&self) -> u32 {
        match self {
            Magic::Flame => 20,
            Magic::Heal => 10,
        }
    }

    pub fn next(&self) -> Self {
        let index = *self as u8;
        let next = (index + 1) % 2;

        Self::from(next)
    }
}

impl From<u8> for Magic {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Flame,
            _ => Self::Heal,
        }
    }
}

pub fn spawn_magic(
    mut commands: Commands,
    current_magic: Res<Magic>,
    player_q: Query<(Entity, &Player)>,
    magic_q: Query<Entity, With<PlayerMagic>>,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    textures: Res<Assets<TexturePack>>,
) {
    let Err(_) = magic_q.get_single() else {
        return;
    };

    let (entity, player) = player_q.single();

    if !player.is_attacking() {
        return;
    }

    // let direction = player.direction;
    // let magic = *current_magic;
    //
    // let name = format!("{magic}_{direction}.png");
    // let handle = asset_server.load("textures/magics.json");
    // let pack = textures.get(&handle).expect("Texture pack must exist");
    // let index = pack.index_of(&name);
    // let frame = &pack.frames[&name];
    //
    // let y_offset = TILE_SIZE - (TILE_SIZE - frame.frame.h) / 2.0 - 4.0;
    // let x_offset = (TILE_SIZE + frame.frame.w) / 2.0;
    //
    // let translation = match direction {
    //     Direction::Down => Vec2::new(0.0, -y_offset),
    //     Direction::Left => Vec2::new(-x_offset, -TILE_SIZE / 4.0),
    //     Direction::Right => Vec2::new(x_offset, -TILE_SIZE / 4.0),
    //     Direction::Up => Vec2::new(0.0, y_offset),
    // };
    //
    // commands.entity(entity).with_children(|parent| {
    //     parent.spawn((
    //         SpriteSheetBundle {
    //             sprite: TextureAtlasSprite::new(index),
    //             texture_atlas: assets.magics.clone(),
    //             transform: Transform::from_translation(translation.extend(0.0)),
    //             ..Default::default()
    //         },
    //         *current_magic,
    //         PlayerMagic,
    //     ));
    // });
}

pub fn switch_magic(mut current_magic: ResMut<Magic>, mut reader: EventReader<SwitchMagic>) {
    for _ in reader.iter() {
        *current_magic = current_magic.next();
    }
}
