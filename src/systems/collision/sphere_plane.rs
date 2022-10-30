use crate::planes::Planes;
use crate::spheres::Spheres;
use crate::systems::collision::Collision;

pub fn sphere_plane(spheres: &Spheres, planes: &Planes) -> Option<Collision> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planes::Plane;
    use crate::spheres::Sphere;
    use crate::{Timestep, Vector3};

    #[test]
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
        assert_eq!(
            sphere_plane(&spheres, &planes),
            Some(Collision {
                timestep: Timestep(1.),
                position: Vector3(1., 0., 0.),
                normal: Vector3(1., 0., 0.),
            }),
        );
    }
}
