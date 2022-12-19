use std::{collections::HashSet, thread};

static INPUT: &str = include_str!("../../assets/day18.txt");

type Point3D = (i32, i32, i32);

const OFFSETS: [Point3D; 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn main() {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    let mut min_z = 0;
    let mut max_z = 0;

    let cubes = INPUT
        .lines()
        .map(|l| {
            let mut split = l.split(',');
            let x = split.next().unwrap().parse::<i32>().unwrap();
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            let y = split.next().unwrap().parse::<i32>().unwrap();
            min_y = min_y.min(y);
            max_y = max_y.max(y);
            let z = split.next().unwrap().parse::<i32>().unwrap();
            min_z = min_z.min(z);
            max_z = max_z.max(z);
            (x, y, z)
        })
        .collect::<HashSet<_>>();

    let mut side_count = 0;

    // Calculate overall surface area
    for (x, y, z) in &cubes {
        for (dx, dy, dz) in &OFFSETS {
            let side = (x + dx, y + dy, z + dz);
            if !cubes.contains(&side) {
                side_count += 1;
            }
        }
    }

    println!("Part 1: {}", side_count);

    // Calculate only exterior surface area
    let bounds = (
        (min_x - 1, max_x + 1),
        (min_y - 1, max_y + 1),
        (min_z - 1, max_z + 1),
    );

    let start = (min_x - 1, min_y - 1, min_z - 1);

    // Increase stack size to avoid stack overflow
    let builder = thread::Builder::new().stack_size(2 * 1024 * 1024);
    let handler = builder
        .spawn(move || {
            let mut visited: HashSet<Point3D> = HashSet::new();
            outside_surface(start, &bounds, &cubes, &mut visited)
        })
        .unwrap();

    let count = handler.join().unwrap();

    println!("Part 2: {:?}", count);
}

fn outside_surface(
    coord @ (x, y, z): Point3D,
    bounds: &((i32, i32), (i32, i32), (i32, i32)),
    cubes: &HashSet<Point3D>,
    visited: &mut HashSet<Point3D>,
) -> i32 {
    visited.insert(coord);

    let mut surface_count = 0;
    for (dx, dy, dz) in &OFFSETS {
        let neighbor = (x + dx, y + dy, z + dz);
        if (neighbor.0 >= bounds.0 .0 && neighbor.0 <= bounds.0 .1)
            && (neighbor.1 >= bounds.1 .0 && neighbor.1 <= bounds.1 .1)
            && (neighbor.2 >= bounds.2 .0 && neighbor.2 <= bounds.2 .1)
            && !visited.contains(&neighbor)
        {
            if cubes.contains(&neighbor) {
                surface_count += 1;
            } else {
                surface_count += outside_surface(neighbor, bounds, cubes, visited);
            }
        }
    }

    surface_count
}
