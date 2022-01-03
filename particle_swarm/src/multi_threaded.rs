use std::error::Error;
use std::sync::{Arc, Mutex};
use rand::Rng;
use crate::{particle::Particle, thread_pool::ThreadPool};
use crate::{PSConfigMultiThread, Optimum, MeritFunction};

pub fn run(config: PSConfigMultiThread) -> Result<Optimum, Box<dyn Error>> {

    // Create shared ownership of best swarm position and merit across entire swarm
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

                iterate(&mut particle, config.cognitive_coeff,
                    config.social_coeff, &merit_function, &optimum);

                let merit = optimum.lock().unwrap().best_swarm_merit;

                if termination_function.should_stop(merit) {
                    break;
                }
            }
        };

        thread_pool.execute(iterate);
    }

    let opt = &*optimum.lock().unwrap();

    let result = Optimum { best_swarm_merit: opt.best_swarm_merit,
                        best_swarm_position: opt.best_swarm_position.clone() };
    Ok(result)
}

fn initialise(config: &PSConfigMultiThread, optimum: &Arc<Mutex<Optimum>>) -> Vec<Particle> {

    let mut swarm = Vec::with_capacity(config.num_particles);

    // Create the swarm of particles
    for _ in 0..config.num_particles {

        let mut optimum = optimum.lock().unwrap();

        let particle = Particle::new(config.num_dimensions,
            &config.search_space,
            &optimum.best_swarm_position, 
            config.particle_weight);

        let merit = config.merit.calculate(&particle.position);

        if merit < optimum.best_swarm_merit {
            optimum.best_swarm_position = particle.position.clone();
            optimum.best_swarm_merit = merit;
        }
        swarm.push(particle);
    }
    swarm
}

fn iterate(particle: &mut Particle, cognitive_coeff: f64, social_coeff: f64,
    merit_function: &Arc<dyn MeritFunction + Send + Sync>, optimum: &Arc<Mutex<Optimum>>) {

    particle.update_position();

    let rp: f64 = rand::thread_rng().gen_range(0.0..1.0);
    let rg: f64 = rand::thread_rng().gen_range(0.0..1.0);

    particle.update_velocity(rp * cognitive_coeff, rg * social_coeff);

    let merit = merit_function.calculate(&particle.position);

    // Update local and global best merits if needed
    if merit < particle.local_best_merit {
        particle.local_best_merit = merit;
        particle.set_local_best_position();

        let mut optimum = optimum.lock().unwrap();
        
        if particle.local_best_merit < optimum.best_swarm_merit {
            optimum.best_swarm_merit = particle.local_best_merit;
            optimum.best_swarm_position = particle.position.clone();
        }
    }
}