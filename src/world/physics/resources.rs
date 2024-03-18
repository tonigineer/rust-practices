use bevy::prelude::Resource;

#[derive(Debug, Resource)]
pub struct PhysicsInfo {
    pub iterations_collisions: u32,
    pub execution_time: u128
}
