use itertools::Itertools;
use std::collections::HashSet;

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
    let mut seen: HashSet<Coordinates> = HashSet::new();
    let (mut h_x, mut h_y) = (0, 0);

    let (mut t_x, mut t_y) = (0, 0);
    seen.insert((t_x, t_y));

    for command in commands.iter() {
        for _ in 0..command.length {
            let h_next_pos = find_next_coordinates((h_x, h_y), &command.direction);

            (h_x, h_y) = h_next_pos;

            let (diff_x, diff_y) = (h_x - t_x, h_y - t_y);

            let is_not_touching = diff_x.abs() > 1 || diff_y.abs() > 1;

            (t_x, t_y) = if is_not_touching {
                (t_x + diff_x.signum(), t_y + diff_y.signum())
            } else {
                (t_x, t_y)
            };
            seen.insert((t_x, t_y));
        }
    }

    Some(seen.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let commands = parse_commands(input);

    let mut seen: HashSet<Coordinates> = HashSet::new();
    let rope_size = 10;

    let mut rope = vec![(0, 0); rope_size];
    seen.insert(rope[rope_size - 1]);

    for command in commands.iter() {
        for _ in 0..command.length {
            // move H
            rope[0] = find_next_coordinates((rope[0].0, rope[0].1), &command.direction);

            // move the rest
            for (head_idx, tail_idx) in (0..rope.len()).tuple_windows() {
                let (head_x, head_y) = rope[head_idx];
                let (tail_x, tail_y) = rope[tail_idx];
                let (diff_x, diff_y) = (head_x - tail_x, head_y - tail_y);

                let is_not_touching = diff_x.abs() > 1 || diff_y.abs() > 1;

                if is_not_touching {
                    rope[tail_idx] = (tail_x + diff_x.signum(), tail_y + diff_y.signum());
                    if tail_idx == rope_size - 1 {
                        seen.insert((tail_x, tail_y));
                    }
                }
            }
        }
    }
    Some(seen.len() as u32)
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
        assert_eq!(part_two(&input), Some(1));
    }
}
