use itertools::Itertools;
use std::{collections::BTreeMap, fmt};

type Coordinates = (i32, i32);

fn find_next_coordinates((x, y): Coordinates, direction: &Direction) -> Coordinates {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Right => (x + 1, y),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Unknown => (x, y),
    }
}

#[derive(Clone, Debug)]
struct Grid {
    cells: BTreeMap<Coordinates, Cell>,
    done: bool,
    height: usize,
    width: usize,
}

impl Grid {
    fn execute(self: &mut Self, command: &Command) {
        dbg!(command);
        for _ in 0..command.length {
            let new_h_pos = self.move_h(&command.direction);

            // println!("{}", &self);

            self.move_t(new_h_pos, &command.direction);

            println!("{}", &self);
        }
    }

    fn move_h(self: &mut Self, direction: &Direction) -> Coordinates {
        let (x, y): Coordinates = self
            .cells
            .iter()
            .find(|(_, cell)| cell.head)
            .map(|(&position, _)| position)
            .expect("H not found");
        self.move_cell((x, y), &direction)
    }

    fn move_t(self: &mut Self, h_pos: Coordinates, direction: &Direction) {
        let (t_x, t_y): Coordinates = self
            .cells
            .iter()
            .find(|(_, cell)| cell.tail)
            .map(|(&position, _)| position)
            .expect("T not found");
        let (h_x, h_y) = h_pos;

        let (mut new_x, mut new_y) = (t_x, t_y);

        if ((t_y != h_y + 1 && t_y != h_y && t_y != h_y - 1)
            || (t_x != h_x - 1 && t_x != h_x + 1 && t_x != h_x))
            && ((t_x != h_x + 1 && t_x != h_x - 1 && t_x != h_x)
                || (t_y != h_y - 1 && t_y != h_y + 1 && t_y != h_y))
        {
            (new_x, new_y) = match direction {
                Direction::Up => (h_x, h_y + 1),
                Direction::Right => (h_x - 1, h_y),
                Direction::Down => (h_x, h_y - 1),
                Direction::Left => (h_x + 1, h_y),
                Direction::Unknown => {
                    println!("====== ERROR =====");
                    (t_x, t_y)
                }
            }
        }

        self.move_to((t_x, t_y), (new_x, new_y));
        // todo!()
    }

    fn move_to(self: &mut Self, (x, y): Coordinates, (new_x, new_y): Coordinates) {
        let cell = self.cells.get_mut(&(x, y)).expect("current cell not found");
        cell.tail = false;
        let next_cell = self
            .cells
            .get_mut(&(new_x, new_y))
            .expect(&format!("next cell not found {:?}", &(new_x, new_y)));

        next_cell.tail = true;
        next_cell.visited = true;
    }

    fn move_cell(self: &mut Self, (x, y): Coordinates, direction: &Direction) -> Coordinates {
        let cell = self.cells.get_mut(&(x, y)).expect("current cell not found");
        cell.head = false;

        let next_pos = find_next_coordinates((x, y), &direction);
        let next_cell = self
            .cells
            .get_mut(&next_pos)
            .expect(&format!("next cell not found {:?}", &next_pos));
        next_cell.head = true;

        next_pos
    }
}

#[derive(Clone, Debug, Default)]
struct Cell {
    head: bool,
    tail: bool,
    visited: bool,
    start: bool,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.done {
            for y in 0..self.height {
                for x in 0..self.width {
                    let cell = self.cells.get(&(x as i32, y as i32)).unwrap();
                    if cell.start {
                        write!(f, "s").unwrap();
                    } else if cell.visited {
                        write!(f, "#").unwrap();
                    } else {
                        write!(f, ".").unwrap();
                    }
                }
                write!(f, "\n").unwrap();
            }

            return Ok(());
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cells.get(&(x as i32, y as i32)).unwrap();

                if cell.head {
                    write!(f, "H").unwrap();
                } else if cell.tail {
                    write!(f, "T").unwrap();
                } else if cell.start {
                    write!(f, "s").unwrap();
                } else {
                    write!(f, ".").unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    Unknown,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    length: u32,
}

fn init_grid() -> Grid {
    let grid_width = 6;
    let grid_height = 5;
    let (start_x, start_y): Coordinates = (0, 4);

    let mut cells = (0..grid_width)
        .cartesian_product(0..grid_height)
        .map(|(x, y)| ((x, y), Cell::default()))
        .collect::<BTreeMap<Coordinates, Cell>>();

    cells.get_mut(&(start_x, start_y)).and_then(|mut cell| {
        *cell = Cell {
            head: true,
            tail: true,
            start: true,
            visited: true,
            ..*cell
        };
        Some(())
    });

    Grid {
        cells,
        done: false,
        height: grid_height as usize,
        width: grid_width as usize,
    }
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let (letter, length) = line.split_once(" ").expect("wrong command input format");
            let direction = match letter {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => Direction::Unknown,
            };
            Command {
                length: length
                    .parse::<u32>()
                    .expect("could not parse length to u32"),
                direction,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let commands = parse_commands(input);
    let mut grid = init_grid();

    for command in commands.iter() {
        grid.execute(command);
    }

    println!("THE END\n{}", &grid);

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
