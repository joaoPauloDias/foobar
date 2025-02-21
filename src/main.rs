use bevy::prelude::*;
use rand::{Rng, rng};

#[derive(Component)]
struct Foobar;

#[derive(Component)]
struct Barfoo;

#[derive(Component)]
struct Velocity(Vec2);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Foobar,
        Velocity(Vec2::new(100.0, 0.0)),
        Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::new(50.0, 50.0)),
        Transform {
            ..Default::default()
        },
    ));

    commands.spawn((
        Barfoo,
        Sprite::from_color(Color::srgb(0.0, 1.0, 0.0), Vec2::new(50.0, 50.0)),
        Transform {
            translation: Vec3::new(
                rng().random_range(-500.0..500.0),
                rng().random_range(-500.0..500.0),
                1.0,
            ),
            ..Default::default()
        },
    ));
}

fn update_position(mut query: Query<(&Velocity, &mut Transform), With<Foobar>>, time: Res<Time>) {
    for (&Velocity(velocity), mut transform) in query.iter_mut() {
        transform.translation += (velocity * time.delta_secs()).extend(0.0);
    }
}

fn update_velocity(
    mut query_foobar: Query<(&mut Velocity, &Transform), With<Foobar>>,
    query_barfoo: Query<&Transform, With<Barfoo>>,
) {
    if let Ok(barfoo_transform) = query_barfoo.get_single() {
        for (mut velocity, foobar_transform) in query_foobar.iter_mut() {
            let direction =
                (barfoo_transform.translation - foobar_transform.translation).truncate();
            if direction.length() != 0.0 {
                let normalized_direction = direction.normalize();
                let speed = velocity.0.length();
                velocity.0 = normalized_direction * speed;
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (update_position, update_velocity))
        .run();
}
