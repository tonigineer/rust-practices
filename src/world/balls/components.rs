use bevy::prelude::{Component, Vec2};

#[derive(Debug, Component)]
pub struct Ball {
    pub mass: f32,
    pub radius: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub prev_position: Vec2
}

impl Ball {
    // Physics: SI unit
    pub const DENSITY: f32 = 0.010;
    pub const RADIUS: f32 = 0.005;
    pub const VX_INIT: f32 = 4.0;
    pub const VY_INIT: f32 = 0.0;
    pub const X_INIT: f32 = 0.0;
    pub const Y_INIT: f32 = 2.0;

    pub fn new(radius: Option<f32>) -> Self {
        let radius = radius.unwrap_or(Self::RADIUS);
        Self {
            mass: Self::DENSITY * 4.0/3.0 * std::f32::consts::PI * radius,
            radius,
            position: Vec2::new(Self::X_INIT, Self::Y_INIT),
            velocity: Vec2::new(Self::VX_INIT, Self::VY_INIT),
            prev_position: Vec2::new(5000.0, 5000.0)
        }
    }

    pub fn drag(&self) -> Vec2 {
        // F_d = 0.5 * C * rho * A * v*v
        let drag_coefficient: f32 = 0.5 * 0.45 * 1.25 * self.radius.powf(2.0) * std::f32::consts::PI;
        return Vec2 {
            x: (drag_coefficient * self.velocity.x.powf(2.0)) / self.mass * self.velocity.x.signum(),
            y: (drag_coefficient * self.velocity.y.powf(2.0)) / self.mass * self.velocity.y.signum(),
        }
    }
}