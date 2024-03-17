use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

mod renderer;
mod spectator;
mod world;

use renderer::RendererPlugin;
use spectator::SpectatorPlugin;
use world::{balls::BallPlugin, physics::PhysicsPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        transparent: true,
                        title: "Ball physics".into(),
                        resolution: (
                            renderer::WINDOW_WIDTH_INIT,
                            renderer::WINDOW_HEIGHT_INIT
                        ).into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            FrameTimeDiagnosticsPlugin,
            BallPlugin,
            PhysicsPlugin,
            RendererPlugin,
            SpectatorPlugin,
        ))
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
