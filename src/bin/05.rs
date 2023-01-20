use regex::Regex;
use std::collections::HashMap;
use std::fmt;

struct Stack {
    id: u32,
    crates: Vec<char>,
}

#[derive(Debug)]
struct Command {
    from: u32,
    to: u32,
    nb: u32,
}

trait Stacks {
    fn move_item(&mut self, command: &Command);
}

// impl fmt::Display for dyn Stacks {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for (i, stack) in self.enumerate() {
//             write!(f, "{} : ({:?})", i, stack);
//         }
//     }
// }

impl Stacks for HashMap<u32, Vec<char>> {
    fn move_item(&mut self, command: &Command) {
        // let from = self.get_mut(&command.from).expect("from not found !");
        // let to = self.get_mut(&command.to).expect("to not found !");

        // let moved
        println!("stacks before move {:#?}", self);
        for _ in 0..command.nb {
            let item: char = self
                .get_mut(&command.from)
                .expect("from not found !")
                .pop()
                .expect("tried to move from an empty stack !");

            // println!("{:#?}", &command.to);

            self.get_mut(&command.to)
                .expect("to not found !")
                .push(item);
        }

        println!("stacks after move {:#?}", self);
    }
}

pub fn part_one(input: &str) -> Option<String> {
    // let st: HashMap<u32, Vec<char>> = HashMap::new();
    let mut stack_lines: Vec<String> = Vec::new();
    let mut command_lines: Vec<String> = Vec::new();
    let lines = input
        .lines()
        .map(|line| line.to_string())
        .for_each(|line| match line {
            line if line.contains("[") => stack_lines.push(line),
            line if line.contains("move") => command_lines.push(line),
            _ => (),
        });

    let initial_state = stack_lines.iter().rev().fold(
        HashMap::new(),
        |mut stacks, line| -> HashMap<u32, Vec<char>> {
            line.chars().enumerate().for_each(|(index, c)| {
                // -1 to skip first [ to ignore
                let pos_lookup = index as i32 % 4 - 1;
                let is_crate_pos = pos_lookup == 0;
                let stack_pos = index as i32 / 4;
                if is_crate_pos && c != ' ' {
                    let mut current_stack: Vec<char> =
                        stacks.get(&(stack_pos as u32)).unwrap_or(&vec![]).to_vec();
                    current_stack.push(c);
                    stacks.insert(stack_pos as u32, current_stack);
                }
            });
            stacks
        },
    );

    // parse stacks
    // let stacks: HashMap<u32, Vec<Option<char>>> = HashMap::new();
    // raw_stacks.fo
    // parse commands
    // let commands: Vec<Command> = Vec::new();
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
                    // .expect("capture index 0 not found")
                    .map(|x| x.as_str().parse::<u32>().expect("ParseIntErro"))
                    .expect("failed map nb"),
                from: captures
                    .get(2)
                    // .expect("capture index 1 not found")
                    .map(|x| x.as_str().parse::<u32>().expect("ParseIntErro"))
                    .expect("failed map from")
                    - 1,
                to: captures
                    .get(3)
                    // .expect("capture index 0 not found")
                    .map(|x| x.as_str().parse::<u32>().expect("ParseIntErro"))
                    .expect("failed map to")
                    - 1,
            };
        })
        // .collect::<Vec<Command>>();
        .fold(
            initial_state,
            |current_state: HashMap<u32, Vec<char>>, command| -> HashMap<u32, Vec<char>> {
                let mut new_stacks = current_state.clone();
                new_stacks.move_item(&command);
                new_stacks
            },
        );

    // println!("{:#?}", cs.next());
    // for command in commands {
    //     raw_stacks.move_item(&command);
    // }

    let mut result = String::new();

    // let mut result = "";
    for index in 0..final_state.len() {
        let to_push = final_state
            .get(&(index as u32))
            .expect("index not found")
            .last()
            .expect("last char not found");
        result.push(*to_push);
    }
    println!("{} {:#?} ", result, final_state);
    // lines.
    // apply commands

    // get first on stacks

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
