
use bevy::prelude::*;
// use log::info;

#[derive(Component, Debug, Clone)]
pub struct Boid {
    pub position: Vec3,
    pub velocity: Vec3,
    pub target_position: Option<Vec3>,
    pub material: Handle<StandardMaterial>
}

impl Boid {
    pub fn new(position: Vec3, material: Handle<StandardMaterial>) -> Boid {
        Boid {
            position: position,
            velocity: Vec3::new(0.0, 0.0, 0.0),
            target_position: Some(Vec3::new(0.0, 0.0, 0.0)),      
            material: material
        }
    }
    pub fn reached_target(&self) -> bool {
        match self.target_position {
            Some(p) => self.position.distance(p) < 0.5,
            None => return true,
        }
    }

    pub fn apply_rules(&mut self, other_boids: &Vec<Boid>, time: &Res<Time>) {

        if self.reached_target() {
            self.velocity = Vec3::new(0.0, 0.0, 0.0);
            return;
        }
        
        let neighborhood_radius = 1.0;
        let neighboring_boids = other_boids.iter()
            .filter(|boid| boid.position.distance(self.position) < neighborhood_radius)
            .collect::<Vec<&Boid>>();


        neighboring_boids.iter().for_each(|boid| {
            if boid.reached_target() {
                self.target_position = None;
            }
        });    

        let acceleration = Vec3::new(0.0, 0.0, 0.0) 
           //+ self.calculate_coherence_acceleration(&neighboring_boids) 
            + self.calculate_seperation_acceleration(&neighboring_boids) 
           // + self.calculate_alignment_acceleration(&neighboring_boids)
            + self.calculate_target_acceleration()
            + Vec3::new(0.0, 0.0, 0.0);
                 
        self.velocity = acceleration * time.delta().as_secs_f32();        
        self.apply_speed_limit();
    }

    pub fn apply_speed_limit(&mut self) {
        let speed_limit = 0.075;
        if self.velocity.length() > speed_limit {
            self.velocity = self.velocity.normalize() * speed_limit;
        }
    }

    pub fn calculate_target_acceleration(&mut self) -> Vec3 {
        match self.target_position {
            Some(p) => 2. * (p - self.position).normalize(),
            None => Vec3::new(0.0, 0.0, 0.0),
        }   
    }

    pub fn calculate_coherence_acceleration(&mut self, boids: &Vec<&Boid>) -> Vec3 {
        let coherence_factor = 1 as f32;
        let mut center = Vec3::new(0.0, 0.0, 0.0);
        let mut neighbors = 0;

        let neighborhood_radius = 2.0;

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

    pub fn calculate_seperation_acceleration(&mut self, boids: &Vec<&Boid>) -> Vec3 {
        let separation_factor = 100 as f32;
        let mut separation = Vec3::new(0.0, 0.0, 0.0);
        let desired_separation = 0.75 as f32;

        for boid in boids.iter() {            
            let distance = boid.position.distance(self.position);    

            if distance < desired_separation {                
                let vector = self.position - boid.position;
                separation += Vec3::lerp(
                    Vec3::new(0., 0., 0.), 
                    vector, 
                    (desired_separation - distance) / desired_separation);
            }            
        }
        separation * separation_factor
    }

    pub fn calculate_alignment_acceleration(&mut self, boids: &Vec<&Boid>) -> Vec3{
        let alignment_factor = 0.1 as f32;
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