use bevy::{prelude::*, render::mesh};
use bevy_mod_raycast::prelude::*;
use bevy_rapier3d::prelude::*;

pub mod boid;
use boid::Boid;

#[derive(Component)]
pub struct GroundPlane;

#[derive(Component)]
pub struct FlockTarget;

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
            .add_systems(Update, mouse_input)
            .add_systems(Update, update_flock);
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

fn mouse_input(
    cursor_ray: Res<CursorRay>, 
    mut raycast: Raycast, 
   // mut gizmos: Gizmos, 
    ground_query: Query<(Entity, &GroundPlane)>,
    mut target_query: Query<(&mut Transform, With<FlockTarget>)>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if let Some(cursor_ray) = **cursor_ray {
        if mouse_button_input.pressed(MouseButton::Left) {      
            let hits = raycast.cast_ray(cursor_ray, &RaycastSettings::default());
            if let Some((entity, intersection_data)) = hits.first() {
                if entity == &ground_query.iter().next().unwrap().0 {            
                    let mut target = target_query.iter_mut().next().unwrap();
                    target.0.translation = intersection_data.position();
                    /* 
                    gizmos.line(
                        intersection_data.position(),
                        intersection_data.position() + (20.0 * intersection_data.normal()),
                        Color::rgb(1.0, 0.0, 0.0),
                    );
                    */
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
        .spawn(PbrBundle {
            mesh: meshes.add(ground_plane),
            material: materials.add(Color::WHITE.into()),
            ..default()
        })
        .insert(Collider::cuboid(50.0, 0.01, 50.0))
        .insert(Friction { 
            coefficient: 0.01, 
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
                  
            let entity = commands
                .spawn(
                    PbrBundle {
                        mesh: mesh.clone(),
                        material: material.clone(),
                        transform: Transform { 
                            translation: position,
                            ..Transform::default()
                        },
                        ..PbrBundle::default()
                    })
                .insert(Boid {
                    position: position,
                    velocity: Vec3::new(0.0, 0.0, 0.0)
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(0.1, 0.1, 0.1))                    
                .insert(Restitution::coefficient(0.5))
                .insert(ColliderMassProperties::Density(1.0))
                .insert(Velocity::default())
                .insert(ExternalForce {
                    force: Vec3::new(0., 0., 0.),                    
                    torque: Vec3::new(0., 0., 0.),     
                }) 
                .id();
        }
    }
}

fn update_flock(
    mut query: Query<(&mut ExternalForce, &mut boid::Boid, &Transform)>,
    mut target_query: Query<(&Transform, With<FlockTarget>, Without<boid::Boid>)>,
    time: Res<Time>,    
) {

    let boids = &mut query.iter_mut().map(|(_, mut boid, transform)| {
        boid.position = transform.translation;
        boid.clone()
    }).collect();

    let target = target_query.iter_mut().next().unwrap().0.translation;
    for (mut impulse, mut boid, transform) in query.iter_mut() {
        boid.apply_rules(boids, &target, &time);        
        impulse.force = 0.1 * boid.velocity;
    }
}

