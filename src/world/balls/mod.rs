pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;

use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;

use self::resources::NumberSpawnedBalls;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(NumberSpawnedBalls(0))
            .add_systems(
                Update,
                systems::spawn_ball
                    .run_if(on_timer(Duration::from_secs_f32(super::BALL_SPAWN_DELAY)))
            );
    }

}
