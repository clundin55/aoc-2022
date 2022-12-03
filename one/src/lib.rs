const EXAMPLE: &str = include_str!("sample.txt");
const PUZZLE_INPUT: &str = include_str!("puzzle_input.txt");

pub fn elf_with_most_calories(input: &str) -> u32 {
    let calories_by_elf = input.split("\n\n").map(|elf: &str| {
        elf.lines()
            .map(|calories: &str| calories.parse::<u32>())
            .filter_map(|calories| calories.ok())
            .sum::<u32>()
    });

    calories_by_elf.max().expect("Unable to find the max calories of an elf")
}

pub fn sum_of_three_elves_with_most_calories(input: &str) -> u32 {
    let calories_by_elf = input.split("\n\n").map(|elf: &str| {
        elf.lines()
            .map(|calories: &str| calories.parse::<u32>())
            .filter_map(|calories| calories.ok())
            .sum::<u32>()
    });

    let mut sorted_calories = calories_by_elf.collect::<Vec<u32>>();
    sorted_calories.sort();
    sorted_calories.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(elf_with_most_calories(EXAMPLE), 24000);
    }

    #[test]
    fn test_sample_part_two() {
        assert_eq!(sum_of_three_elves_with_most_calories(EXAMPLE), 45000);
    }

    #[test]
    fn test_puzzle() {
        assert_eq!(elf_with_most_calories(PUZZLE_INPUT), 72511);
    }

    #[test]
    fn test_puzzle_part_two() {
        assert_eq!(sum_of_three_elves_with_most_calories(PUZZLE_INPUT), 72511);
    }
}
