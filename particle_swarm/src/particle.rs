use crate::SearchSpace;
use rand::Rng;
use itertools::izip;

pub struct Particle {
    pub position: Vec<f64>,
    pub local_best_merit: f64,
    velocity: Vec<f64>,
    local_best: Vec<f64>,
    global_best: Vec<f64>,
    weight: f64,
}

impl Particle {
    pub fn new(num_dimensions: usize, search_space: &SearchSpace,
                global_best: &Vec<f64>, weight: f64) -> Particle {

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
        let global_best = global_best.clone();

        let local_best_merit = f64::INFINITY;

        Particle { position, velocity, local_best, global_best, local_best_merit, weight }
    }

    pub fn update_position(&mut self) {
        for it in self.position.iter_mut().zip(self.velocity.iter()) {
            let (xi, vi) = it;
            *xi += *vi;
        }
    }

    pub fn update_velocity(&mut self, cognitive_coeff: f64, social_coeff: f64) {
        izip!(&self.position,
            &mut self.velocity,
            &self.local_best,
            &self.global_best).into_iter().for_each(|(xi, vi, pi, gi)| {

            *vi = self.weight * *vi + cognitive_coeff * (pi - xi) + social_coeff * (gi - xi);
        });
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
        let best_swarm_position: Vec<f64> = vec![0.0, 0.0, 0.0];
        let weight = 1.0;
        let particle = Particle::new(num_dimensions, &search_space, &best_swarm_position, weight);
        let expected = 3;
        assert_eq!(expected, particle.position.len());
    }

    #[test]
    fn update_pos_works() {
        let num_dimensions = 3;
        let lower = vec![0.0; 3];
        let upper = vec![1.0; 3];
        let search_space = SearchSpace::new(lower, upper);
        let best_swarm_position: Vec<f64> = vec![0.0, 0.0, 0.0];
        let weight = 1.0;
        let mut particle = Particle::new(num_dimensions, &search_space, &best_swarm_position, weight);
        let expected = 3;
        
        particle.update_position();

        assert_eq!(expected, particle.position.len());
    }

    #[test]
    fn update_vel_works() {
        let num_dimensions = 3;
        let lower = vec![0.0; 3];
        let upper = vec![1.0; 3];
        let search_space = SearchSpace::new(lower, upper);
        let best_swarm_position: Vec<f64> = vec![0.0, 0.0, 0.0];
        let weight = 1.0;
        let mut particle = Particle::new(num_dimensions, &search_space, &best_swarm_position, weight);
        let expected = 3;
        let cognitive_coeff = 0.8;
        let social_coeff = 0.7;

        particle.update_velocity(cognitive_coeff, social_coeff);
        particle.update_position();

        assert_eq!(expected, particle.position.len());
    }
}
