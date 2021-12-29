use particle_swarm::MeritFunction;
use particle_swarm::Termination;
use particle_swarm::PSConfig;

pub struct ClientMerit;
pub struct ClientTermination;

impl MeritFunction for ClientMerit {
    fn calculate(&self, data: &Vec<f64>) -> f64 {
        0.0
    }
}

impl Termination for ClientTermination {
    fn should_stop(&self, merit: f64) -> bool {
        true
    }
}
 