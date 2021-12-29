use std::error::Error;

mod particle;

pub struct PSConfig 
{
    num_iters: u32,
    num_particles: u32,
    num_dimensions: usize,
    merit: Box<dyn MeritFunction>,
    termination: Box<dyn Termination>,
}

impl PSConfig {
    pub fn new(num_iters: u32,
        num_particles: u32,
        num_dimensions: usize,
        merit: Box<dyn MeritFunction>,
        termination: Box<dyn Termination>) -> PSConfig {

        PSConfig { num_iters, num_particles, num_dimensions, merit, termination }
    }
}

pub fn run(config: PSConfig) -> Result<f64, Box<dyn Error>>
{
    println!("Number of particles {}", config.num_particles);

    let mut merit = 0.0;

    let mut particle = particle::Particle::new(config.num_dimensions);

    for _ in 0..config.num_iters {
        merit = config.merit.calculate(particle.get_position());

        particle.update_vel();

        particle.update_pos();

        if config.termination.should_stop(merit) {
            break;
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
