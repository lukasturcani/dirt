use crate::{rectangles::Rectangles, spheres::Spheres, Timestep, Vector3};

#[derive(Debug, PartialEq)]
pub struct Collision {
    pub timestep: Timestep,
    pub position: Vector3,
    pub normal: Vector3,
}

pub fn sphere_rectangle(_spheres: &Spheres, _rectangles: &Rectangles) -> Option<Collision> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{rectangles::Dimensions, rectangles::Rectangle, spheres::Sphere};

    #[test]
    fn test_sphere_rectangle_collision() {
        let spheres = Spheres::new(
            vec![Sphere {
                position: Vector3(0., 0., 0.),
                velocity: Vector3(1., 0., 0.),
            }]
            .into_iter(),
        );
        let rectangles = Rectangles::new(
            vec![Rectangle {
                position: Vector3(1., 0., 0.),
                orientation: Vector3(1., 0., 0.),
                dimensions: Dimensions {
                    width: 1.,
                    length: 1.,
                },
            }]
            .into_iter(),
        );
        assert_eq!(
            sphere_rectangle(&spheres, &rectangles),
            Some(Collision {
                timestep: Timestep(1.),
                position: Vector3(1., 0., 0.),
                normal: Vector3(1., 1., 1.)
            }),
        );
    }
}
