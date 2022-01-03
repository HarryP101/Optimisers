use particle_swarm::{PSConfigMultiThread, PSConfigSingleThread, SearchSpace, Optimum};
use common::{SimpleFn, MultiDFn, ClientTermination};
use std::sync::Arc;

mod common;

#[test]
fn simple_fn_single_threaded() {
    let num_iters = 100;
    let num_particles = 10;
    let num_dimensions = 1;
    let lower = vec![-1.0];
    let upper = vec![1.0];
    let search_space = SearchSpace::new(lower, upper);
    let merit = Box::new(SimpleFn);
    let termination = Box::new(ClientTermination);

    let config = PSConfigSingleThread::new(num_iters, num_particles, num_dimensions, search_space, merit, termination);

    let result = match particle_swarm::run_singlethreaded(config) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error when running {}", e);
            Optimum { best_swarm_merit: f64::INFINITY,
                best_swarm_position: vec![0.0] }
        }
    };
    assert!(1.1 > result.best_swarm_merit);
    assert!(0.9 < result.best_swarm_merit);
}

#[test]
fn simple_fn_works() {
    let num_iters = 10000;
    let num_particles = 10;
    let num_dimensions = 1;
    let num_threads = 5;
    let lower = vec![-1.0];
    let upper = vec![1.0];
    let search_space = SearchSpace::new(lower, upper);
    let merit = Arc::new(SimpleFn);
    let termination = Arc::new(ClientTermination);

    let config = PSConfigMultiThread::new(num_iters, num_particles, num_dimensions, num_threads, search_space, merit, termination);

    let result = match particle_swarm::run_multithreaded(config) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error when running {}", e);
            Optimum { best_swarm_merit: f64::INFINITY,
                best_swarm_position: vec![0.0] }
        }
    };
    assert!(1.1 > result.best_swarm_merit);
    assert!(0.9 < result.best_swarm_merit);
}

#[test]
fn three_dimension_fn_works() {
    let num_iters = 3000;
    let num_particles = 100;
    let num_dimensions = 3;
    let num_threads = 5;
    let lower = vec![-1.0, -1.0, -1.0];
    let upper = vec![1.0, 1.0, 1.0];
    let search_space = SearchSpace::new(lower, upper);
    let merit = Arc::new(MultiDFn);
    let termination = Arc::new(ClientTermination);

    let config = PSConfigMultiThread::new(num_iters, num_particles, num_dimensions, num_threads, search_space, merit, termination);

    let result = match particle_swarm::run_multithreaded(config) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error when running {}", e);
            Optimum { best_swarm_merit: f64::INFINITY,
                best_swarm_position: vec![0.0] }
        }
    };
    assert!(0.1 > result.best_swarm_merit);
    assert!(-0.1 < result.best_swarm_merit);
}

#[test]
fn six_dimension_fn_works() {
    let num_iters = 4000;
    let num_particles = 600;
    let num_dimensions = 6;
    let num_threads = 60;
    let lower = vec![-1.0; 6];
    let upper = vec![1.0; 6];
    let search_space = SearchSpace::new(lower, upper);
    let merit = Arc::new(MultiDFn);
    let termination = Arc::new(ClientTermination);

    let config = PSConfigMultiThread::new(num_iters, num_particles, num_dimensions, num_threads, search_space, merit, termination);

    let result = match particle_swarm::run_multithreaded(config) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error when running {}", e);
            Optimum { best_swarm_merit: f64::INFINITY,
                best_swarm_position: vec![0.0] }
        }
    };
    assert!(0.3 > result.best_swarm_merit);
    assert!(-0.3 < result.best_swarm_merit);
}