mod components;
pub mod resources;
mod systems;

use bevy::prelude::{Plugin, App, Update, IntoSystemConfigs};
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;

use self::resources::{CurrentWindowSize, CurrentWorldSize, PixelPerMeter};

pub const WINDOW_WIDTH_INIT : f32 = 1280.0;
pub const WINDOW_HEIGHT_INIT: f32 = 720.0;
const PIXEL_PER_METER_DEFAULT: u32 = 300;
const FPS: f32 = 100.0;

pub struct RendererPlugin;


impl Plugin for RendererPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PixelPerMeter(PIXEL_PER_METER_DEFAULT as f32))
            .insert_resource(
                CurrentWindowSize {
                    width: WINDOW_HEIGHT_INIT,
                    height: WINDOW_HEIGHT_INIT
                }
            )
            .insert_resource(
                    CurrentWorldSize {
                        width: WINDOW_HEIGHT_INIT*PIXEL_PER_METER_DEFAULT as f32,
                        height: WINDOW_HEIGHT_INIT*PIXEL_PER_METER_DEFAULT as f32
                    }
            )
            .add_systems(Update, systems::update_window_size)
            .add_systems(Update,
                systems::update_balls
                    .run_if(on_timer(Duration::from_secs_f32(1.0/FPS)))
            );
    }
}
