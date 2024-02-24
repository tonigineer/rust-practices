use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::balls::Ball;

pub const PIXEL_PER_METER: f32 = 500.0;

pub const POOL_DEPTH: f32 = 1.0;
pub const POOL_WIDTH: f32 = 3.0;
pub const POOL_THICKNESS: f32 = 0.02;

const WINDOW_WIDTH: f32 = POOL_WIDTH*PIXEL_PER_METER;
const WINDOW_HEIGHT: f32 = POOL_DEPTH*PIXEL_PER_METER;

const BALL_SPAWN_RATE: f32 = 10.0;  // hz
const MAX_BALLS: u32 = 1;

// From top left corner
const BALL_SPAWN_X: f32 = POOL_WIDTH*0.05;
const BALL_SPAWN_Y: f32 = POOL_DEPTH*0.10;


#[derive(Resource)]
pub struct NumSpawnedBalls(pub u32);

#[derive(Resource)]
struct BallSpawnTimer(Timer);


pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_pool)
            .insert_resource(NumSpawnedBalls(0))
            .insert_resource(BallSpawnTimer(Timer::from_seconds(1.0/BALL_SPAWN_RATE, TimerMode::Repeating)))
            .add_systems(Update, spawn_ball)
            .add_systems(Update, update_balls);
    }
}

fn update_balls(
    mut balls: Query<(Entity, &mut Transform, &Ball)>,
) {

        println!("Update screen: ");
        for (ball_entity, mut tf, ball) in &mut balls {
            // println!("{} {}", ball.x, ball.y);

            let (x, y) = coord2pixel(ball.x, ball.y);
            tf.translation.x = x;
            tf.translation.y = y;
            // println!("{:?} - {:?} - {}", ball_entity.index(), tf.translation, ball.x);

        }
}

fn coord2pixel(x: f32, y: f32) -> (f32, f32) {
    const WINDOW_WIDTH_HALF: f32 = WINDOW_WIDTH/2.0;
    const WINDOW_HEIGHT_HALF: f32 = WINDOW_HEIGHT/2.0;

    (-WINDOW_WIDTH_HALF + x*PIXEL_PER_METER, WINDOW_HEIGHT_HALF - y*PIXEL_PER_METER)
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut num_spawned_balls: ResMut<NumSpawnedBalls>,
    mut timer: ResMut<BallSpawnTimer>,
    time: Res<Time>
) {
    if timer.0.tick(time.delta()).just_finished() && num_spawned_balls.0 < MAX_BALLS{
        num_spawned_balls.0 += 1;
        let (x, y) = coord2pixel(BALL_SPAWN_X, BALL_SPAWN_Y);

        commands.spawn(
            (
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(Ball::RADIUS * PIXEL_PER_METER)).into(),
                    transform: Transform::from_xyz(
                        x,
                        y,
                        0.0
                    ),
                    material: materials.add(Color::RED),
                    ..default()
                },
                Ball::new(BALL_SPAWN_X, BALL_SPAWN_Y)
            )
        );

        println!("Balls: {}  at  {}", num_spawned_balls.0, time.elapsed_seconds());
    }
}

fn spawn_pool(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const POOL_COLOR: Color = Color::Rgba{
        red: (233.0/255.0),
        green: (48.0/255.0),
        blue: (106.0/255.0),
        alpha: (0.1)
    };

    // Left side
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::from_size(Vec2::from_array([POOL_THICKNESS*PIXEL_PER_METER, (POOL_DEPTH+POOL_THICKNESS*2.0)*PIXEL_PER_METER]))).into(),
        // transform: Transform::default().with_scale(Vec3::splat(128.)),
        transform: Transform::from_xyz(-(POOL_WIDTH*PIXEL_PER_METER)/2.0 - (POOL_THICKNESS*PIXEL_PER_METER)/2.0, 0.0, 0.0),
        material: materials.add(POOL_COLOR),
        ..default()
    });

    // Right side
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::from_size(Vec2::from_array([POOL_THICKNESS*PIXEL_PER_METER, (POOL_DEPTH+POOL_THICKNESS*2.0)*PIXEL_PER_METER]))).into(),
        // transform: Transform::default().with_scale(Vec3::splat(128.)),
        transform: Transform::from_xyz((POOL_WIDTH*PIXEL_PER_METER)/2.0 + (POOL_THICKNESS*PIXEL_PER_METER)/2.0, 0.0, 0.0),
        material: materials.add(POOL_COLOR),
        ..default()
    });

    // Bottom
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::from_size(Vec2::from_array([POOL_WIDTH*PIXEL_PER_METER, POOL_THICKNESS*PIXEL_PER_METER]))).into(),
        // transform: Transform::default().with_scale(Vec3::splat(128.)),
        transform: Transform::from_xyz(0.0, -(POOL_THICKNESS*PIXEL_PER_METER)/2.0 - (POOL_DEPTH*PIXEL_PER_METER)/2.0, 0.0),
        material: materials.add(POOL_COLOR),
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}