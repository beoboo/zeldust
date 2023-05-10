use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;

use crate::map::WorldMap;
use crate::player::{animate_player, handle_input, move_camera, Player, PlayerPositionEvent, spawn_player};
use crate::settings::{CAMERA_SCALE, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};
use crate::ui::show_ui;

mod settings;
mod player;
mod map;
mod layer;
mod ui;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Load,
    SpawnMap,
    Playing,
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
pub struct Position {
    x: f32,
    y: f32,
    layer: u32,
}

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

#[derive(Resource)]
pub struct MapBackground {
    handle: Handle<Image>,
}

#[derive(Resource)]
pub struct MapSize {
    width: u32,
    height: u32,
}

#[derive(Component, Reflect)]
pub struct Map;

#[derive(Component)]
pub struct MapBorder;

#[derive(Resource)]
pub struct GameAssets {
    player: Handle<TextureAtlas>,
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
        .add_plugin(ShapePlugin)
        .register_type::<Position>()
        .register_type::<Player>()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(
            WorldMap::new()
                .load_layer("assets/map/map_FloorBlocks.csv")
        )
        .add_state::<AppState>()
        .add_system(load_background.in_schedule(OnEnter(AppState::Load)))
        .add_system(load_assets.in_schedule(OnEnter(AppState::Load)))
        .add_system(setup_bounds.in_set(OnUpdate(AppState::Load)))
        .add_system(spawn_map.in_set(OnUpdate(AppState::SpawnMap)))
        .add_system(show_ui.in_set(OnUpdate(AppState::SpawnMap)))
        .add_system(handle_input.in_set(OnUpdate(AppState::Playing)))
        .add_system(move_camera.in_set(OnUpdate(AppState::Playing)))
        .add_system(animate_player.in_set(OnUpdate(AppState::Playing)))
        .add_system(position_tiles.in_base_set(CoreSet::PostUpdate))
        .add_event::<PlayerPositionEvent>()
        .run();
}

fn load_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // println!("load background");
    let image = asset_server.load("map/ground.png");

    commands.spawn(SpriteBundle {
        texture: image.clone(),
        ..Default::default()
    })
        .insert(Map {})
        .insert(Position { x: 0., y: 0., layer: 0 });

    commands.insert_resource(MapBackground { handle: image });
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player/player.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 24, 1, None, None);

    let assets = GameAssets {
        player: texture_atlases.add(texture_atlas)
    };

    commands.insert_resource(assets);
}

fn setup_bounds(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut map_query: Query<&mut Position, With<Map>>,
    assets: Res<Assets<Image>>,
    bg: Res<MapBackground>,
) {
    // println!("setup bounds");
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                if *handle == bg.handle {
                    let img = assets.get(&bg.handle).unwrap();

                    let size = MapSize {
                        width: img.texture_descriptor.size.width,
                        height: img.texture_descriptor.size.height,
                    };

                    // let window = windows.get_primary().unwrap();
                    let mut map_pos = map_query.single_mut();
                    map_pos.x = (size.width as f32) / 2.;
                    map_pos.y = (size.height as f32) / 2.;

                    commands.insert_resource(size);
                    app_state.set(AppState::SpawnMap);
                }
            }
            _ => {}
        }
    }
}

fn spawn_map(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    map_size: Res<MapSize>,
    assets: Res<GameAssets>,
) {
    spawn_cameras(&mut commands, &map_size);
    spawn_player(&mut commands, assets, &map_size);

    app_state.set(AppState::Playing);
}

fn spawn_cameras(
    commands: &mut Commands,
    map_size: &Res<MapSize>,
) {
    // println!("spawn cameras");

    let (x, y) = (map_size.width as f32 / 2., map_size.height as f32 / 2.);

    let mut camera = Camera2dBundle {
        projection: OrthographicProjection {
            scale: CAMERA_SCALE,
            ..default()
        },
        ..default()
    };

    commands.spawn(camera)
        .insert(Position { x, y, layer: 999 })
    ;
    // commands.spawn(UiCameraBundle::default());
}

fn spawn_tiles(
    commands: &mut Commands,
    world_map: Res<WorldMap>,
    asset_server: Res<AssetServer>,
) {
    // println!("spawn tiles");

    let tile_size = TILE_SIZE as f32;
    let half_tile_size = tile_size / 2.;
    // Spawn the world
    for (row_idx, row) in world_map.layers[0].data.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            // print!("{}", cell);
            let x = col_idx as f32 * tile_size + half_tile_size;
            let y = row_idx as f32 * tile_size + half_tile_size;

            match cell {
                // 'p' => {
                //     commands.spawn(OrthographicCameraBundle::new_2d())
                //         .insert(Position { x, y })
                //     ;
                //
                //     commands.spawn(SpriteBundle {
                //         texture: asset_server.load("test/player.png"),
                //         ..Default::default()
                //     })
                //         .insert(Position { x, y })
                //         .insert(Player::default())
                //     ;
                // }
                395 => {
                    commands.spawn(SpriteBundle {
                        texture: asset_server.load("test/rock.png"),
                        ..Default::default()
                    })
                        .insert(MapBorder)
                        .insert(Position { x, y, layer: 1 })
                        .insert(Size::default())
                    ;
                }
                _ => {
                    if cell != -1 {
                        println!("Ignoring: {}", cell);
                    }
                }
            }
        }
        // println!();
    }
}

fn position_tiles(
    window: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform), Changed<Position>>,
) {
    fn convert(pos: f32, bound_dim: f32) -> f32 {
        pos - (bound_dim / 2.)
    }

    let Ok(window) = window.get_single() else { return; };

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x, window.width() as f32),
            -convert(pos.y, window.height() as f32),
            pos.layer as f32,
        );

        // dbg!(transform.translation);
    }
}

