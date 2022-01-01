use std::error::Error;
use crate::particle::Particle;
use crate::thread_pool::ThreadPool;
use std::sync::{Arc, Mutex};

mod particle;
mod thread_pool;

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
    num_threads: usize,
    search_space: SearchSpace,
    merit: Arc<dyn MeritFunction + Send + Sync>,
    termination: Arc<dyn Termination + Send + Sync>,
}

struct Optimum {
    best_swarm_merit: f64,
    best_swarm_position: Vec<f64>,
}

impl PSConfig {
    pub fn new(num_iters: u32,
        num_particles: usize,
        num_dimensions: usize,
        num_threads: usize,
        search_space: SearchSpace,
        merit: Arc<dyn MeritFunction + Send + Sync>,
        termination: Arc<dyn Termination + Send + Sync>) -> PSConfig {

        PSConfig { num_iters, num_particles, num_dimensions, num_threads, search_space, merit, termination }
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

    // Create swarm of particles
    let swarm = initialise(&config, &optimum);

    // Create a thread pool
    let thread_pool = ThreadPool::new(config.num_threads);

    // Iterate over all particles in the swarm
    for mut particle in swarm.into_iter() {

        let optimum = Arc::clone(&optimum);

        let merit_function = Arc::clone(&config.merit);

        let termination_function = Arc::clone(&config.termination);

        let iterate = move || {

            for _ in 0..config.num_iters {

                iterate(&mut particle, &merit_function, &optimum);

                let merit = merit_function.calculate(&optimum.lock().unwrap().best_swarm_position);

                if termination_function.should_stop(merit) {
                    break;
                }
            }
        };

        thread_pool.execute(iterate);
    }

    let merit = optimum.lock().unwrap().best_swarm_merit;
    Ok(merit)
}

fn initialise(config: &PSConfig, optimum: &Arc<Mutex<Optimum>>) -> Vec<Particle> {

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
        swarm.push(particle);
    }
    swarm
}

fn iterate(particle: &mut Particle, merit_function: &Arc<dyn MeritFunction + Send + Sync>, optimum: &Arc<Mutex<Optimum>>) {

    particle.update_position();

    particle.update_velocity();

    let merit = merit_function.calculate(&particle.position);

    // Update local and global best merits if needed
    if merit < particle.get_best_merit() {
        particle.set_best_merit(merit);
        particle.set_local_best_position();

        let mut optimum = optimum.lock().unwrap();

        if particle.get_best_merit() < optimum.best_swarm_merit {
            optimum.best_swarm_merit = particle.get_best_merit();
            optimum.best_swarm_position = particle.position.clone();
        }
    }
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
