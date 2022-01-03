use std::error::Error;
use std::sync::Arc;

mod particle;
mod thread_pool;
mod single_threaded;
mod multi_threaded;

pub struct SearchSpace {
    lower: Vec<f64>,
    upper: Vec<f64>,
}

impl SearchSpace {
    pub fn new(lower: Vec<f64>, upper: Vec<f64>) -> SearchSpace {
        SearchSpace { lower, upper }
    }
}

pub struct PSConfigMultiThread {
    num_iters: u32,
    num_particles: usize,
    num_dimensions: usize,
    num_threads: usize,
    search_space: SearchSpace,
    merit: Arc<dyn MeritFunction + Send + Sync>,
    termination: Arc<dyn Termination + Send + Sync>,
}

pub struct PSConfigSingleThread {
    num_iters: u32,
    num_particles: usize,
    num_dimensions: usize,
    search_space: SearchSpace,
    merit: Box<dyn MeritFunction>,
    termination: Box<dyn Termination>,
}

pub struct Optimum {
    pub best_swarm_merit: f64,
    pub best_swarm_position: Vec<f64>,
}

impl PSConfigMultiThread {
    pub fn new(num_iters: u32,
        num_particles: usize,
        num_dimensions: usize,
        num_threads: usize,
        search_space: SearchSpace,
        merit: Arc<dyn MeritFunction + Send + Sync>,
        termination: Arc<dyn Termination + Send + Sync>) -> PSConfigMultiThread {

        PSConfigMultiThread { num_iters, num_particles, num_dimensions, num_threads, search_space, merit, termination }
    }
}

impl PSConfigSingleThread {
    pub fn new(num_iters: u32,
        num_particles: usize,
        num_dimensions: usize,
        search_space: SearchSpace,
        merit: Box<dyn MeritFunction>,
        termination: Box<dyn Termination>) -> PSConfigSingleThread {

        PSConfigSingleThread { num_iters, num_particles, num_dimensions, search_space, merit, termination }
    }
}

impl Optimum {
    pub fn new(num_dimensions: usize) -> Optimum {
        let best_swarm_merit = f64::INFINITY;
        let best_swarm_position = Vec::with_capacity(num_dimensions);

        Optimum { best_swarm_merit, best_swarm_position}
    }
}

pub fn run_multithreaded(config: PSConfigMultiThread) -> Result<Optimum, Box<dyn Error>> {

    multi_threaded::run(config)
}

pub fn run_singlethreaded(config: PSConfigSingleThread) -> Result<Optimum, Box<dyn Error>> {  

    single_threaded::run(config) 
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
