use bevy::prelude::*;
use bevy_third_person_camera::{camera::{Offset, Zoom}, ThirdPersonCamera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: 1,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 25.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        ThirdPersonCamera {
            offset_enabled: true,
            offset: Offset::new(0.0, 2.0),
            zoom_enabled: true,
            zoom: Zoom::new(40.0, 50.0),
            ..default()
        }
    ));
}