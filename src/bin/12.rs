extern crate pathfinding;
use colored::{ColoredString, Colorize};
use std::{fmt::Display, hash::Hash};

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

#[derive(Clone)]
struct Grid {
    height: i32,
    width: i32,
    cells: Vec<Cell>,
    goal_index: i32,
    start_index: i32,
}

fn convert_to_char(num: u8) -> char {
    let base = 'a' as u8;
    let result = base + num;
    result as char
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid_str = String::from("");
        for (i, cell) in (&self.cells).iter().enumerate() {
            if i as i32 % self.width == 0 {
                grid_str += "\n";
            }

            if i == self.goal_index as usize {
                grid_str += &"E".green().to_string();
                continue;
            }

            if i == self.start_index as usize {
                grid_str += &"S".red().to_string();
                continue;
            }

            if cell.visited {
                grid_str += &format!("{}", convert_to_char(cell.height))
                    .yellow()
                    .to_string();
            }

            if !cell.visited {
                grid_str += &format!("{}", convert_to_char(cell.height))
                    .blue()
                    .to_string();
            }
        }

        writeln!(f, "{}", grid_str)?;

        Ok(())
    }
}

fn is_climbable(test: &Cell, curr: &Cell) -> bool {
    ((test.height as i16 - curr.height as i16) as i16) <= 1
}

impl Grid {
    fn successors(&mut self, current_pos: &Pos) -> Vec<(Pos, u32)> {
        let Pos(x, y) = current_pos;

        let curr_i = *x as i32 + *y as i32 * self.width;

        let current = &self
            .cells
            .get_mut((x + y * self.width as u32) as usize)
            .expect("cell not found")
            .clone();
        let mut successors = vec![];

        let mut up_cell = if *y as i32 - 1 >= 0 {
            self.cells.get_mut((curr_i - self.width as i32) as usize)
        } else {
            None
        };
        match up_cell {
            Some(ref mut cell) if is_climbable(&cell, &current) => {
                // Some(ref mut cell) if !cell.visited && is_climbable(&cell, &current) => {
                successors.push((cell.pos.clone(), 1));
                cell.visited = true;
            }
            _ => (),
        }

        let mut right_cell = if ((x + 1) as i32) < self.width {
            self.cells.get_mut((curr_i + 1) as usize)
        } else {
            None
        };
        match right_cell {
            Some(ref mut cell) if is_climbable(&cell, &current) => {
                // Some(ref mut cell) if !cell.visited && is_climbable(&cell, &current) => {
                successors.push((cell.pos.clone(), 1));
                cell.visited = true;
            }
            _ => (),
        }
        let mut down_cell = if ((y + 1) as i32) < self.height {
            self.cells.get_mut((curr_i + self.width as i32) as usize)
        } else {
            None
        };

        match down_cell {
            Some(ref mut cell) if is_climbable(&cell, &current) => {
                // Some(ref mut cell) if !cell.visited && is_climbable(&cell, &current) => {
                successors.push((cell.pos.clone(), 1));
                cell.visited = true;
            }
            _ => (),
        }

        let mut left_cell = if *x as i32 - 1 >= 0 {
            self.cells.get_mut((curr_i - 1) as usize)
        } else {
            None
        };

        match left_cell {
            Some(ref mut cell) if !cell.visited && is_climbable(&cell, &current) => {
                successors.push((cell.pos.clone(), 1));
                cell.visited = true;
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

    let mut cells: Vec<Cell> = join_input
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

    cells[start_index as usize].visited = true;

    (
        Grid {
            cells,
            height: grid_height,
            width: grid_width,
            goal_index: end_index as i32,
            start_index: start_index as i32,
        },
        start_pos,
        end_pos,
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, start, end) = parse_grid(input);
    let result = astar(
        &start,
        |p| grid.successors(p),
        |p| p.distance(&end) / 3,
        |p| *p == end,
    );
    Some(result.unwrap().0.len() as u32 - 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, _, end) = parse_grid(input);

    let mut min_len = 99999;

    for start in grid.cells.clone().iter().filter(|c| c.height == 0) {
        let mut this_grid = grid.clone();
        let result = astar(
            &start.pos,
            |p| this_grid.successors(p),
            |p| p.distance(&end) / 10,
            |p| *p == end,
        );

        min_len = match result {
            Some((res, l)) if l < min_len => l,
            _ => min_len,
        }
    }

    Some(min_len)
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
        assert_eq!(part_two(&input), Some(29));
    }
}
