use crate::shared::Point;

const LINE: [Point; 4] = [
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 2, y: 0 },
    Point { x: 3, y: 0 },
];
const ELBOW: [Point; 5] = [
    Point { x: 2, y: 0 },
    Point { x: 2, y: 1 },
    Point { x: 2, y: 2 },
    Point { x: 0, y: 2 },
    Point { x: 1, y: 2 },
];
const POLE: [Point; 4] = [
    Point { x: 0, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 0, y: 2 },
    Point { x: 0, y: 3 },
];
const CROSS: [Point; 5] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
    Point { x: 2, y: 1 },
    Point { x: 1, y: 2 },
];
const BOX: [Point; 4] = [
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
];

pub struct Shape {
    pub points: Vec<Point>,
    pub bottom_y: usize,
    pub left_x: usize,
    pub right_x: usize,
}

impl Shape {
    pub fn new(points: &[Point]) -> Self {
        Self {
            points: Vec::from(points),
            bottom_y: points.iter().map(|pt| pt.y).max().unwrap(),
            left_x: points.iter().map(|pt| pt.x).min().unwrap(),
            right_x: points.iter().map(|pt| pt.x).max().unwrap(),
        }
    }
}

pub fn shapes() -> Vec<Shape> {
    vec![
        Shape::new(&LINE),
        Shape::new(&ELBOW),
        Shape::new(&POLE),
        Shape::new(&CROSS),
        Shape::new(&BOX),
    ]
}

pub struct Field {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<usize>,
}

impl Field {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            height: 4,
            tiles: vec![0; 4_000 * width],
        }
    }

    fn set_tile(&mut self, pt: Point) {
        self.tiles[pt.y * self.width + pt.x] = 1;
    }

    fn set_shape(&mut self, pos: Point, shape: Shape) {
        for pt in shape.points {
            self.set_tile(pos + pt);
        }
    }

    pub fn drop_rock(&mut self, shape: Shape) {}
}

pub fn run() {}
