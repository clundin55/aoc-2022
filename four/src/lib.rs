const EXAMPLE: &str = include_str!("sample.txt");
const PUZZLE_INPUT: &str = include_str!("puzzle_input.txt");

#[derive(Debug)]
struct ElfAssignment {
    upper: u32,
    lower: u32,
}

impl ElfAssignment {
    fn is_subset_or_superset(&self, other: &Self) -> bool {
        if self.lower <= other.lower && self.upper >= other.upper {
            return true;
        }

        if self.lower >= other.lower && self.upper <= other.upper {
            return true;
        }

        false
    }
}

impl From<&str> for ElfAssignment {
    fn from(input: &str) -> Self {
        let input = input
            .split_once("-")
            .expect("There should be a \"-\" seperating each assignment.");

        let lower = input
            .0
            .parse::<u32>()
            .expect("Unable to parse the upper bound to a u32.");
        let upper = input
            .1
            .parse::<u32>()
            .expect("Unable to parse the upper bound to a u32.");

        ElfAssignment { upper, lower }
    }
}

pub fn overlapping_assignments(input: &str) -> u32 {
    let assignments = input.lines().filter_map(|line| {
        let line = line.split_once(",").expect("Could not split assignments.");
        let elf1 = ElfAssignment::from(line.0);
        let elf2 = ElfAssignment::from(line.1);

        if elf1.is_subset_or_superset(&elf2) {
            return Some(elf1);
        }

        None
    });

    assignments.count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(overlapping_assignments(EXAMPLE), 2);
    }

    #[test]
    fn test_puzzle() {
        assert_eq!(overlapping_assignments(PUZZLE_INPUT), 2);
    }
}
