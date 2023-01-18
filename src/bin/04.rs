type Sections = Vec<u32>;

// impl Sections {
//     fn new(start: u32, end: u32) {}
// }

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
    let split = line
        .split(",")
        .map(|section| -> Sections {
            let mut split_section = section.split("-");
            vec![
                split_section.next().unwrap().parse::<u32>().unwrap(),
                split_section.next().unwrap().parse::<u32>().unwrap(),
            ]
        })
        .collect::<Vec<Sections>>();
    Pair {
        first: split[0].clone(),
        second: split[1].clone(),
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
