static INPUT: &str = include_str!("../../assets/day08.txt");

fn main() {
    let mut trees = Vec::new();

    for line in INPUT.lines() {
        let tree_line = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>();
        trees.push(tree_line);
    }

    let mut visible_trees = 0;
    let mut max_score = usize::MIN;

    for y in 0..trees.len() {
        for x in 0..trees[y].len() {
            if is_visible_vertical(&trees, x, y) || is_visible_horizontal(&trees, x, y) {
                visible_trees += 1;
            }

            let score = vertical_view_score(&trees, x, y) * horizontal_view_score(&trees, x, y);
            if score > max_score {
                max_score = score;
            }
        }
    }

    println!("Part 1: {}", visible_trees);
    println!("Part 2: {}", max_score);
}

fn is_visible_vertical(trees: &[Vec<u8>], x: usize, y: usize) -> bool {
    (0..y).all(|i| trees[i][x] < trees[y][x])
        || (y + 1..trees.len()).all(|i| trees[i][x] < trees[y][x])
}

fn is_visible_horizontal(trees: &[Vec<u8>], x: usize, y: usize) -> bool {
    (0..x).all(|i| trees[y][i] < trees[y][x])
        || (x + 1..trees[y].len()).all(|i| trees[y][i] < trees[y][x])
}

fn vertical_view_score(trees: &[Vec<u8>], x: usize, y: usize) -> usize {
    if y == 0 || y == trees.len() - 1 {
        return 0;
    }

    return (0..y)
        .rev()
        .take_while(|&i| trees[i][x] < trees[y][x])
        .count()
        * ((y + 1..trees.len())
            .take_while(|&i| trees[i][x] < trees[y][x])
            .count()
            + 1);
}

fn horizontal_view_score(trees: &[Vec<u8>], x: usize, y: usize) -> usize {
    if x == 0 || x == trees[y].len() - 1 {
        return 0;
    }

    return (0..x)
        .rev()
        .take_while(|&i| trees[y][i] < trees[y][x])
        .count()
        * ((x + 1..trees[y].len())
            .take_while(|&i| trees[y][i] < trees[y][x])
            .count());
}
