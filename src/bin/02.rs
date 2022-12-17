#[derive(PartialEq)]
enum Shape { Rock, Paper, Scissors }

#[derive(PartialEq)]
struct Sign {
    shape: Shape,
    wins: Shape,
    loses: Shape,
    value: u32
}

fn shape_to_sign (shape: Shape) -> Sign{
    let rock = Sign { shape: Shape::Rock, wins: Shape::Scissors, loses: Shape::Paper, value: 1 };
    let paper = Sign { shape: Shape::Paper, wins: Shape::Rock, loses: Shape::Scissors, value: 2 };
    let scissors = Sign { shape: Shape::Scissors, wins: Shape::Paper, loses: Shape::Rock, value: 3 };

    match shape {
        Shape::Rock => rock,
        Shape::Paper => paper,
        Shape::Scissors => scissors
    } 
}

fn letter_to_sign (letter: char) -> Option<Sign> {
    match letter {
        'A' | 'X' => Some(shape_to_sign(Shape::Rock)),
        'B' | 'Y' => Some(shape_to_sign(Shape::Paper)),
        'C' | 'Z' => Some(shape_to_sign(Shape::Scissors)),
        _ => None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    fn process_turn(turn: &str) -> u32 {
        let op_play = letter_to_sign(turn.chars().next().unwrap());
        let my_play = letter_to_sign(turn.chars().last().unwrap());
    
        let play = letter_to_sign(turn.chars().last().unwrap()).unwrap().value;
    
        let outcome = match (op_play.unwrap(), my_play.unwrap()) {
            (op_play, my_play) if(op_play == my_play) => 3,
            (op_play, my_play) if(op_play.shape == my_play.wins) => 6,
            (op_play, my_play) if(op_play.wins == my_play.shape) => 0,
            _ => 0
        };
    
        play + outcome
    }

    let result = input.lines()
        .map(process_turn)
        .sum::<u32>();

    Some(result)
}

fn second_col_to_val (xyz: Option<char>) -> u32 {
    match xyz {
        Some('X') => 0,
        Some('Y') => 3,
        Some('Z') => 6,
        _ => 0
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    fn process_turn(turn: &str) -> u32 {
        let op_play = letter_to_sign(turn.chars().next().unwrap()).unwrap();

        let outcome = match turn.chars().last() {
            Some('X') => Some(shape_to_sign(op_play.wins)),
            Some('Y') => Some(shape_to_sign(op_play.shape)),
            Some('Z') => Some(shape_to_sign(op_play.loses)),
            _ => None
        };

        outcome.unwrap().value + second_col_to_val(turn.chars().last())
    }

    let result = input.lines()
        .map(process_turn)
        .sum::<u32>();

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
