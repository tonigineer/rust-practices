use bevy::prelude::*;

mod environment;
use environment::{EnvironmentPlugin, POOL_DEPTH, POOL_THICKNESS, POOL_WIDTH, PIXEL_PER_METER};

const WINDOW_WIDTH: f32 = POOL_WIDTH*PIXEL_PER_METER + POOL_THICKNESS*PIXEL_PER_METER*2.0;
const WINDOW_DEPTH: f32 = POOL_DEPTH*PIXEL_PER_METER + POOL_THICKNESS*PIXEL_PER_METER*2.0;

mod balls;
use balls::PhysicsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        transparent: true,
                        title: "Test-3D".into(),
                        resolution: (WINDOW_WIDTH, WINDOW_DEPTH).into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            EnvironmentPlugin,
            PhysicsPlugin
        ))
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
