#[derive(Clone, Copy)]
pub struct Timestep(pub f32);

pub mod spheres;

mod vector3;
pub use vector3::*;
