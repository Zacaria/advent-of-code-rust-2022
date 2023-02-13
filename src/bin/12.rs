extern crate pathfinding;
use std::hash::Hash;

use pathfinding::prelude::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(u32, u32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Cell {
    pos: Pos,
    visited: bool,
    height: u8,
}

struct Grid {
    height: i32,
    width: i32,
    cells: Vec<Cell>,
}

fn check_height(curr: &Cell, test: &Cell) -> bool {
    ((test.height as i16 - curr.height as i16) as i16).abs() < 2
}

impl Grid {
    fn successors(&self, current: &Cell) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = &current.pos;
        let index = x as i32 + y as i32 * self.width;
        let up_cell = if (index - self.width as i32) >= 0 {
            self.cells.get((index - self.width as i32) as usize)
        } else {
            None
        };
        let right_cell = self.cells.get((index + 1) as usize);
        let down_cell = self.cells.get((index + self.width as i32) as usize);
        let left_cell = if (index - 1) >= 0 {
            self.cells.get((index - 1) as usize)
        } else {
            None
        };

        let mut successors = vec![];
        match up_cell {
            Some(cell) if y as i32 - 1 >= 0 && check_height(&cell, &current) => {
                successors.push((cell.pos.clone(), 1));
            }
            _ => (),
        }

        match right_cell {
            Some(cell) if ((x + 1) as i32) < self.width && check_height(&cell, &current) => {
                successors.push((cell.pos.clone(), 1));
            }
            _ => (),
        }

        match down_cell {
            Some(cell) if ((y + 1) as i32) < self.height && check_height(&cell, &current) => {
                successors.push((cell.pos.clone(), 1));
            }
            _ => (),
        }

        match left_cell {
            Some(cell) if x as i32 - 1 >= 0 && check_height(&cell, &current) => {
                successors.push((cell.pos.clone(), 1));
            }
            _ => (),
        }

        successors
    }
}

fn parse_grid(input: &str) -> (Grid, Pos, Pos) {
    let grid_width = input.chars().take_while(|&c| c != '\n').count() as i32;
    let join_input = input.replace("\n", "");
    dbg!(join_input.len());

    let grid_height = join_input.len() as i32 / grid_width;

    let start_index = join_input.find("S").expect("S not found !") as u32;
    let start_pos = Pos(
        start_index % grid_width as u32,
        start_index / grid_width as u32,
    );
    let end_index = join_input.find("E").expect("E not found !") as u32;
    let end_pos = Pos(end_index % grid_width as u32, end_index / grid_width as u32);

    let cells: Vec<Cell> = join_input
        .bytes()
        .map(|b| -> u8 {
            if b == b'S' {
                return 0;
            }
            if b == b'E' {
                return 25;
            }
            b - 97
        })
        .enumerate()
        .map(|(i, height)| -> Cell {
            Cell {
                visited: false,
                pos: Pos(i as u32 % grid_width as u32, i as u32 / grid_width as u32),
                height,
            }
        })
        .collect();

    (
        Grid {
            cells,
            height: grid_height,
            width: grid_width,
        },
        start_pos,
        end_pos,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    // let size = 8;

    let (grid, start, end) = parse_grid(input);
    let result = astar(
        &start,
        |Pos(x, y)| {
            grid.successors(
                &grid
                    .cells
                    .get((x + y * grid.width as u32) as usize)
                    .expect("cell not found"),
            )
        },
        |p| p.distance(&end) / 3,
        |p| *p == end,
    );
    Some(result.unwrap().0.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
