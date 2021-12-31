use std::error::Error;
use crate::particle::Particle;
use std::sync::{Arc, Mutex};
use std::thread;

mod particle;

pub struct SearchSpace {
    lower: Vec<f64>,
    upper: Vec<f64>,
}

impl SearchSpace {
    pub fn new(lower: Vec<f64>, upper: Vec<f64>) -> SearchSpace {
        SearchSpace { lower, upper }
    }
}

pub struct PSConfig {
    num_iters: u32,
    num_particles: usize,
    num_dimensions: usize,
    search_space: SearchSpace,
    merit: Arc<dyn MeritFunction + Send + Sync>,
    termination: Box<dyn Termination>,
}

struct Optimum {
    best_swarm_merit: f64,
    best_swarm_position: Vec<f64>,
}

impl PSConfig {
    pub fn new(num_iters: u32,
        num_particles: usize,
        num_dimensions: usize,
        search_space: SearchSpace,
        merit: Arc<dyn MeritFunction + Send + Sync>,
        termination: Box<dyn Termination>) -> PSConfig {

        PSConfig { num_iters, num_particles, num_dimensions, search_space, merit, termination }
    }
}

impl Optimum {
    pub fn new(num_dimensions: usize) -> Optimum {
        let best_swarm_merit = f64::INFINITY;
        let best_swarm_position = Vec::with_capacity(num_dimensions);

        Optimum { best_swarm_merit, best_swarm_position}
    }
}

pub fn run(config: PSConfig) -> Result<f64, Box<dyn Error>> {

    // Shared ownership of best swarm position and merit across entire swarm
    let optimum = Arc::new(Mutex::new(Optimum::new(config.num_dimensions)));

    let mut swarm = Vec::with_capacity(config.num_particles);

    // Create the swarm of particles
    for _ in 0..config.num_particles {

        let mut optimum = optimum.lock().unwrap();

        let particle = Particle::new(config.num_dimensions,
            &config.search_space,
            &optimum.best_swarm_position);

        let merit = config.merit.calculate(&particle.position);

        if merit < optimum.best_swarm_merit {
            optimum.best_swarm_position = particle.position.clone();
            optimum.best_swarm_merit = merit;
        }
        swarm.push(Arc::new(Mutex::new(particle)));
    }
    
    for _ in 0..config.num_iters {

        // Spawn a set of threads
        let mut handles: Vec<_> = Vec::new();

        // Create a thread and closure for each particle
        for particle in swarm.iter().cloned() {

            let optimum = Arc::clone(&optimum);

            let merit_function = Arc::clone(&config.merit);

            let handle = thread::spawn(move || {

                let mut particle = particle.lock().unwrap();

                particle.update_position();

                particle.update_velocity();

                let merit = merit_function.calculate(&particle.position);
        
                if merit < particle.get_best_merit() {
                    particle.set_best_merit(merit);
                    particle.set_local_best_position();
        
                    let mut optimum = optimum.lock().unwrap();

                    if particle.get_best_merit() < optimum.best_swarm_merit {
                        optimum.best_swarm_merit = particle.get_best_merit();
                        optimum.best_swarm_position = particle.position.clone();
                    }
                }
            });           
            
            handles.push(handle);
        }

        // Wait for each thread to finish
        for handle in handles {
            handle.join().unwrap();
        }

        let merit = config.merit.calculate(&optimum.lock().unwrap().best_swarm_position);
        if config.termination.should_stop(merit) {
            return Ok(merit)
        }
    }

    let merit = config.merit.calculate(&optimum.lock().unwrap().best_swarm_position);
    Ok(merit)
}

pub trait MeritFunction {
    fn calculate(&self, data: &Vec<f64>) -> f64;
}

pub trait Termination {
    fn should_stop(&self, merit: f64) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn initialise() {
        assert_eq!(1, 1);
    }
}
