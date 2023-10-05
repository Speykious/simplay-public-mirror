mod chunk;
mod block;
mod voxel;
mod world;
mod mesher;
mod random;
mod noise;
mod filesystem;
mod asset_manager;
mod dir;
mod places;
mod log;

use std::env;
use bevy::prelude::*;
use bevy::pbr::wireframe::*;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::log::LogPlugin;

use chunk::*;
use block::*;

// ==== DEBUG ====
const WIREFRAME: bool = true;
// ===============

#[derive(PartialEq, Eq)]
enum ExitCode {
    Success,
    Fail,
}

fn main() {
    if app() == ExitCode::Fail {
        std::process::exit(1);
    }
}

fn app() -> ExitCode {
    env::set_var("BEVY_ASSET_ROOT", places::assets().to_string());

    match places::create_all_dirs() {
        Ok(_) => (),
        Err(_) => return ExitCode::Fail,
    };

    match asset_manager::build_assets_if_needed() {
        Ok(_) => (),
        Err(_) => return ExitCode::Fail,
    };

    App::new()
        .add_plugins((DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Simplay Survival".into(),
                    ..default()
                }),
                ..default()
            }
        ).set(
            RenderPlugin {
                wgpu_settings: WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }
            }
        ).set(
            LogPlugin {
                filter: "wgpu_core=warn,wgpu_hal=off,rechannel=warn".into(),
                level: bevy::log::Level::WARN,
            }
        ).set(
            ImagePlugin::default_nearest()
        ), WireframePlugin))
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_random_shit)
        .run();

    return ExitCode::Success;
}

fn setup(
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    wireframe_config.global = WIREFRAME;
}

fn spawn_random_shit(
    mut cmds: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: ResMut<AssetServer>,
) {
    cmds.spawn(
        PointLightBundle {
            point_light: PointLight {
                intensity: 99000.0,
                shadows_enabled: true,
                range: 5000.0,
                ..default()
            },
            transform: Transform::from_xyz(-30.0, 30.0, 30.0),
            ..default()
        }
    );

    let mut chunk = Chunk::new((0, 0, 0));

    for x in 0..CHUNK_SIZE.0 {
        for y in 0..CHUNK_SIZE.1 {
            for z in 0..CHUNK_SIZE.2 {
                chunk.set_block_u8((x, y, z), BlockType::Debug);
            }
        }
    }

    let chunk_mesh = chunk.mesh();

    cmds.spawn(
        PbrBundle {
            mesh: meshes.add(chunk_mesh),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(StandardMaterial {
                // base_color: Color::rgb(0.05, 0.5, 0.35),
                base_color_texture: Some(asset_server.load("textures/block/grass_side.png")),
                // double_sided: true, // debug
                // cull_mode: None, // debug
                ..default()
            }),
            ..default()
        }
    );

    cmds.spawn(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(-1.0, -1.0, -1.0),
            material: materials.add(StandardMaterial {
                // base_color: Color::rgb(0.05, 0.5, 0.35),
                base_color_texture: Some(asset_server.load("textures/block/diamond.png")),
                // double_sided: true, // debug
                // cull_mode: None, // debug
                ..default()
            }),
            ..default()
        }
    );

    cmds.spawn(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(CHUNK_SIZE.0 as f32, CHUNK_SIZE.1 as f32, CHUNK_SIZE.2 as f32),
            material: materials.add(StandardMaterial {
                // base_color: Color::rgb(0.05, 0.5, 0.35),
                base_color_texture: Some(asset_server.load("textures/block/oak_log_end.png")),
                // double_sided: true, // debug
                // cull_mode: None, // debug
                ..default()
            }),
            ..default()
        }
    );
}

fn spawn_camera(mut cmds: Commands) {
    cmds.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(-8.0, CHUNK_SIZE.1 as f32 * 0.5, 8.0)
                .looking_at(Vec3::new(CHUNK_SIZE.0 as f32 * 0.5, CHUNK_SIZE.1 as f32 * 0.5, CHUNK_SIZE.2 as f32 * 0.5), Vec3::Y),
            ..default()
        }
    );
}
