use particle_swarm;

mod common;

#[test]
fn it_works() {
    let config = Box::new(common::PSConfig::new());

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