use crate::assets_loader::SceneAssets;
use bevy::prelude::*;

const ROAD_TRANSLATION: Vec3 = Vec3::new(0.0, -8.5, 0.0);

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_roads);
    }
}

fn spawn_roads(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    for i in 0..30 {
        commands.spawn(SceneBundle {
            scene: scene_assets.road.clone(),
            transform: Transform::from_translation(
                ROAD_TRANSLATION + Vec3::new(0.0, 0.0, i as f32 * 230.0),
            )
            .with_scale(Vec3::splat(5.0))
            .with_rotation(Quat::from_rotation_y(0.0)),
            ..default()
        });
    }
}
