use bevy::{prelude::*, render::mesh, utils::petgraph::visit::EdgeRef};
use bevy_mod_raycast::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_egui::EguiContexts;

pub mod boid;
use boid::{Boid, BoidConfig};

#[derive(Component)]
pub struct GroundPlane;

#[derive(Component)]
pub struct FlockTarget;

#[derive(Event)]
pub struct TargetSelectedEvent(Option<Vec3>);
impl From<ListenerInput<Pointer<Click>>> for TargetSelectedEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        TargetSelectedEvent(event.event.hit.position)
    }
}

pub struct BoidPlugin; 
impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultRaycastingPlugin)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .insert_resource(RapierConfiguration {
           //     gravity: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            })

            .add_systems(Startup, (initialize_flock, initialize_scene))
            //.add_systems(Update, debug_boids)
            //.add_systems(Update, mouse_input_system)
            .add_systems(Update, update_boid_targets)
            .add_systems(Update, update_flock)
            .add_systems(PreUpdate, target_select_system)
            .add_event::<TargetSelectedEvent>();
    }
}


fn target_select_system(
    mut events: EventReader<TargetSelectedEvent>,
    mut egui_contexts: EguiContexts,
    interaction_query: Query<&Interaction>,
    mut target_query: Query<(&mut Transform, With<FlockTarget>)>,
    mut boid_query: Query<&mut boid::Boid>,
) {

    let ctx = egui_contexts.ctx_mut();
    if ctx.wants_pointer_input() {
        //println!("egui wants pointer input");
        return;
    }

    let matches = interaction_query.iter().any(|i| matches!(i, Interaction::Hovered));
    if matches { 
        println!("hovered");
        return;
    }

    

    for event in events.iter() {

        if Some(event.0) == None {
            println!("target deselected");
            return;
        }

        println!("target selected {:?}", event.0);
        let mut target = target_query.iter_mut().next().unwrap();
        target.0.translation = event.0.unwrap();

        println!("mouse clicked {:?}", event.0.unwrap());


        // update all boid targets to the new target position
        for mut boid in boid_query.iter_mut() {
            boid.target_position = Some(event.0.unwrap());
            boid.boid_state = boid::BoidState::Moving;
        }

    }
}
/* 
fn debug_boids(boid_store: Res<BoidEntityStore> , mut gizmos: Gizmos) {
    for boid in boid_store.get_all() {
        gizmos.line(
            boid.position,
            boid.position + (20.0 * boid.velocity),
            Color::rgb(0.0, 1.0, 0.0),
        );
    }
}
*/

fn mouse_input_system(
    cursor_ray: Res<CursorRay>, 
    mut raycast: Raycast, 
   // mut gizmos: Gizmos, 
    ground_query: Query<(Entity, &GroundPlane)>,
    mut target_query: Query<(&mut Transform, With<FlockTarget>)>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut boid_query: Query<&mut boid::Boid>,
) {

    if mouse_button_input.just_pressed(MouseButton::Left) {   
        if let Some(cursor_ray) = **cursor_ray {
            let hits = raycast.cast_ray(cursor_ray, &RaycastSettings::default());
            if let Some((entity, intersection_data)) = hits.first() {
                if entity == &ground_query.iter().next().unwrap().0 {            
                    let mut target = target_query.iter_mut().next().unwrap();
                    target.0.translation = intersection_data.position();


                    println!("mouse clicked {:?}", intersection_data.position());


                    // update all boid targets to the new target position
                    for mut boid in boid_query.iter_mut() {
                        boid.target_position = Some(intersection_data.position());
                        boid.boid_state = boid::BoidState::Moving;
                    }

                }                
            }
        }
    }
}

fn initialize_scene(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_plane = shape::Plane { size: 100.0, subdivisions: 4 }.into();
   
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(ground_plane),
                material: materials.add(Color::WHITE.into()),
                ..default()
            }, 
            On::<Pointer<Click>>::send_event::<TargetSelectedEvent>()))        
        .insert(PickableBundle::default())
        .insert(Collider::cuboid(50.0, 0.01, 50.0))
        .insert(Friction { 
            coefficient: 0.1, 
            combine_rule: CoefficientCombineRule::Average 
        })
        
        .insert(GroundPlane);

    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 1.0, 0.0),
        perceptual_roughness: 0.6,
        metallic: 0.5,
        ..default()
    });

    let mesh =  meshes.add(Mesh::from(mesh::shape::Cube { 
        size: 0.1
    }));

    commands
        .spawn(
            PbrBundle {
                mesh: mesh.clone(),
                material: material.clone(),
                transform: Transform { 
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..Transform::default()
                },
                ..PbrBundle::default()
            })
        .insert(FlockTarget);

}

fn initialize_flock(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.5, 0.5, 1.0),
        perceptual_roughness: 0.6,
        metallic: 0.5,
        ..default()
    });

    let mesh =  meshes.add(Mesh::from(mesh::shape::Cube { 
        size: 0.2 
    }));


    for i in -5..5 {
        for j in -5..5 {         
            let position = Vec3::new(i as f32, 0.5, j as f32);                  
            let boid = Boid::new(position, material.clone());

            commands
                .spawn(boid)
                .insert(
                    PbrBundle {
                        mesh: mesh.clone(),
                        material: material.clone(),
                        transform: Transform { 
                            translation: position,
                            ..Transform::default()
                        },
                        ..PbrBundle::default()
                    })
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(0.1, 0.1, 0.1))                    
                .insert(Restitution::coefficient(0.1))
                .insert(ColliderMassProperties::Density(1.0))
                .insert(Velocity::default())
                .insert(ExternalForce {
                    force: Vec3::new(0., 0., 0.),                    
                    torque: Vec3::new(0., 0., 0.),     
                });

        }
    }
}

fn update_flock(
    mut query: Query<(&mut ExternalForce, &mut boid::Boid, &Transform)>,
    config: Res<BoidConfig>,
    time: Res<Time>,    
) {

    let boids = &mut query.iter_mut().map(|(_, mut boid, transform)| {
        boid.position = transform.translation;
        boid.clone()
    }).collect();

    for (mut impulse, mut boid, _) in query.iter_mut() {
        boid.apply_rules(boids, &time, &config);        
        
        impulse.force = boid.velocity;
    }
}

fn update_boid_targets(
    mut _query: Query<&mut boid::Boid>,
) {
    return; 
}

