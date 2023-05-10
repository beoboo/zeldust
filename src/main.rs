use std::collections::HashMap;

use bevy::{
    asset::LoadState,
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiPlugin},
    bevy_inspector, egui,
};
use bevy_kira_audio::{AudioPlugin, Audio, AudioControl};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use enum_iterator::{all, Sequence};
use parse_display::Display;

use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    debug::{DEBUG_PHYSICS, DEBUG_WORLD},
    entities::{Attackable, Enemy, Player},
    events::{
        DamageAttackable, EmitParticleEffect, KillAttackable, MagicCollision, PlayerCollision, SwitchMagic,
        SwitchWeapon, WeaponCollision,
    },
    frames::TexturePack,
    magic::Magic,
    map::{LayerType, WorldMap},
    screens::{GameMode, GameScreenPlugin, UpgradeScreenPlugin},
    ui::{MagicItemBox, WeaponItemBox},
    weapon::Weapon,
    widgets::WidgetsPlugin,
};

mod camera;
mod collisions;
mod constants;
mod debug;
mod entities;
mod events;
mod frames;
mod layer;
mod magic;
mod map;
mod particles;
mod screens;
mod stats;
mod ui;
mod weapon;
mod widgets;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    LoadLevel,
    RunLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
pub struct Layer(LayerType);

#[derive(Debug, Resource)]
pub struct MapSize {
    width: f32,
    height: f32,
}

#[derive(Component, Reflect)]
pub struct Map;

#[derive(Component)]
pub struct StaticCollider;

#[derive(Debug, Clone, Copy, Display, PartialEq, Eq, Hash, Sequence)]
#[display(style = "snake_case")]
pub enum GameAssetType {
    Grass,
    Monsters,
    Objects,
    Particles,
    Player,
    Weapons,
}

impl From<&LayerType> for GameAssetType {
    fn from(layer: &LayerType) -> Self {
        match layer {
            LayerType::Grass => GameAssetType::Grass,
            LayerType::Objects => GameAssetType::Objects,
            s => unreachable!("Level {s:?} cannot be converted"),
        }
    }
}

#[derive(Resource)]
pub struct GameAssets {
    handles: HashMap<GameAssetType, Handle<TextureAtlas>>,
}

impl GameAssets {
    pub fn get(&self, ty: GameAssetType) -> &Handle<TextureAtlas> {
        &self.handles[&ty]
    }
}

#[derive(Default, Resource)]
pub struct LoadingAssets {
    handles: HashMap<HandleUntyped, bool>,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Zeldust".to_string(),
            resolution: WindowResolution::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32),
            ..default()
        }),
        ..default()
    }))
    .add_plugin(AudioPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(ShapePlugin)
    .add_plugin(JsonAssetPlugin::<TexturePack>::new(&["json"]))
    .add_plugin(WidgetsPlugin)
    .register_type::<Attackable>()
    .register_type::<Weapon>()
    .register_type::<Magic>()
    .register_type::<Enemy>()
    .register_type::<MagicItemBox>()
    .register_type::<Player>()
    .register_type::<GameMode>()
    .register_type::<WeaponItemBox>()
    .register_type::<GameMode>()
    .add_event::<SwitchMagic>()
    .add_event::<SwitchWeapon>()
    .add_event::<PlayerCollision>()
    .add_event::<MagicCollision>()
    .add_event::<WeaponCollision>()
    .add_event::<EmitParticleEffect>()
    .add_event::<KillAttackable>()
    .add_event::<DamageAttackable>()
    .insert_resource(ClearColor(Color::hex("70deee").unwrap()))
    .init_resource::<GameMode>()
    .init_resource::<LoadingAssets>()
    .init_resource::<Weapon>()
    .init_resource::<Magic>()
    .add_state::<AppState>()
    .add_system(load_map.in_schedule(OnEnter(AppState::LoadLevel)))
    .add_systems((load_ground, load_assets, finish_loading).in_set(OnUpdate(AppState::LoadLevel)))
    .add_system(prepare_assets.in_schedule(OnExit(AppState::LoadLevel)))
    .add_plugin(GameScreenPlugin)
    .add_plugin(UpgradeScreenPlugin);

    if DEBUG_WORLD {
        // app.add_plugin(WorldInspectorPlugin::default());
        app.add_plugin(EguiPlugin)
            .add_plugin(bevy_inspector_egui::DefaultInspectorConfigPlugin)
            .add_system(inspector_ui);
    }

    if DEBUG_PHYSICS {
        app.add_plugin(RapierDebugRenderPlugin::default());
    }

    app.run();
}

fn load_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    // commands.insert_resource(WorldMap::debug_grass());
    commands.insert_resource(
        WorldMap::new()
            .load_layer(LayerType::Blocks, "assets/map/map_FloorBlocks.csv")
            .load_layer(LayerType::Grass, "assets/map/map_Grass.csv")
            .load_layer(LayerType::Objects, "assets/map/map_Objects.csv")
            .load_layer(LayerType::Entities, "assets/map/map_Entities.csv"),
    );

    // audio.play(asset_server.load("audio/main.ogg")).looped().with_playback_rate(1.2);
}

fn load_ground(asset_server: Res<AssetServer>, mut assets: ResMut<LoadingAssets>) {
    load(&asset_server, &mut assets, "map/ground.png");
}

fn load_assets(asset_server: Res<AssetServer>, mut assets: ResMut<LoadingAssets>) {
    for ty in all::<GameAssetType>() {
        for asset in vec!["json", "png"] {
            let path = format!("textures/{ty}.{asset}");
            load(&asset_server, &mut assets, path);
        }
    }
}

fn load(asset_server: &Res<AssetServer>, assets: &mut LoadingAssets, path: impl Into<String>) {
    let handle = asset_server.load_untyped(path.into());

    match asset_server.get_load_state(handle.clone()) {
        LoadState::Loaded => {
            assets.handles.insert(handle, true);
        },
        _ => {
            assets.handles.insert(handle, false);
        },
    }
}

fn finish_loading(
    mut app_state: ResMut<NextState<AppState>>,
    assets: Res<LoadingAssets>,
    mut game_mode: ResMut<GameMode>,
) {
    if assets.handles.is_empty() {
        return;
    }

    for loaded in assets.handles.values() {
        if !loaded {
            return;
        }
    }

    app_state.set(AppState::RunLevel);
    *game_mode = GameMode::Playing;
}

fn prepare_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    tiles_data: Res<Assets<TexturePack>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let handle = asset_server.load("map/ground.png");

    let image = images.get(&handle).expect("Ground image does not exist");

    let width = image.texture_descriptor.size.width as f32;
    let height = image.texture_descriptor.size.height as f32;

    let size = MapSize { width, height };

    commands.insert_resource(size);

    let handles = all::<GameAssetType>()
        .map(|ty| {
            (
                ty,
                build_texture_atlas(ty, &asset_server, &mut images, &mut texture_atlases, &tiles_data),
            )
        })
        .collect::<HashMap<_, _>>();

    let assets = GameAssets { handles };

    commands.insert_resource(assets);
}

fn build_texture_atlas(
    ty: GameAssetType,
    asset_server: &Res<AssetServer>,
    images: &mut ResMut<Assets<Image>>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    textures: &Res<Assets<TexturePack>>,
) -> Handle<TextureAtlas> {
    let path = format!("textures/{ty}");

    let handle = asset_server.load(format!("{path}.json"));
    let pack = textures.get(&handle).expect("Texture pack not loaded");

    let handle = asset_server.load(format!("{path}.png"));
    let image = images.get(&handle).expect("Image not loaded");

    let mut atlas = TextureAtlas::new_empty(handle, image.size());

    for (_, tile) in &pack.frames {
        let frame = &tile.frame;
        let rect = Rect::new(frame.x, frame.y, frame.x + frame.w, frame.y + frame.h);
        atlas.add_texture(rect);
    }

    texture_atlases.add(atlas)
}

fn inspector_ui(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    let window = world.query_filtered::<&Window, With<PrimaryWindow>>().single(world);

    egui::Window::new("World Inspector")
        .default_width(200.0)
        .default_height(window.height() - 50.0)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                bevy_inspector::ui_for_world(world, ui);
                ui.allocate_space(ui.available_size());
            });
        });
}
