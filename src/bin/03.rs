fn find_common_char(line: &str) -> u8 {
    let first_half: Vec<char> = line
        .chars()
        .into_iter()
        .take(line.len()/2)
        .collect();
    let second_half: Vec<char> = line
        .chars()
        .into_iter()
        .skip(line.len()/2)
        .collect();

    // println!("Split line {:?} {:?}", first_half, second_half);

    let res = first_half
        .into_iter()
        .find(|&c| second_half.contains(&c));

    // println!("unique letter found {}, corresponding number {}", res.unwrap(), res.unwrap() as u32);

    res.expect("common letter not found") as u8
}

fn reduce_common_char <'a> (lines: &'a &Vec<&str>) -> Box<dyn Fn(Option<char>, char) -> Option<char> + 'a> {
    Box::new(move |current_common, char_to_test| {
        
        match current_common {
            Some(maybe_common) => {
                if lines.iter().all(|line| line.contains(char_to_test)) {
                    Some(char_to_test)
                } else {
                    Some(maybe_common)
                }
            },
            None => Some(char_to_test)
        }
    })
}

fn find_common_char_in_lines (lines: &Vec<&str>) -> u8 {

    // println!("{:?}", lines);
   
    let common_char = lines
        .iter()
        .flat_map(|s| s.chars())
        .fold(None, reduce_common_char(&lines))
        .expect("common char not found");

    // println!("common char found {}", common_char);
    common_char as u8   
}

fn to_value (letter: u8) -> u32 {
    let res = if letter > 96 { letter - 96 } else { letter - 38 };

    // println!("receive char {} value {}, return {}", letter as char, letter, res);

    res as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(find_common_char)
        .map(to_value)
        .map(|x| x as u32)
        .sum::<u32>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res: u32 = input
        .lines()
        .enumerate()
        .fold((Vec::new(), 0), |(mut chunks, chunk_size), (_, line)| {
            if chunk_size == 0 {
                chunks.push(vec![line]);
            } else {
                chunks.last_mut().unwrap().push(line);
            }
            (chunks, (chunk_size + 1) % 3)
        })
        .0
        .iter()
        .map(|bunch_of_lines| find_common_char_in_lines(&bunch_of_lines))
        .map(to_value)
        .sum::<u32>();

    Some(res)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }

    #[test]
    fn test_to_value() {
        assert_eq!(to_value('a' as u8), 1);
        assert_eq!(to_value('z' as u8), 26);
        assert_eq!(to_value('A' as u8), 27);
        assert_eq!(to_value('Z' as u8), 52);
    }
}
