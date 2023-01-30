#[derive(Debug)]
struct Tree {
    height: i32,
    seen: bool,
    visible: bool,
}

enum Side {
    Top,
    Down,
    Right,
    Left,
}

fn is_visible(trees_facing: Vec<i32>, tree: &i32) -> bool {
    if let Some(higher_facing) = trees_facing.iter().max() {
        return higher_facing < tree;
    } else {
        true
    }
}

fn get_trees_facing(forest: Vec<u32>, side: Side) -> Vec<u32> {
    forest
}

fn count_visible_trees(grid: &Vec<Vec<Tree>>) -> i32 {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let curr = &grid[i][j];
            if curr.visible {
                continue;
            }

            if i == 0 || i == grid.len() - 1 || j == 0 || j == grid[0].len() - 1 {
                count += 1;
                // curr.visible = true;
                continue;
            }

            let mut is_visible = true;
            for k in 0..grid.len() {
                if grid[k][j].height > curr.height {
                    is_visible = false;
                    break;
                }
            }

            if is_visible {
                for k in 0..grid[0].len() {
                    if grid[i][k].height > curr.height {
                        is_visible = false;
                        break;
                    }
                }
            }

            if is_visible {
                count += 1;
                // curr.visible = true;
            }
        }
    }
    count + 
}

fn parse_grid(input: &str) -> Vec<Vec<Tree>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("could not go to digit") as i32)
                .map(|t| Tree {
                    height: t,
                    seen: false,
                    visible: false,
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<Tree>> = parse_grid(input);

    let result = count_visible_trees(&grid);
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
