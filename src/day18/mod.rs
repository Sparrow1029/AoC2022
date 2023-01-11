use std::{collections::HashSet, ops::Add};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Vertex3D {
    x: isize,
    y: isize,
    z: isize,
}

impl Add<(isize, isize, isize)> for Vertex3D {
    type Output = Vertex3D;
    fn add(self, rhs: (isize, isize, isize)) -> Self::Output {
        Vertex3D {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
            z: self.z + rhs.2,
        }
    }
}

impl From<(isize, isize, isize)> for Vertex3D {
    fn from(value: (isize, isize, isize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl Default for Vertex3D {
    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

impl std::fmt::Debug for Vertex3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:>2}, {:>2}, {:>2})", self.x, self.y, self.z)
    }
}

impl Vertex3D {
    fn neighbors(&self) -> HashSet<Vertex3D> {
        [
            (-1, 0, 0), // left
            (0, 1, 0),  // up
            (1, 0, 0),  // right
            (0, -1, 0), // down
            (0, 0, 1),  // above
            (0, 0, -1), // below
        ]
        .iter()
        .map(|pt| *self + *pt)
        .collect()
    }

    fn in_bounds(&self, bounds: &[Self; 2]) -> bool {
        let [mins, maxs] = bounds;
        self.x >= mins.x - 1
            && self.x <= maxs.x + 1
            && self.y >= mins.y - 1
            && self.y <= maxs.y + 1
            && self.z >= mins.z - 1
            && self.z <= maxs.z + 1
    }
}

fn parse(input: String) -> HashSet<Vertex3D> {
    input
        .lines()
        .map(|line| {
            let digits = line
                .split(',')
                .map(|d| d.parse::<isize>().unwrap())
                .collect::<Vec<isize>>();
            Vertex3D {
                x: digits[0],
                y: digits[1],
                z: digits[2],
            }
        })
        .collect()
}

fn grid_bounds(verteces: &HashSet<Vertex3D>) -> [Vertex3D; 2] {
    verteces.iter().fold(
        [Vertex3D::default(), Vertex3D::default()],
        |[mut mins, mut maxs], cube| {
            mins.x = mins.x.min(cube.x);
            mins.y = mins.y.min(cube.y);
            mins.z = mins.z.min(cube.z);
            maxs.x = maxs.x.max(cube.x);
            maxs.y = maxs.y.max(cube.y);
            maxs.z = maxs.z.max(cube.z);
            [mins, maxs]
        },
    )
}

fn flood_fill_exposed_cubes(verteces: &HashSet<Vertex3D>) -> HashSet<Vertex3D> {
    let bounds = grid_bounds(verteces);
    let mut exposed = HashSet::new();

    let start = Vertex3D::default();
    let mut stack = Vec::new();
    let mut seen = HashSet::new();

    stack.push(start);
    seen.insert(start);

    while let Some(vertex) = stack.pop() {
        for vx in vertex.neighbors() {
            if verteces.contains(&vx) || !vx.in_bounds(&bounds) {
                continue;
            }
            // `insert` returns true if was not previously in set, false if already in set.
            if seen.insert(vx) {
                stack.push(vx);
                exposed.insert(vx);
            }
        }
    }
    exposed
}

/// for each vertex (center of a cube in the 3D grid), check how many of its neighbors
/// are present in the set of other verteces. If there is a neighbor present, that is
/// an adjoining cube, meaning that face is not exposed as part of the surface area.
///
/// The set difference is the number of faces exposed.
fn part_1(input: String) -> usize {
    let verteces = parse(input);
    verteces.iter().fold(0, |acc, vx| {
        acc + vx.neighbors().difference(&verteces).count()
    })
}

/// Only count the neighbors for cubes on the outside of the lava droplet
fn part_2(input: String) -> usize {
    let verteces = parse(input);
    let exposed = flood_fill_exposed_cubes(&verteces);
    verteces
        .iter()
        .flat_map(|vx| vx.neighbors())
        .filter(|vx| exposed.contains(vx))
        .count()
}

pub fn run() {
    let input = std::fs::read_to_string("src/day18/input.txt").expect("error reading input file");
    println!("\n=== Day 18 ===");
    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input))
}

#[cfg(test)]
const SAMPLE_INPUT: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

#[test]
fn test_parse() {
    let verteces = parse(SAMPLE_INPUT.to_string());
    assert_eq!(verteces.len(), 13);
}

#[test]
fn test_add_tuple_to_vertex() {
    let test_vertex = Vertex3D::from((1, 1, 1));
    let new = test_vertex + (-1, 1, 2);
    assert_eq!((new.x, new.y, new.z), (0, 2, 3));
}

#[test]
fn test_neighbors() {
    let test_vertex = Vertex3D::from((0, 0, 0));
    let test_set: HashSet<Vertex3D> = HashSet::from_iter(vec![
        Vertex3D::from((-1, 0, 0)),
        Vertex3D::from((0, 1, 0)),
        Vertex3D::from((1, 0, 0)),
        Vertex3D::from((0, -1, 0)),
        Vertex3D::from((0, 0, 1)),
        Vertex3D::from((0, 0, -1)),
    ]);
    assert!(test_vertex.neighbors().is_superset(&test_set))
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(SAMPLE_INPUT.to_string()), 64);
}

#[test]
fn test_grid_bounds() {
    let verteces = parse(SAMPLE_INPUT.to_string());
    let bounds = grid_bounds(&verteces);
    assert_eq!(
        bounds,
        [Vertex3D { x: 0, y: 0, z: 0 }, Vertex3D { x: 3, y: 3, z: 6 }]
    )
}

#[test]
fn test_in_bounds() {
    let in_bounds_vertex = Vertex3D::from((2, 2, 2));
    let oob_vertex = Vertex3D::from((2, 3, 4)); // +/- 1 in bounds for neighbors' sake
    let bounds = [Vertex3D::default(), Vertex3D::from((2, 2, 2))];
    assert!(in_bounds_vertex.in_bounds(&bounds));
    assert!(!oob_vertex.in_bounds(&bounds));
}

#[test]
fn test_exposed_flood_fill() {
    let verteces = parse(SAMPLE_INPUT.to_string());
    let exposed = flood_fill_exposed_cubes(&verteces);
    assert!(exposed.is_disjoint(&verteces));
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(SAMPLE_INPUT.to_string()), 58);
}
