use super::Puzzle;

pub struct Day02;

#[derive(Debug)]
struct Game {
    num: usize,
    rounds: Vec<Round>,
}

#[derive(Debug, Default)]
struct Round {
    red: usize,
    blue: usize,
    green: usize,
}

impl Puzzle for Day02 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let games = Self::parse_input(input);
        let score = games
            .into_iter()
            .filter(|g| {
                g.rounds
                    .iter()
                    .all(|r| r.red <= 12 && r.green <= 13 && r.blue <= 14)
            })
            .map(|g| g.num)
            .sum::<usize>();

        Ok(score.to_string())
    }

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let games = Self::parse_input(input);
        let score = games
            .iter()
            .map(Self::game_mins)
            .map(|r| r.power())
            .sum::<usize>();
        Ok(score.to_string())
    }
}

impl Day02 {
    fn parse_input(input: &str) -> Vec<Game> {
        let games = input
            .lines()
            .map(|line| {
                let line = line.strip_prefix("Game ").unwrap();
                let (line, game_num) = nom::character::complete::digit1::<&str, ()>(line).unwrap();
                let line = line.strip_prefix(": ").unwrap();
                Game {
                    num: game_num.parse().unwrap(),
                    rounds: line.split(';').map(|r| Self::parse_round(r)).collect(),
                }
            })
            .collect();

        games
    }

    fn parse_round(input: &str) -> Round {
        let mut round = Round::default();
        for term in input.split(", ") {
            let (term, num) = nom::character::complete::digit1::<&str, ()>(term.trim()).unwrap();
            match term.trim() {
                "red" => round.red = num.parse().unwrap(),
                "blue" => round.blue = num.parse().unwrap(),
                "green" => round.green = num.parse().unwrap(),
                c => panic!("unexpected color: {}", c),
            }
        }
        round
    }

    fn game_mins(game: &Game) -> Round {
        let mut min_round = Round::default();

        for round in &game.rounds {
            min_round.red = min_round.red.max(round.red);
            min_round.blue = min_round.blue.max(round.blue);
            min_round.green = min_round.green.max(round.green);
        }

        min_round
    }
}

impl Round {
    fn power(&self) -> usize {
        self.red * self.blue * self.green
    }
}
