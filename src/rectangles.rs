use super::Vector3;

pub struct Rectangle {
    pub position: Vector3,
    pub orientation: Vector3,
    pub dimensions: Dimensions,
}

pub struct Rectangles {
    pub positions: Vec<Vector3>,
    pub orientations: Vec<Vector3>,
    pub dimensions: Vec<Dimensions>,
}

pub struct Dimensions {
    pub width: f32,
    pub length: f32,
}

impl Rectangles {
    pub fn new(rectangles: impl Iterator<Item = Rectangle>) -> Self {
        let mut result = Rectangles {
            positions: vec![],
            orientations: vec![],
            dimensions: vec![],
        };
        rectangles.for_each(|rectangle| {
            result.positions.push(rectangle.position);
            result.orientations.push(rectangle.orientation);
            result.dimensions.push(rectangle.dimensions);
        });
        result
    }
}
