use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum Files<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

// 8504156 c.dat
// dir d
fn file(input: &str) -> IResult<&str, Files> {
    // dbg!("file i", input);

    let (input, (size, name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        take_while(|c| c != '\n'),
    )(input)?;

    // dbg!("file o", input);

    Ok((input, Files::File { size, name }))
}

fn directory(input: &str) -> IResult<&str, Files> {
    // dbg!("directory i", input);

    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;

    // dbg!("directory o", input);

    Ok((input, Files::Dir(name)))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    // dbg!("ls", input);
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, directory)))(input)?;

    // dbg!(&files);

    Ok((input, Operation::Ls(files)))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    // dbg!("cd", input);
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let op = match dir {
        "/" => Operation::Cd(Cd::Root),
        ".." => Operation::Cd(Cd::Up),
        name => Operation::Cd(Cd::Down(name)),
    };

    // dbg!(&op);

    Ok((input, op))
}

fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmd) = separated_list1(newline, alt((ls, cd)))(input)?;

    Ok((input, cmd))
}

#[derive(Debug)]
struct File<'a> {
    size: u32,
    name: &'a str,
}

fn build_directories(cmds: Vec<Operation>) -> BTreeMap<String, Vec<File>> {
    let mut directories: BTreeMap<String, Vec<File>> = BTreeMap::new();
    let mut context: Vec<&str> = vec![];
    for command in cmds.iter() {
        match command {
            Operation::Cd(Cd::Root) => {
                context.push("");
            }
            Operation::Cd(Cd::Up) => {
                context.pop();
            }
            Operation::Cd(Cd::Down(name)) => {
                context.push(name);
            }
            Operation::Ls(files) => {
                directories.entry(context.join("/")).or_insert(vec![]);
                for file in files.iter() {
                    match file {
                        Files::File { size, name } => {
                            directories
                                .entry(context.join("/"))
                                .and_modify(|directory| directory.push(File { size: *size, name }));
                        }
                        Files::Dir(_) => (),
                    }
                    // dbg!("file", file, &context);
                }
            }
        }
    }
    return directories;
}

fn get_directories_sizes(directories: &BTreeMap<String, Vec<File>>) -> BTreeMap<String, u32> {
    let mut folder_sizes: BTreeMap<String, u32> = BTreeMap::new();
    for (path, _) in directories {
        folder_sizes.entry(path.to_string()).or_insert(0);

        folder_sizes
            .entry(path.to_string())
            .and_modify(|current_size| {
                // dbg!(&path, &current_size);
                *current_size += directories.iter().fold(0, |sum, (key, files)| {
                    if key.starts_with(path) {
                        let size = files.iter().fold(0, |sum, &File { size, .. }| sum + size);
                        sum + size
                    } else {
                        sum
                    }
                });
            });
    }

    folder_sizes
}

pub fn part_one(input: &str) -> Option<u32> {
    let cmds = commands(input).unwrap().1;

    let directories: BTreeMap<String, Vec<File>> = build_directories(cmds);
    // dbg!(&directories);

    let folder_sizes: BTreeMap<String, u32> = get_directories_sizes(&directories);

    let result = folder_sizes
        .iter()
        .filter(|(_, &size)| size <= 100000 as u32)
        .fold(0 as u32, |sum, (_, size)| sum + size);

    // dbg!(folder_sizes, &duplicated_folder_sizes);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cmds = commands(input).unwrap().1;
    let directories: BTreeMap<String, Vec<File>> = build_directories(cmds);
    let folder_sizes: BTreeMap<String, u32> = get_directories_sizes(&directories);

    // dbg!(&folder_sizes);

    let used_space = folder_sizes.get("").expect("no root folder");
    let free_space = 70000000 as u32 - used_space;

    let result = folder_sizes
        .iter()
        .filter(|(_name, &size)| size + free_space >= 30000000 as u32)
        .map(|(_, size)| *size)
        .min();

    result
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
