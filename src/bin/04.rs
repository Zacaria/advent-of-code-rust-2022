type Sections = Vec<u32>;

struct Pair {
    first: Sections,
    second: Sections,
}

impl Pair {
    fn are_fully_overlapping(&self) -> bool {
        (self.first[0] >= self.second[0] && self.first[1] <= self.second[1])
            || (self.second[0] >= self.first[0] && self.second[1] <= self.first[1])
    }

    fn are_overlapping(&self) -> bool {
        self.first[0] <= self.second[1] && self.second[0] <= self.first[1]
    }
}

fn parse_pairs(line: &str) -> Pair {
    let (left, right) = line.split_once(",").unwrap();
    let ((left_start, left_end), (right_start, right_end)) = (
        left.split_once("-").unwrap(),
        right.split_once("-").unwrap(),
    );
    Pair {
        first: vec![
            left_start.parse::<u32>().unwrap(),
            left_end.parse::<u32>().unwrap(),
        ],
        second: vec![
            right_start.parse::<u32>().unwrap(),
            right_end.parse::<u32>().unwrap(),
        ],
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(parse_pairs)
        .filter(|pairs| pairs.are_fully_overlapping())
        .collect::<Vec<Pair>>()
        .len();

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(parse_pairs)
        .filter(|pairs| pairs.are_overlapping())
        .collect::<Vec<Pair>>()
        .len();

    Some(res as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
