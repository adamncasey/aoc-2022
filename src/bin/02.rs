#[derive(Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Result {
    Lose,
    Draw,
    Win,
}

impl Move {
    fn read(input: &str) -> Move {
        match input {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Bad input {input}"),
        }
    }

    fn value(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn match_score(&self, other: &Move) -> u32 {
        match self {
            Move::Rock => match other {
                Move::Rock => 3,
                Move::Paper => 0,
                Move::Scissors => 6,
            },
            Move::Paper => match other {
                Move::Rock => 6,
                Move::Paper => 3,
                Move::Scissors => 0,
            },
            Move::Scissors => match other {
                Move::Rock => 0,
                Move::Paper => 6,
                Move::Scissors => 3,
            },
        }
    }
}

impl Result {
    fn read(input: &str) -> Result {
        match input {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Bad input {input}"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut plays: Vec<(Move, Move)> = Vec::new();
    for game in input.split("\n") {
        let mut moves = game.split(" ").map(Move::read);

        plays.push((moves.next().unwrap(), moves.next().unwrap()));
    }

    let mut score: u32 = 0;
    for play in plays {
        let round_score = play.1.value() + play.1.match_score(&play.0);
        score += round_score;
    }

    Some(score)
}

fn correct_play(play: Move, result: Result) -> Move {
    match play {
        Move::Rock => match result {
            Result::Win => Move::Paper,
            Result::Draw => Move::Rock,
            Result::Lose => Move::Scissors,
        },
        Move::Paper => match result {
            Result::Draw => Move::Paper,
            Result::Lose => Move::Rock,
            Result::Win => Move::Scissors,
        },
        Move::Scissors => match result {
            Result::Lose => Move::Paper,
            Result::Win => Move::Rock,
            Result::Draw => Move::Scissors,
        },
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut plays: Vec<(Move, Result)> = Vec::new();
    for game in input.split("\n") {
        let mut moves = game.split(" ");

        plays.push((
            Move::read(moves.next().unwrap()),
            Result::read(moves.next().unwrap()),
        ));
    }

    let mut score = 0;
    for play in plays {
        let our_move = correct_play(play.0, play.1);

        let round_score = our_move.value() + our_move.match_score(&play.0);

        score += round_score;
    }

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rps_match_score() {
        assert_eq!(Move::Rock.match_score(&Move::Paper), 0);
        assert_eq!(Move::Paper.match_score(&Move::Rock), 6);
    }
}
