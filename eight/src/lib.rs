use std::collections::HashSet;

const EXAMPLE: &str = include_str!("sample.txt");
const PUZZLE_INPUT: &str = include_str!("puzzle_input.txt");

pub type Coord = (usize, usize);

#[derive(Clone, Debug)]
pub struct Tree {
    coord: Coord,
    height: u32,
}

impl Tree {
    fn new(x: usize, y: usize, height: u32) -> Self {
        Self {
            coord: (x, y),
            height,
        }
    }

    fn x(&self) -> usize {
        self.coord.0
    }

    fn y(&self) -> usize {
        self.coord.1
    }

    fn height(&self) -> u32 {
        self.height
    }
}

pub fn check_if_tree_is_visible(tree: &Tree, trees: &Vec<Vec<u32>>) -> bool {
    let mut results = HashSet::new();

    results.insert({
        let mut result = true;
        for other_tree in trees[tree.x()].iter().skip(tree.y() + 1) {
            if tree.height() <= *other_tree {
                result = false;
            }
        }
        result
    });

    results.insert({
        let mut result = true;
        for other_tree in trees[tree.x()].iter().take(tree.y()) {
            if tree.height() <= *other_tree {
                result = false;
            }
        }
        result
    });

    results.insert({
        let mut result = true;
        for other_trees in trees.iter().take(tree.x()) {
            if tree.height() <= other_trees[tree.y()] {
                result = false;
            }
        }
        result
    });

    results.insert({
        let mut result = true;
        for other_trees in trees.iter().skip(tree.x() + 1) {
            if tree.height() <= other_trees[tree.y()] {

                result = false;
            }
        }
        result
    });

   results.contains(&true)
}

pub fn check_for_visible_trees(input: &str) -> usize {
    let trees: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|tree| tree.to_digit(10)).collect())
        .collect();

    let last_tree_index: usize = trees[0].len() - 1;
    let mut visible_trees = Vec::new();

    for (x, line_of_trees) in trees.iter().enumerate() {
        for (y, height) in line_of_trees.iter().enumerate() {
            let coord: Coord = (x, y);
            match coord {
                (x, y) if x == 0 || y == 0 || x == last_tree_index || y == last_tree_index => {
                    visible_trees.push(coord)
                }
                (x, y) => {
                    // Now iterate in each direction.
                    let tree = Tree::new(x, y, *height);
                    if check_if_tree_is_visible(&tree, &trees) {
                        visible_trees.push(coord);
                    }
                }
            }
        }
    }

    visible_trees.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(check_for_visible_trees(EXAMPLE), 21);
    }

    #[test]
    fn test_puzzle() {
        assert_eq!(check_for_visible_trees(PUZZLE_INPUT), 1843);
    }
}
