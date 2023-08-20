mod voxel;
mod world;
mod library;
mod chunk;
mod chunk_manager;
mod perlin;

use bevy::prelude::*;
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::pbr::wireframe::*;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::window::PrimaryWindow;
use world::Axis;
use voxel::*;
use chunk::*;
use library::*;
use chunk_manager::*;

const ROTATE: bool = true;
const ROTATE_DETAILS: (bool, bool, bool) = (false, true, false);
const MOVE: bool = false;
const WIREFRAME: bool = true;
const DEBUG_MOUSE_MOVE: bool = true;

fn test_code() {
}

fn main() {
    test_code();

    App::new()
        .add_plugins((DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Simplay Survival".into(),
                    ..default()
                }),
                ..default()
            },
        ).set(
            RenderPlugin {
                wgpu_settings: WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }
            }
        ), WireframePlugin))
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(ChunkManager::new())
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, setup)
        .add_systems(Update, transform_system)
        .add_systems(Update, chunk_manager_system)
        .run();
}

fn transform_system(
    mut query: Query<&mut Transform, Without<StaticObj>>,
    camera_query: Query<&Transform, (With<Camera3d>, With<StaticObj>)>,
    time: Res<Time>,
    // mousebtn_input: Res<Input<MouseButton>>,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if ROTATE {
        for mut i in query.iter_mut() {
            if ROTATE_DETAILS.0 {
                i.rotate(Quat::from_rotation_x(0.5 * time.delta_seconds()));
            } if ROTATE_DETAILS.1 {
                i.rotate(Quat::from_rotation_y(0.25 * time.delta_seconds()));
            } if ROTATE_DETAILS.2 {
                i.rotate(Quat::from_rotation_z(0.0 * time.delta_seconds()));
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

    if DEBUG_MOUSE_MOVE {
        let window = windows.single();
        let pos = window.cursor_position().unwrap_or(Vec2::ZERO);

        let mut drag_pos = None;

        for mousebtn in mousebtn_evr.iter() {
            if mousebtn.button == MouseButton::Left && mousebtn.state == ButtonState::Released {
                drag_pos = None;
            }

            if mousebtn.button == MouseButton::Left && mousebtn.state == ButtonState::Pressed {
                drag_pos = Some(pos);
            }
        }

        let mut no_rotate = false;

        let drag_pos = match drag_pos {
            Some(s) => s,
            None => {
                no_rotate = true;
                Vec2::ZERO
            },
        };

        if no_rotate == false {
            let camera_transform = camera_query.single();

            for mut i in query.iter_mut() {
                let screenspace_mouse_vector = Vec2::new((drag_pos.x as f32) / window.width() * 2.0 - 1.0, (drag_pos.y as f32) / window.height() * 2.0 - 1.0);
                let worldspace_mouse_vector = (camera_transform.forward() + (camera_transform.right() * screenspace_mouse_vector.x) + (camera_transform.up() * -screenspace_mouse_vector.y)).normalize();
                let obj_to_camera_vec = (i.translation - camera_transform.translation).normalize();
                let offset_from_obj_to_camera_vec = (obj_to_camera_vec + worldspace_mouse_vector).normalize();
                let desired_rotation = Quat::from_rotation_arc(obj_to_camera_vec, offset_from_obj_to_camera_vec);

                i.rotate(desired_rotation);
            }
        }
    }
}

#[derive(Component)]
struct StaticObj;

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    wireframe_config.global = WIREFRAME;

    cmds.spawn(
        PointLightBundle {
            point_light: PointLight {
                intensity: 95000.0,
                shadows_enabled: true,
                range: 400.0,
                ..default()
            },
            transform: Transform::from_xyz(15.5, 40.3, 40.2),
            ..default()
        }
    );
}

fn spawn_camera(mut cmds: Commands) {
    cmds.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(15.0, 40.0, 40.0).looking_at(Vec3::new(5.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },

        StaticObj,
    ));
}
