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
enum FileSystem<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

#[derive(Debug)]
struct File<'a> {
    size: u32,
    name: &'a str,
}

pub fn part_one(input: &str) -> Option<u32> {
    let cmds = commands(input).unwrap().1;

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
    dbg!(&directories);

    let folder_sizes: BTreeMap<String, u32> =
        directories
            .iter()
            .fold(BTreeMap::new(), |mut acc, (path, files)| {
                acc.entry(path.to_string()).or_insert(0);
                acc.entry(path.to_string()).and_modify(|size| {
                    *size = *size + files.iter().map(|&File { size, .. }| size).sum::<u32>();
                });
                acc
            });

    let mut duplicated_folder_sizes: BTreeMap<String, u32> = BTreeMap::new();
    for (path, _) in &folder_sizes {
        duplicated_folder_sizes.entry(path.to_string()).or_insert(0);

        duplicated_folder_sizes
            .entry(path.to_string())
            .and_modify(|current_size| {
                // dbg!(&path, &current_size);
                *current_size += folder_sizes.iter().fold(0, |sum, (key, size)| {
                    if key.starts_with(path) {
                        sum + size
                    } else {
                        sum
                    }
                });

                // .filter(|(key, _)| key.starts_with(path))
                // .map(|(_, size)| size)
                // .sum::<u32>();
            });
    }

    let result = duplicated_folder_sizes
        .iter()
        .filter(|(_, &size)| size <= 100000 as u32)
        .fold(0 as u32, |sum, (_, size)| sum + size);
    // .map(|(_, size)| size)
    // .sum::<u32>();

    // dbg!(folder_sizes, &duplicated_folder_sizes);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
