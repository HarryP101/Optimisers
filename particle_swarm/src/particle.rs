pub struct Particle {
    position: Vec<f64>,
    velocity: Vec<f64>,
    local_best: f64,
    global_best: f64,
}

impl Particle {
    pub fn new(num_dimensions: usize) -> Particle {
        let position: Vec<f64> = vec![0.0; num_dimensions];
        let velocity: Vec<f64> = vec![1.0; num_dimensions];
        let local_best = 0.0;
        let global_best = 0.0;
        Particle { position, velocity, local_best, global_best }
    }

    pub fn update_pos(&mut self) {
        for it in self.position.iter_mut().zip(self.velocity.iter())
        {
            let (xi, vi) = it;
            *xi += *vi;
        }
    }

    pub fn update_vel(&mut self) {

    }

    pub fn get_position(&self) -> &Vec<f64> {
        &self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialise_particle() {
        let num_dimensions = 3;
        let particle = Particle::new(num_dimensions);
        let expected: Vec<f64> = vec![0.0; num_dimensions];
        assert_eq!(&expected, particle.get_position());
    }

    fn update_pos_works() {
        let num_dimensions = 3;
        let mut particle = Particle::new(num_dimensions);
        let expected: Vec<f64> = vec![1.0; num_dimensions];
        
        particle.update_pos();

        assert_eq!(&expected, particle.get_position());
    }

    fn update_vel_works() {
        let num_dimensions = 3;
        let mut particle = Particle::new(num_dimensions);
        let expected: Vec<f64> = vec![1.0; num_dimensions];
        
        particle.update_vel();
        particle.update_pos();

        assert_eq!(&expected, particle.get_position());
    }
}
