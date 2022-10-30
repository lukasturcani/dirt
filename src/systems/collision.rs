use crate::{Timestep, Vector3};

#[derive(Debug, PartialEq)]
pub struct Collision {
    pub timestep: Timestep,
    pub position: Vector3,
    pub normal: Vector3,
}

mod sphere_rectangle;
pub use sphere_rectangle::*;

mod sphere_plane;
pub use sphere_plane::*;
