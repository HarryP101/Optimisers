use std::error::Error;

pub struct PSConfig 
{
    num_iters: u32,
    num_particles: u32,
    merit: Box<dyn MeritFunction>,
    termination: Box<dyn Termination>,
}

impl PSConfig {
    pub fn new(num_iters: u32,
        num_particles: u32,
        merit: Box<dyn MeritFunction>,
        termination: Box<dyn Termination>) -> PSConfig {

        PSConfig { num_iters, num_particles, merit, termination }
    }
}

pub fn run(config: PSConfig) -> Result<f64, Box<dyn Error>>
{
    println!("Number of particles {}", config.num_particles);

    let mut merit = 0.0;
    for _ in 0..config.num_iters {
        merit = config.merit.calculate();
    }
    let stop = config.termination.should_stop();

    Ok(merit)
}

pub trait MeritFunction {
    fn calculate(&self) -> f64;
}

pub trait Termination {
    fn should_stop(&self) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn initialise() {
        assert_eq!(1, 1);
    }
}
