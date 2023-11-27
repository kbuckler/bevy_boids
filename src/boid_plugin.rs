use bevy::{prelude::*, render::mesh, utils::HashMap};
pub mod boid;
use boid::Boid;
use log::info;

#[derive(Resource)]
pub struct BoidEntityStore {
    entities: HashMap<Entity, Boid>
}

impl BoidEntityStore {
    pub fn new() -> BoidEntityStore {
        BoidEntityStore {
            entities: HashMap::new()
        }
    }

    pub fn add(&mut self, entity: Entity, boid: Boid) {
        self.entities.insert(entity, boid);
    }

    pub fn get(&self, entity: Entity) -> Boid {
        return self.entities.get(&entity).unwrap().clone();
    }

    pub fn get_all(&self) -> Vec<Boid> {
        let mut boids = Vec::new();
        for (_, boid) in self.entities.iter() {
            boids.push(boid.clone());
        }
        return boids;
    }

}
pub struct BoidPlugin; 
impl Plugin for BoidPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BoidEntityStore::new())
            .add_systems(Startup, initialize_flock)
            .add_systems(Update, update_flock);
    }
}

fn initialize_flock(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut boid_store: ResMut<BoidEntityStore>
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.5, 0.5, 1.0),
        perceptual_roughness: 0.6,
        metallic: 0.5,
        ..default()
    });

    let mesh =  meshes.add(Mesh::from(mesh::shape::Cube { 
        size: 0.1 
    }));

    for i in -3..3 {
        for j in -3..3 {
            let boid = Boid {
                position: Vec3::new(i as f32, 0.0, j as f32),
                velocity: Vec3::new(0.0, 0.0, 0.0),            
            };
            let entity = commands
                .spawn(
                    PbrBundle {
                        mesh: mesh.clone(),
                        material: material.clone(),
                        transform: Transform { 
                            translation: boid.position,
                            ..Transform::default()
                        },
                        ..PbrBundle::default()
                    })
                .insert(boid)
                .id();

            boid_store.add(entity, boid);
        }
    }
}

fn update_flock(mut query: Query<(Entity, &mut Transform, &mut boid::Boid)>, mut boid_store: ResMut<BoidEntityStore>, time: Res<Time>) {
    let boids = boid_store.get_all();
    //info!("boids: {:?}", boids);
    for (entity, mut transform, mut boid) in query.iter_mut() {
        boid.apply_rules(&boids, &time);

        transform.translation.x += boid.velocity.x;
        transform.translation.y += boid.velocity.y;
        transform.translation.z += boid.velocity.z;

        boid.position = transform.translation;
        boid_store.add(entity, boid.clone());

    }

}