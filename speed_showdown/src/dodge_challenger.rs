use crate::assets_loader::SceneAssets;
use crate::movement::{Acceleration, MovingObjectBundle, Velocity};
use crate::schedule::InGameSet;
use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;

const STARTING_TRANSLATION: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 20.0,
};

pub struct DodgeChallengerPlugin;

impl Plugin for DodgeChallengerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_dodge_challenger)
            .add_systems(
                Update,
                (drive_dodge_challenger)
                    .chain()
                    .in_set(InGameSet::UserInput),
            );
    }
}

#[derive(Debug, Component)]
struct DodgeChallenger;

fn spawn_dodge_challenger(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity::new(0.0),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SceneBundle {
                scene: scene_assets.dodge_challenger.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION)
                    .with_scale(Vec3::splat(2.0))
                    .with_rotation(Quat::from_rotation_y(1.0 * std::f32::consts::PI)),
                ..Default::default()
            },
        },
        DodgeChallenger,
        ThirdPersonCameraTarget,
    ));
}

fn drive_dodge_challenger(
    mut query: Query<(&mut Transform, &mut Velocity), With<DodgeChallenger>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
        let mut direction = 0.0;
        // left and right movement
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction += 1.0;
        }

        // forward and backward movement
        if keyboard_input.pressed(KeyCode::Space) {
            velocity.value += 0.3;
            // move car forward in the direction it is facing
            let z = transform.local_z();
            transform.translation += z * time.delta_seconds() * velocity.value;
        }

        // change car direction
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() * direction));
    }
}
