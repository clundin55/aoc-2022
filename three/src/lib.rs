const EXAMPLE: &str = include_str!("sample.txt");
const PUZZLE_INPUT: &str = include_str!("puzzle_input.txt");

use std::collections::HashSet;

fn char_to_point(c: &char) -> u32 {
    match c {
        'a'..='z' => (*c as u32 - 'a' as u32) + 1,
        'A'..='Z' => (*c as u32 - 'A' as u32) + 27,
        _ => unreachable!(),
    }
}

pub fn sum_of_duplicate_items(packing_list: &str) -> u32 {
    let duplicate_items = packing_list.lines().filter_map(|line| {
        let half = line.len() / 2;

        let mut first_rucksack = HashSet::new();
        let mut second_rucksack = HashSet::new();

        for (idx, val) in line.chars().enumerate() {
            if idx < half {
                first_rucksack.insert(val);
            } else {
                second_rucksack.insert(val);
            }
        }

        let intersection = first_rucksack.intersection(&second_rucksack);
        intersection.into_iter().nth(0).cloned()
    });
    let sum_of_duplicate_items = duplicate_items.map(|item| char_to_point(&item)).sum();

    sum_of_duplicate_items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(sum_of_duplicate_items(EXAMPLE), 157);
    }

    #[test]
    fn test_puzzle() {
        assert_eq!(sum_of_duplicate_items(PUZZLE_INPUT), 7727);
    }
}
