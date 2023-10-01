mod chunk;
mod block;
mod voxel;
mod world;
mod mesher;
mod random;
mod noise;

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

fn main() {
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
                filter: "info,wgpu_core=warn,wgpu_hal=off,rechannel=warn".into(),
                level: bevy::log::Level::DEBUG,
            }
        ), WireframePlugin))
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_random_shit)
        .run();
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
) {
    cmds.spawn(
        PointLightBundle {
            point_light: PointLight {
                intensity: 95000.0,
                shadows_enabled: true,
                range: 500.0,
                ..default()
            },
            transform: Transform::from_xyz(26.0, 24.0, 25.0),
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
                base_color: Color::rgb(0.05, 0.5, 0.35),
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
            transform: Transform::from_xyz(40.0, 40.0, 40.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        }
    );
}
