use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    IResult,
};

enum Comparison {
    Ordered,
    UnOrdered,
    Undetermined,
}

#[derive(Debug)]
enum Cell {
    Val(u32),
    Cells(Vec<Cell>),
}

impl Cell {
    fn compare(self: &Self, b: &Cell) -> Option<bool> {
        match (self, b) {
            (Cell::Val(v_a), Cell::Val(v_b)) if v_a < v_b => Some(true),
            (Cell::Val(v_a), Cell::Val(v_b)) if v_a == v_b => None,
            (Cell::Val(v_a), Cell::Val(v_b)) if v_a > v_b => Some(false),
            (Cell::Val(_), Cell::Val(_)) => unreachable!(),
            (Cell::Cells(c_a), Cell::Cells(c_b)) => {
                // loop on B elements
                for (index_b, item_b) in c_b.iter().enumerate() {
                    // if a is shorter than b, then we have the good order
                    let Some(item_a) = c_a.get(index_b) else {
                        return Some(true);
                    };

                    // return result of a and b comparison if there's a result
                    match item_a.compare(item_b) {
                        Some(comp) => return Some(comp),
                        None => continue,
                    };
                }

                // if a is longer than b, then we have not the good order
                if c_a.len() > c_b.len() {
                    return Some(false);
                }

                // no order was found
                None
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

pub fn part_one(input: &str) -> Option<usize> {
    let (_, pairs) = parse_input(input).expect("parsing error");

    Some(
        pairs
            .iter()
            .enumerate()
            .filter_map(|(index, (pair_a, pair_b))| {
                if pair_a.compare(&pair_b).unwrap_or(true) {
                    Some(index + 1)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_compare_cells() {
        let a = Cell::Val(2);
        let b = Cell::Val(5);

        assert_eq!(a.compare(&b).unwrap(), true);

        let a = Cell::Val(5);
        let b = Cell::Val(2);

        assert_eq!(a.compare(&b).unwrap(), false);
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

        assert_eq!(a.compare(&b).unwrap(), true);

        let a = Cell::Cells(vec![Cell::Val(1), Cell::Val(2)]);
        let b = Cell::Cells(vec![Cell::Val(2), Cell::Val(3)]);

        assert_eq!(a.compare(&b).unwrap(), true);

        let a = Cell::Cells(vec![Cell::Val(5), Cell::Val(2)]);
        let b = Cell::Cells(vec![Cell::Val(2), Cell::Val(3)]);

        assert_eq!(a.compare(&b).unwrap(), false);

        let a = Cell::Cells(vec![Cell::Val(1)]);
        let b = Cell::Cells(vec![Cell::Val(2), Cell::Val(3)]);

        assert_eq!(a.compare(&b).unwrap(), true);

        let a = Cell::Cells(vec![Cell::Val(1), Cell::Val(2)]);
        let b = Cell::Cells(vec![Cell::Val(2)]);

        assert_eq!(a.compare(&b).unwrap(), true);
    }

    #[test]

    fn test_compare_cell_vec() {
        let a = Cell::Cells(vec![
            Cell::Cells(vec![Cell::Val(1)]),
            Cell::Cells(vec![Cell::Val(2), Cell::Val(3), Cell::Val(4)]),
        ]);
        let b = Cell::Cells(vec![Cell::Cells(vec![Cell::Val(1)]), Cell::Val(4)]);

        assert_eq!(a.compare(&b).unwrap(), true);

        let a = Cell::Val(1);
        let b = Cell::Cells(vec![Cell::Val(2)]);

        assert_eq!(a.compare(&b).unwrap(), true);

        let a = Cell::Cells(vec![Cell::Val(1)]);
        let b = Cell::Val(2);

        assert_eq!(a.compare(&b).unwrap(), true);

        let a = Cell::Cells(vec![Cell::Val(3)]);
        let b = Cell::Val(2);

        assert_eq!(a.compare(&b).unwrap(), false);

        let a = Cell::Val(1);
        let b = Cell::Cells(vec![Cell::Val(2), Cell::Val(3)]);

        assert_eq!(a.compare(&b).unwrap(), true);

        let a = Cell::Cells(vec![Cell::Val(1), Cell::Val(2)]);
        let b = Cell::Val(2);

        assert_eq!(a.compare(&b).unwrap(), true);
    }
}
