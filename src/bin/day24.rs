use std::collections::{BTreeSet, HashMap, HashSet};

static INPUT: &str = include_str!("../../assets/day24.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let mut walls = HashSet::new();
    let mut blizzards = Vec::new();

    for (y, l) in INPUT.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let pos = (x as i32, y as i32);
            match c {
                '>' => blizzards.push((pos, Facing::Right)),
                '<' => blizzards.push((pos, Facing::Left)),
                '^' => blizzards.push((pos, Facing::Up)),
                'v' => blizzards.push((pos, Facing::Down)),
                '#' => _ = walls.insert(pos),
                '.' => {}
                _ => panic!("Unknown character: {}", c),
            }
        }
    }

    let height = walls.iter().filter(|(x, _)| *x == 0).max().unwrap().1;
    let width = walls.iter().filter(|(_, y)| *y == 0).max().unwrap().0;

    let start = (1, 0);
    let end = (width - 1, height);

    let mut total_steps = 0;

    // From start to end
    let (steps, blizzards) = find_fastest_path(&walls, &blizzards, start, end, width, height);
    total_steps += steps;

    println!("Part 1: {}", steps);

    // Move back to start
    let (steps, blizzards) = find_fastest_path(&walls, &blizzards, end, start, width, height);
    total_steps += steps;

    // Move back to end
    let (steps, _) = find_fastest_path(&walls, &blizzards, start, end, width, height);
    total_steps += steps;

    println!("Part 2: {}", total_steps);
}

fn find_fastest_path(
    walls: &HashSet<(i32, i32)>,
    blizzard: &Vec<((i32, i32), Facing)>,
    start: (i32, i32),
    end: (i32, i32),
    width: i32,
    height: i32,
) -> (i32, Vec<((i32, i32), Facing)>) {
    let mut queue = BTreeSet::new();

    // Initial state
    queue.insert((0, start));

    let mut blizzard_map = HashMap::new();
    blizzard_map.insert(0, blizzard.clone());

    while let Some((steps, pos @ (x, y))) = queue.pop_first() {
        // Check if we reached the end
        if pos == end {
            return (steps - 1, blizzard_map.get(&(steps - 1)).cloned().unwrap());
        }

        // Move blizzards if needed
        let blizzards = match blizzard_map.get(&steps) {
            Some(b) => b,
            None => {
                // Calculate new blizzard positions once
                let prev = blizzard_map.get(&(steps - 1)).unwrap();
                let b = move_blizzards(walls, prev, width, height);
                blizzard_map.insert(steps, b.clone());
                blizzard_map.get(&steps).unwrap()
            }
        };

        // Try moving to adjacent or stay on current tile
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)] {
            let new_pos @ (_, ny) = (x + dx, y + dy);

            // Check if we can move to the new position
            if !(walls.contains(&new_pos) || ny < 0 || ny > height)
                && !blizzards.iter().any(|(b, _)| b == &new_pos)
            {
                queue.insert((steps + 1, new_pos));
            }
        }
    }

    panic!("No path found");
}

fn move_blizzards(
    walls: &HashSet<(i32, i32)>,
    blizzards: &[((i32, i32), Facing)],
    width: i32,
    height: i32,
) -> Vec<((i32, i32), Facing)> {
    blizzards
        .iter()
        .map(|((x, y), facing)| {
            let (x, y) = match facing {
                Facing::Up => (*x, y - 1),
                Facing::Down => (*x, y + 1),
                Facing::Left => (x - 1, *y),
                Facing::Right => (x + 1, *y),
            };
            if walls.contains(&(x, y)) {
                match facing {
                    Facing::Down => ((x, 1), Facing::Down),
                    Facing::Up => ((x, height - 1), Facing::Up),
                    Facing::Right => ((1, y), Facing::Right),
                    Facing::Left => ((width - 1, y), Facing::Left),
                }
            } else {
                ((x, y), *facing)
            }
        })
        .collect()
}
