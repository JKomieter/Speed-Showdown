use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub dodge_challenger: Handle<Scene>,
    pub road: Handle<Scene>,
    pub coin: Handle<Scene>,
}

pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        dodge_challenger: asset_server.load("Dodge_Challenger.glb#Scene0"),
        road: asset_server.load("road.glb#Scene0"),
        coin: asset_server.load("Coin.glb#Scene0")
    }
}