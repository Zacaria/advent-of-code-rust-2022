use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::preceded,
    *,
};

#[derive(Debug)]
enum Command {
    Noop,
    Add(i64),
}

fn add(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("addx ")(input)?;
    let (input, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        i64::from_str_radix(s, 10)
    })(input)?;
    Ok((input, Command::Add(number)))
}
fn noop(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("noop")(input)?;

    Ok((input, Command::Noop))
}
fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, cmds) = separated_list1(newline, alt((noop, add)))(input)?;
    Ok((input, cmds))
}

pub fn part_one(input: &str) -> Option<i64> {
    let (_, cmds) = commands(input).expect("parsing error");

    let mut x = 1;
    let mut cycles = 1;
    let mut total = 0;

    for cmd in cmds.iter() {
        if cycles % 40 == 20 {
            total += cycles * x;
        }
        cycles += 1;

        if let Command::Add(num) = cmd {
            if cycles % 40 == 20 {
                total += cycles * x;
            }
            x += num;
            cycles += 1;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let screen_size = 40;
    let mut cycles = 1;
    let mut sprite_position = 1;

    let (_, cmds) = commands(input).expect("parsing error");

    for cmd in cmds.iter() {
        let draw_pos = (cycles - 1) % screen_size;
        if draw_pos == 0 {
            print!("\n");
        }
        let sprite_span = (sprite_position - 1)..=(sprite_position + 1);
        if sprite_span.contains(&draw_pos) {
            print!("#");
        } else {
            print!(".");
        }

        cycles += 1;

        if let Command::Add(num) = cmd {
            let draw_pos = (cycles - 1) % screen_size;
            if draw_pos == 0 {
                print!("\n");
            }
            // let sprite_span = (sprite_position - 1)..(sprite_position + 1);
            if sprite_span.contains(&draw_pos) {
                print!("#");
            } else {
                print!(".");
            }

            sprite_position += num;
            cycles += 1;
        }
    }
    print!("\n");
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
