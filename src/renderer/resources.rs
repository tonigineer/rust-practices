use bevy::prelude::Resource;

#[derive(Resource)]
pub struct PixelPerMeter(pub f32);

#[derive(Resource)]
pub struct CurrentWindowSize {
    pub width: f32,
    pub height: f32
}

#[derive(Resource)]
pub struct CurrentWorldSize {
    pub width: f32,
    pub height: f32
}
