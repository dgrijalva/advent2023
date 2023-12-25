use itertools::Itertools;

use super::Puzzle;

pub struct Day15;

enum Instruction {
    Insert(u8),
    Remove,
}

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

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let steps = parse_instructions(input);
        let mut boxen: Vec<Vec<(String, usize)>> = Vec::from_iter((0..255).map(|_| vec![]));

        // Process the instructions
        for (id, box_idx, instruction) in steps {
            match instruction {
                Instruction::Remove => boxen[box_idx].retain(|v| v.0 != id),
                Instruction::Insert(val) => {
                    let found = boxen[box_idx]
                        .iter()
                        .enumerate()
                        .find(|(_, (i, _))| i == &id)
                        .map(|(idx, _)| idx);
                    if let Some(found) = found {
                        boxen[box_idx][found] = (id, val as usize);
                    } else {
                        boxen[box_idx].push((id, val as usize));
                    }
                }
            }
        }

        // Compute the power
        // For each box
        let power = boxen
            .into_iter()
            .enumerate()
            .map(|(box_idx, boxx)| {
                // For each lens
                boxx.into_iter()
                    .enumerate()
                    .map(|(lens_idx, (_, lens_power))| (box_idx + 1) * (lens_idx + 1) * lens_power)
                    .sum::<usize>()
            })
            .sum::<usize>();

        Ok(power.to_string())
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

fn parse_instructions(input: &str) -> Vec<(String, usize, Instruction)> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|i| {
            if i.contains('-') {
                let id = i.split('-').next().unwrap().to_string();
                (id.clone(), hash(&id), Instruction::Remove)
            } else {
                let parts = i.split('=').collect_vec();
                (
                    parts[0].to_string(),
                    hash(parts[0]),
                    Instruction::Insert(parts[1].parse().unwrap()),
                )
            }
        })
        .collect_vec()
}
