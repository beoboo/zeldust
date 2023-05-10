use bevy::prelude::*;
use enum_iterator::all;

use crate::{
    constants::{
        BACK_COLOR,
        BAR_COLOR,
        BAR_WIDTH,
        BORDER_COLOR,
        FONT_SIZE,
        MARGIN,
        PADDING,
        SELECTED_BACK_COLOR,
        SELECTED_BAR_COLOR,
        SELECTED_TEXT_COLOR,
        TEXT_COLOR,
    },
    entities::{Player, PlayerStat},
    screens::GameMode,
};

#[derive(Default, Component)]
pub struct UpgradeScreen {
    pub selection_index: u32,
}

#[derive(Default, Component)]
pub struct Stat {
    pub selection_index: u32,
}

#[derive(Default, Component)]
pub struct Box;

#[derive(Default, Component)]
pub struct Bar;

#[derive(Default, Component)]
pub struct Slider;

#[derive(Default, Component)]
pub struct Cost;

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/joystix.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    padding: UiRect::all(Val::Px(PADDING)),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,

                    ..default()
                },
                background_color: BACK_COLOR.into(),
                ..default()
            },
            UpgradeScreen::default(),
        ))
        .with_children(|parent| {
            for ty in all::<PlayerStat>() {
                spawn_bar(parent, ty, BAR_WIDTH, Color::RED, &font);
            }
        });
}

fn spawn_bar(parent: &mut ChildBuilder, ty: PlayerStat, width: f32, color: Color, font: &Handle<Font>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(width), Val::Percent(80.)),
                padding: UiRect::all(Val::Px(PADDING)),
                ..default()
            },
            background_color: BORDER_COLOR.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        ..default()
                    },
                    ty,
                    Box,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_sections([TextSection::new(
                            format!("{ty}"),
                            TextStyle {
                                font: font.clone(),
                                font_size: FONT_SIZE,
                                color: Color::WHITE,
                            },
                        )])
                        .with_style(Style {
                            align_self: AlignSelf::Center,
                            margin: UiRect::vertical(Val::Px(MARGIN)),
                            ..default()
                        }),
                        ty,
                    ));

                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::SpaceBetween,
                                    ..default()
                                },
                                ..default()
                            },
                            ty,
                            Box,
                        ))
                        .with_children(|parent| {
                            // Vertical bar
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(5.), Val::Percent(100.)),
                                        align_self: AlignSelf::Center,
                                        ..default()
                                    },
                                    ..default()
                                },
                                ty,
                                Bar,
                            ));

                            // Slider
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(25.), Val::Px(10.)),
                                        position_type: PositionType::Relative,
                                        align_self: AlignSelf::Center,
                                        ..default()
                                    },
                                    ..default()
                                },
                                ty,
                                Bar,
                                Slider,
                            ));
                        });

                    parent.spawn((
                        TextBundle::from_sections([TextSection::new(
                            format!("{}", ty.cost()),
                            TextStyle {
                                font: font.clone(),
                                font_size: FONT_SIZE,
                                ..default()
                            },
                        )])
                        .with_style(Style {
                            align_self: AlignSelf::Center,
                            margin: UiRect::vertical(Val::Px(MARGIN)),
                            ..default()
                        }),
                        ty,
                        Cost,
                    ));
                });
        });
}

pub fn show_ui(mut screen_q: Query<&mut Visibility, With<UpgradeScreen>>, game_mode: Res<GameMode>) {
    if game_mode.is_changed() {
        let mut visibility = screen_q.single_mut();

        *visibility = if *game_mode == GameMode::Playing {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
}

pub fn update_ui(
    player_q: Query<&Player>,
    mut slider_q: Query<(&mut Style, &PlayerStat), With<Slider>>,
    mut cost_q: Query<(&mut Text, &PlayerStat), With<Cost>>,
) {
    let player = player_q.single();

    for (mut style, stat) in slider_q.iter_mut() {
        let ratio = player.stats.limit(*stat) as f32 / player.stats.max(*stat) as f32 * 100.;
        style.position.bottom = Val::Percent(ratio);
    }

    for (mut text, stat) in cost_q.iter_mut() {
        text.sections[0].value = format!("{}", player.cost_by(*stat));
    }
}

pub fn highligh_box(
    mut screen_q: Query<Ref<UpgradeScreen>>,
    mut box_q: Query<(&mut BackgroundColor, &PlayerStat), With<Box>>,
    mut bar_q: Query<(&mut BackgroundColor, &PlayerStat), (With<Bar>, Without<Box>)>,
    mut text_q: Query<(&mut Text, &PlayerStat)>,
) {
    let screen = screen_q.single();

    if screen.is_changed() {
        let selected_stat = PlayerStat::from(screen.selection_index);

        for (mut color, stat) in box_q.iter_mut() {
            *color = if stat == &selected_stat {
                SELECTED_BACK_COLOR.into()
            } else {
                BACK_COLOR.into()
            }
        }

        for (mut color, stat) in bar_q.iter_mut() {
            *color = if stat == &selected_stat {
                SELECTED_BAR_COLOR.into()
            } else {
                BAR_COLOR.into()
            }
        }

        for (mut color, stat) in bar_q.iter_mut() {
            *color = if stat == &selected_stat {
                SELECTED_BAR_COLOR.into()
            } else {
                BAR_COLOR.into()
            }
        }

        for (mut text, stat) in text_q.iter_mut() {
            text.sections[0].style.color = if stat == &selected_stat {
                SELECTED_TEXT_COLOR
            } else {
                TEXT_COLOR
            }
        }
    }
}
