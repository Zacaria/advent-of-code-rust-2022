use std::collections::HashSet;

#[allow(dead_code)]
fn index_of_window_without_duplicate_imperative(input: &str, window_size: u32) -> Option<u32> {
    for index in 0..(input.len() - window_size as usize) {
        let mut seen = HashSet::new();

        for character in (&input[index..index + window_size as usize]).as_bytes() {
            if !seen.insert(character) {
                break;
            }
        }
        if seen.len() == window_size as usize {
            return Some(index as u32 + window_size);
        }
    }

    None
}

// 10x faster than imperative
// maybe because of separate hashmap lookup
fn index_of_window_without_duplicate_functionnal(input: &str, window_size: u32) -> Option<u32> {
    Some(
        input
            .as_bytes()
            .windows(window_size as usize)
            // thats a double for loop
            .position(|b| {
                !(0..window_size - 1)
                    .any(|i| (i + 1..window_size).any(|j| b[i as usize] == b[j as usize]))
            })
            .unwrap() as u32
            + window_size,
    )
}

// 250x faster than imperative
// only works for window smaller than 32

// bitwise OR sets the bit in the seen lookup
// the bitwise shift to 1 creates a binary number with only one 1
// bitwise AND detects if the 1 in the mask has the same position in the seen lookup
// lets us conclude that the bit has already been set by an earlier bitwise OR
fn index_of_window_without_duplicate_bitwise_mask(input: &str, window_size: usize) -> Option<u32> {
    let data = input.as_bytes();

    let mut window_start_index = 0;
    'main: loop {
        // holds seen characters
        // 0b00000000_00000000_00000000_00000000
        let mut seen = 0u32;
        // reverse to find most far right duplicate in the window
        // so we can do window_start_index += index_in_window + 1; earlier
        for index_in_window in (0..window_size).rev() {
            // assuming input is only lowercase char, subtracting by 'a' which the number 97, gives a result between 0 an 25
            // 1 << 'b' - 97 -> 1 << 98 - 97 -> 1 << 1 -> 0b00000000_00000000_00000000_00000010 -> visually shift of one position to the left (<<)
            let mask = 1 << data[window_start_index + index_in_window] - b'a';

            // seen 0b00000000_00000000_00000000_01010010 AND mask 0b00000000_00000000_00000000_00000010 -> 0b00000000_00000000_00000000_00000010
            // bitwise AND with a mask equals to mask if the shifted 1 is also present in the seen variable
            if seen & mask == mask {
                // found a duplicate
                // shift the current character index to the right most duplicate
                window_start_index += index_in_window + 1;
                continue 'main;
            }
            // bitwise OR to keep seen chars
            // sets the bit to 1 even if its already seen
            seen |= mask;
        }
        break;
    }

    Some(window_size as u32 + window_start_index as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    // index_of_window_without_duplicate_imperative(input, 4)
    index_of_window_without_duplicate_functionnal(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    // index_of_window_without_duplicate_imperative(input, 14)
    // index_of_window_without_duplicate_functionnal(input, 14)
    index_of_window_without_duplicate_bitwise_mask(input, 14)
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
        assert_eq!(part_two(&input), Some(19));
    }
}
