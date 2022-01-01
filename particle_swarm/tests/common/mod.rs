use particle_swarm::MeritFunction;
use particle_swarm::Termination;

pub struct ClientMerit;
pub struct ClientTermination;

impl MeritFunction for ClientMerit {
    fn calculate(&self, data: &Vec<f64>) -> f64 {
        // Minimise the function x^2 + 1;
        1.0 + data[0] * data[0]
    }
}

impl Termination for ClientTermination {
    fn should_stop(&self, merit: f64) -> bool {
        if merit < 1.1 && merit > 0.9 {
            true
        }
        else {
            false
        }
    }
}
 