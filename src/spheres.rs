use super::{Timestep, Vector3};
use rand::distributions::Standard;
use rand::Rng;
use rand::SeedableRng;

pub struct Spheres {
    pub positions: Vec<Vector3>,
    pub velocities: Vec<Vector3>,
}

impl Spheres {
    pub fn random(num_spheres: usize, max_position: f32, max_velocity: f32, seed: u64) -> Self {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        Self {
            positions: (&mut rng)
                .sample_iter(Standard)
                .take(num_spheres)
                .map(|position: Vector3| position * max_position)
                .collect(),
            velocities: (&mut rng)
                .sample_iter(Standard)
                .take(num_spheres)
                .map(|velocity: Vector3| velocity * max_velocity)
                .collect(),
        }
    }
}

pub struct PositionUpdate {
    pub simulated_time: Timestep,
}

impl Spheres {
    pub fn update_positions(&mut self, timestep: Timestep) -> PositionUpdate {
        std::iter::zip(&mut self.positions, &self.velocities).for_each(|(position, velocity)| {
            position.0 += velocity.0 * timestep.0;
            position.1 += velocity.1 * timestep.0;
            position.2 += velocity.2 * timestep.0;
        });
        PositionUpdate {
            simulated_time: timestep,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positions_updated() {
        let mut spheres = Spheres {
            positions: vec![Vector3(0., 0., 0.)],
            velocities: vec![Vector3(1., 0., 0.)],
        };
        spheres.update_positions(Timestep(1.));
        assert_eq!(spheres.positions, vec![Vector3(1., 0., 0.)]);
    }
}
