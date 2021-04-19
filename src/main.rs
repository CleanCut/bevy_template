use bevy::prelude::*;

struct Velocity(f32);
struct Position(f32);
#[derive(Debug)]
enum Choice {
    Yes,
    No,
}

// this system spawns entities with the Position and Velocity components
fn setup(commands: &mut Commands) {
    commands
        .spawn((Position(0.0), Velocity(1.0), Choice::Yes))
        .spawn((Position(1.0), Velocity(2.0), Choice::No));
}

// this system runs on each entity with a Position and Velocity component
fn movement(mut query: Query<(&mut Position, &Velocity, &Choice)>) {
    for (mut position, velocity, choice) in query.iter_mut() {
        position.0 += velocity.0;
        println!("{:?} {}", choice, position.0);
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
