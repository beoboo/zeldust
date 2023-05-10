mod settings;

use bevy::prelude::*;
use bevy_tileset::prelude::*;

use crate::settings::{build_world, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

#[derive(Default)]
struct WorldMap {
    data: Vec<String>,
}

#[derive(Default)]
struct MyTileset {
    handle: Option<Handle<Tileset>>,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: u32,
    y: u32,
}

fn load_tiles(mut my_tileset: ResMut<MyTileset>, asset_server: Res<AssetServer>) {
    my_tileset.handle = Some(asset_server.load("tilesets/test.ron"));
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
        .add_plugins(DefaultPlugins)
        .add_plugin(TilesetPlugin::default())
        // /== Required === //
        .init_resource::<WorldMap>()
        .init_resource::<MyTileset>()
        .add_startup_system(load_world_map)
        .add_startup_system(build_camera)
        .add_system(spawn_tiles)
        .add_system(position_tiles)
        .run();
}

fn load_world_map(mut world_map: ResMut<WorldMap>) {
    world_map.data = build_world();
}

fn build_camera(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_tiles(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    mut has_built_map: Local<bool>,
    asset_server: Res<AssetServer>,
) {
    if *has_built_map {
        return;
    }

    // Spawn the world
    for (row_idx, row) in world_map.data.iter().enumerate() {
        for (col_idx, ch) in row.chars().enumerate() {
            print!("{}", ch);
            match ch {
                'p' => {
                    commands.spawn_bundle(SpriteBundle {
                        texture: asset_server.load("test/player.png"),
                        ..Default::default()
                    }).insert(Position { x: col_idx as u32, y: row_idx as u32 });
                }
                'x' => {
                    commands.spawn_bundle(SpriteBundle {
                        // texture: rock_handle.clone_weak(),
                        texture: asset_server.load("test/rock.png"),
                        // texture: rock_handle.clone(),
                        ..Default::default()
                    }).insert(Position { x: col_idx as u32, y: row_idx as u32 });
                }
                _ => {
                    if ch != ' ' {
                        println!("Ignoring: {}", ch);
                    }
                }
            }
        }
        println!();
    }

    *has_built_map = true;
}

fn position_tiles(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_dim: f32) -> f32 {
        // let num_tiles = bound_dim / TILE_SIZE;
        let tile_size = TILE_SIZE as f32;
        pos * tile_size - (bound_dim / 2.) + (tile_size / 2.)
    //      // (bound_dim / 2.) - pos * tile_size
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32),
            -convert(pos.y as f32, window.height() as f32),
            0.0,
        )
    }
}

