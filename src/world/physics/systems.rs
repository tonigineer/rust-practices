use bevy::prelude::*;

use crate::{renderer::resources::CurrentWorldSize, world::balls::components::Ball};

pub(crate) fn integrate(mut balls: Query<(Entity, &Transform, &mut Ball)>) {
    let dt: f32 = super::PHYSICS_UPDATE_TIME;
    const GRAVITY: Vec2 = Vec2::new(0.0, -9.81);

    for (_entity, mut _tf, mut ball) in &mut balls {
        ball.prev_position = ball.position;
        let velocity = ball.velocity + dt * GRAVITY - dt * ball.drag();

        ball.position += velocity * dt;
        ball.velocity = velocity;
    }
}

pub(crate) fn boundary_collision(
    mut balls: Query<&mut Ball>,
    world_size: ResMut<CurrentWorldSize>
) {
    // All collisions feature a boundary with infinite mass, therefore
    // velocity is simply inverted with losses.
    const FRICTION: f32 = 0.5;
    let dt: f32 = super::PHYSICS_UPDATE_TIME;

    for mut ball in &mut balls {
        if ball.position.x > world_size.width - ball.radius {
            ball.position.x = world_size.width - ball.radius;
            ball.velocity.x *= -1.0;
            ball.velocity.y -= FRICTION * dt * ball.velocity.y.signum();
        }
        else if ball.position.x < ball.radius {
            ball.position.x = ball.radius;
            ball.velocity.x *= -1.0;
            ball.velocity.y -= FRICTION * dt * ball.velocity.y.signum();
        }

        if ball.position.y > world_size.height - ball.radius {
            ball.position.x = world_size.height - ball.radius;
            ball.velocity.y *= -1.0;
            if ball.velocity.length_squared() < 9.81 * 2.0 * dt { ball.velocity = Vec2::ZERO }
            ball.velocity.x -= FRICTION * dt * ball.velocity.x.signum();
        }
        else if ball.position.y < ball.radius {
            ball.position.y = ball.radius;
            ball.velocity.y *= -1.0;
            if ball.velocity.length_squared() < 9.81 * 2.0 * dt { ball.velocity = Vec2::ZERO }
            ball.velocity.x -= FRICTION * dt * ball.velocity.x.signum();
        }
    }
}

pub(crate) fn ball_collision(
    mut balls_query: Query<(Entity, &mut Transform, &mut Ball)>
) {
    let mut iteration = 0;
    loop {
        let mut collision_occurred = false;
        iteration += 1;
        info!("COllision check {}", iteration);
        let mut iter = balls_query.iter_combinations_mut();

        let mut num = 0;

        while let Some([item1, item2]) = iter.fetch_next() {
            let mut ball1 = item1.2;
            let mut ball2 = item2.2;

            num += 1;

            let vec_ball_12 = ball2.position - ball1.position;

            let distance = vec_ball_12.length();
            let combined_radius = ball1.radius + ball2.radius;

            if distance < combined_radius {
                collision_occurred = true;
                let rebound_dist = (combined_radius - distance) / 2.0;
                let ball1_ratio = ball2.mass / (ball1.mass + ball2.mass);
                let ball2_ratio = 1.0 - ball1_ratio;

                let normal = vec_ball_12.normalize();

                ball1.position -= normal * rebound_dist * ball1_ratio;
                ball2.position += normal * rebound_dist * ball2_ratio;

                let temp_ball1 = ball1.velocity - ball1_ratio * 2.0 * (
                    (ball1.velocity - ball2.velocity).dot(normal) * normal
                );
                
                let temp_ball2 = ball2.velocity - ball2_ratio * 2.0 * (
                    (ball2.velocity - ball1.velocity).dot(normal) * normal
                );

                ball1.velocity = temp_ball1;
                ball2.velocity = temp_ball2;
            }
        }

        for (_, _, mut ball) in &mut balls_query {
            // if (ball.prev_position - ball.position).length() < 9.81 * 0.01  && ball.position.y <= 0.05 {
            //     ball.velocity = Vec2::ZERO;
            // }
            if ball.velocity.length() < 9.81 * 0.01 * 2.0 {
                ball.velocity = Vec2::ZERO;
            }
        }
        info!("{:?}", num);

        if !collision_occurred || iteration > 20 { break }
    }
}
