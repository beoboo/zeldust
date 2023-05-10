mod settings;
mod player;
mod map;
mod layer;

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use crate::map::WorldMap;
use crate::player::{move_camera, move_player, Player, PlayerPositionEvent};

use crate::settings::{FPS, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Load,
    SpawnMap,
    Playing,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    x: f32,
    y: f32,
    layer: u32,
}

#[derive(Component)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Default for Size {
    fn default() -> Self {
        Self { width: TILE_SIZE as f32, height: TILE_SIZE as f32 }
    }
}

pub struct MapBackground {
    handle: Handle<Image>,
}

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct MapBorder;

#[derive(Component)]
pub struct MapSize {
    width: u32,
    height: u32,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Zeldust".to_string(),
            width: SCREEN_WIDTH as f32,
            height: SCREEN_HEIGHT as f32,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(
            WorldMap::new()
                .load_layer("assets/map/map_FloorBlocks.csv")
        )
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Load)
        .add_system_set(
            SystemSet::on_enter(AppState::Load)
                .with_system(load_background)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Load)
                .with_system(setup_bounds)
        )
        .add_system_set(
            SystemSet::on_enter(AppState::SpawnMap)
                .with_system(spawn_player_and_cameras)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::SpawnMap)
                .with_system(spawn_tiles)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Playing)
                .with_run_criteria(FixedTimestep::step(1.0 / FPS as f64))
                .with_system(move_player)
                .with_system(move_camera)
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            position_tiles,
        )
        .add_event::<PlayerPositionEvent>()
        .run();
}

fn load_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // println!("load background");
    let image = asset_server.load("map/ground.png");

    commands.spawn_bundle(SpriteBundle {
        texture: image.clone(),
        ..Default::default()
    })
        .insert(Map {})
        .insert(Position { x: 0., y: 0., layer: 0 });

    commands.insert_resource(MapBackground { handle: image });
}

fn setup_bounds(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut map_query: Query<&mut Position, With<Map>>,
    windows: Res<Windows>,
    assets: Res<Assets<Image>>,
    bg: Res<MapBackground>,
) {
    // println!("setup bounds");
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                if *handle == bg.handle {
                    let img = assets.get(bg.handle.clone()).unwrap();

                    let size = MapSize {
                        width: img.texture_descriptor.size.width,
                        height: img.texture_descriptor.size.height,
                    };

                    // let window = windows.get_primary().unwrap();
                    let mut map_pos = map_query.single_mut();
                    map_pos.x = (size.width as f32) / 2.;
                    map_pos.y = (size.height as f32) / 2.;

                    commands.insert_resource(size);
                    app_state.set(AppState::SpawnMap).unwrap();
                }
            }
            _ => {}
        }
    }
}

fn spawn_player_and_cameras(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
    map_size: Res<MapSize>,
) {
    println!("spawn player and cameras");

    let (x, y) = (map_size.width as f32 / 2., map_size.height as f32 / 2.);

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 5.;

    commands.spawn_bundle(camera)
        .insert(Position { x, y, layer: 999 })
    ;

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("test/player.png"),
        ..Default::default()
    })
        .insert(Player::default())
        .insert(Position { x, y, layer: 1 })
        .insert(Size::default())
    ;

    app_state.set(AppState::Playing).unwrap();
}

fn spawn_tiles(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    asset_server: Res<AssetServer>,
) {
    println!("spawn tiles");

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
                //     commands.spawn_bundle(OrthographicCameraBundle::new_2d())
                //         .insert(Position { x, y })
                //     ;
                //
                //     commands.spawn_bundle(SpriteBundle {
                //         texture: asset_server.load("test/player.png"),
                //         ..Default::default()
                //     })
                //         .insert(Position { x, y })
                //         .insert(Player::default())
                //     ;
                // }
                395 => {
                    commands.spawn_bundle(SpriteBundle {
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
    windows: Res<Windows>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_dim: f32) -> f32 {
        pos - (bound_dim / 2.)
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x, window.width() as f32),
            -convert(pos.y, window.height() as f32),
            pos.layer as f32,
        );
    }
}

