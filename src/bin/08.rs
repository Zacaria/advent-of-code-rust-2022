#[derive(Debug)]
struct Tree {
    height: i32,
    visible: bool,
}

enum Side {
    Top,
    Down,
    Right,
    Left,
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

fn parse_grid(input: &str) -> Vec<Vec<Tree>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("could not go to digit") as i32)
                .map(|t| Tree {
                    height: t,
                    // seen: false,
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
    None
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
        assert_eq!(part_two(&input), None);
    }
}
