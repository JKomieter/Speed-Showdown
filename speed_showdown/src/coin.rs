use bevy::prelude::*;
use crate::assets_loader::SceneAssets;
use rand::{thread_rng, Rng};

pub struct CoinPlugin;

const COIN_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 5.0);

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_coins);
    }
}

#[derive(Component)]
pub struct Coin(pub u32);

fn spawn_coins(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    for i in 0..30 {
        let space = thread_rng().gen_range(-10.0..10.0);
        commands.spawn((
            SceneBundle {
                scene: scene_assets.coin.clone(),
                transform: Transform::from_translation(
                    COIN_TRANSLATION + Vec3::new(space, 0.0, i as f32 * 50.0 + space)
                )
                .with_scale(Vec3::splat(3.0)),
                ..default()
            },
            Coin(i),
        ));
    }
}