use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: i32,
    visible: bool,
}

fn count_visible_trees(grid: &mut Vec<Vec<Tree>>) -> i32 {
    let line_len = grid.len();
    let col_len = grid[0].len();

    for y in 0..line_len {
        let mut current_higher_tree = 0;
        for x in 0..col_len {
            if x == 0 {
                current_higher_tree = grid[y][x].height;
                grid[y][x].visible = true;
            } else if current_higher_tree < grid[y][x].height {
                current_higher_tree = grid[y][x].height;
                grid[y][x].visible = true;
            }
        }
    }
    for y in (0..line_len).rev() {
        let mut current_higher_tree = 0;
        for x in (0..col_len).rev() {
            if x == line_len - 1 {
                current_higher_tree = grid[y][x].height;
                grid[y][x].visible = true;
            } else if current_higher_tree < grid[y][x].height {
                current_higher_tree = grid[y][x].height;
                grid[y][x].visible = true;
            }
        }
    }

    for x in 0..line_len {
        let mut current_higher_tree = 0;
        for y in 0..col_len {
            if y == 0 {
                current_higher_tree = grid[y][x].height;
                grid[y][x].visible = true;
            } else if current_higher_tree < grid[y][x].height {
                current_higher_tree = grid[y][x].height;
                grid[y][x].visible = true;
            }
        }
    }
    for x in (0..line_len).rev() {
        let mut current_higher_tree = 0;
        for y in (0..col_len).rev() {
            if y == line_len - 1 {
                current_higher_tree = grid[y][x].height;
                grid[y][x].visible = true;
            } else if current_higher_tree < grid[y][x].height {
                current_higher_tree = grid[y][x].height;
                grid[y][x].visible = true;
            }
        }
    }
    grid.iter().flatten().filter(|tree| tree.visible).count() as i32
}

fn directions(grid: &Vec<Vec<Tree>>, x: usize, y: usize) -> [Vec<Tree>; 4] {
    let row = grid[y].clone();
    let column = grid.iter().map(|row| row[x]).collect::<Vec<Tree>>();

    let (up, down) = column.split_at(y);
    let (left, right) = row.split_at(x);

    let up = up.iter().cloned().rev().collect();
    let down = down[1..].to_vec();
    let left = left.iter().cloned().rev().collect();
    let right = right[1..].to_vec();

    [up, down, right, left]
}

fn find_highest_scenic_score(grid: &mut Vec<Vec<Tree>>) -> i32 {
    let line_len = grid.len();
    let col_len = grid[0].len();

    (0..line_len)
        .cartesian_product(0..col_len)
        .map(|(x, y)| -> usize {
            let height = grid[y][x].height;

            directions(&grid, x, y)
                .map(|direction| {
                    direction
                        .iter()
                        .position(|tree| tree.height >= height)
                        .map(|position| position + 1)
                        .unwrap_or_else(|| direction.len())
                })
                .into_iter()
                .product::<usize>()
        })
        .max()
        .unwrap() as i32
}

fn parse_grid(input: &str) -> Vec<Vec<Tree>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("could not go to digit") as i32)
                .map(|t| Tree {
                    height: t,
                    visible: false,
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<Tree>> = parse_grid(input);

    let result = count_visible_trees(&mut grid);
    dbg!(&result);

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<Tree>> = parse_grid(input);

    let result = find_highest_scenic_score(&mut grid);

    Some(result as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
