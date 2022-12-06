use itertools::Itertools;
use std::collections::{BTreeMap, VecDeque};

use regex::Regex;

const EXAMPLE: &str = include_str!("sample.txt");
const PUZZLE_INPUT: &str = include_str!("puzzle_input.txt");

#[derive(Debug, Clone)]
struct Crate(char);

impl Crate {
    fn val(&self) -> char {
        self.0
    }
}

#[derive(Debug)]
struct Move {
    pub number_of_crates: usize,
    pub source_index: usize,
    pub target_index: usize,
}

impl TryFrom<&str> for Move {
    type Error = Box<dyn std::error::Error>;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;

        let matches = re.captures(input).ok_or("Could not get any matches")?;
        Ok(Self {
            number_of_crates: matches
                .get(1)
                .ok_or("Missing item")?
                .as_str()
                .parse::<usize>()?,
            source_index: matches
                .get(2)
                .ok_or("Missing item")?
                .as_str()
                .parse::<usize>()?,
            target_index: matches
                .get(3)
                .ok_or("Missing item")?
                .as_str()
                .parse::<usize>()?,
        })
    }
}

struct CrateMap(BTreeMap<usize, VecDeque<Crate>>);

impl CrateMap {
    fn new() -> Self {
        Self(BTreeMap::new())
    }
    fn move_crate(&mut self, m: Move) {
        let source_stack = self.0.get_mut(&m.source_index).unwrap();
        let mut moving_crates = VecDeque::new();

        for _ in 0..m.number_of_crates {
            if let Some(item) = source_stack.pop_back() {
                moving_crates.push_back(item);
            }
        }

        if let Some(target_stack) = self.0.get_mut(&m.target_index) {
            target_stack.append(&mut moving_crates);
        } else {
            self.0.insert(m.target_index, moving_crates);
        }
    }

    pub fn build_map(input: &str) -> CrateMap {
        let mut crate_map: CrateMap = CrateMap::new();
        for line in input.lines() {
            let mut index = 0;
            let mut count = 0;
            if !line.contains("[") {
                break;
            }

            for c in line.chars() {
                if count % 4 == 0 {
                    index += 1;
                }

                match c {
                    'A'..='Z' => {
                        if let Some(vec) = crate_map.0.get_mut(&index) {
                            vec.push_front(Crate(c));
                        } else {
                            let mut new_vec = VecDeque::new();
                            new_vec.push_front(Crate(c));
                            crate_map.0.insert(index, new_vec);
                        }
                    }
                    _ => count += 1,
                }
            }
        }
        crate_map
    }

    pub fn stack_tops(&self) -> Vec<(usize, Crate)> {
        let mut tops: Vec<(usize, Crate)> = Vec::new();

        for (k, v) in self.0.iter() {
            if let Some(v) = v.back() {
                tops.push((*k, v.clone()));
            }
        }

        tops
    }
}

pub fn find_top_of_crate_stacks(input: &str) -> String {
    let mut stack_map = CrateMap::build_map(input);
    let moves: Vec<Move> = input
        .lines()
        .filter_map(|line| {
            if let Ok(m) = Move::try_from(line) {
                return Some(m);
            }
            None
        })
        .collect();

    for m in moves {
        stack_map.move_crate(m);
    }

    let tops = stack_map.stack_tops();

    let mut stack_top = String::new();

    for (index, crate_name) in tops {
        stack_top.push(crate_name.val());
    }

    stack_top
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(find_top_of_crate_stacks(EXAMPLE), "CMZ".to_owned());
    }

    #[test]
    fn test_puzzle() {
        assert_eq!(
            find_top_of_crate_stacks(PUZZLE_INPUT),
            "TLFGBZHCN".to_owned()
        );
    }
}
