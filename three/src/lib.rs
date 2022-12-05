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

    duplicate_items.map(|item| char_to_point(&item)).sum()
}

pub fn sum_of_badge_items(packing_list: &str, group_size: usize) -> u32 {
    let mut duplicate_items = Vec::new();
    let mut packing_list_items = packing_list.lines().peekable();

    while packing_list_items.peek().is_some() {
        let packed_items: Vec<HashSet<char>> = packing_list_items
            .by_ref()
            .take(group_size)
            .map(|line| {
                let h: HashSet<char> = line.chars().collect();
                h
            })
            .collect();

        let last_rucksack = packed_items.last().unwrap();
        let badge = last_rucksack.iter().find_map(|c| {
            if packed_items.iter().all(|s| s.contains(&c)) {
                return Some(c);
            }
            None
        });

        if let Some(badge) = badge {
            duplicate_items.push(char_to_point(badge));
        }
    }

    duplicate_items.into_iter().sum()
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

    #[test]
    fn test_sample_part_two() {
        assert_eq!(sum_of_badge_items(EXAMPLE, 3), 70);
    }

    #[test]
    fn test_puzzle_part_two() {
        assert_eq!(sum_of_badge_items(PUZZLE_INPUT, 3), 2609);
    }
}
