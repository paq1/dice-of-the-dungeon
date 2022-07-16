use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(
            WindowDescriptor {
                title: "dice of the dungeon -- gamejam".to_string(),
                width: 600.,
                height: 600.,
                ..Default::default()
            }
        )
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup_system) // todo add startup system
        .run();
}

fn startup_system(
    mut commands: Commands
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
