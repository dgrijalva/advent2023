use super::Puzzle;

pub struct Day15;

impl Puzzle for Day15 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let result = input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|s| hash(s))
            .inspect(|h| println!("{h}"))
            .sum::<usize>();
        Ok(result.to_string())
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}

fn hash(input: &str) -> usize {
    let mut hash = 0usize;
    for b in input.as_bytes().iter().map(|b| *b as usize) {
        hash += b;
        hash = hash * 17;
        hash = hash % 256;
    }
    return hash;
}
