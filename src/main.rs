use bevy::prelude::*;

struct Velocity(f32);
struct Position(f32);

// this system spawns entities with the Position and Velocity components
fn setup(commands: &mut Commands) {
    commands
        .spawn((Position(0.0), Velocity(1.0)))
        .spawn((Position(1.0), Velocity(2.0)));
}

// this system runs on each entity with a Position and Velocity component
fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.0 += velocity.0;
        println!("{}", position.0);
    }
}

// the app entry point
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(movement.system())
        .run();
}
