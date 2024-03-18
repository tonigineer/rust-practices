use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use super::components::{MainCamera, TextHud};
use crate::world::balls::resources::NumberSpawnedBalls;
use crate::world::physics::resources::PhysicsInfo;

pub(crate) fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera
    ));
}

pub(crate) fn show_text(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(1.0),
            left: Val::Px(5.0),
            ..default()
        }),
        TextHud,
    ));
}

pub(crate) fn update_hud(
    mut query_text: Query<&mut Text, With<TextHud>>,
    diagnostics: Res<DiagnosticsStore>,
    number_spawned_balls: Res<NumberSpawnedBalls>,
    physics_info: Res<PhysicsInfo>
) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(text_fps) = fps.smoothed() {
            let mut text = query_text.single_mut();
            let text_num_balls = number_spawned_balls.0;

            text.sections[0].value = format!(
                "FPS: {:.2}\nBalls: {}\nCol-Iter: {}\nPhysic-Time: {:.0}ms",
                text_fps,
                text_num_balls,
                physics_info.iterations_collisions,
                physics_info.execution_time
            );
        }

    }


    // for mut text in &mut query_fps {
    //     if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
    //         if let Some(value) = fps.smoothed() {
    //             info!("{:?}", text);
    //             // Update the value of the second section
    //             text.sections[0].value = format!(
    //                 "FPS: {value:.2}\nBalls: {value}"
    //             );
    //         }
    //     }
    // }

    // for mut text in &mut query_balls {
    //     text.sections[0].value = format!("Balls: {}", number_spawned_balls.0);
    // }
}

// use bevy::{
//     prelude::*,
//     window::CursorGrabMode,
//     sprite::{MaterialMesh2dBundle, Mesh2dHandle}
// };
// use crate::{renderer::resources::PixelPerMeter, world::balls::components::Ball};
// use super::resources::GrabbedBall;

// use super::components::Cue;

// pub(crate) fn grab_ball(
//     mut windows: Query<&mut Window>,
//     camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
//     mouse: Res<ButtonInput<MouseButton>>,
//     key: Res<ButtonInput<KeyCode>>,
//     mut balls: Query<(Entity, &Transform, &mut Ball)>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     pixel_per_meter: Res<PixelPerMeter>,
//     grabbed_ball: Option<Res<GrabbedBall>>,
//     mut query: Query<(Entity, &Cue, &Transform)>
// ) {
//     let mut window = windows.single_mut();
//     let (camera, camera_transform) = camera.single();

//     let mut test = 1;

//     if mouse.just_pressed(MouseButton::Left) {
//         if let Some(mouse_pos) = window
//             .cursor_position()
//             .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)
//         )
//         {
//             for (e, tf, ball) in &balls {

//                 // println!("{:?}", tf.translation.xy() - mouse_pos);
//                 if (tf.translation.xy() - mouse_pos).length() <= ball.radius * pixel_per_meter.0 {
//                     window.cursor.visible = true;
//                     window.cursor.grab_mode = CursorGrabMode::Locked;
//                     println!("grabbed");
//                     println!("{:?} {:?}", tf.translation.xy(), mouse_pos);
//                     // info!("{:?}", grabbed_ball);
//                     // grabbed_ball.;

//                     commands.insert_resource(GrabbedBall { id: e.index() });

//                     // let shape = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0)));

//                     // commands.spawn((
//                     //     MaterialMesh2dBundle {
//                     //         mesh: shape,
//                     //         material: materials.add(Color::RED),
//                     //         transform: Transform {
//                     //             translation: Vec3::new(0.0, 0.0, 0.0),
//                     //             rotation: Quat::from_rotation_z(2f32),
//                     //             scale: Vec3::ONE
//                     //         },
//                     //         ..default()
//                     //     },
//                     //     Cue
//                     // ));
//                 }
//             }
//         }
//     }

//     if mouse.just_released(MouseButton::Left) {
//         window.cursor.visible = true;
//         window.cursor.grab_mode = CursorGrabMode::None;




//         for (entity, _, mut ball) in &mut balls {
//             if let Some(gb) = &grabbed_ball {
//                 info!("{}", gb.id);
//                 if let Some(mouse_pos) = window
//                     .cursor_position()
//                     .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)
//                 ) {
//                     if entity.index() == gb.id {
//                         // commands.entity(entity).despawn();
//                         ball.velocity = 0.01 * (ball.position - Vec2 {x: mouse_pos.x, y: mouse_pos.y });
//                     }
//                 }
//             }
//         }

//         commands.remove_resource::<GrabbedBall>();
//     }

//     // let shape = shapes::RegularPolygon {
//     //     sides: 6,
//     //     feature: shapes::RegularPolygonFeature::Radius(200.0),
//     //     ..shapes::RegularPolygon::default()
//     // };
    
//     // commands.spawn(Camera2dBundle::default());
//     // commands.spawn((
//     //     ShapeBundle {
//     //         path: GeometryBuilder::build_as(&shape),
//     //         ..default()
//     //     },
//     //     Fill::color(Color::CYAN),
//     //     Stroke::new(Color::BLACK, 10.0),
//     // ));

//     if window.cursor.grab_mode == CursorGrabMode::Locked {
//         if let Some(gb) = grabbed_ball {
//             info!("{}", gb.id);

//             // use single!            
//             // for (entity, mut tf, _) in &mut query {
//             //     tf.rotation = Quat::from_rotation_z(4.0);
//             // }
//             // if let Some(mouse_pos) = window
//             // .cursor_position()
//             // .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)
//             // )
//             // {
//             //     commands.spawn(MaterialMesh2dBundle {
//             //         mesh: shape,
//             //         material: materials.add(Color::RED),
//             //         transform: Transform::from_xyz(
//             //             // Distribute shapes from -X_EXTENT to +X_EXTENT.
//             //             mouse_pos.x,
//             //             mouse_pos.y,
//             //             0.0,
//             //         ),
//             //         ..default()
//             //     });
//             // }

//             // commands.spawn(MaterialMesh2dBundle {
//             //     mesh: shape,
//             //     material: materials.add(Color::RED),
//             //     transform: Transform::from_xyz(
//             //         // Distribute shapes from -X_EXTENT to +X_EXTENT.
//             //         0.0,
//             //         0.0,
//             //         0.0,
//             //     ),
//             //     ..default()
//             // });
//         }
        
//     // if mouse.pressed(MouseButton::Left) {
//         // info!("left mouse currently pressed");
//         // println!("{:?} {:?}", tf.translation.xy(), mouse_pos);
//         // println!("{:?}", window.cursor_position());
//         // commands.spawn(MaterialMesh2dBundle {
//         //     mesh: meshes.add(Rectangle::from_size(Vec2::from_array([222.0, 333.0]))).into(),
//         //     // transform: Transform::default().with_scale(Vec3::splat(128.)),
//         //     transform: Transform::from_xyz(-0.0, 0.0, 0.0),
//         //     material: materials.add(Color::RED),
//         //     ..default()
//         // });

//     }
// }