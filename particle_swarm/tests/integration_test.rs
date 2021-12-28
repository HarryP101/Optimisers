use particle_swarm;
use particle_swarm::PSConfig;
use common::ClientMerit;
use common::ClientTermination;

mod common;

#[test]
fn it_works() {
    let num_iters = 1;
    let num_particles = 1;
    let merit = Box::new(ClientMerit);
    let termination = Box::new(ClientTermination);

    let config = PSConfig::new(num_iters, num_particles, merit, termination);

    let expected = 0.0;
    let unexpected = 1.0;

    let result = match particle_swarm::run(config) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error when running {}", e);
            unexpected
        }
    };

    assert_eq!(expected, result);
}