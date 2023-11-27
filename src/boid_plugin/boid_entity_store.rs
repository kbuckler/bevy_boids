use bevy::{prelude::*, utils::HashMap};
use crate::boid_plugin::boid::Boid;

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

    /*
    pub fn get(&self, entity: Entity) -> Boid {
        return self.entities.get(&entity).unwrap().clone();
    }
    */

    pub fn get_all(&self) -> Vec<Boid> {
        let mut boids = Vec::new();
        for (_, boid) in self.entities.iter() {
            boids.push(boid.clone());
        }
        return boids;
    }

}