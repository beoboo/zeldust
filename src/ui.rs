use bevy::prelude::*;
use parse_display::Display;

use crate::{
    constants::{
        BACK_COLOR,
        BAR_HEIGHT,
        BORDER_COLOR,
        BORDER_WIDTH,
        ENERGY_BAR_WIDTH,
        ENERGY_COLOR,
        FONT_SIZE,
        HEALTH_BAR_WIDTH,
        HEALTH_COLOR,
        ITEM_BOX_SIZE,
        MARGIN,
        PADDING,
        SWITCH_ITEM_DURATION,
    },
    entities::{Player, PlayerStat},
    frames::TexturePack,
    magic::Magic,
    weapon::Weapon,
    widgets::{AtlasImageBundle, UiAtlasImage},
    GameAssetType,
    GameAssets,
};

#[derive(Component, Resource, Deref, DerefMut)]
pub struct SwitchMagicTimer(pub Timer);

#[derive(Component, Resource, Deref, DerefMut)]
pub struct SwitchWeaponTimer(pub Timer);

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct Experience;

#[derive(Component)]
pub struct EnergyBar;

#[derive(Clone, Copy, Display)]
pub enum ItemBoxType {
    #[display("{0}")]
    Magic(Magic, MagicItemBox),
    #[display("{0}")]
    Weapon(Weapon, WeaponItemBox),
}

impl ItemBoxType {
    pub fn name(&self) -> String {
        match self {
            ItemBoxType::Magic(m, _) => format!("particles/{m}"),
            ItemBoxType::Weapon(w, _) => format!("weapons/{w}"),
        }
    }

    pub fn asset_name(&self) -> &str {
        match self {
            ItemBoxType::Magic(_, _) => "particles",
            ItemBoxType::Weapon(_, _) => "weapons",
        }
    }
}

#[derive(Clone, Copy, Component, Reflect)]
pub struct MagicItemBox;

#[derive(Clone, Copy, Component, Reflect)]
pub struct WeaponItemBox;

pub fn spawn_ui(
    mut commands: Commands,
    weapon: Res<Weapon>,
    magic: Res<Magic>,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    textures: Res<Assets<TexturePack>>,
) {
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
                0.0,
                HEALTH_BAR_WIDTH,
                HEALTH_COLOR,
                UiRect::top(Val::Px(0.0)),
                HealthBar,
            );

            spawn_bar(
                parent,
                0.0,
                ENERGY_BAR_WIDTH,
                ENERGY_COLOR,
                UiRect::top(Val::Px(2. * PADDING)),
                EnergyBar,
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
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::FlexStart,
                                size: Size::width(Val::Percent(100.)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            spawn_item_box(
                                ItemBoxType::Weapon(*weapon, WeaponItemBox),
                                parent,
                                &asset_server,
                                &assets,
                                &textures,
                                UiRect::all(Val::Px(0.)),
                            );
                            spawn_item_box(
                                ItemBoxType::Magic(*magic, MagicItemBox),
                                parent,
                                &asset_server,
                                &assets,
                                &textures,
                                UiRect::all(Val::Px(-4.0 * PADDING)),
                            );
                        });
                    spawn_experience(parent, &asset_server);
                });
        });
}

fn spawn_bar<B: Bundle>(
    parent: &mut ChildBuilder,
    width: f32,
    max_width: f32,
    color: Color,
    position: UiRect,
    bundle: B,
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
            let mut cmds = parent.spawn(ImageBundle {
                style: Style {
                    size: Size::width(Val::Px(width)),
                    ..default()
                },
                background_color: color.into(),
                ..default()
            });

            cmds.insert(bundle);
        });
}

fn spawn_item_box(
    ty: ItemBoxType,
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    assets: &Res<GameAssets>,
    textures: &Res<Assets<TexturePack>>,
    margin: UiRect,
) {
    let name = format!("{}/full.png", ty.name());
    let asset_name = format!("textures/{}.json", ty.asset_name());
    // println!("{name} from {asset_name}");

    let handle = asset_server.load(asset_name);
    let pack = textures.get(&handle).expect("Texture pack must exist");
    let index = pack.index_of(&name);

    let mut commands = parent.spawn((NodeBundle {
        style: Style {
            size: Size::AUTO,
            align_self: AlignSelf::FlexEnd,
            padding: UiRect::all(Val::Px(BORDER_WIDTH)),
            margin,
            ..default()
        },
        background_color: BORDER_COLOR.into(),
        ..default()
    },));

    let atlas = match ty {
        ItemBoxType::Magic(_, m) => {
            commands.insert(m);
            assets.get(GameAssetType::Particles).clone()
        },
        ItemBoxType::Weapon(_, w) => {
            commands.insert(w);
            assets.get(GameAssetType::Weapons).clone()
        },
    };

    commands.with_children(|parent| {
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
                let mut commands = parent.spawn((AtlasImageBundle {
                    atlas_image: UiAtlasImage::new(atlas, index),
                    ..default()
                },));

                match ty {
                    ItemBoxType::Magic(_, _) => commands.insert(Magic::default()),
                    ItemBoxType::Weapon(_, _) => commands.insert(Weapon::default()),
                };
            });
    });
}

fn spawn_experience(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                size: Size::AUTO,
                align_self: AlignSelf::FlexEnd,
                padding: UiRect::new(Val::Px(MARGIN), Val::Px(MARGIN), Val::Px(PADDING), Val::Px(PADDING)),
                ..default()
            },
            background_color: BACK_COLOR.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([TextSection::new(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/joystix.ttf"),
                        font_size: FONT_SIZE,
                        color: Color::WHITE,
                    },
                )])
                .with_style(Style { ..default() }),
                Experience,
            ));
        });
}

pub fn change_magic_item(
    mut commands: Commands,
    mut box_q: Query<&mut BackgroundColor, With<MagicItemBox>>,
    mut magic_q: Query<(&mut UiAtlasImage, &mut Magic)>,
    current_magic: Res<Magic>,
    asset_server: Res<AssetServer>,
    textures: Res<Assets<TexturePack>>,
) {
    if current_magic.is_changed() {
        if !current_magic.is_added() {
            let mut back_color = box_q.single_mut();
            back_color.0 = Color::GOLD;

            commands.insert_resource(SwitchMagicTimer(Timer::new(SWITCH_ITEM_DURATION, TimerMode::Once)));
        }

        let current_magic = *current_magic;

        for (mut image, mut magic) in magic_q.iter_mut() {
            *magic = current_magic;

            let name = format!("particles/{current_magic}/full.png");
            let handle = asset_server.load("textures/particles.json");
            let pack = textures.get(&handle).expect("Texture pack must exist");

            image.index = pack.index_of(&name);
        }
    }
}

pub fn end_switch_magic(
    mut commands: Commands,
    time: Res<Time>,
    timer: Option<ResMut<SwitchMagicTimer>>,
    mut box_q: Query<&mut BackgroundColor, With<MagicItemBox>>,
) {
    if let Some(mut timer) = timer {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            let mut back_color = box_q.single_mut();
            back_color.0 = BORDER_COLOR;

            commands.remove_resource::<SwitchMagicTimer>();
        }
    }
}

pub fn change_weapon_item(
    mut commands: Commands,
    mut box_q: Query<&mut BackgroundColor, With<WeaponItemBox>>,
    mut weapon_q: Query<(&mut UiAtlasImage, &mut Weapon)>,
    current_weapon: Res<Weapon>,
    asset_server: Res<AssetServer>,
    textures: Res<Assets<TexturePack>>,
) {
    if current_weapon.is_changed() {
        if !current_weapon.is_added() {
            let mut back_color = box_q.single_mut();
            back_color.0 = Color::GOLD;

            commands.insert_resource(SwitchWeaponTimer(Timer::new(SWITCH_ITEM_DURATION, TimerMode::Once)));
        }

        let current_weapon = *current_weapon;

        for (mut image, mut weapon) in weapon_q.iter_mut() {
            *weapon = current_weapon;

            let name = format!("weapons/{current_weapon}/full.png");
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
    mut box_q: Query<&mut BackgroundColor, With<WeaponItemBox>>,
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

pub fn update_health_ui(player_q: Query<&Player>, mut health_q: Query<&mut Style, With<HealthBar>>) {
    let player = player_q.single();

    let mut health = health_q.single_mut();

    health.size.width = Val::Px(player.stats.ratio_by_limit_of(PlayerStat::Health) * HEALTH_BAR_WIDTH);
}

pub fn update_energy_ui(player_q: Query<&Player>, mut energy_q: Query<&mut Style, With<EnergyBar>>) {
    let player = player_q.single();

    let mut energy = energy_q.single_mut();

    energy.size.width = Val::Px(player.stats.ratio_by_limit_of(PlayerStat::Energy) * ENERGY_BAR_WIDTH);
}

pub fn update_xp_ui(player_q: Query<&Player>, mut xp_q: Query<&mut Text, With<Experience>>) {
    let player = player_q.single();

    let mut text = xp_q.single_mut();

    text.sections[0].value = format!("{}", player.xp);
}
