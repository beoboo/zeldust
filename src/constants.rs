use bevy::prelude::Color;
use std::time::Duration;

pub const STARTING_XP: u32 = 500;
pub const SPEED: f32 = 100.0;
pub const TILE_SIZE: f32 = 64.0;
pub const SCREEN_WIDTH: u32 = 1280;
pub const SCREEN_HEIGHT: u32 = 720;
pub const CAMERA_SCALE: f32 = 2.;

pub const SWITCH_ITEM_DURATION: Duration = Duration::from_millis(200);
pub const ANIMATION_DURATION: Duration = Duration::from_millis(150);
pub const ENERGY_RECOVERY_DURATION: Duration = Duration::from_millis(600);
pub const ATTACK_DURATION: Duration = Duration::from_millis(500);
pub const HIT_DURATION: Duration = Duration::from_millis(400);

pub const MARGIN: f32 = 10.;
pub const PADDING: f32 = 2.;
pub const HEALTH_BAR_WIDTH: f32 = 200.;
pub const ENERGY_BAR_WIDTH: f32 = 140.;
pub const BAR_WIDTH: f32 = 140.;
pub const BAR_HEIGHT: f32 = 20.;
pub const BORDER_WIDTH: f32 = 3.;
pub const FONT_SIZE: f32 = 18.0;
pub const ITEM_BOX_SIZE: f32 = 80.0;

pub const ENERGY_COLOR: Color = Color::rgba(0., 0., 1., 0.9);
pub const HEALTH_COLOR: Color = Color::rgba(1., 0., 0., 0.9);
pub const BACK_COLOR: Color = Color::rgba(0.13, 0.13, 0.13, 0.9); // #222222
pub const SELECTED_BACK_COLOR: Color = Color::rgb(0.93, 0.93, 0.93); // #EEEEEE
pub const BORDER_COLOR: Color = Color::rgb(0.07, 0.07, 0.07); // #111111
pub const SELECTED_TEXT_COLOR: Color = Color::rgb(0.07, 0.07, 0.07); // #111111
pub const TEXT_COLOR: Color = Color::rgb(0.93, 0.93, 0.93); // #EEEEEE
pub const SELECTED_BAR_COLOR: Color = Color::rgb(0.07, 0.07, 0.07); // #111111
pub const BAR_COLOR: Color = Color::rgb(0.93, 0.93, 0.93); // #EEEEEE
