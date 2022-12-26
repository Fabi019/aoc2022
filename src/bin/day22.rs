use std::collections::HashMap;

static INPUT: &str = include_str!("../../assets/day22.txt");

#[derive(Debug, Clone)]
enum Type {
    Wall,
    Empty,
}

#[derive(Debug, Clone)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

fn main() {
    let mut map = HashMap::new();
    let mut password = "";

    let mut lines = INPUT.lines().enumerate();

    let mut width = 0;
    let mut height = 0;

    while let Some((y, line)) = lines.next() {
        if line.is_empty() {
            password = lines.next().unwrap().1;
            break;
        }

        for (x, c) in line.chars().enumerate() {
            let pos = (x as i32, y as i32);
            match c {
                ' ' => continue,
                '#' => _ = map.insert(pos, Type::Wall),
                '.' => _ = map.insert(pos, Type::Empty),
                _ => panic!("Unknown character: {}", c),
            }
            width = width.max(x as i32);
        }
        height = height.max(y as i32);
    }

    // Find first position in the fist column
    let mut current = *map
        .keys()
        .filter(|(_, y)| *y == 0)
        .min_by_key(|(x, _)| x)
        .unwrap();

    let mut facing = Facing::Right;

    let mut password = password.chars().peekable();

    while let Some(c) = password.next() {
        match c {
            'L' => {
                facing = match facing {
                    Facing::Right => Facing::Up,
                    Facing::Down => Facing::Right,
                    Facing::Left => Facing::Down,
                    Facing::Up => Facing::Left,
                };
            }
            'R' => {
                facing = match facing {
                    Facing::Right => Facing::Down,
                    Facing::Down => Facing::Left,
                    Facing::Left => Facing::Up,
                    Facing::Up => Facing::Right,
                };
            }
            _ => {
                let mut number = c.to_string();
                while let Some(c) = password.peek() {
                    if c.is_ascii_digit() {
                        number.push(password.next().unwrap());
                    } else {
                        break;
                    }
                }

                let steps = number.parse::<usize>().unwrap();

                for _ in 0..steps {
                    let mut next = match facing {
                        Facing::Right => (current.0 + 1, current.1),
                        Facing::Down => (current.0, current.1 + 1),
                        Facing::Left => (current.0 - 1, current.1),
                        Facing::Up => (current.0, current.1 - 1),
                    };

                    match map.get(&next) {
                        Some(Type::Empty) => current = next,
                        Some(Type::Wall) => break,
                        None => {
                            // Part 1: Check opposite side of the map
                            /* let mut increment = 0;
                            while let None = map.get(&next) {
                                next = match facing {
                                    Facing::Right => (increment, next.1),
                                    Facing::Down => (next.0, increment),
                                    Facing::Left => (width - increment, next.1),
                                    Facing::Up => (next.0, height - increment),
                                };
                                increment += 1;
                            }
                            match map.get(&next) {
                                Some(Type::Empty) => current = next,
                                Some(Type::Wall) => break,
                                None => panic!("Should not happen"),
                            } */

                            // Part 2: Go around the cube (Only works with real input)
                            let (x, y, face) = match facing {
                                Facing::Right | Facing::Left => {
                                    wrap_cube_horizontal(next, &facing)
                                }
                                Facing::Down | Facing::Up => {
                                    wrap_cube_vertical(next, &facing)
                                }
                            };

                            next = (x, y);

                            match map.get(&next) {
                                Some(Type::Empty) => {
                                    current = next;
                                    facing = face;
                                }
                                Some(Type::Wall) => break,
                                None => panic!("Should not happen"),
                            }
                        }
                    }
                }
            }
        }
    }

    let (column, row) = current;

    println!("Result: {}", 1000 * (row + 1) + 4 * (column + 1) + facing as i32);
}

fn wrap_cube_horizontal(pos: (i32, i32), facing: &Facing) -> (i32, i32, Facing) {
    let (mut x, mut y) = pos;
    let mut d = facing.clone();

    if (0..50).contains(&y) {
        if x >= 150 {
            x = 99;
            y = 149 - y;
            d = Facing::Left;
        } else if x < 50 {
            x = 0;
            y = 149 - y;
            d = Facing::Right;
        }
    } else if (50..100).contains(&y) {
        if x >= 100 {
            x = y + 50;
            y = 49;
            d = Facing::Up;
        } else if x < 50 {
            x = y - 50;
            y = 100;
            d = Facing::Down;
        }
    } else if (100..150).contains(&y) {
        if x >= 100 {
            x = 149;
            y = 149 - y;
            d = Facing::Left;
        } else if x < 0 {
            x = 50;
            y = 149 - y;
            d = Facing::Right;
        }
    } else if (150..200).contains(&y) {
        if x < 0 {
            x = y - 100;
            y = 0;
            d = Facing::Down;
        } else if x >= 50 {
            x = y - 100;
            y = 149;
            d = Facing::Up;
        }
    }

    (x, y, d)
}

fn wrap_cube_vertical(pos: (i32, i32), facing: &Facing) -> (i32, i32, Facing) {
    let (mut x, mut y) = pos;
    let mut d = facing.clone();

    if (0..50).contains(&x) {
        if y < 100 {
            y = x + 50;
            x = 50;
            d = Facing::Right;
        } else if y >= 200 {
            y = 0;
            x += 100;
        }
    } else if (50..100).contains(&x) {
        if y < 0 {
            y = x + 100;
            x = 0;
            d = Facing::Right;
        } else if y >= 150 {
            y = x + 100;
            x = 49;
            d = Facing::Left;
        }
    } else if (100..150).contains(&x) {
        if y < 0 {
            x -= 100;
            y = 199;
        } else if y >= 50 {
            y = x - 50;
            x = 99;
            d = Facing::Left;
        }
    }

    (x, y, d)
}
