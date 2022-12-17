pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .split("\n\n")
        .map(|goblin|
            goblin
                .lines()
                .map(|nb| nb.parse::<u32>().unwrap())
                .sum::<u32>()
        )
        .max();

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut goblins_values = input
        .split("\n\n")
        .map(|goblin|
            goblin
                .lines()
                .map(|nb| nb.parse::<u32>().unwrap())
                .sum::<u32>()
        )
        .collect::<Vec<u32>>();
    
    goblins_values.sort_unstable();
    let result = goblins_values.into_iter().rev().take(3).sum::<u32>();
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
