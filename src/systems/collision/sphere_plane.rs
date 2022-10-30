use std::iter;

use crate::planes::Planes;
use crate::spheres::Spheres;
use crate::systems::collision::Collision;
use crate::Vector3;

pub fn sphere_plane(_spheres: &Spheres, _planes: &Planes) -> impl Iterator<Item = Collision> {
    iter::empty()
}

struct PlaneHandle<'plane> {
    position: &'plane Vector3,
    normal: &'plane Vector3,
}

struct RayHandle<'ray> {
    position: &'ray Vector3,
    orientation: &'ray Vector3,
}

#[derive(Debug, PartialEq)]
struct Intersection {
    position: Vector3,
}

fn ray_plane(ray: RayHandle, plane: PlaneHandle) -> Option<Intersection> {
    let denominator = ray.orientation.dot(plane.normal);
    let numerator = ray.position.dot(plane.normal) + plane.position.dot(plane.normal);
    let t = numerator / denominator;
    Some(Intersection {
        position: ray.position + ray.orientation * t,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planes::Plane;
    use crate::spheres::Sphere;
    use crate::Timestep;
    use test_case::test_case;

    #[test]
    #[ignore]
    fn test_sphere_plane_collision() {
        let spheres = Spheres::new(
            vec![Sphere {
                position: Vector3(0., 0., 0.),
                velocity: Vector3(1., 0., 0.),
            }]
            .into_iter(),
        );
        let planes = Planes::new(
            vec![Plane {
                position: Vector3(1., 0., 0.),
                normal: Vector3(1., 0., 0.),
            }]
            .into_iter(),
        );
        let collisions: Vec<_> = sphere_plane(&spheres, &planes).collect();
        assert_eq!(
            collisions,
            vec![Collision {
                timestep: Timestep(1.),
                position: Vector3(1., 0., 0.),
                normal: Vector3(1., 0., 0.),
            }],
        );
    }

    #[test_case(1. ; "positive normal")]
    #[test_case(-1. ; "negative normal" )]
    fn test_plane_orientation_changes(plane_orientation: f32) {
        let collision = ray_plane(
            RayHandle {
                position: &Vector3(0., 0., 0.),
                orientation: &Vector3(1., 0., 0.),
            },
            PlaneHandle {
                position: &Vector3(1., 0., 0.),
                normal: &Vector3(plane_orientation, 0., 0.),
            },
        );
        assert_eq!(
            collision,
            Some(Intersection {
                position: Vector3(1., 0., 0.)
            }),
        );
    }

    #[test_case(0. ; "position 0" )]
    #[test_case(1. ; "position 1" )]
    #[test_case(2. ; "position 2" )]
    #[test_case(-1. ; "position negative 1" )]
    #[test_case(-2. ; "position negative 2" )]
    fn test_ray_position_changes(ray_position: f32) {
        let collision = ray_plane(
            RayHandle {
                position: &Vector3(ray_position, 0., 0.),
                orientation: &Vector3(1., 0., 0.),
            },
            PlaneHandle {
                position: &Vector3(10., 0., 0.),
                normal: &Vector3(1., 0., 0.),
            },
        );
        assert_eq!(
            collision,
            Some(Intersection {
                position: Vector3(10., 0., 0.)
            }),
        );
    }
}
