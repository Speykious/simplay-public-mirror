mod voxel;
mod world;
mod library;
mod chunk;
mod perlin;

use bevy::prelude::*;
use world::Axis;
use voxel::*;
use chunk::*;
use library::*;

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
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, setup)
        .add_systems(Update, transform_system)
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
    let mut chunk = Chunk::new(0, 0, 0);

    for x in 0..CHUNK_SIZE.0 {
        for y in 0..CHUNK_SIZE.1 {
            for z in 0..CHUNK_SIZE.2 {
                let v = perlin::noise_3d(x as f32, y as f32, z as f32);
                // println!("DEBUG: {}", v);
                if v > 5.0 {
                    chunk.set_block((x, y, z), Block::Debug);
                }
            }
        }
    }

    let voxel = Voxel {
        id: Block::Debug,
        sides: VoxelSide::vec_from_axis_vec(&vec![
            Axis::North,
            Axis::South,
            Axis::East,
            Axis::West,
            Axis::Up,
            Axis::Down,
        ], (0, 0, 0)),
    };

    let cube_mesh = voxel.into_mesh();

    cmds.spawn(
        PbrBundle {
            mesh: meshes.add(chunk.draw()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.2, 1.0),
                double_sided: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_xyz(-5.0, 0.0, 0.0),
            ..default()
        }
    );

    cmds.spawn(
        PbrBundle {
            mesh: meshes.add(cube_mesh),
            material: materials.add(Color::rgb(0.2, 1.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }
    );

    cmds.spawn(
        PointLightBundle {
            point_light: PointLight {
                intensity: 9500.0,
                shadows_enabled: true,
                range: 100.0,
                ..default()
            },
            transform: Transform::from_xyz(20.5, 30.3, 20.2),
            ..default()
        }
    );
}

fn spawn_camera(mut cmds: Commands) {
    cmds.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(15.0, 40.0, 40.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },

        StaticObj,
    ));
}
