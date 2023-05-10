use std::collections::HashMap;

use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::{CAMERA_SCALE, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};
use crate::events::{PlayerPositionChanged, SwitchWeapon};
use crate::frames::TexturePack;
use crate::map::{LayerType, WorldMap};
use crate::player::{
    end_attack, handle_input, move_camera, render_player, spawn_player, update_player_position,
    Player,
};
use crate::ui::{change_ui_weapon, spawn_ui};
use crate::weapon::{spawn_weapon, switch_weapon, Weapon};
use crate::widgets::WidgetsPlugin;

mod collisions;
mod constants;
mod events;
mod frames;
mod layer;
mod map;
mod player;
mod ui;
mod weapon;
mod widgets;

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
        Self {
            width: TILE_SIZE as f32,
            height: TILE_SIZE as f32,
        }
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
    weapons: Handle<TextureAtlas>,
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
        .add_plugin(RapierDebugRenderPlugin::default().disabled())
        .add_plugin(ShapePlugin)
        .add_plugin(JsonAssetPlugin::<TexturePack>::new(&["json"]))
        .add_plugin(WidgetsPlugin)
        .add_plugin(ResourceInspectorPlugin::<Weapon>::default())
        .register_type::<Position>()
        .register_type::<Player>()
        .add_event::<SwitchWeapon>()
        .add_event::<PlayerPositionChanged>()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(
            WorldMap::new(), // .load_layer(LayerType::Blocks, "assets/map/map_FloorBlocks.csv")
                             // .load_layer(LayerType::Grass, "assets/map/map_Grass.csv")
                             // .load_layer(LayerType::Objects, "assets/map/map_Objects.csv"),
        )
        .init_resource::<LoadingAssets>()
        .init_resource::<Weapon>()
        .add_state::<AppState>()
        .add_systems((load_ground, load_assets, finish_loading).in_set(OnUpdate(AppState::Loading)))
        .add_systems((prepare_assets,).in_schedule(OnExit(AppState::Loading)))
        .add_systems(
            (
                // debug_tiles,
                spawn_ground,
                spawn_cameras,
                spawn_tiles.after(spawn_cameras),
                spawn_player.after(spawn_cameras),
                spawn_ui.after(spawn_player),
            )
                .in_schedule(OnEnter(AppState::Playing)),
        )
        .add_systems(
            (
                handle_input,
                update_player_position,
                move_camera,
                render_player,
                spawn_weapon,
                switch_weapon,
                end_attack,
                change_ui_weapon,
                // handle_collisions,
            )
                .in_set(OnUpdate(AppState::Playing)),
        )
        .add_system(position_tiles.in_base_set(CoreSet::PostUpdate))
        .run();
}

fn load_ground(asset_server: Res<AssetServer>, mut assets: ResMut<LoadingAssets>) {
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

fn load_assets(asset_server: Res<AssetServer>, mut assets: ResMut<LoadingAssets>) {
    for ty in vec!["grass", "objects", "player", "weapons"] {
        for asset in vec!["json", "png"] {
            let path = format!("textures/{ty}.{asset}");
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

fn finish_loading(mut app_state: ResMut<NextState<AppState>>, assets: Res<LoadingAssets>) {
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
    tiles_data: Res<Assets<TexturePack>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let handle = asset_server.load("map/ground.png");

    let image = images.get(&handle).expect("Ground image does not exist");

    let width = image.texture_descriptor.size.width as f32;
    let height = image.texture_descriptor.size.height as f32;

    let size = MapSize { width, height };

    commands.insert_resource(size);

    let player_atlas_handle = build_texture_atlas(
        "player",
        &asset_server,
        &mut images,
        &mut texture_atlases,
        &tiles_data,
    );
    let weapons_atlas_handle = build_texture_atlas(
        "weapons",
        &asset_server,
        &mut images,
        &mut texture_atlases,
        &tiles_data,
    );

    let mut layers = HashMap::new();

    let texture_atlas_handle = build_texture_atlas(
        "grass",
        &asset_server,
        &mut images,
        &mut texture_atlases,
        &tiles_data,
    );
    layers.insert(LayerType::Grass, texture_atlas_handle);

    let texture_atlas_handle = build_texture_atlas(
        "objects",
        &asset_server,
        &mut images,
        &mut texture_atlases,
        &tiles_data,
    );
    layers.insert(LayerType::Objects, texture_atlas_handle);

    let assets = GameAssets {
        player: player_atlas_handle,
        weapons: weapons_atlas_handle,
        layers,
    };

    commands.insert_resource(assets);
}

fn build_texture_atlas(
    ty: &str,
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

fn spawn_ground(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<&Window, With<PrimaryWindow>>,
    size: Res<MapSize>,
) {
    let Ok(window) = window.get_single() else { return; };

    let handle = asset_server.load("map/ground.png");

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
}

fn spawn_cameras(mut commands: Commands, map_size: Res<MapSize>) {
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

fn debug_tiles(
    mut commands: Commands,
    assets: Res<GameAssets>,
    atlases: Res<Assets<TextureAtlas>>,
) {
    let handle = &assets.layers[&LayerType::Objects];
    let atlas = atlases.get(&handle).unwrap();

    for (id, texture) in atlas.textures.iter().enumerate() {
        commands.spawn((SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(id),
            texture_atlas: handle.clone(),
            transform: Transform::from_translation(texture.center().extend(0.0)),
            ..Default::default()
        },));
    }
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
                // if cell != 14 && cell != 395 {
                //     continue;
                // }
                // if cell != -1 {
                //     info!("{layer_type:?}: {cell}");
                // }

                match cell {
                    0..=20 => {
                        let index = layer_type.to_index(cell as usize);

                        let atlas_handle = &assets.layers[layer_type];
                        let atlas = atlases.get(atlas_handle).unwrap();
                        let image = atlas.textures[index];
                        let offset = (image.height() - TILE_SIZE) / 2.0;

                        let x = (col_idx as f32 + 0.5) * TILE_SIZE;
                        let y = (row_idx as f32 + 0.5) * TILE_SIZE - offset;
                        //
                        // if cell == 14 {
                        //     info!("{col_idx}, {row_idx}, {x}, {y}, {index}");
                        // }

                        let collider_height = TILE_SIZE / 2.0;

                        commands
                            .spawn((
                                SpriteSheetBundle {
                                    sprite: TextureAtlasSprite::new(index),
                                    texture_atlas: atlas_handle.clone(),
                                    ..Default::default()
                                },
                                RigidBody::Fixed,
                                Position { x, y },
                                Layer(*layer_type),
                                Size::default(),
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    // Restitution::coefficient(0.1),
                                    Collider::cuboid(image.width() / 2.0, collider_height / 2.0),
                                    Transform::from_xyz(0.0, -offset, 0.0),
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
                            info!("Not mapped yet: {}", cell);
                        }
                    }
                }
            }
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
