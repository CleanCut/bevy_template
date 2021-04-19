use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(template_setup.system())
        .add_system(template_animation.system())
        .run();
}

fn template_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "Would you like to play a game?",
            TextStyle {
                font: asset_server.load("fonts/tiny.ttf"),
                font_size: 58.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    });
}

fn template_animation(time: Res<Time>, mut query: Query<&mut Transform, With<Text>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x = 100.0 * time.seconds_since_startup().sin() as f32;
        transform.translation.y = 100.0 * time.seconds_since_startup().cos() as f32;
    }
}
