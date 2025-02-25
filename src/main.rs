use bevy::prelude::*;
#[derive(Component)]
struct Foobar;

#[derive(Component)]
struct Barfoo;

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component)]
struct Speed(f32);

fn spawn_barfoo(mut commands: Commands, coords: Vec2) {
    commands.spawn((
        Barfoo,
        Sprite::from_color(Color::srgb(0.0, 1.0, 0.0), Vec2::new(50.0, 50.0)),
        Transform {
            translation: coords.extend(-1.0),
            ..Default::default()
        },
    ));
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Foobar,
        Direction(Vec2::ZERO),
        Speed(1.5),
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::new(50.0, 50.0)),
        Transform {
            ..Default::default()
        },
    ));
}

fn update_position(mut query: Query<(&Direction, &Speed, &mut Transform)>) {
    for (&Direction(direction), &Speed(speed), mut transform) in query.iter_mut() {
        transform.translation += match direction.length() {
            0.0 => Vec3::ZERO,
            _ => (direction.normalize() * speed).extend(0.0),
        };
    }
}

fn update_direction(
    mut query_foobar: Query<(&mut Direction, &Transform), With<Foobar>>,
    query_barfoo: Query<&Transform, With<Barfoo>>,
) {
    if let Some(barfoo_transform) = query_barfoo.iter().next() {
        for (mut direction, foobar_transform) in query_foobar.iter_mut() {
            direction.0 = (barfoo_transform.translation - foobar_transform.translation).truncate();
        }
    }
}

fn spawn_barfoo_on_click(
    commands: Commands,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            spawn_barfoo(commands, world_position);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (update_position, update_direction))
        .add_systems(FixedUpdate, spawn_barfoo_on_click)
        .run();
}
