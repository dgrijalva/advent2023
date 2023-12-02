//! This is a template for a puzzle solution.  Copy this file to a new file.
//! Files in this folder are auto-discovered at build time.

use super::Puzzle;
use nom::IResult;

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
        let data = Self::parse_input(input);
        println!("{:?}", data);

        todo!("implement part one")
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}

impl Day02 {
    fn parse_input(mut input: &str) -> Vec<Game> {
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
}
