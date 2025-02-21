use bevy::prelude::*;

#[derive(Component)]
struct Foobar;

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
}

fn update_position(mut query: Query<(&Velocity, &mut Transform), With<Foobar>>, time: Res<Time>) {
    for (&Velocity(velocity), mut transform) in query.iter_mut() {
        transform.translation += (velocity * time.delta_secs()).extend(0.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, update_position)
        .run();
}
