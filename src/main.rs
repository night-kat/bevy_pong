use bevy::{color::palettes::tailwind::SKY_50, prelude::*};

const PADDLE_SIZE: Vec2 = Vec2::new(20.0, 220.0);
const CANVAS_SIZE: Vec2 = Vec2::new(1280., 720.0);
const PADDLE_SPEED: f32 = 400.0;
// Distance of the paddle from the wall
const PADDLE_PADDING: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (left_paddle_movement, right_paddle_movement))
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct RightPaddle;

#[derive(Component)]
struct LeftPaddle;

fn setup(mut commands: Commands) {
    // Left paddle
    commands.spawn((
        Sprite {
            custom_size: Some(PADDLE_SIZE),
            color: SKY_50.into(),
            ..default()
        },
        Transform::from_xyz(CANVAS_SIZE.x * (5. / 8.) - PADDLE_PADDING, 0.0, 0.0),
        Paddle,
        RightPaddle,
    ));

    // Right paddle
    commands.spawn((
        Sprite {
            custom_size: Some(PADDLE_SIZE),
            color: SKY_50.into(),
            ..default()
        },
        Transform::from_xyz(-CANVAS_SIZE.x * (5. / 8.) + PADDLE_PADDING, 0.0, 0.0),
        Paddle,
        LeftPaddle,
    ));

    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            // scaling_mode: ScalingMode::AutoMin
            //     min_width: CANVAS_SIZE.x,
            //     min_height: CANVAS_SIZE.y,
            // },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn left_paddle_movement(
    paddles: Query<&mut Transform, With<LeftPaddle>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in paddles {
        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += PADDLE_SPEED * time.delta_secs();
        };
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= PADDLE_SPEED * time.delta_secs();
        };
    }
}

fn right_paddle_movement(
    paddles: Query<&mut Transform, With<RightPaddle>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in paddles {
        if input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += PADDLE_SPEED * time.delta_secs();
        };
        if input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= PADDLE_SPEED * time.delta_secs();
        };
    }
}
