use std::ops::Mul;

use rand::{
    distributions::{Standard, Uniform},
    prelude::Distribution,
    Rng,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Vector3(pub f32, pub f32, pub f32);

impl Distribution<Vector3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vector3 {
        Vector3(
            Uniform::new(0.0, 1.0).sample(rng),
            Uniform::new(0.0, 1.0).sample(rng),
            Uniform::new(0.0, 1.0).sample(rng),
        )
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
        self
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(mut self, rhs: Vector3) -> Self::Output {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
        self
    }
}
