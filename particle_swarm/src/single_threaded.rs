use std::error::Error;
use crate::particle::Particle;
use crate::{PSConfigSingleThread, Optimum, MeritFunction};

pub fn run(config: PSConfigSingleThread) -> Result<Optimum, Box<dyn Error>> {

    // Create optimum point for the entire swarm
    let mut optimum = Optimum::new(config.num_dimensions);

    // Create swarm of particles
    let mut swarm = initialise(&config, &mut optimum);

    for _ in 0..config.num_iters {

        // Iterate over all particles in the swarm
        for particle in &mut swarm {

            iterate(particle, &config.merit, &mut optimum);

            let merit = optimum.best_swarm_merit;

            if config.termination.should_stop(merit) {
                break;
            }
        }
    }

    Ok(optimum)
}

fn initialise(config: &PSConfigSingleThread, optimum: &mut Optimum) -> Vec<Particle> {

    let mut swarm = Vec::with_capacity(config.num_particles);

    // Create the swarm of particles
    for _ in 0..config.num_particles {

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

fn iterate(particle: &mut Particle, merit_function: &Box<dyn MeritFunction>, optimum: &mut Optimum) {

    particle.update_position();

    particle.update_velocity();

    let merit = merit_function.calculate(&particle.position);

    // Update local and global best merits if needed
    if merit < particle.local_best_merit {
        particle.local_best_merit = merit;
        particle.set_local_best_position();
        
        if particle.local_best_merit < optimum.best_swarm_merit {
            optimum.best_swarm_merit = particle.local_best_merit;
            optimum.best_swarm_position = particle.position.clone();
        }
    }
}