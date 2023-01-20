use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Command {
    from: u32,
    to: u32,
    nb: u32,
}

#[derive(Clone)]
struct Stacks {
    crates: HashMap<u32, Vec<char>>,
}

impl Stacks {
    fn new() -> Self {
        Self {
            crates: HashMap::new(),
        }
    }

    fn move_item(mut self, command: &Command) -> Self {
        println!("stacks before move {:#?}", self.crates);
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

        println!("stacks after move {:#?}", self.crates);

        self
    }
}

pub fn part_one(input: &str) -> Option<String> {
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

    let initial_state =
        stack_lines
            .iter()
            .rev()
            .fold(Stacks::new(), |mut stacks, line| -> Stacks {
                line.chars().enumerate().for_each(|(index, c)| {
                    // -1 to skip first [ to ignore
                    let pos_lookup = index as i32 % 4 - 1;
                    let is_crate_pos = pos_lookup == 0;
                    let stack_pos = index as i32 / 4;
                    if is_crate_pos && c != ' ' {
                        let mut current_stack: Vec<char> = stacks
                            .crates
                            .get(&(stack_pos as u32 + 1))
                            .unwrap_or(&vec![])
                            .to_vec();
                        current_stack.push(c);
                        stacks.crates.insert(stack_pos as u32 + 1, current_stack);
                    }
                });
                stacks
            });

    let reg = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let final_state = command_lines
        .iter()
        .map(|command| -> Command {
            println!("text command : {:?}", &command);
            let captures = reg
                .captures(&command)
                .expect("malformed command in input data");

            println!("capture : {:?}", &captures);
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
        .fold(initial_state, |current_state: Stacks, command| -> Stacks {
            let new_stacks = current_state.clone();
            new_stacks.move_item(&command)
        });

    let mut result = String::new();

    for index in 1..=final_state.crates.len() {
        let to_push = final_state
            .crates
            .get(&(index as u32))
            .expect("index not found")
            .last()
            .expect("last char not found");
        result.push(*to_push);
    }
    println!("{} {:#?} ", result, final_state.crates);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
