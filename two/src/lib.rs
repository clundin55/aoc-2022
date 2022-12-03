const EXAMPLE: &str = include_str!("sample.txt");
const PUZZLE_INPUT: &str = include_str!("puzzle_input.txt");

enum GameResult {
    Win,
    Loss,
    Draw,
}

impl GameResult {
    fn points(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

enum Moves {
    Rock,
    Paper,
    Scissors,
}

impl Moves {
    fn points(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn play(&self, opponent: &Self) -> GameResult {
        match (self, opponent) {
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => GameResult::Win,
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => GameResult::Draw,
            (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock)
            | (Self::Rock, Self::Paper) => GameResult::Loss,
        }
    }
}

impl From<&str> for Moves {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

pub fn follow_strategy(strategy: &str) -> u32 {
    let moves = strategy
        .lines()
        .map(|line: &str| {
            line.split_once(" ")
                .map(|(opponent, elf): (&str, &str)| (Moves::from(opponent), Moves::from(elf)))
        })
        .filter_map(|turn| turn);

    let total_points = moves.fold(0, |acc, turn| {
        let (opponent, elf) = turn;
        let move_points = Moves::points(&elf);
        let game = Moves::play(&elf, &opponent);
        let game_points = GameResult::points(&game);

        acc + move_points + game_points
    });

    total_points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(follow_strategy(EXAMPLE), 15);
    }

    #[test]
    fn test_puzzle() {
        assert_eq!(follow_strategy(PUZZLE_INPUT), 13675);
    }
}
