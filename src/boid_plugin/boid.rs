
use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Boid {
    pub position: Vec3,
    pub velocity: Vec3,
}

impl Boid {
    /* 
    pub fn new(entity: Entity) -> Boid {
        Boid {
            position: Vec3::new(0.0, 0.0, 0.0),
            velocity: Vec3::new(0.0, 0.0, 0.0),        
        }
    }
    */

    pub fn apply_rules(&mut self, boids: &Vec<Boid>, time: &Res<Time>) {
        let acceleration = self.calculate_coherence_acceleration(boids) 
            + self.calculate_seperation_acceleration(boids) 
            + self.calculate_alignment_acceleration(boids);

        self.velocity += acceleration * time.delta().as_secs_f32();        
        self.apply_speed_limit();
    }

    pub fn apply_speed_limit(&mut self) {
        let speed_limit = 0.3;
        if self.velocity.length() > speed_limit {
            self.velocity = self.velocity.normalize() * speed_limit;
        }
    }


    pub fn calculate_coherence_acceleration(&mut self, boids: &Vec<Boid>) -> Vec3 {
        let coherence_factor = 0.05;
        let mut center = Vec3::new(0.0, 0.0, 0.0);
        let mut neighbors = 0;

        let neighborhood_radius = 10.0;

        for boid in boids.iter() {
            if boid.position.distance(self.position) < neighborhood_radius {
                center += boid.position;
                neighbors += 1;
            }
        }

        if neighbors > 0 {
            center /= neighbors as f32;
            center -= self.position;
            center *= coherence_factor;
            return center;
        }
        return Vec3::new(0.0, 0.0, 0.0);
    }

    pub fn calculate_seperation_acceleration(&mut self, boids: &Vec<Boid>) -> Vec3 {
        let separation_factor = 1 as f32;
        let mut separation = Vec3::new(0.0, 0.0, 0.0);
        let desired_separation = 1 as f32;

        for boid in boids.iter() {
            let distance = boid.position.distance(self.position);
            if distance < desired_separation {
                separation += (self.position - boid.position) * (1.0 - (distance / desired_separation));
            }            
        }
        separation * separation_factor
    }

    pub fn calculate_alignment_acceleration(&mut self, boids: &Vec<Boid>) -> Vec3{
        let alignment_factor = 1 as f32;
        let mut alignment = Vec3::new(0.0, 0.0, 0.0);
        let neighborhood_radius = 3.0;
        let mut neighbors = 0;

        for boid in boids.iter() {
            if boid.position.distance(self.position) < neighborhood_radius {
                alignment += boid.velocity;
                neighbors += 1;
            }
        }

        if neighbors > 0 {
            alignment /= neighbors as f32;
            return (alignment - self.velocity) * alignment_factor;
        }   
        return Vec3::new(0.0, 0.0, 0.0);
        
    }
}