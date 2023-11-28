
use bevy::prelude::*;
use log::info;

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

    pub fn apply_rules(&mut self, other_boids: &Vec<Boid>, target: &Vec3, time: &Res<Time>) {
        let neighborhood_radius = 10.0;
        let neighboring_boids = other_boids.iter()
            .filter(|boid| boid.position.distance(self.position) < neighborhood_radius)
            .collect::<Vec<&Boid>>();

        let mut acceleration = Vec3::new(0.0, 0.0, 0.0) 
            + self.calculate_coherence_acceleration(&neighboring_boids) 
            + self.calculate_seperation_acceleration(&neighboring_boids) 
            + self.calculate_alignment_acceleration(&neighboring_boids)
            + self.calculate_target_acceleration(target)
            + Vec3::new(0.0, 0.0, 0.0);
        

        acceleration.y = 0.0;        
        
        self.velocity = acceleration * time.delta().as_secs_f32();        
        //self.apply_speed_limit();
    }

    pub fn apply_speed_limit(&mut self) {
        let speed_limit = 2.0;
        if self.velocity.length() > speed_limit {
            self.velocity = self.velocity.normalize() * speed_limit;
        }
    }

    pub fn calculate_target_acceleration(&mut self, target: &Vec3) -> Vec3 {
        let vector = *target - self.position;        
        vector.normalize() * vector.length()
    }

    /// Calculates the coherence acceleration for the current boid based on its neighbors.
    /// 
    /// # Arguments
    /// 
    /// * `boids` - A vector of references to neighboring boids.
    /// 
    /// # Returns
    /// 
    /// The coherence acceleration as a `Vec3` (3D vector).
    pub fn calculate_coherence_acceleration(&mut self, boids: &Vec<&Boid>) -> Vec3 {
        let coherence_factor = 1 as f32;
        let mut center = Vec3::new(0.0, 0.0, 0.0);
        let mut neighbors = 0;

        let neighborhood_radius = 5.0;

        for boid in boids.iter() {
            if boid.position.distance(self.position) < neighborhood_radius {
                center += boid.velocity;
                neighbors += 1;
            }
           // if neighbors > 10 { break };
        }

        if neighbors > 0 {
            center /= neighbors as f32;
            center -= self.velocity;
            center *= coherence_factor;
            return center;
        }

        return Vec3::new(0.0, 0.0, 0.0);
    }

    /// Calculates the separation acceleration for the current boid based on the positions of other boids.
    /// 
    /// # Arguments
    /// 
    /// * `boids` - A vector of references to other boids.
    /// 
    /// # Returns
    /// 
    /// The separation acceleration as a `Vec3` (3D vector).
    pub fn calculate_seperation_acceleration(&mut self, boids: &Vec<&Boid>) -> Vec3 {
        let separation_factor = 100 as f32;
        let mut separation = Vec3::new(0.0, 0.0, 0.0);
        let desired_separation = 0.75 as f32;

        for boid in boids.iter() {            
            let distance = boid.position.distance(self.position);        
            if distance < desired_separation {                
                let vector = self.position - boid.position;
    
                separation += vector;
            }            
        }
        separation * separation_factor
    }


    pub fn calculate_alignment_acceleration(&mut self, boids: &Vec<&Boid>) -> Vec3{
        let alignment_factor = 1 as f32;
        let mut alignment = Vec3::new(0.0, 0.0, 0.0);
        let neighborhood_radius = 1.5;
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