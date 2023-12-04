use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use boid_plugin::BoidPlugin;

mod boid_plugin;



fn main() {
    App::new()
    
        .add_plugins((DefaultPlugins, BoidPlugin))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, initialize_scene)
        .add_systems(Update, ui_system)
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

fn ui_system(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("Sidepanel")
        .default_width(250.0)
        .show(ctx, |ui| {
            ui.heading("Boid Settings");
        });
}