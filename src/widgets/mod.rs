use bevy::prelude::*;
use bevy::render::RenderApp;
use bevy::ui::RenderUiSystem;
use bevy::ui::UiSystem;

use crate::widgets::ui_atlas_image::{
    extract_texture_atlas_image_uinodes, texture_atlas_image_node_system,
};

pub use ui_atlas_image::*;

mod ui_atlas_image;

pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UiAtlasImage>().add_system(
            texture_atlas_image_node_system
                .before(UiSystem::Flex)
                .in_base_set(CoreSet::PostUpdate),
        );

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app.add_system(
            extract_texture_atlas_image_uinodes
                .after(RenderUiSystem::ExtractNode)
                .in_schedule(ExtractSchedule),
        );
    }
}
