use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::collisions::handle_collisions;
use crate::map::WorldMap;
use crate::player::{animate_player, handle_input, move_camera, Player, PlayerPositionEvent, spawn_player, update_player_position};
use crate::settings::{CAMERA_SCALE, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};
use crate::ui::show_ui;

mod settings;
mod player;
mod map;
mod layer;
mod ui;
mod collisions;

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
}

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
pub struct Layer(u32);

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
    grass: Handle<TextureAtlas>,
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
            .disabled())
        .add_plugin(ShapePlugin)
        .register_type::<Position>()
        .register_type::<Player>()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(
            WorldMap::new()
                .load_layer("assets/map/map_FloorBlocks.csv")
                .load_layer("assets/map/map_Grass.csv")
        )
        .add_state::<AppState>()
        .add_systems((load_background, load_assets).in_schedule(OnEnter(AppState::Load)))
        .add_system(setup_map_position.in_set(OnUpdate(AppState::Load)))
        .add_systems((spawn_map, show_ui).in_set(OnUpdate(AppState::SpawnMap)))
        .add_systems((
            handle_input,
            // move_player,
            update_player_position,
            move_camera,
            animate_player,
            handle_collisions,
        ).in_set(OnUpdate(AppState::Playing)))
        .add_system(position_tiles.in_base_set(CoreSet::PostUpdate))
        .add_event::<PlayerPositionEvent>()
        .run();
}

fn load_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let image = asset_server.load("map/ground.png");

    commands.spawn((
        SpriteBundle {
            texture: image.clone(),
            transform: Transform::from_xyz(0., 0., -1000.0),
            ..Default::default()
        },
        Map,
    ));

    commands.insert_resource(MapBackground { handle: image });
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("images/player.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 24, 1, None, None);

    let player_handle = texture_atlases.add(texture_atlas);

    let texture_handle = asset_server.load("images/grass.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 24, 1, None, None);
    let grass_handle = texture_atlases.add(texture_atlas);

    let assets = GameAssets {
        player: player_handle,
        grass: grass_handle,
    };

    commands.insert_resource(assets);
}

fn setup_map_position(
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut map_query: Query<&mut Transform, With<Map>>,
    window: Query<&Window, With<PrimaryWindow>>,
    assets: Res<Assets<Image>>,
    bg: Res<MapBackground>,
) {
    // println!("setup bounds");
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                if *handle == bg.handle {
                    let Ok(window) = window.get_single() else { return; };

                    let img = assets.get(&bg.handle).unwrap();
                    let width= img.texture_descriptor.size.width as f32;
                    let height = img.texture_descriptor.size.height as f32;

                    let size = MapSize {
                        width,
                        height,
                    };

                    // println!("Window: {}x{}", window.width(), window.height());
                    // println!("Map size: {size:?}");

                    let mut map_transform = map_query.single_mut();
                    map_transform.translation.x = (size.width - window.width()) / 2.;
                    map_transform.translation.y = -((size.height - window.height()) / 2.);

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
    window: Query<&Window, With<PrimaryWindow>>,
    map_size: Res<MapSize>,
    assets: Res<GameAssets>,
    world_map: Res<WorldMap>,
    asset_server: Res<AssetServer>,
) {
    let Ok(window) = window.get_single() else { return; };

    spawn_cameras(&mut commands, &map_size);
    spawn_tiles(&mut commands, world_map, &assets, asset_server);
    spawn_player(&mut commands, window, &assets, &map_size);

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
            near: -10000.0,
            far: 10000.0,
            ..default()
        },
        ..default()
    };

    commands.spawn((camera, Position { x, y }, Layer(999)))
    ;
    // commands.spawn(UiCameraBundle::default());
}

fn spawn_tiles(
    commands: &mut Commands,
    world_map: Res<WorldMap>,
    assets: &Res<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    let tile_size = TILE_SIZE as f32;
    let half_tile_size = tile_size / 2.;

    // Spawn the world
    for (layer_id, layer) in world_map.layers.iter().enumerate() {
        for (row_idx, row) in layer.data.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                // info!("{}", cell);
                let x = col_idx as f32 * tile_size + half_tile_size;
                let y = row_idx as f32 * tile_size + half_tile_size;

                match cell {
                    8..=10 => {
                        let mut rng = rand::thread_rng();

                        commands.spawn((
                            SpriteSheetBundle {
                                sprite: TextureAtlasSprite::new(rng.gen_range(0..3)),
                                texture_atlas: assets.grass.clone(),
                                ..Default::default()
                            },
                            RigidBody::Fixed,
                            Position { x, y },
                            Layer(layer_id as u32),
                            Size::default(),
                        )).with_children(|parent| {
                            parent.spawn((
                                // Restitution::coefficient(0.1),
                                Collider::cuboid(32.0, 16.0),
                                Transform::from_xyz(0.0, -16.0, 0.0),
                                ColliderDebugColor(Color::WHITE),
                            ));
                        });
                    }
                    395 => {
                        commands.spawn((
                            SpriteBundle {
                                texture: asset_server.load("test/rock.png"),
                                ..Default::default()
                            },
                            RigidBody::Fixed,
                            Position { x, y },
                            Layer(layer_id as u32),
                            Size::default(),
                        )).with_children(|parent| {
                            parent.spawn((
                                // Restitution::coefficient(0.1),
                                Collider::cuboid(32.0, 16.0),
                                Transform::from_xyz(0.0, -16.0, 0.0),
                                ColliderDebugColor(Color::WHITE),
                            ));
                        });
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

