use particle_swarm::MeritFunction;

pub struct PSConfig;

impl MeritFunction for PSConfig {
    fn calculate(&self) -> f64 {
        0.0
    }
}

impl PSConfig {
    pub fn new() -> PSConfig {
        PSConfig
    }
}
 