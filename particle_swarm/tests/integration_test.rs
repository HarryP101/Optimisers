use particle_swarm::PSConfig;
use particle_swarm::SearchSpace;
use common::ClientMerit;
use common::ClientTermination;
use std::sync::Arc;

mod common;

#[test]
fn it_works() {
    let num_iters = 100;
    let num_particles = 10;
    let num_dimensions = 1;
    let lower = vec![-1.0];
    let upper = vec![1.0];
    let search_space = SearchSpace::new(lower, upper);
    let merit = Arc::new(ClientMerit);
    let termination = Box::new(ClientTermination);

    let config = PSConfig::new(num_iters, num_particles, num_dimensions, search_space, merit, termination);

    let result = match particle_swarm::run(config) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error when running {}", e);
            0.0
        }
    };
    println!("the final value is: {}", result);
    assert!(1.1 > result);
    assert!(0.9 < result);
}