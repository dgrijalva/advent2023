//! This is a template for a puzzle solution.  Copy this file to a new file.
//! Files in this folder are auto-discovered at build time.

use super::Puzzle;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::str::FromStr;

pub struct Day07;

#[derive(Debug, Ord, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
    bid: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Joker,
    Number(u8), // 2-10
    Jack,
    Queen,
    King,
    Ace,
}

impl Puzzle for Day07 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let mut hands = input
            .lines()
            .map(|l| l.parse::<Hand>().unwrap())
            .collect::<Vec<_>>();
        hands.sort();

        let result = hands
            .into_iter()
            .enumerate()
            .inspect(|(i, h)| println!("{}: {:?} -> {}", i + 1, h, (i + 1) * h.bid))
            .map(|(i, h)| h.bid * (i + 1))
            .sum::<usize>();

        Ok(result.to_string())
    }

    // Correct answer: 251735672
    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let mut hands = input
            .replace('J', "X") // J is for Joker in part two
            .lines()
            .map(|l| l.parse::<Hand>().unwrap())
            .collect::<Vec<_>>();
        hands.sort();

        let result = hands
            .into_iter()
            .enumerate()
            .inspect(|(i, h)| println!("{}: {:?} -> {}", i + 1, h, (i + 1) * h.bid))
            .map(|(i, h)| h.bid * (i + 1))
            .sum::<usize>();

        Ok(result.to_string())
    }
}

impl Card {
    /// Assign numeric value, for sorting purposes
    fn value(&self) -> u8 {
        match self {
            Self::Number(n) => *n,
            Self::Jack => 11,
            Self::Queen => 12,
            Self::King => 13,
            Self::Ace => 14,
            Self::Joker => 1,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = self.hand_type.partial_cmp(&other.hand_type)?;
        if ord == Ordering::Equal {
            Some(self.cards.cmp(&other.cards))
        } else {
            Some(ord)
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl From<&[Card; 5]> for HandType {
    fn from(cards: &[Card; 5]) -> Self {
        let mut counts = [0; 15];
        for card in cards {
            counts[card.value() as usize] += 1;
        }
        let jokers = counts[1];
        let mut counts = counts[2..].to_vec();
        counts.sort();
        counts.reverse();

        match counts[0] + jokers {
            5 => Self::FiveOfKind,
            4 => Self::FourOfKind,
            3 => {
                if counts[1] == 2 {
                    Self::FullHouse
                } else {
                    Self::ThreeOfKind
                }
            }
            2 => {
                if counts[1] == 2 {
                    Self::TwoPair
                } else {
                    Self::Pair
                }
            }
            _ => Self::HighCard,
        }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let cards = parts.next().unwrap().chars().collect::<Vec<_>>();
        let bid = parts.next().unwrap().parse::<usize>()?;
        if cards.len() != 5 {
            panic!("hand must be 5 cards");
        }
        let cards = [
            cards[0].into(),
            cards[1].into(),
            cards[2].into(),
            cards[3].into(),
            cards[4].into(),
        ];
        let hand_type = HandType::from(&cards);

        Ok(Self {
            cards,
            bid,
            hand_type,
        })
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2'..='9' => Self::Number(c.to_digit(10).unwrap() as u8),
            'T' => Self::Number(10),
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            'X' => Self::Joker,
            _ => panic!("invalid card"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ranking() {
        assert!(Card::Number(2) > Card::Joker);
        assert!(Card::Jack > Card::Number(10));
        assert!(Card::Queen > Card::Jack);
        assert!(Card::Queen > Card::Joker);
        assert!([Card::Joker, Card::Queen] > [Card::Joker, Card::Joker]);
        assert!(
            [
                Card::Joker,
                Card::Joker,
                Card::Queen,
                Card::Queen,
                Card::Queen
            ] > [
                Card::Joker,
                Card::Joker,
                Card::Joker,
                Card::Joker,
                Card::Joker
            ]
        );
    }
}
