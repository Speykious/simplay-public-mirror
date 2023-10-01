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
use bevy::render::texture::ImageSampler;
use bevy::log::LogPlugin;

use chunk::*;
use block::*;

// ==== DEBUG ====
const WIREFRAME: bool = false;
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
        ).set(
            ImagePlugin::default_nearest()
        ), WireframePlugin))
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_random_shit)
        .add_systems(Update, transform_chunk_system) // debug stuff
        .run();
}

// debug stuff
#[derive(Component)]
struct TransformChunk;

fn transform_chunk_system(
    mut cmds: Commands,
    mut cq: Query<&mut Transform, With<TransformChunk>>,
    time: Res<Time>
) {
    for mut i in cq.iter_mut() {
        i.rotate(Quat::from_rotation_y(1.0 * time.delta_seconds()));
    }
}
// ^ debug stuff

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
            transform: Transform::from_xyz(30.0, 30.0, 30.0),
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

    cmds.spawn((
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
        }, TransformChunk // debug stuff
    ));
}

fn spawn_camera(mut cmds: Commands) {
    cmds.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(20.0, 20.0, 20.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        }
    );
}
