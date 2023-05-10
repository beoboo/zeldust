use crate::{screens::is_upgrading, AppState};
use bevy::prelude::*;

use crate::screens::upgrade::{
    input::handle_input,
    ui::{highligh_box, show_ui, spawn_ui, update_ui},
};

mod input;
mod ui;

pub struct UpgradeScreenPlugin;

impl Plugin for UpgradeScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_input.run_if(is_upgrading));
        app.add_system(spawn_ui.in_schedule(OnEnter(AppState::RunLevel)));
        app.add_systems((show_ui, highligh_box, update_ui).in_set(OnUpdate(AppState::RunLevel)));
    }
}
