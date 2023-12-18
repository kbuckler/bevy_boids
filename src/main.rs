use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_mod_picking::prelude::*;
use boid_plugin::BoidPlugin;
use boid_plugin::boid::BoidConfig;

mod boid_plugin;

fn main() {
    App::new()
    
        .add_plugins((            
            DefaultPlugins.set(low_latency_window_plugin()),          
            EguiPlugin,
            DefaultPickingPlugins,
            BoidPlugin))
        .insert_resource(BoidConfig::default())
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

fn ui_system(mut contexts: EguiContexts, mut config: ResMut<BoidConfig>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("Sidepanel")
        .default_width(250.0)
        .show(ctx, |ui| {
            ui.heading("Boid Settings");

            ui.separator();

            ui.add(egui::Slider::new(&mut config.separation_factor, 0.0..=200.0).text("Separation Factor"));

            ui.add(egui::Slider::new(&mut config.desired_separation, 0.0..=10.0).text("Desired Separation"));

            ui.add(egui::Slider::new(&mut config.alignment_factor, 0.0..=1.0).text("Alignment Factor"));
        });
}