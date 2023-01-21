use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    for index in 0..(input.len() - 4) {
        let mut seen: HashSet<char> = HashSet::new();
        // println!("{:?}", (&input[index..index + 4]).chars());

        for character in (&input[index..index + 4]).chars() {
            if !seen.insert(character) {
                break;
            }
        }
        if seen.len() == 4 {
            return Some(index as u32 + 4);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
