mod components;
mod resources;
mod systems;

use bevy::prelude::{Plugin, Startup, Update, App, IntoSystemConfigs};
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;

const FPS: f32 = 100.0;

pub struct SpectatorPlugin;

impl Plugin for SpectatorPlugin {
    fn build(&self, app: &mut App) {
        app       
            .add_systems(Startup, systems::spawn_camera)
            .add_systems(Startup, systems::show_text)
            .add_systems(Update,
                systems::update_fps
                    .run_if(on_timer(Duration::from_secs_f32(1.0/FPS)))
            )
            .add_systems(Update,
                systems::update_balls
                    .run_if(on_timer(Duration::from_secs_f32(1.0/FPS)))
            );
            // .add_systems(Update, systems::grab_ball);
    }
}
