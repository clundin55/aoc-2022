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

    fn is_overlapping(&self, other: &Self) -> bool {
        self.lower <= other.upper && other.lower <= self.upper
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

fn create_assignment_list(input: &str) -> Vec<(ElfAssignment, ElfAssignment)> {
    let assignments = input.lines().map(|line| {
        let line = line.split_once(",").expect("Could not split assignments.");
        let elf1 = ElfAssignment::from(line.0);
        let elf2 = ElfAssignment::from(line.1);

        (elf1, elf2)
    });

    assignments.collect()
}

pub fn redudant_assignments(input: &str) -> u32 {
    let assignments = create_assignment_list(input);

    let redudant_pairs = assignments.iter().filter(|pair| {
        let elf1 = &pair.0;
        let elf2 = &pair.1;

        elf1.is_subset_or_superset(&elf2)
    });

    redudant_pairs.count() as u32
}

pub fn overlapping_assignments(input: &str) -> u32 {
    let assignments = create_assignment_list(input);

    let redudant_pairs = assignments.iter().filter(|pair| {
        let elf1 = &pair.0;
        let elf2 = &pair.1;

        elf1.is_overlapping(&elf2)
    });

    redudant_pairs.count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(redudant_assignments(EXAMPLE), 2);
    }

    #[test]
    fn test_puzzle() {
        assert_eq!(redudant_assignments(PUZZLE_INPUT), 536);
    }

    #[test]
    fn test_sample_part_two() {
        assert_eq!(overlapping_assignments(EXAMPLE), 4);
    }

    #[test]
    fn test_puzzle_part_two() {
        assert_eq!(overlapping_assignments(PUZZLE_INPUT), 2);
    }
}
