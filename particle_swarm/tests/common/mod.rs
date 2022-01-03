use particle_swarm::MeritFunction;
use particle_swarm::Termination;

pub struct SimpleFn;
pub struct ClientTermination;
pub struct MultiDFn;

impl MeritFunction for SimpleFn {
    fn calculate(&self, data: &Vec<f64>) -> f64 {
        // Minimise the function x^2 + 1;
        1.0 + data[0] * data[0]
    }
}

impl MeritFunction for MultiDFn {
    fn calculate(&self, data: &Vec<f64>) -> f64 {
        let mut total = 0.0;
        for elem in data {
            total += elem * elem;
        }
        total
    }
}

impl Termination for ClientTermination {
    fn should_stop(&self, merit: f64) -> bool {
        false
    }
}
 