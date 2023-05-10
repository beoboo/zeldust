use std::collections::HashMap;

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

// use crate::collisions::handle_collisions;
use crate::map::{LayerType, WorldMap};
use crate::player::{animate_player, handle_input, move_camera, Player, PlayerPositionEvent, spawn_player, update_player_position};
use crate::settings::{CAMERA_SCALE, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};
use crate::tiles::TileSet;

mod settings;
mod player;
mod map;
mod layer;
mod ui;
mod collisions;
mod tiles;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Playing,
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
pub struct Layer(LayerType);

#[derive(Component, Reflect)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Default for Size {
    fn default() -> Self {
        Self { width: TILE_SIZE as f32, height: TILE_SIZE as f32 }
    }
}

#[derive(Debug, Resource)]
pub struct MapSize {
    width: f32,
    height: f32,
}

#[derive(Component, Reflect)]
pub struct Map;

#[derive(Component)]
pub struct StaticCollider;

#[derive(Resource)]
pub struct GameAssets {
    player: Handle<TextureAtlas>,
    layers: HashMap<LayerType, Handle<TextureAtlas>>,
}

#[derive(Default, Resource)]
pub struct LoadingAssets {
    handles: HashMap<HandleUntyped, bool>,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zeldust".to_string(),
                resolution: WindowResolution::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::default())
        // .add_plugin(TilesetPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default()
                    // .disabled()
        )
        .add_plugin(ShapePlugin)
        .add_plugin(JsonAssetPlugin::<TileSet>::new(&["json"]))
        .register_type::<Position>()
        .register_type::<Player>()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(
            WorldMap::new()
                .load_layer(LayerType::Blocks, "assets/map/map_FloorBlocks.csv")
                .load_layer(LayerType::Grass, "assets/map/map_Grass.csv")
                .load_layer(LayerType::Objects, "assets/map/map_Objects.csv")
        )
        .init_resource::<LoadingAssets>()
        .add_state::<AppState>()
        .add_systems((load_ground, load_assets, finish_loading).in_set(OnUpdate(AppState::Loading)))
        .add_systems((
            prepare_assets,
            spawn_ground,
        ).in_schedule(OnExit(AppState::Loading)))
        .add_systems((
            spawn_cameras.after(spawn_ground),
            spawn_tiles.after(spawn_cameras),
            spawn_player.after(spawn_tiles),
        ).in_schedule(OnEnter(AppState::Playing)))
        .add_systems((
            handle_input,
            update_player_position,
            move_camera,
            animate_player,
            // handle_collisions,
        ).in_set(OnUpdate(AppState::Playing)))
        .add_system(position_tiles.in_base_set(CoreSet::PostUpdate))
        .add_event::<PlayerPositionEvent>()
        .run();
}

fn load_ground(
    asset_server: Res<AssetServer>,
    mut assets: ResMut<LoadingAssets>,
) {
    let handle = asset_server.load_untyped("map/ground.png");

    match asset_server.get_load_state(handle.clone()) {
        LoadState::Loaded => {
            assets.handles.insert(handle, true);
        }
        _ => {
            assets.handles.insert(handle, false);
        }
    }
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut assets: ResMut<LoadingAssets>,
) {
    for ty in vec!["objects"] {
        for asset in vec!["json", "png"] {
            let path = format!("tiles/{ty}.{asset}");
            let handle = asset_server.load_untyped(path);

            match asset_server.get_load_state(handle.clone()) {
                LoadState::Loaded => {
                    assets.handles.insert(handle, true);
                }
                _ => {
                    assets.handles.insert(handle, false);
                }
            }
        }
    }
}

fn finish_loading(
    mut app_state: ResMut<NextState<AppState>>,
    assets: Res<LoadingAssets>,
) {
    if assets.handles.is_empty() {
        return;
    }

    for loaded in assets.handles.values() {
        if !loaded {
            return;
        }
    }

    app_state.set(AppState::Playing);
}

fn prepare_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    tiles_data: Res<Assets<TileSet>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("images/player.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 24, 1, None, None);

    let player_handle = texture_atlases.add(texture_atlas);

    let mut layers = HashMap::new();

    let texture_handle = asset_server.load("images/grass.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 24, 1, None, None);
    layers.insert(LayerType::Grass, texture_atlases.add(texture_atlas));


    let handle = asset_server.load("tiles/objects.json");
    let tile_set = tiles_data.get(&handle).expect("Tile not loaded");

    let image_handle = asset_server.load("tiles/objects.png");
    let image = images.get(&image_handle).expect("Image not loaded");

    let mut builder = TextureAtlasBuilder::default();
    builder.add_texture(image_handle, image);
    let mut atlas = builder.finish(&mut images).expect("Cannot build texture atlas");

    for (_, tile) in &tile_set.frames {
        let frame = &tile.frame;
        let rect = Rect::new(frame.x, frame.y, frame.x + frame.w, frame.y + frame.h);
        atlas.add_texture(rect);
    }

    let texture_atlas_handle = texture_atlases.add(atlas);

    layers.insert(LayerType::Objects, texture_atlas_handle);

    let assets = GameAssets {
        player: player_handle,
        layers,
    };

    commands.insert_resource(assets);
}

fn spawn_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else { return; };

    let handle = asset_server.load("map/ground.png");

    let image = images.get(&handle).expect("Ground image does not exist");

    let width = image.texture_descriptor.size.width as f32;
    let height = image.texture_descriptor.size.height as f32;

    let size = MapSize {
        width,
        height,
    };

    let x = (size.width - window.width()) / 2.;
    let y = -((size.height - window.height()) / 2.);

    commands.spawn((
        SpriteBundle {
            texture: handle.clone(),
            transform: Transform::from_xyz(x, y, -1000.0),
            ..Default::default()
        },
        Map,
    ));

    commands.insert_resource(size);
}

fn spawn_cameras(
    mut commands: Commands,
    map_size: Res<MapSize>,
) {
    // println!("spawn cameras");

    let (x, y) = (map_size.width as f32 / 2., map_size.height as f32 / 2.);

    let camera = Camera2dBundle {
        projection: OrthographicProjection {
            scale: CAMERA_SCALE,
            near: -10000.0,
            far: 10000.0,
            ..default()
        },
        ..default()
    };

    commands.spawn((camera, Position { x, y }));
}

fn spawn_tiles(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    atlases: Res<Assets<TextureAtlas>>,
) {
    // Spawn the world
    for (layer_type, layer) in world_map.layers.iter() {
        for (row_idx, row) in layer.data.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell != -1 {
                    info!("{layer_type:?}: {cell}");
                }

                match cell {
                    0..=20 => {
                        let index = layer_type.to_index(cell as usize);

                        let atlas_handle = &assets.layers[layer_type];
                        let atlas = atlases.get(atlas_handle).unwrap();
                        let image = atlas.textures[index];
                        let x = (col_idx as f32 + 0.5) * image.width();
                        let y = (row_idx as f32 + 0.5) * image.height();

                        let collider_height = image.height() - TILE_SIZE / 2.0;

                        commands.spawn((
                            SpriteSheetBundle {
                                sprite: TextureAtlasSprite::new(index),
                                texture_atlas: atlas_handle.clone(),
                                ..Default::default()
                            },
                            RigidBody::Fixed,
                            Position { x, y },
                            Layer(*layer_type),
                            Size::default(),
                        )).with_children(|parent| {
                            parent.spawn((
                                // Restitution::coefficient(0.1),
                                Collider::cuboid(image.width() / 2.0, collider_height / 2.0),
                                Transform::from_xyz(0.0, 0.0, 0.0),
                                ColliderDebugColor(Color::ALICE_BLUE),
                            ));
                        });
                    }
                    395 => {
                        let x = (col_idx as f32 + 0.5) * TILE_SIZE;
                        let y = (row_idx as f32 + 0.5) * TILE_SIZE;

                        commands.spawn((
                            SpriteBundle {
                                // sprite: Sprite {
                                //     color: Color::BLACK,
                                //     ..default()
                                // },
                                texture: asset_server.load("test/rock.png"),
                                ..Default::default()
                            },
                            RigidBody::Fixed,
                            Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
                            Position { x, y },
                            Layer(*layer_type),
                            ColliderDebugColor(Color::NAVY),
                            Size::default(),
                        ));
                    }
                    _ => {
                        if cell != -1 {
                            info!("Ignoring: {}", cell);
                        }
                    }
                }
            }
            // println!();
        }
    }
}

pub fn from_position(position: &Position, window: &Window) -> Vec3 {
    fn convert(pos: f32, bound_dim: f32) -> f32 {
        pos - (bound_dim / 2.)
    }

    Vec3::new(
        convert(position.x, window.width()),
        -convert(position.y, window.height()),
        convert(position.y, window.height()) + 1000.0,
    )
}

pub fn from_translation(translation: Vec3, window: &Window) -> Position {
    fn convert(pos: f32, bound_dim: f32) -> f32 {
        pos - (bound_dim / 2.)
    }

    Position {
        x: convert(translation.x, -window.width()),
        y: -convert(translation.y, window.height()),
    }
}

fn position_tiles(
    window: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform), (Changed<Position>, Without<Player>)>,
) {
    let Ok(window) = window.get_single() else { return; };

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = from_position(pos, window);
    }
}

