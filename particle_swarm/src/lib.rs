use std::error::Error;
use crate::particle::Particle;

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
    merit: Box<dyn MeritFunction>,
    termination: Box<dyn Termination>,
}

impl PSConfig {
    pub fn new(num_iters: u32,
        num_particles: usize,
        num_dimensions: usize,
        search_space: SearchSpace,
        merit: Box<dyn MeritFunction>,
        termination: Box<dyn Termination>) -> PSConfig {

        PSConfig { num_iters, num_particles, num_dimensions, search_space, merit, termination }
    }
}

pub fn run(config: PSConfig) -> Result<f64, Box<dyn Error>> {
    let mut merit = 0.0;

    // TODO: Put this into a swarm struct
    let mut best_swarm_pos = 0.0;
    let mut swarm: Vec<Particle> = Vec::with_capacity(config.num_particles);

    // Create the swarm of particles
    for _ in 0..config.num_particles {
        let particle = particle::Particle::new(config.num_dimensions, &config.search_space);

        swarm.push(particle);
    }
    
    for _ in 0..config.num_iters {

        for particle in &mut swarm {
            merit = config.merit.calculate(particle.get_position());

            particle.update_vel();
    
            particle.update_pos();

            if config.termination.should_stop(merit) {
                break;
            }
        }
    }

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
