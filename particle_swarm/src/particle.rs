struct Particle {
    position: Vec<f64>,
    velocity: Vec<f64>,
    local_best: f64,
    global_best: f64,
}

impl Particle {
    pub fn new() -> Particle {
        let position: Vec<f64> = Vec::new();
        let velocity: Vec<f64> = Vec::new();
        let local_best = 0.0;
        let global_best = 0.0;
        Particle { position, velocity, local_best, global_best }
    }

    pub fn update_pos(&mut self) {
        for i in 0..self.position.len() {
            self.position[i] += self.velocity[i];
        }
    }

    pub fn update_vel(&mut self) {

    }

    pub fn apply_merit_fn<T, F>(&mut self, merit: F)
    where 
        F: Fn(&Vec<f64>) -> f64
    {
        self.local_best = merit(&self.position);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn initialise_particle() {
        assert_eq!(1, 1);
    }

    fn update_pos_works() {
        assert_eq!(1, 1);
    }

    fn update_vel_works() {
        assert_eq!(1, 1);
    }
}
