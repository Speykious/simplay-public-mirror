mod voxel;
mod world;
mod library;
mod chunk;
mod chunk_manager;
mod perlin;

use bevy::prelude::*;
use world::Axis;
use voxel::*;
use chunk::*;
use library::*;
use chunk_manager::*;

const ROTATE: bool = false;
const ROTATE_DETAILS: (bool, bool, bool) = (false, true, false);
const MOVE: bool = false;

fn test_code() {
}

fn main() {
    test_code();

    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Simplay Survival".into(),
                    ..default()
                }),
                ..default()
            }
        ))
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(ChunkManager::new())
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, setup)
        .add_systems(Update, transform_system)
        .add_systems(Update, chunk_manager_system)
        .run();
}

fn transform_system(mut query: Query<&mut Transform, Without<StaticObj>>, time: Res<Time>) {
    if ROTATE {
        for mut i in query.iter_mut() {
            if ROTATE_DETAILS.0 {
                i.rotate(Quat::from_rotation_x(3.0 * time.delta_seconds()));
            } if ROTATE_DETAILS.1 {
                i.rotate(Quat::from_rotation_y(2.0 * time.delta_seconds()));
            } if ROTATE_DETAILS.2 {
                i.rotate(Quat::from_rotation_z(1.0 * time.delta_seconds()));
            }
        }
    }

    if MOVE {
        for mut i in query.iter_mut() {
            let i_forward = i.forward();
            let move_vec = Vec3::new(0.1, 0.0, 0.0);
            i.translation += move_vec * 4.0 * time.delta_seconds();
        }
    }
}

#[derive(Component)]
struct StaticObj;

fn setup(mut cmds: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    cmds.spawn(
        PointLightBundle {
            point_light: PointLight {
                intensity: 95000.0,
                shadows_enabled: true,
                range: 400.0,
                ..default()
            },
            transform: Transform::from_xyz(55.5, 110.3, 110.2),
            ..default()
        }
    );
}

fn spawn_camera(mut cmds: Commands) {
    cmds.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(85.0, 110.0, 110.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },

        StaticObj,
    ));
}
