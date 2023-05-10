use bevy::{
    app::App,
    prelude::{Plugin, *},
    window::PrimaryWindow,
};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use enum_iterator::Sequence;
use parse_display::Display;

use crate::{
    camera::{move_camera, spawn_camera},
    collisions::{
        damage_attackable,
        handle_collisions,
        handle_magic_collisions,
        handle_weapon_collisions,
        kill_attackable,
        OBJECTS_COLLISION_GROUP,
    },
    constants::TILE_SIZE,
    debug::{can_spawn, MAX_ENEMIES, MAX_TILES},
    entities::{
        end_enemy_attack,
        end_player_attack,
        end_player_spell_cast,
        from_position,
        handle_enemy_hit,
        handle_player_hit,
        move_enemy,
        render_enemy,
        render_player,
        spawn_enemy,
        spawn_player,
        update_depth,
        Attackable,
    },
    frames::TexturePack,
    magic::{cast_spell, recover_energy, switch_magic},
    map::{LayerType, WorldMap},
    particles::{animate_particles, spawn_particles},
    screens::{game::input::handle_input, is_playing},
    ui::{
        change_magic_item,
        change_weapon_item,
        end_switch_magic,
        end_switch_weapon,
        spawn_ui,
        update_energy_ui,
        update_health_ui,
        update_xp_ui,
    },
    weapon::{spawn_weapon, switch_weapon},
    AppState,
    GameAssets,
    Layer,
    Map,
    MapSize,
};
use crate::collisions::{damage_player};

mod input;

pub struct GameScreenPlugin;

impl Plugin for GameScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                // debug_tiles,
                spawn_ground,
                spawn_camera,
                spawn_tiles.after(spawn_camera),
                spawn_ui.after(spawn_tiles),
            )
                .in_schedule(OnEnter(AppState::RunLevel)),
        )
        .add_systems(
            (move_camera, update_energy_ui, update_health_ui, update_xp_ui).in_set(OnUpdate(AppState::RunLevel)),
        )
        .add_systems(
            (
                switch_weapon.run_if(is_playing),
                spawn_weapon.run_if(is_playing),
                change_weapon_item.run_if(is_playing),
                end_switch_weapon.run_if(is_playing),
                end_player_attack.run_if(is_playing),
                handle_weapon_collisions.run_if(is_playing),
            )
                .in_set(OnUpdate(AppState::RunLevel)),
        )
        .add_systems(
            (
                switch_magic.run_if(is_playing),
                cast_spell.run_if(is_playing),
                change_magic_item.run_if(is_playing),
                end_switch_magic.run_if(is_playing),
                end_player_spell_cast.run_if(is_playing),
                handle_magic_collisions.run_if(is_playing),
                recover_energy.run_if(is_playing),
            )
                .in_set(OnUpdate(AppState::RunLevel)),
        )
        .add_system(handle_input.run_if(is_playing))
        .add_systems(
            (
                end_enemy_attack.run_if(is_playing),
                handle_enemy_hit.run_if(is_playing),
                handle_player_hit.run_if(is_playing),
                move_enemy.run_if(is_playing),
                render_player,
                render_enemy,
                update_depth,
                handle_collisions.run_if(is_playing),
                damage_player.run_if(is_playing),
                damage_attackable.run_if(is_playing),
                kill_attackable.after(damage_attackable).run_if(is_playing),
            )
                .in_set(OnUpdate(AppState::RunLevel)),
        )
        .add_systems(
            (spawn_particles.run_if(is_playing), animate_particles.run_if(is_playing))
                .in_set(OnUpdate(AppState::RunLevel)),
        );
    }
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
//
// fn debug_tiles(
//     mut commands: Commands,
//     assets: Res<GameAssets>,
//     atlases: Res<Assets<TextureAtlas>>,
// ) {
//     let handle = &assets.layers[&LayerType::Objects];
//     let atlas = atlases.get(&handle).unwrap();
//
//     for (id, texture) in atlas.textures.iter().enumerate() {
//         commands.spawn((SpriteSheetBundle {
//             sprite: TextureAtlasSprite::new(id),
//             texture_atlas: handle.clone(),
//             transform: Transform::from_translation(texture.center().extend(0.0)),
//             ..Default::default()
//         },));
//     }
// }

fn spawn_tiles(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    world_map: Res<WorldMap>,
    assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    atlases: Res<Assets<TextureAtlas>>,
    textures: Res<Assets<TexturePack>>,
) {
    let window = window.single();
    let mut num_tiles = 0;
    let mut num_enemies = 0;

    // Spawn the world
    for (layer_type, layer) in world_map.layers.iter() {
        for (row_idx, row) in layer.data.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                let x = (col_idx as f32 + 0.5) * TILE_SIZE;
                let y = (row_idx as f32 + 0.5) * TILE_SIZE;

                match cell {
                    0..=20 => {
                        num_tiles += 1;
                        if !can_spawn(num_tiles, MAX_TILES) {
                            continue;
                        }
                        spawn_tile(&mut commands, &window, &assets, &atlases, layer_type, cell, x, y);
                    },
                    390..=393 => {
                        num_enemies += 1;
                        if !can_spawn(num_enemies, MAX_ENEMIES) {
                            continue;
                        }

                        spawn_enemy(
                            &mut commands,
                            &window,
                            &asset_server,
                            &assets,
                            &atlases,
                            &textures,
                            cell,
                            x,
                            y,
                        );
                    },
                    394 => {
                        spawn_player(&mut commands, &window, &assets, x, y);
                    },
                    395 => {
                        spawn_block(&mut commands, &window, &asset_server, layer_type, x, y);
                    },
                    _ => {
                        if cell != -1 {
                            info!("Not mapped yet: {}", cell);
                        }
                    },
                }
            }
        }
    }
}

fn spawn_tile(
    commands: &mut Commands,
    window: &Window,
    assets: &Res<GameAssets>,
    atlases: &Res<Assets<TextureAtlas>>,
    layer_type: &LayerType,
    cell: i32,
    x: f32,
    y: f32,
) {
    let index = layer_type.to_index(cell as usize);
    let asset_type = layer_type.into();

    let atlas_handle = assets.get(asset_type);
    let atlas = atlases.get(atlas_handle).unwrap();
    let rect = atlas.textures[index];
    let offset = (rect.height() - TILE_SIZE) / 2.0;

    let y = y - offset;

    let collider_height = TILE_SIZE / 2.0;

    let mut cmd = commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(index),
            texture_atlas: atlas_handle.clone(),
            transform: Transform::from_translation(from_position(x, y, window)),
            ..Default::default()
        },
        RigidBody::Fixed,
        Layer(*layer_type),
    ));

    cmd.with_children(|parent| {
        let mut child = parent.spawn((
            Collider::cuboid(rect.width() / 2.0, collider_height / 2.0),
            Transform::from_xyz(0.0, -offset, 0.0),
            ColliderDebugColor(Color::BLUE),
        ));

        if layer_type.is_attackable() {
            // println!("Rect: {:?} {:?}", rect.width(), rect.height());
            child.insert((
                Collider::cuboid(rect.width() / 2.0, rect.height() / 2.0),
                OBJECTS_COLLISION_GROUP.clone(),
                ActiveEvents::COLLISION_EVENTS,
                ColliderDebugColor(Color::BLACK),
            ));
        }
    });

    if layer_type.is_attackable() {
        cmd.insert(Attackable::new(1));
    }
}

fn spawn_block(
    commands: &mut Commands,
    window: &Window,
    asset_server: &Res<AssetServer>,
    layer_type: &LayerType,
    x: f32,
    y: f32,
) {
    commands
        .spawn((
            SpriteBundle {
                texture: asset_server.load("test/rock.png"),
                transform: Transform::from_translation(from_position(x, y, window)),
                ..Default::default()
            },
            RigidBody::Fixed,
            Layer(*layer_type),
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
                ColliderDebugColor(Color::NAVY),
            ));
        });
}
