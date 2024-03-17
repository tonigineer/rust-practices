pub mod components;
mod systems;

use bevy::prelude::*;

use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;

const PHYSICS_UPDATE_TIME: f32 = 0.01;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,
                systems::integrate
                    .run_if(on_timer(
                        Duration::from_secs_f32(PHYSICS_UPDATE_TIME))
                    )
            )
            .add_systems(
                Update,
                systems::boundary_collision
                    .run_if(on_timer(
                        Duration::from_secs_f32(PHYSICS_UPDATE_TIME))
                    )
                    .after(systems::integrate)
            )
            .add_systems(
                Update,
                systems::ball_collision
                    .run_if(on_timer(
                        Duration::from_secs_f32(PHYSICS_UPDATE_TIME))
                    )
                    .after(systems::boundary_collision)
            );
    }
}