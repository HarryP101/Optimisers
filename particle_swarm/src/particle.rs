use crate::SearchSpace;
use rand::Rng;
use itertools::izip;

pub struct Particle {
    position: Vec<f64>,
    velocity: Vec<f64>,
    local_best: Vec<f64>,
    global_best: Vec<f64>,
    local_best_merit: f64,
}

impl Particle {
    pub fn new(num_dimensions: usize, search_space: &SearchSpace) -> Particle {

        // Initialise position with uniformly distributed vector in search_space
        let mut position: Vec<f64> = Vec::with_capacity(num_dimensions);
        let mut velocity: Vec<f64> = Vec::with_capacity(num_dimensions);

        for i in 0..num_dimensions {
            let x_lower = search_space.lower[i];
            let x_upper = search_space.upper[i];
            let xi = rand::thread_rng().gen_range(x_lower..x_upper);

            position.push(xi);

            let v_lower = -(x_upper - x_lower).abs();
            let v_upper = (x_upper - x_lower).abs();
            let vi: f64 = rand::thread_rng().gen_range(v_lower..v_upper);

            velocity.push(vi);
        }

        let local_best = position.clone();
        let global_best = position.clone();

        let local_best_merit = f64::INFINITY;

        Particle { position, velocity, local_best, global_best, local_best_merit }
    }

    pub fn update_position(&mut self) {
        for it in self.position.iter_mut().zip(self.velocity.iter()) {
            let (xi, vi) = it;
            *xi += *vi;
        }
    }

    pub fn update_velocity(&mut self) {
        for (xi, vi, pi, gi) in izip!(&self.position,
                                    &mut self.velocity,
                                    &self.local_best,
                                    &self.global_best)
        {
            let rp: f64 = rand::thread_rng().gen_range(0.0..1.0);
            let rg: f64 = rand::thread_rng().gen_range(0.0..1.0);
            let w: f64 = 0.9;

            *vi = w * *vi + rp * (pi - xi) + rg * (gi - xi);
        }
    }

    pub fn get_position(&self) -> Vec<f64> {
        self.position.clone()
    }

    pub fn get_best_merit(&self) -> f64 {
        self.local_best_merit
    }

    pub fn set_best_merit(&mut self, merit: f64) {
        self.local_best_merit = merit;
    }

    pub fn set_global_best_position(&mut self, pos: &Vec<f64>) {
        self.global_best = pos.clone();
    }

    pub fn set_local_best_position(&mut self) {
        self.local_best = self.position.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialise_particle() {
        let num_dimensions = 3;
        let lower = vec![0.0; 3];
        let upper = vec![1.0; 3];
        let search_space = SearchSpace::new(lower, upper);
        let particle = Particle::new(num_dimensions, &search_space);
        let expected = 3;
        assert_eq!(expected, particle.get_position().len());
    }

    fn update_pos_works() {
        let num_dimensions = 3;
        let lower = vec![0.0; 3];
        let upper = vec![1.0; 3];
        let search_space = SearchSpace::new(lower, upper);
        let mut particle = Particle::new(num_dimensions, &search_space);
        let expected = 3;
        
        particle.update_position();

        assert_eq!(expected, particle.get_position().len());
    }

    fn update_vel_works() {
        let num_dimensions = 3;
        let lower = vec![0.0; 3];
        let upper = vec![1.0; 3];
        let search_space = SearchSpace::new(lower, upper);
        let mut particle = Particle::new(num_dimensions, &search_space);
        let expected = 3;
        
        particle.update_velocity();
        particle.update_position();

        assert_eq!(expected, particle.get_position().len());
    }
}
