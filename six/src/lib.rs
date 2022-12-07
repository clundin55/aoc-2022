use std::collections::{HashSet, VecDeque};

const PUZZLE_INPUT: &str = include_str!("puzzle_input.txt");

fn protocol_parser(input: &str, distinct_char_count: usize) -> usize {
    let mut char_queue = VecDeque::new();

    for (idx, c) in input.chars().enumerate() {
        char_queue.push_back(c);

        if (idx + 1) >= distinct_char_count {
            if char_queue.iter().collect::<HashSet<&char>>().len() == distinct_char_count {
                return (idx + 1) as usize;
            }

            char_queue.pop_front();
        }
    }

    0
}

pub fn elf_protocol_start_index(input: &str) -> usize {
    protocol_parser(input, 4)
}

pub fn elf_message_start_index(input: &str) -> usize {
    protocol_parser(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        assert_eq!(
            elf_protocol_start_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            7
        );
    }

    #[test]
    fn test_sample2() {
        assert_eq!(elf_protocol_start_index("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }

    #[test]
    fn test_sample3() {
        assert_eq!(elf_protocol_start_index("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn test_sample4() {
        assert_eq!(
            elf_protocol_start_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            10
        );
    }

    #[test]
    fn test_sample5() {
        assert_eq!(
            elf_protocol_start_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            11
        );
    }

    #[test]
    fn test_puzzle() {
        assert_eq!(elf_protocol_start_index(PUZZLE_INPUT), 1544);
    }

    #[test]
    fn test_sample1_part_two() {
        assert_eq!(
            elf_message_start_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            19
        );
    }

    #[test]
    fn test_sample2_part_two() {
        assert_eq!(elf_message_start_index("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }

    #[test]
    fn test_sample3_part_two() {
        assert_eq!(elf_message_start_index("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }

    #[test]
    fn test_sample4_part_two() {
        assert_eq!(
            elf_message_start_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            29
        );
    }

    #[test]
    fn test_sample5_part_two() {
        assert_eq!(
            elf_message_start_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            26
        );
    }

    #[test]
    fn test_puzzle_part_two() {
        assert_eq!(elf_message_start_index(PUZZLE_INPUT), 2145);
    }
}
