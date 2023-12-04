
use bevy::prelude::*;
// use log::info;


#[derive(Debug, Clone)]
pub enum BoidState {
    Idle,
    Moving,
}

impl BoidState{

}

impl Default for BoidState {
    fn default() -> Self {
        BoidState::Idle
    }
}
#[derive(Component, Debug, Clone)]
pub struct Boid {
    pub boid_state: BoidState,
    pub position: Vec3,
    pub velocity: Vec3,
    pub target_position: Option<Vec3>,
    pub material: Handle<StandardMaterial>

}



impl Boid {
    pub fn new(position: Vec3, material: Handle<StandardMaterial>) -> Boid {
        Boid {
            boid_state: BoidState::Idle,
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
        match self.boid_state {
            BoidState::Idle => { 

            },
            BoidState::Moving => {
                self.apply_moving_rules(other_boids, time);
            }
        }
    }

    fn apply_moving_rules(&mut self, other_boids: &Vec<Boid>, time: &Res<Time>) {
        let neighborhood_radius = 100.0;
        let neighboring_boids = other_boids.iter()
            .filter(|boid| boid.position.distance(self.position) < neighborhood_radius)
            .collect::<Vec<&Boid>>();
/*
        neighboring_boids.iter().for_each(|boid| {
            match boid.boid_state {
                BoidState::Idle => {
                    match self.target_position {
                        Some(p) => {
                            if self.position.distance(p) < 0.5 {
                                self.target_position = None;
                                self.boid_state = BoidState::Idle;
                            }
                        },
                        None => { },
                    }
                },
                BoidState::Moving => { },
            }
        });    
*/

        // The coherence factor determines the strength of the coherence behavior for the boid.
        // A higher coherence factor will result in stronger alignment behavior.
        let coherence_factor = 25 as f32;

        // The separation factor determines the strength of separation behavior for the boid.
        // A higher separation factor will result in boids maintaining a greater distance from each other.
        let separation_factor = 100 as f32;

        // The factor used to determine the influence of the target on the boid's behavior.
        // A higher value will result in a stronger attraction towards the target.
        let target_factor = 25 as f32;

        // The alignment factor determines how strongly a boid aligns its velocity with its neighbors.
        // It is a value between 0 and 1, where 0 means no alignment and 1 means full alignment.
        let alignment_factor = 0.9 as f32;

        let acceleration = Vec3::new(0.0, 0.0, 0.0) 
            + (coherence_factor * self.calculate_cohesion_force(&neighboring_boids))
            + (separation_factor * self.calculate_seperation_acceleration(&neighboring_boids))
            + (target_factor * self.calculate_target_acceleration())
            + Vec3::new(0.0, 0.0, 0.0);
                 
        let initial_velocity = acceleration * time.delta().as_secs_f32();  
        let alignment_velocity = self.calculate_alignment_velocity(&neighboring_boids);

        self.velocity =  Vec3::lerp(initial_velocity, alignment_velocity, alignment_factor);     
        self.apply_speed_limit();
    }

    pub fn apply_speed_limit(&mut self) {
        let speed_limit = 0.075;
        if self.velocity.length() > speed_limit {
            self.velocity = self.velocity.normalize() * speed_limit;
        }
    }

    fn calculate_target_acceleration(&mut self) -> Vec3 {
        match self.target_position {
            Some(p) => (p - self.position).normalize(),
            None => Vec3::new(0.0, 0.0, 0.0),
        }   
    }

    fn calculate_cohesion_force(&mut self, boids: &Vec<&Boid>) -> Vec3 {
        let mut center = Vec3::new(0.0, 0.0, 0.0);
        let mut neighbors = 0;
        let neighborhood_radius = 0.5;
    
        for boid in boids.iter() {
            if self.position == boid.position {
                continue;
            }

            if boid.position.distance(self.position) < neighborhood_radius {
                center += boid.position;
                neighbors += 1;
            }
        }
    
        if neighbors > 0 {
            center /= neighbors as f32;
            return -1. * center.normalize();
        }
    
        return Vec3::new(0.0, 0.0, 0.0);
    }

     fn calculate_seperation_acceleration(&mut self, boids: &Vec<&Boid>) -> Vec3 {
        let separation_factor = 15 as f32;
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

    pub fn calculate_alignment_velocity(&mut self, boids: &Vec<&Boid>) -> Vec3{
        let mut alignment = Vec3::new(0.0, 0.0, 0.0);
        let neighborhood_radius = 1.;
        let mut neighbors = 0;

        for boid in boids.iter() {
            if boid.position.distance(self.position) < neighborhood_radius {
                alignment += boid.velocity;
                neighbors += 1;
            }
        }

        if neighbors > 0 {
            alignment /= neighbors as f32;
            return alignment - self.velocity;
        }   
        return Vec3::new(0.0, 0.0, 0.0);
        
    }
}