use bevy::prelude::*;
use boid_plugin::BoidPlugin;

mod boid_plugin;

fn main() {
    App::new()
        .add_systems(Startup, initialize_scene)
        .add_plugins((DefaultPlugins, BoidPlugin))
        .run();
}

fn initialize_scene(mut commands: Commands){
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

