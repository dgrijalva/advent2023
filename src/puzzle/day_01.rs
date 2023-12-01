use super::Puzzle;
use itertools::Itertools;

pub struct Day01;

impl Puzzle for Day01 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        Ok(input
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| {
                let numbers = line.chars().filter_map(|c| c.to_digit(10)).collect_vec();
                let value = (numbers.first().unwrap() * 10) + numbers.last().unwrap();
                // println!("NUMBERS: {line} {value}");
                value
            })
            .sum::<u32>()
            .to_string())
    }

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let real_input = input
            .split("\n")
            .map(|line| Self::swap_digits(line))
            .join("\n");
        self.part_one(&real_input)
    }
}

impl Day01 {
    const NUMS: [&'static str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn swap_digits(input: &str) -> String {
        let data = Self::find_all_matches(input)
            .into_iter()
            .sorted_by_key(|(idx, _)| *idx)
            .map(|(_, val)| val.to_string())
            .join("");

        println!("SWAP: {} -> {}", input, data);
        data
    }

    /// Find all matching numbers and words in the input
    /// Return the index and the value of each match
    fn find_all_matches(input: &str) -> Vec<(usize, usize)> {
        Self::NUMS
            .iter()
            .enumerate()
            .flat_map(|(val, word)| {
                input
                    .match_indices(word)
                    .map(|(idx, _)| (idx, val))
                    .chain(
                        input
                            .match_indices(&val.to_string())
                            .map(|(idx, _)| (idx, val)),
                    )
                    .collect_vec()
            })
            .collect_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_swap_digits() {
        assert_eq!(Day01::swap_digits("one"), "1");
        assert_eq!(Day01::swap_digits("oneone"), "11");
        assert_eq!(Day01::swap_digits("oneoneone"), "111");
        assert_eq!(Day01::swap_digits("oneoneoneone"), "1111");
        assert_eq!(Day01::swap_digits("oneoneoneoneone"), "11111");
        assert_eq!(Day01::swap_digits("oneoneoneoneoneone"), "111111");
        assert_eq!(Day01::swap_digits("oneoneoneoneoneoneone"), "1111111");
        assert_eq!(Day01::swap_digits("oneoneoneoneoneoneoneone"), "11111111");
        assert_eq!(Day01::swap_digits("two"), "2");
        assert_eq!(Day01::swap_digits("three"), "3");
        assert_eq!(Day01::swap_digits("four"), "4");
        assert_eq!(Day01::swap_digits("five"), "5");
        assert_eq!(Day01::swap_digits("six"), "6");
        assert_eq!(Day01::swap_digits("seven"), "7");
        assert_eq!(Day01::swap_digits("eight"), "8");
        assert_eq!(Day01::swap_digits("nine"), "9");
        assert_eq!(Day01::swap_digits("eightwo"), "82");
        assert_eq!(Day01::swap_digits("twone"), "21");
        assert_eq!(Day01::swap_digits("sevenine"), "79");
        assert_eq!(Day01::swap_digits("fiveight"), "58");
    }

    #[test]
    fn test_sample_part_two() {
        let res = Day01
            .part_two(
                "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen",
            )
            .unwrap();
        assert_eq!(res, "281".to_string());
    }
}
