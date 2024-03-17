use bevy::prelude::*;

use bevy::window::WindowResized;

use super::resources::{CurrentWindowSize, CurrentWorldSize, PixelPerMeter};

pub(crate) fn update_window_size(
    mut resize_events: EventReader<WindowResized>,
    mut window_size: ResMut<CurrentWindowSize>,
    mut world_size: ResMut<CurrentWorldSize>,
    pixel_per_meter: Res<PixelPerMeter>
) {
    // Get new size after resizing and store in resource
    for event in resize_events.read().into_iter() {
        window_size.width = event.width;
        window_size.height = event.height;
    }

    world_size.width = window_size.width / pixel_per_meter.0;
    world_size.height = window_size.height / pixel_per_meter.0;
}

use crate::world::balls::components::Ball;

pub(crate) fn update_balls(
    mut balls: Query<(Entity, &mut Transform, &Ball)>,
    window_size: ResMut<CurrentWindowSize>,
    pixel_per_meter: Res<PixelPerMeter>
) {
    let half_width: f32 = window_size.width / 2.0;
    let half_height: f32 = window_size.height / 2.0;

    for (_entity, mut tf, ball) in &mut balls {
        tf.translation.x = -half_width + ball.position.x * pixel_per_meter.0;
        tf.translation.y = -half_height + ball.position.y * pixel_per_meter.0;

        tf.scale = Vec3 {
            x: ball.radius * pixel_per_meter.0,
            y: ball.radius * pixel_per_meter.0,
            z: 0.0,
        }
    }
}
