use super::{Timestep, Vector3};

pub struct Spheres {
    pub positions: Vec<Vector3>,
    pub velocities: Vec<Vector3>,
}

impl Spheres {
    pub fn update_positions(&mut self, timestep: Timestep) {
        std::iter::zip(&mut self.positions, &self.velocities).for_each(|(position, velocity)| {
            position.0 += velocity.0 * timestep.0;
            position.1 += velocity.1 * timestep.0;
            position.2 += velocity.2 * timestep.0;
        })
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
