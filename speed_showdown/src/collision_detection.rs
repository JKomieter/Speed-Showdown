use bevy::prelude::*;
use crate::schedule::InGameSet;
use bevy::utils::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub struct Collider {
    pub radius: usize,
    pub colliding_entities: Vec<Entity>
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection.in_set(InGameSet::CollisionDetection));
    }
}

pub fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let _colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();
    
}
