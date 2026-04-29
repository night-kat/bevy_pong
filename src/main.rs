use bevy::{
    color::palettes::{
        css::WHITE,
        tailwind::{SKY_50, SKY_800},
    },
    prelude::*,
};

/// Width of the death zone
const DEATH_ZONE_WIDTH: f32 = 20.0;

/// Wall thickness
const WALL_THICKNESS: f32 = 4.0;

/// Size of the paddle
const PADDLE_SIZE: Vec2 = Vec2::new(20.0, 220.0);

/// Size of the canvas
const CANVAS_SIZE: Vec2 = Vec2::new(1280., 720.0);

/// Paddle movement speed
const PADDLE_SPEED: f32 = 400.0;

/// Distance of the paddle from the wall
const PADDLE_PADDING: f32 = 50.0;

const BALL_VELOCITY: Vec2 = Vec2::new(200., 400.);
const BALL_SIZE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (ball_movement, left_paddle_movement, right_paddle_movement),
        )
        .run();
}

#[derive(Component)]
struct Wall(Plane2d);

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct RightPaddle;

#[derive(Component)]
struct LeftPaddle;

#[derive(Component)]
struct Velocity(Vec2);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Left paddle
    commands.spawn((
        Sprite {
            custom_size: Some(PADDLE_SIZE),
            color: SKY_50.into(),
            ..default()
        },
        Transform::from_xyz(CANVAS_SIZE.x / 2.0 - PADDLE_PADDING, 0.0, 0.0),
        Paddle,
        RightPaddle,
    ));

    // upper wall
    commands.spawn((
        Wall(Plane2d::new(Vec2::X)),
        Transform::from_xyz(0., CANVAS_SIZE.y / 2., 0.),
        Mesh2d(meshes.add(Rectangle::new(CANVAS_SIZE.x, WALL_THICKNESS))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
    ));

    // lower wall
    commands.spawn((
        Wall(Plane2d::new(Vec2::X)),
        Transform::from_xyz(0., -CANVAS_SIZE.y / 2., 0.),
        Mesh2d(meshes.add(Rectangle::new(CANVAS_SIZE.x, WALL_THICKNESS))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
    ));

    // right wall
    commands.spawn((
        Wall(Plane2d::new(Vec2::X)),
        Transform::from_xyz(CANVAS_SIZE.x / 2., 0., 0.),
        Mesh2d(meshes.add(Rectangle::new(WALL_THICKNESS, CANVAS_SIZE.y))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
    ));

    // left wall
    commands.spawn((
        Wall(Plane2d::new(Vec2::X)),
        Transform::from_xyz(-CANVAS_SIZE.x / 2., 0., 0.),
        // This magic number makes the walls perfectly line up, no gaps, and no overshoot
        Mesh2d(meshes.add(Rectangle::new(WALL_THICKNESS, CANVAS_SIZE.y))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
    ));
    //
    // // right death zone
    // commands.spawn((
    //     Wall(Plane2d::new(Vec2::Y)),
    //     Transform::from_xyz(CANVAS_SIZE.x / 2., 0., 0.),
    //     Mesh2d(meshes.add(Rectangle::new(DEATH_ZONE_WIDTH, -CANVAS_SIZE.y))),
    //     // Kept this for debugging.
    //     // Uncomment to make the kill zone visible
    //     // MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
    // ));
    //
    // // left death zone
    // commands.spawn((
    //     Wall(Plane2d::new(Vec2::Y)),
    //     Transform::from_xyz(-(CANVAS_SIZE.x / 2.), 0., 0.),
    //     Mesh2d(meshes.add(Rectangle::new(DEATH_ZONE_WIDTH, CANVAS_SIZE.y))),
    //     // Kept this for debugging.
    //     // Uncomment to make the kill zone visible
    //     // MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
    // ));

    // Right paddle
    commands.spawn((
        Sprite {
            custom_size: Some(PADDLE_SIZE),
            color: SKY_50.into(),
            ..default()
        },
        Transform::from_xyz(-CANVAS_SIZE.x / 2.0 + PADDLE_PADDING, 0.0, 0.0),
        Paddle,
        LeftPaddle,
    ));

    // Create ball
    commands.spawn((
        Ball,
        Velocity(BALL_VELOCITY),
        Mesh2d(meshes.add(Circle::new(BALL_SIZE))),
        Transform::from_xyz(0., 0., 0.),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(WHITE))),
    ));

    // Spawn a camera
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::AutoMin {
                min_width: CANVAS_SIZE.x,
                min_height: CANVAS_SIZE.y,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // Spawn a black background to signify play area in case
    // the window size does not match canvas size
    commands.spawn((
        Sprite {
            custom_size: Some(CANVAS_SIZE),
            color: Color::BLACK,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -2.0),
    ));

    // Border rectangle
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(
                // 4px border
                CANVAS_SIZE.x + 4.,
                CANVAS_SIZE.y + 4.,
            )),
            color: Color::from(SKY_50),
            ..default()
        },
        Transform::from_xyz(0., 0., -3.0),
    ));
}

fn ball_movement(
    // it is not possible to have two queries on the same component
    // when one requests mutable access to it in the same system.
    // Without<Ball> means that we will not be using a transform
    // component of ball, since that would mean having a mutable reference
    // and a reference to transform at the same time, which is not allowed
    // by the borrow checker
    walls: Query<(&Wall, &mut Transform), Without<Ball>>,
    ball: Single<(&mut Transform, &mut Velocity), With<Ball>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = ball.into_inner();

    let ball_movement_this_frame = velocity.0 * time.delta_secs();
    let ball_move_distance = ball_movement_this_frame.length();
    transform.translation += (ball_movement_this_frame).extend(0.0);

    // a ray that casts infinitely in the direction
    // the ball is moving
    let ball_ray = Ray2d::new(
        // the location of the ball
        transform.translation.xy(),
        // the Direction the ball is moving in
        Dir2::new(velocity.0).unwrap(),
    );

    for (wall, origin) in walls {
        if let Some(hit_distance) = ball_ray.intersect_plane(origin.translation.xy(), wall.0)
            && hit_distance <= ball_move_distance
        {
            dbg!(hit_distance);

            velocity.0 = velocity.0.reflect(wall.0.normal.as_vec2());
            return;
        }
    }
}

fn move_paddle_helper(paddle: Mut<Transform>, move_by: f32) {
    static MOVE_LIMIT: f32 = CANVAS_SIZE.y / 2.0 - PADDLE_SIZE.y / 2.0;

    let moved = paddle.translation.y + move_by;
    let clamped = moved.clamp(-MOVE_LIMIT, MOVE_LIMIT);

    paddle
        .map_unchanged(|x| &mut x.translation.y)
        .set_if_neq(clamped);
}

fn left_paddle_movement(
    paddles: Query<&mut Transform, With<LeftPaddle>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let paddle_move_by = PADDLE_SPEED * time.delta_secs();

    for transform in paddles {
        if input.pressed(KeyCode::KeyW) {
            move_paddle_helper(transform, paddle_move_by);
        } else if input.pressed(KeyCode::KeyS) {
            move_paddle_helper(transform, -paddle_move_by);
        };
    }
}

fn right_paddle_movement(
    paddles: Query<&mut Transform, With<RightPaddle>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let paddle_move_by = PADDLE_SPEED * time.delta_secs();

    for transform in paddles {
        if input.pressed(KeyCode::ArrowUp) {
            move_paddle_helper(transform, paddle_move_by);
        } else if input.pressed(KeyCode::ArrowDown) {
            move_paddle_helper(transform, -paddle_move_by);
        };
    }
}
