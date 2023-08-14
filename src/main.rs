mod voxel;
mod world;
mod library;

use bevy::prelude::*;
use world::Axis;
use voxel::*;

fn main() {
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
    for mut i in query.iter_mut() {
        i.rotate(Quat::from_rotation_y(2.0 * time.delta_seconds()));
    }

    for mut i in query.iter_mut() {
        let i_forward = i.forward();
        i.translation += i_forward * 4.0 * time.delta_seconds();
    }
}

#[derive(Component)]
struct StaticObj;

fn setup(mut cmds: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let cube_mesh = create_voxel_mesh(Voxel {
        id: Block::Debug,
        sides: vec![
            Axis::North,
            Axis::South,
            Axis::East,
            Axis::West,
            Axis::Up,
            Axis::Down,
        ],
        position: Vec3::new(0.0, 0.0, 0.0),
    });

    cmds.spawn(
        PbrBundle {
            mesh: meshes.add(cube_mesh),
            material: materials.add(Color::rgb(1.0, 0.2, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }
    );

    cmds.spawn(
        PointLightBundle {
            point_light: PointLight {
                intensity: 2500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.5, 7.3, 5.2),
            ..default()
        }
    );
}

fn spawn_camera(mut cmds: Commands) {
    cmds.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(10.0, 8.0, 8.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },

        StaticObj,
    ));
}
