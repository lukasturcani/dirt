use super::Vector3;

#[derive(Debug)]
pub struct Plane {
    pub position: Vector3,
    pub normal: Vector3,
}

#[derive(Debug)]
pub struct Planes {
    pub positions: Vec<Vector3>,
    pub normals: Vec<Vector3>,
}

impl Planes {
    pub fn new(planes: impl Iterator<Item = Plane>) -> Self {
        let mut result = Self {
            positions: vec![],
            normals: vec![],
        };
        planes.for_each(|plane| {
            result.positions.push(plane.position);
            result.normals.push(plane.normal);
        });
        result
    }
}
