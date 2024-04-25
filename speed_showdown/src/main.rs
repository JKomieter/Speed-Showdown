mod camera;
mod dodge_challenger;
mod light;
mod assets_loader;
mod movement;
mod schedule;
mod road;
mod coin;
mod collision_detection;

use bevy::prelude::*;
use camera::CameraPlugin;
use dodge_challenger::DodgeChallengerPlugin;
use light::LightPlugin;
use assets_loader::AssetsLoaderPlugin;
use schedule::SchedulePlugin;
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use road::RoadPlugin;
use coin::CoinPlugin;
use collision_detection::CollisionPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LightPlugin)
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(CameraPlugin)
        .add_plugins(ThirdPersonCameraPlugin)
        .add_plugins(RoadPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(AssetsLoaderPlugin)
        .add_plugins(DodgeChallengerPlugin)
        .add_plugins(CoinPlugin)
        .add_plugins(CollisionPlugin)
        .run();
}
