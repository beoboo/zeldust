use bevy::prelude::*;

pub use game::*;
pub use upgrade::*;

mod game;
mod upgrade;

#[derive(Debug, Clone, Default, PartialEq, Resource, Reflect)]
#[reflect(Resource)]
pub enum GameMode {
    #[default]
    Loading,
    Playing,
    Upgrading,
    Paused,
}

pub fn is_playing(game_mode: Res<GameMode>) -> bool {
    matches!(*game_mode, GameMode::Playing)
}

pub fn is_upgrading(game_mode: Res<GameMode>) -> bool {
    matches!(*game_mode, GameMode::Upgrading)
}
