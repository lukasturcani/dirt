#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Timestep(pub f32);

pub mod planes;
pub mod rectangles;
pub mod spheres;
pub mod systems;

mod vector3;
pub use vector3::*;
