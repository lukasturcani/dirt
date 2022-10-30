use super::Vector3;

#[derive(Debug)]
pub struct Rectangle {
    pub position: Vector3,
    pub normal: Vector3,
    pub dimensions: Dimensions,
}

#[derive(Debug)]
pub struct Rectangles {
    pub positions: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub dimensions: Vec<Dimensions>,
}

#[derive(Debug)]
pub struct Dimensions {
    pub width: f32,
    pub length: f32,
}

impl Rectangles {
    pub fn new(rectangles: impl Iterator<Item = Rectangle>) -> Self {
        let mut result = Rectangles {
            positions: vec![],
            normals: vec![],
            dimensions: vec![],
        };
        rectangles.for_each(|rectangle| {
            result.positions.push(rectangle.position);
            result.normals.push(rectangle.normal);
            result.dimensions.push(rectangle.dimensions);
        });
        result
    }
}
