use std::{collections::VecDeque, ops::Add, process::Command};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1, newline},
    multi::{many1, separated_list0, separated_list1},
    sequence::{delimited, tuple},
    IResult, *,
};

type MonkeyIndex = u32;
type ItemWorryLevel = u32;

#[derive(Clone, Debug)]
struct Test {
    divisible: u32,
    true_recipient: MonkeyIndex,
    false_recipient: MonkeyIndex,
}

#[derive(Clone, Debug)]
struct Monkey {
    index: MonkeyIndex,
    items: VecDeque<ItemWorryLevel>,
    operation: Operation,
    throw_divisor: Test,
    inspections: u32,
}

#[derive(Clone, Debug)]
enum Value {
    Old,
    Num(ItemWorryLevel),
}
impl Value {
    fn number(&self, old: ItemWorryLevel) -> ItemWorryLevel {
        match self {
            Value::Num(n) => *n,
            Value::Old => old,
        }
    }
}

#[derive(Clone, Debug)]
enum Operation {
    Add((Value, Value), String),
    Multiply((Value, Value), String),
}
impl Operation {
    fn apply(&self, old: ItemWorryLevel) -> ItemWorryLevel {
        let (s, b, result) = match self {
            Operation::Add((a, b), s) => (s, b, a.number(old) + b.number(old)),
            Operation::Multiply((a, b), s) => (s, b, a.number(old) * b.number(old)),
        };
        println!("{} {} to {}", s, b.number(old), result);
        result
    }
}

impl Monkey {
    fn inspect_item(self: &mut Self) -> Option<ItemWorryLevel> {
        if self.items.len() == 0 {
            println!("Monkey {} has no item to inspect", self.index);
            return None;
        }
        let mut item = self.items.pop_front().expect("pop_front failed");
        println!(
            "Monkey {} inspects an item with a worry level of === {} ===",
            self.index, item
        );
        self.inspections += 1;

        let mut new = self.operation.apply(item);

        // println!("{}", format!(&message, item));

        // round
        // new = (new as f64 / 3 as f64).round() as ItemWorryLevel;
        new = new / 3;
        println!(
            "  Monkey gets bored with item. Worry level is divided by 3 to {}.",
            new
        );

        Some(new)
    }

    fn decide_throw(self: &Self, item: &ItemWorryLevel) -> MonkeyIndex {
        if item % self.throw_divisor.divisible == 0 {
            self.throw_divisor.true_recipient
        } else {
            self.throw_divisor.false_recipient
        }
    }
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((
        tag("old").map(|_| Value::Old),
        nom::character::complete::u32.map(|num| Value::Num(num)),
    ))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, (a, raw_operation, b)) = tuple((
        value,
        delimited(multispace1, alt((tag("*"), tag("+"))), multispace1),
        value,
    ))(input)?;

    let operation = match raw_operation {
        "*" => Operation::Multiply((a, b), String::from("  Worry level is multiplied by")),
        "+" => Operation::Add((a, b), String::from("  Worry level is increased by")),
        _ => panic!("Invalid input"),
    };

    Ok((input, operation))
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let (input, (_, _, divisor)) = tuple((
        multispace1,
        tag("Test: divisible by "),
        nom::character::complete::u32,
    ))(input)?;
    let (input, (_, _, true_recipient)) = tuple((
        multispace1,
        tag("If true: throw to monkey "),
        nom::character::complete::u32,
    ))(input)?;

    let (input, (_, _, false_recipient)) = tuple((
        multispace1,
        tag("If false: throw to monkey "),
        nom::character::complete::u32,
    ))(input)?;

    Ok((
        input,
        Test {
            divisible: divisor,
            true_recipient,
            false_recipient,
        },
    ))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, (_, index, _, _)) = tuple((
        tag("Monkey "),
        nom::character::complete::u32, // a verifier
        tag(":"),
        newline,
    ))(input)?;
    let (input, (_, _, items, _)) = tuple((
        multispace1,
        tag("Starting items: "),
        separated_list1(tag(", "), digit1),
        newline,
    ))(input)?;
    let (input, (_, _, operation)) =
        tuple((multispace1, tag("Operation: new = "), parse_operation))(input)?;

    let (input, test) = parse_test(input)?;

    let items: VecDeque<u32> = VecDeque::from(
        items
            .iter()
            .map(|i| i.parse::<u32>().expect("parse items error"))
            .collect::<VecDeque<u32>>(),
    );

    dbg!(input, index);
    let monkey = Monkey {
        index,
        items,
        operation,
        throw_divisor: test,
        inspections: 0,
    };

    Ok((input, monkey))
}

fn parse_data(input: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, monkeys) = separated_list1(tag("\n\n"), parse_monkey)(input)?;
    Ok((input, monkeys))
}

fn get_score(monkeys: Vec<Monkey>) -> u32 {
    let mut vec = monkeys.clone();

    vec.sort_unstable_by(|monkey1, monkey2| monkey2.inspections.cmp(&monkey1.inspections));

    vec[0].inspections * vec[1].inspections
}

pub fn part_one(input: &str) -> Option<u32> {
    let number_turns = 20;

    let mut monkeys = parse_data(input).unwrap().1;

    dbg!(&monkeys);

    for turn in 0..number_turns {
        println!("========== Turn {} ==========", turn);
        for monkey_index in 0..monkeys.len() {
            while let Some(item) = monkeys[monkey_index].inspect_item() {
                let receiver_index = monkeys[monkey_index].decide_throw(&item);
                monkeys[receiver_index as usize].items.push_back(item);
                println!(
                    "  Item with worry level {} is thrown to monkey {}",
                    item, receiver_index
                );
            }
        }
    }

    dbg!(&monkeys);

    Some(get_score(monkeys))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
