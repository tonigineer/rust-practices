use bevy::prelude::*;

use crate::environment::{POOL_DEPTH, POOL_WIDTH};

const PHYSICS_UPDATE_RATE: f32 = 50.0;

#[derive(Component)]
pub struct Ball {
    pub mass: f32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub ax: f32,
    pub ay: f32
}

impl Ball {
    // Physics: SI unit
    pub const MASS: f32 = 0.1;
    pub const RADIUS: f32 = 0.025;
    pub const VX_INIT: f32 = 1.0;
    pub const VY_INIT: f32 = 0.0;
    pub const AX_INIT: f32 = 0.0;
    pub const AY_INIT: f32 = 0.0;

    pub fn new(x: f32, y: f32) -> Self {
        Self {
            mass: Self::MASS,
            x,
            y,
            vx: Self::VX_INIT,
            vy: Self::VY_INIT,
            ax: Self::AX_INIT,
            ay: Self::AY_INIT,
        }
    }
}

#[derive(Resource)]
pub struct PhysicsTimer(Timer);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PhysicsTimer(Timer::from_seconds(1.0/PHYSICS_UPDATE_RATE, TimerMode::Repeating)))
            .add_systems(Update, update_physics);
    }
}


// // pub struct GravityPlugin;

// // pub struct CollisionPlugin;
// fn spawn_ball(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut number_of_balls: ResMut<NumberOfBalls>,
//     time: Res<Time>,
//     mut timer: ResMut<BallSpawnTimer>
// ) {
//     if timer.0.tick(time.delta()).just_finished() {
//         number_of_balls.0 += 1;

//         commands.spawn(
//             (
//                 MaterialMesh2dBundle {
//                     mesh: meshes.add(Circle::new(Ball::RADIUS * PIXEL_PER_METER)).into(),
//                     transform: Transform::from_xyz(Ball::X_INIT * PIXEL_PER_METER, Ball::Y_INIT * PIXEL_PER_METER, 0.0),
//                     material: materials.add(Color::RED),
//                     ..default()
//                 },
//                 Ball::new()
//             )
//         );

//         println!("Balls: {}  at  {}", number_of_balls.0, time.elapsed_seconds());
//     }
// }

fn update_physics(
    mut balls: Query<(Entity, &mut Transform, &mut Ball)>,
    time: Res<Time>,
    mut timer: ResMut<PhysicsTimer>
) {
    // for (i, ball_tf) in query_balls.iter().enumerate() {
    //     println!("{} {:?}", i, ball_tf);
    // }

    if timer.0.tick(time.delta()).just_finished() {
        // println!("Update physics: ");
        for (ball_entity, mut tf, mut ball) in &mut balls {
            println!("{} {} {}", ball.x, ball.y, ball.vx * time.delta_seconds());
    
            ball.x += ball.vx * time.delta_seconds();
            // println!("{:?} - {:?} - {}", ball_entity.index(), tf.translation, ball.x);
    
        }
    }
}