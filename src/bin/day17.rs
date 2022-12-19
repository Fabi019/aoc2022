use std::{collections::HashMap, iter::Cycle, slice::Iter};

static INPUT: &str = include_str!("../../assets/day17.txt");

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Piece {
    pos: (usize, usize),
    size: (usize, usize),
    pixels: Vec<Vec<bool>>,
}

fn main() {
    let pieces: [Piece; 5] = [
        // Minus
        Piece {
            pos: (0, 0),
            size: (4, 1),
            pixels: vec![vec![true, true, true, true]],
        },
        // Plus
        Piece {
            pos: (0, 0),
            size: (3, 3),
            pixels: vec![
                vec![false, true, false],
                vec![true, true, true],
                vec![false, true, false],
            ],
        },
        // L
        Piece {
            pos: (0, 0),
            size: (3, 3),
            pixels: vec![
                vec![true, true, true],
                vec![false, false, true],
                vec![false, false, true],
            ],
        },
        // I
        Piece {
            pos: (0, 0),
            size: (1, 4),
            pixels: vec![vec![true], vec![true], vec![true], vec![true]],
        },
        // Square
        Piece {
            pos: (0, 0),
            size: (2, 2),
            pixels: vec![vec![true, true], vec![true, true]],
        },
    ];

    let pattern = INPUT
        .chars()
        .filter(|c| c == &'>' || c == &'<')
        .collect::<Vec<_>>();

    let mut piece = pieces.iter().cycle();
    let mut movement = pattern.iter().cycle();

    let p1 = simulate(2022, &mut piece, &mut movement);
    println!("Part 1: {}", p1);

    //let p2 = simulate(1000000000000, &mut piece, &mut movement);
    //println!("Part 2: {}", p1);
}

fn simulate(
    rock_count: usize,
    piece: &mut Cycle<Iter<Piece>>,
    movement: &mut Cycle<Iter<char>>,
) -> usize {
    let mut chamber: HashMap<(usize, usize), bool> = HashMap::new();
    let mut height = 0;

    let mut count = 0;
    while count < rock_count {
        let mut m;
        let mut p = piece.next().unwrap().clone();

        // Set initial position
        p.pos = (2, height + 3);

        loop {
            let mut old @ (x, y) = p.pos;
            let (width, _) = p.size;

            // Move left, right first
            m = movement.next().unwrap();
            match m {
                '<' if x > 0 => p.pos = (x - 1, y),
                '>' if x + width < 7 => p.pos = (x + 1, y),
                _ => {}
            }

            // Check vertical collision with other pieces
            if check_collision(&p, &chamber) {
                p.pos = old;
            }

            old = p.pos;

            // Check bottom reached
            if p.pos.1 as isize - 1 < 0 {
                place_in_chamber(&p, &mut chamber);
                break;
            } else {
                p.pos.1 -= 1;

                // Check horizontal collision with other pieces
                if check_collision(&p, &chamber) {
                    // Revert down movement
                    p.pos = old;
                    place_in_chamber(&p, &mut chamber);
                    break;
                }
            }
        }

        height = chamber
            .iter()
            .filter(|(_, p)| **p)
            .map(|(pos, _)| pos.1 + 1)
            .max()
            .unwrap();

        count += 1;
    }

    height
}

fn place_in_chamber(piece: &Piece, chamber: &mut HashMap<(usize, usize), bool>) {
    let (x, y) = piece.pos;
    let (width, height) = piece.size;

    for dy in 0..height {
        for dx in 0..width {
            if piece.pixels[dy][dx] {
                chamber.insert((x + dx, y + dy), true);
            }
        }
    }
}

fn check_collision(piece: &Piece, chamber: &HashMap<(usize, usize), bool>) -> bool {
    let (x, y) = piece.pos;
    let (width, height) = piece.size;

    for dy in 0..height {
        for dx in 0..width {
            if piece.pixels[dy][dx] {
                let (x, y) = (x + dx, y + dy);
                if chamber.contains_key(&(x, y)) {
                    return true;
                }
            }
        }
    }

    false
}
