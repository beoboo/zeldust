mod settings;
mod player;

use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_tileset::prelude::*;
use crate::player::{move_camera, move_player, Player, PlayerPositionEvent};

use crate::settings::{build_world, FPS, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};

#[derive(Default)]
struct WorldMap {
    data: Vec<String>,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum GameLabel {
    WorldMap,
    SpawnTiles,
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
        .init_resource::<WorldMap>()
        .add_startup_system(build_scene)
        .add_startup_system(load_world_map.label(GameLabel::WorldMap))
        .add_startup_system(spawn_tiles.label(GameLabel::SpawnTiles).after(GameLabel::WorldMap))
        .add_system_set(
            SystemSet::new()
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

fn load_world_map(mut world_map: ResMut<WorldMap>) {
    world_map.data = build_world();
}

fn build_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("map/ground.png"),
        transform: Transform {
            translation: Vec3::new(-(SCREEN_WIDTH as f32 / 2.), -(SCREEN_HEIGHT as f32 / 2.), 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_tiles(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    asset_server: Res<AssetServer>,
) {
    // Spawn the world
    for (row_idx, row) in world_map.data.iter().enumerate() {
        for (col_idx, ch) in row.chars().enumerate() {
            print!("{}", ch);
            let x = col_idx as f32 * TILE_SIZE as f32;
            let y = row_idx as f32 * TILE_SIZE as f32;

            match ch {
                'p' => {
                    commands.spawn_bundle(OrthographicCameraBundle::new_2d())
                        .insert(Position { x, y })
                    ;

                    commands.spawn_bundle(SpriteBundle {
                        texture: asset_server.load("test/player.png"),
                        ..Default::default()
                    })
                        .insert(Position { x, y })
                        .insert(Player::default())
                    ;
                }
                'x' => {
                    commands.spawn_bundle(SpriteBundle {
                        texture: asset_server.load("test/rock.png"),
                        ..Default::default()
                    })
                        .insert(Position { x, y })
                    ;
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
}

fn position_tiles(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_dim: f32) -> f32 {
        let tile_size = TILE_SIZE as f32;
        pos - (bound_dim / 2.) + (tile_size / 2.)
    }

    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x, window.width() as f32),
            -convert(pos.y, window.height() as f32),
            1.,
        );
    }
}

