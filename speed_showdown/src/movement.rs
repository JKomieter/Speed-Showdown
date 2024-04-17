use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: f32
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3
}

impl Velocity {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SceneBundle,
}