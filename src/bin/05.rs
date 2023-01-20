use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Command {
    from: u32,
    to: u32,
    nb: u32,
}

fn parse_commands(command_lines: Vec<String>) -> Vec<Command> {
    let reg = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    command_lines
        .iter()
        .map(|command| -> Command {
            // println!("text command : {:?}", &command);
            let captures = reg
                .captures(&command)
                .expect("malformed command in input data");

            // println!("capture : {:?}", &captures);
            return Command {
                nb: captures
                    .get(1)
                    .map(|x| x.as_str().parse::<u32>().expect("ParseIntErro"))
                    .expect("failed map nb"),
                from: captures
                    .get(2)
                    .map(|x| x.as_str().parse::<u32>().expect("ParseIntErro"))
                    .expect("failed map from"),
                to: captures
                    .get(3)
                    .map(|x| x.as_str().parse::<u32>().expect("ParseIntErro"))
                    .expect("failed map to"),
            };
        })
        .collect::<Vec<Command>>()
}

fn parse_stacks(stack_lines: Vec<String>) -> HashMap<u32, Vec<char>> {
    stack_lines.iter().rev().fold(
        HashMap::new(),
        |mut stacks, line| -> HashMap<u32, Vec<char>> {
            line.chars().enumerate().for_each(|(index, c)| {
                // -1 to skip first [ to ignore
                let pos_lookup = index as i32 % 4 - 1;
                let is_crate_pos = pos_lookup == 0;
                let stack_pos = index as i32 / 4;
                if is_crate_pos && c != ' ' {
                    let mut current_stack: Vec<char> = stacks
                        .get(&(stack_pos as u32 + 1))
                        .unwrap_or(&vec![])
                        .to_vec();
                    current_stack.push(c);
                    stacks.insert(stack_pos as u32 + 1, current_stack);
                }
            });
            stacks
        },
    )
}

struct Stacks {
    crates: HashMap<u32, Vec<char>>,
}

impl Stacks {
    fn new(crates: HashMap<u32, Vec<char>>) -> Self {
        Self { crates }
    }

    fn move_item(mut self, command: &Command) -> Self {
        // println!("stacks before move {:#?}", self.crates);
        for _ in 0..command.nb {
            let item: char = self
                .crates
                .get_mut(&command.from)
                .expect("from not found !")
                .pop()
                .expect("tried to move from an empty stack !");

            self.crates
                .get_mut(&command.to)
                .expect("to not found !")
                .push(item);
        }

        // println!("stacks after move {:#?}", self.crates);

        self
    }

    fn move_items(mut self, command: &Command) -> Self {
        // println!(
        //     "command {:?}, stacks before move {:#?}",
        //     &command, &self.crates
        // );
        let final_len = &self
            .crates
            .get(&command.from)
            .expect("from not found !")
            .len()
            - command.nb as usize;
        let mut items: Vec<char> = self
            .crates
            .get_mut(&command.from)
            .expect("from not found !")
            .split_off(final_len);

        self.crates
            .get_mut(&command.to)
            .expect("to not found !")
            .append(&mut items);

        // println!(
        //     "command {:?}, stacks after move {:#?}",
        //     &command, &self.crates
        // );

        self
    }
}

fn get_top_crates(stacks: &Stacks) -> String {
    let mut result = String::new();

    for index in 1..=stacks.crates.len() {
        let to_push = stacks
            .crates
            .get(&(index as u32))
            .expect("index not found")
            .last()
            .expect("last char not found");
        result.push(*to_push);
    }

    result
}

fn split_lines(input: &str) -> (Vec<String>, Vec<String>) {
    let mut stack_lines: Vec<String> = Vec::new();
    let mut command_lines: Vec<String> = Vec::new();
    input
        .lines()
        .map(|line| line.to_string())
        .for_each(|line| match line {
            line if line.contains("[") => stack_lines.push(line),
            line if line.contains("move") => command_lines.push(line),
            _ => (),
        });

    (stack_lines, command_lines)
}

pub fn part_one(input: &str) -> Option<String> {
    let (stack_lines, command_lines) = split_lines(input);

    let stacks = Stacks::new(parse_stacks(stack_lines));
    let commands = parse_commands(command_lines);

    let final_state = commands
        .iter()
        .fold(stacks, |current_state: Stacks, command| -> Stacks {
            current_state.move_item(&command)
        });

    let result = get_top_crates(&final_state);

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (stack_lines, command_lines) = split_lines(input);

    let stacks = Stacks::new(parse_stacks(stack_lines));
    let commands = parse_commands(command_lines);

    let final_state = commands
        .iter()
        .fold(stacks, |current_state: Stacks, command| -> Stacks {
            current_state.move_items(&command)
        });

    let result = get_top_crates(&final_state);

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
