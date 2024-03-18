use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

use super::{
    components::Ball,
    resources::NumberSpawnedBalls,
    super::MAX_BALLS_SPAWNING
};

pub(crate) fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut number_spawned_balls: ResMut<NumberSpawnedBalls>,
    time: Res<Time>
) {
    // Renderer takes care of position and size, therefore a blank ball
    // is spawned.
    if number_spawned_balls.0 >= MAX_BALLS_SPAWNING { return }

    commands.spawn(
        (
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(1.0)).into(),
                transform: Transform::from_xyz(
                    5000.0,
                    5000.0,
                    0.0
                ),
                material: materials.add(get_rainbow_color(time)),
                ..default()
            },
            Ball::new(Some(Ball::RADIUS))
        )
    );

    number_spawned_balls.0 += 1;
}

fn get_rainbow_color(time: Res<Time>) -> Color {
    let t = time.elapsed_seconds() / 5.0;

    let r = t.sin();
    let g = (t + 0.33 * 2.0 * std::f32::consts::PI).sin();
    let b = (t + 0.66 * 2.0 * std::f32::consts::PI).sin();

    Color::rgb(r*r, g*g, b*b)
}