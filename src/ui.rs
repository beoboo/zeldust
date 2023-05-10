use bevy::prelude::*;

use crate::constants::SWITCH_WEAPON_DURATION;
use crate::frames::TexturePack;
use crate::player::{StatType, Stats};
use crate::weapon::Weapon;
use crate::widgets::{AtlasImageBundle, UiAtlasImage};
use crate::GameAssets;

const MARGIN: f32 = 10.;
const PADDING: f32 = 2.;
const HEALTH_BAR_WIDTH: f32 = 200.;
const ENERGY_BAR_WIDTH: f32 = 140.;
const BAR_HEIGHT: f32 = 20.;
const ENERGY_COLOR: Color = Color::rgba(0., 0., 1., 0.9);
const HEALTH_COLOR: Color = Color::rgba(1., 0., 0., 0.9);
const BACK_COLOR: Color = Color::rgba(0.2, 0.2, 0.2, 0.9);
const BORDER_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const BORDER_WIDTH: f32 = 3.;
const FONT_SIZE: f32 = 18.0;
const ITEM_BOX_SIZE: f32 = 80.0;

#[derive(Component, Resource, Deref, DerefMut)]
pub struct SwitchWeaponTimer(pub Timer);

#[derive(Component)]
pub struct ItemBox;

pub fn spawn_ui(
    mut commands: Commands,
    current_weapon: Res<Weapon>,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    textures: Res<Assets<TexturePack>>,
) {
    let stats = Stats::default();

    // Top content
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                padding: UiRect::all(Val::Px(MARGIN)),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            spawn_bar(
                parent,
                stats.ratio(StatType::Health) * HEALTH_BAR_WIDTH,
                HEALTH_BAR_WIDTH,
                HEALTH_COLOR,
                UiRect::top(Val::Px(0.0)),
            );
            spawn_bar(
                parent,
                stats.ratio(StatType::Energy) * ENERGY_BAR_WIDTH,
                ENERGY_BAR_WIDTH,
                ENERGY_COLOR,
                UiRect::top(Val::Px(2. * PADDING)),
            );
        });

    // Bottom content
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                padding: UiRect::all(Val::Px(MARGIN)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        size: Size::width(Val::Percent(100.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    spawn_item_box(parent, &current_weapon, &asset_server, &assets, &textures);
                    spawn_experience(parent, &asset_server);
                });
        });
}

fn spawn_bar(
    parent: &mut ChildBuilder,
    width: f32,
    max_width: f32,
    color: Color,
    position: UiRect,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(max_width), Val::Px(BAR_HEIGHT)),
                padding: UiRect::all(Val::Px(PADDING)),
                position_type: PositionType::Relative,
                position,
                ..default()
            },
            background_color: BACK_COLOR.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::width(Val::Px(width)),
                    ..default()
                },
                background_color: color.into(),
                ..default()
            });
        });
}

fn spawn_item_box(
    parent: &mut ChildBuilder,
    current_weapon: &Res<Weapon>,
    asset_server: &Res<AssetServer>,
    assets: &Res<GameAssets>,
    textures: &Res<Assets<TexturePack>>,
) {
    let weapon = **current_weapon;

    let name = format!("full_{weapon}.png");
    let handle = asset_server.load("textures/weapons.json");
    let pack = textures.get(&handle).expect("Texture pack must exist");
    let index = pack.index_of(&name);

    parent
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::AUTO,
                    align_self: AlignSelf::FlexEnd,
                    padding: UiRect::all(Val::Px(BORDER_WIDTH)),
                    ..default()
                },
                background_color: BORDER_COLOR.into(),
                ..default()
            },
            ItemBox,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::all(Val::Px(ITEM_BOX_SIZE)),
                        align_self: AlignSelf::FlexStart,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(MARGIN)),
                        ..default()
                    },

                    background_color: BACK_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        AtlasImageBundle {
                            atlas_image: UiAtlasImage::new(assets.weapons.clone(), index),
                            ..default()
                        },
                        Weapon::default(),
                    ));
                });
        });
}

fn spawn_experience(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                size: Size::AUTO,
                align_self: AlignSelf::FlexEnd,
                padding: UiRect::new(
                    Val::Px(MARGIN),
                    Val::Px(MARGIN),
                    Val::Px(PADDING),
                    Val::Px(PADDING),
                ),
                ..default()
            },
            background_color: BACK_COLOR.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_sections([TextSection::new(
                    "123",
                    TextStyle {
                        font: asset_server.load("fonts/joystix.ttf"),
                        font_size: FONT_SIZE,
                        color: Color::WHITE,
                    },
                )])
                .with_style(Style { ..default() }),
            );
        });
}

pub fn change_ui_weapon(
    mut commands: Commands,
    mut box_q: Query<&mut BackgroundColor, With<ItemBox>>,
    mut weapon_q: Query<(&mut UiAtlasImage, &mut Weapon)>,
    current_weapon: Res<Weapon>,
    asset_server: Res<AssetServer>,
    textures: Res<Assets<TexturePack>>,
) {
    if current_weapon.is_changed() {
        if !current_weapon.is_added() {
            let mut back_color = box_q.single_mut();
            back_color.0 = Color::GOLD;

            commands.insert_resource(SwitchWeaponTimer(Timer::new(
                SWITCH_WEAPON_DURATION,
                TimerMode::Once,
            )));
        }

        let current_weapon = *current_weapon;

        for (mut image, mut weapon) in weapon_q.iter_mut() {
            *weapon = current_weapon;

            let name = format!("full_{current_weapon}.png");
            let handle = asset_server.load("textures/weapons.json");
            let pack = textures.get(&handle).expect("Texture pack must exist");

            image.index = pack.index_of(&name);
        }
    }
}

pub fn end_switch_weapon(
    mut commands: Commands,
    time: Res<Time>,
    timer: Option<ResMut<SwitchWeaponTimer>>,
    mut box_q: Query<&mut BackgroundColor, With<ItemBox>>,
) {
    if let Some(mut timer) = timer {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            let mut back_color = box_q.single_mut();
            back_color.0 = BORDER_COLOR;

            commands.remove_resource::<SwitchWeaponTimer>();
        }
    }
}
