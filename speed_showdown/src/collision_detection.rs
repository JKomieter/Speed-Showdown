use crate::coin::Coin;
use crate::dodge_challenger::DodgeChallenger;
use crate::schedule::InGameSet;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Debug, PartialEq, Component)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_detection.in_set(InGameSet::CollisionDetection),
        )
        .add_systems(
            Update,
            handle_coin_collection
                .chain()
                .in_set(InGameSet::DespawnEntities),
        );
    }
}

pub fn collision_detection(
    dodge_challenger_query: Query<(Entity, &Collider), With<DodgeChallenger>>,
    coin_query: Query<(Entity, &Collider), With<Coin>>,
) {
   let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::default();

   for ()
}

fn handle_coin_collection(
    mut commands: Commands, 
    dodge_challenger_query: Query<&Collider, With<DodgeChallenger>>,
    coin_query: Query<(Entity, &Collider), With<Coin>>
)
{
   
    
}
