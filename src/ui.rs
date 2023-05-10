// use bevy::prelude::*;
// use bevy::window::PrimaryWindow;
// use bevy_prototype_lyon::prelude::*;
// use crate::{Position, SCREEN_WIDTH};
//
// const MARGIN: f32 = 20.;
// const PADDING: f32 = 3.;
// const HEALTH_BAR_WIDTH: f32 = 200.;
// const BAR_HEIGHT: f32 = 20.;
// const BACK_COLOR: Color = Color::rgba(0.4, 0.4, 0.4, 0.9);
// const BORDER_COLOR: Color = Color::rgb(1.2, 0.2, 0.2);
// const BORDER_WIDTH: f32 = 3.;
//
// pub fn show_ui(
//     mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>,
// ) {
//     // commands.spawn(SpriteBundle {
//     //     sprite: Sprite {
//     //         color: Color::rgb(0.25, 0.25, 0.75),
//     //         custom_size: Some(Vec2::new(500.0, 500.0)),
//     //         ..Default::default()
//     //     },
//     //     ..Default::default()
//     // })
//     //     .insert(Position {
//     //         x: -(SCREEN_WIDTH as f32/ 2.),
//     //         y: 0.,
//     //         layer: 10,
//     //     });
//     // commands.spawn(SpriteBundle {
//     //     sprite: Sprite {
//     //         color: Color::rgb(0.25, 0.25, 0.75),
//     //         custom_size: Some(Vec2::new(500.0, 500.0)),
//     //         ..Default::default()
//     //     },
//     //     ..Default::default()
//     // })
//     //     .insert(Position {
//     //         x: 0.,
//     //         y: 0.,
//     //         layer: 10,
//     //     });
// }