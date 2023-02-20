use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    IResult,
};

// #[derive(PartialEq, Debug)]
// enum Comparison {
//     Ordered,
//     UnOrdered,
//     Undetermined,
// }

// impl Ord for Comparison {
//     fn cmp(&self, other: &Self) -> Ordering {
//         match Comparison {
//             Comparison::
//         }
//     }
// }

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Val(u32),
    Cells(Vec<Cell>),
}

impl Cell {
    fn compare(self: &Self, b: &Cell) -> Ordering {
        match (self, b) {
            (Cell::Val(v_a), Cell::Val(v_b)) if v_a < v_b => Ordering::Less,
            (Cell::Val(v_a), Cell::Val(v_b)) if v_a == v_b => Ordering::Equal,
            (Cell::Val(v_a), Cell::Val(v_b)) if v_a > v_b => Ordering::Greater,
            (Cell::Val(_), Cell::Val(_)) => unreachable!(),
            (Cell::Cells(c_a), Cell::Cells(c_b)) => {
                // loop on B elements
                for (index_b, item_b) in c_b.iter().enumerate() {
                    // if a is shorter than b, then we have the good order
                    let Some(item_a) = c_a.get(index_b) else {
                        return Ordering::Less;
                    };

                    // return result of a and b comparison if there's a result
                    match item_a.compare(item_b) {
                        Ordering::Equal => continue,
                        any => return any,
                    };
                }

                // if a is longer than b, then we have not the good order
                if c_a.len() > c_b.len() {
                    return Ordering::Greater;
                }

                // no order was found
                Ordering::Equal
            }
            (Cell::Val(v_a), Cell::Cells(_)) => Cell::Cells(vec![Cell::Val(*v_a)]).compare(b),
            (Cell::Cells(_), Cell::Val(v_b)) => self.compare(&Cell::Cells(vec![Cell::Val(*v_b)])),
        }
    }
}

fn parse_plain_cell(input: &str) -> IResult<&str, Cell> {
    let (input, plain) = digit1(input)?;

    Ok((input, Cell::Val(plain.parse::<u32>().unwrap())))
}

fn parse_array_cell(input: &str) -> IResult<&str, Cell> {
    let (input, _) = tag("[")(input)?;

    let (input, cells) = separated_list0(tag(","), parse_item)(input)?;

    let (input, _) = tag("]")(input)?;

    Ok((input, Cell::Cells(cells)))
}

fn parse_item(input: &str) -> IResult<&str, Cell> {
    let (input, c) = alt((parse_array_cell, parse_plain_cell))(input)?;
    Ok((input, c))
}

fn parse_pairs(input: &str) -> IResult<&str, (Cell, Cell)> {
    let (input, (item_a, _, item_b)) = tuple((parse_item, tag("\n"), parse_item))(input)?;

    Ok((input, (item_a, item_b)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Cell, Cell)>> {
    separated_list1(tag("\n\n"), parse_pairs)(input)
}

fn parse_input_2(input: &str) -> IResult<&str, Vec<Cell>> {
    separated_list1(alt((tag("\n\n"), tag("\n"))), parse_item)(input)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (_, pairs) = parse_input(input).expect("parsing error");

    Some(
        pairs
            .iter()
            .enumerate()
            .filter_map(|(index, (pair_a, pair_b))| {
                if let Ordering::Less = pair_a.compare(&pair_b) {
                    Some(index + 1)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = format!("[[2]]\n[[6]]\n{}", input);
    let (_, mut items) = parse_input_2(&input[..]).expect("parsing error");

    items.sort_by(|a, b| a.compare(&b));

    let pos = items
        .iter()
        .clone()
        .enumerate()
        .filter(|&(_, item)| {
            item == &Cell::Cells(vec![Cell::Cells(vec![Cell::Val(2)])])
                || item == &Cell::Cells(vec![Cell::Cells(vec![Cell::Val(6)])])
        })
        .map(|(index, _)| index + 1)
        .product::<usize>();

    Some(pos)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }

    #[test]
    fn test_compare_cells() {
        let a = Cell::Val(2);
        let b = Cell::Val(5);

        assert_eq!(a.compare(&b), Ordering::Less);

        let a = Cell::Val(5);
        let b = Cell::Val(2);

        assert_eq!(a.compare(&b), Ordering::Greater);
    }

    #[test]
    fn test_compare_vecs() {
        let a = Cell::Cells(vec![
            Cell::Val(1),
            Cell::Val(1),
            Cell::Val(3),
            Cell::Val(1),
            Cell::Val(1),
        ]);
        let b = Cell::Cells(vec![
            Cell::Val(1),
            Cell::Val(1),
            Cell::Val(5),
            Cell::Val(1),
            Cell::Val(1),
        ]);

        assert_eq!(a.compare(&b), Ordering::Less);
        let a = Cell::Cells(vec![Cell::Val(1), Cell::Val(1)]);
        let b = Cell::Cells(vec![Cell::Val(1), Cell::Val(1)]);

        assert_eq!(a.compare(&b), Ordering::Equal);

        let a = Cell::Cells(vec![Cell::Val(1), Cell::Val(2)]);
        let b = Cell::Cells(vec![Cell::Val(2), Cell::Val(3)]);

        assert_eq!(a.compare(&b), Ordering::Less);

        let a = Cell::Cells(vec![Cell::Val(5), Cell::Val(2)]);
        let b = Cell::Cells(vec![Cell::Val(2), Cell::Val(3)]);

        assert_eq!(a.compare(&b), Ordering::Greater);

        let a = Cell::Cells(vec![Cell::Val(1)]);
        let b = Cell::Cells(vec![Cell::Val(2), Cell::Val(3)]);

        assert_eq!(a.compare(&b), Ordering::Less);

        let a = Cell::Cells(vec![Cell::Val(1), Cell::Val(2)]);
        let b = Cell::Cells(vec![Cell::Val(2)]);

        assert_eq!(a.compare(&b), Ordering::Less);
    }

    #[test]

    fn test_compare_cell_vec() {
        let a = Cell::Cells(vec![
            Cell::Cells(vec![Cell::Val(1)]),
            Cell::Cells(vec![Cell::Val(2), Cell::Val(3), Cell::Val(4)]),
        ]);
        let b = Cell::Cells(vec![Cell::Cells(vec![Cell::Val(1)]), Cell::Val(4)]);

        assert_eq!(a.compare(&b), Ordering::Less);

        let a = Cell::Val(1);
        let b = Cell::Cells(vec![Cell::Val(2)]);

        assert_eq!(a.compare(&b), Ordering::Less);

        let a = Cell::Cells(vec![Cell::Val(1)]);
        let b = Cell::Val(2);

        assert_eq!(a.compare(&b), Ordering::Less);

        let a = Cell::Cells(vec![Cell::Val(3)]);
        let b = Cell::Val(2);

        assert_eq!(a.compare(&b), Ordering::Greater);

        let a = Cell::Val(1);
        let b = Cell::Cells(vec![Cell::Val(2), Cell::Val(3)]);

        assert_eq!(a.compare(&b), Ordering::Less);

        let a = Cell::Cells(vec![Cell::Val(1), Cell::Val(2)]);
        let b = Cell::Val(2);

        assert_eq!(a.compare(&b), Ordering::Less);
    }
}
