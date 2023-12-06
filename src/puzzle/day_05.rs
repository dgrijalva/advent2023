//! This is a template for a puzzle solution.  Copy this file to a new file.
//! Files in this folder are auto-discovered at build time.

use super::Puzzle;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::ops::Range;
use std::str::FromStr;

pub struct Day05;

#[derive(Debug)]
struct PuzzleInput {
    seeds: Vec<usize>,
    seed_ranges: Vec<Range<usize>>,
    seed_soil: Table,
    soil_fertilizer: Table,
    fertilizer_water: Table,
    water_light: Table,
    light_temp: Table,
    temp_humidity: Table,
    humidity_location: Table,
}

#[derive(Debug)]
struct Table {
    #[allow(dead_code)]
    name: String, // For debug logging
    mappings: Vec<Mapping>,
}

#[derive(Debug)]
struct Mapping {
    to: usize,
    from: usize,
    len: usize,
}

impl Puzzle for Day05 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, input: &str) -> super::PuzzleResult {
        let input = PuzzleInput::from_str(input)?;
        // println!("{:#?}", input);

        let mut locations = BTreeMap::new();
        for seed in &input.seeds {
            let location = input.lookup_location(*seed);
            locations.insert(location, seed);
        }

        Ok(locations.pop_first().unwrap().0.to_string())
    }

    fn part_two(&self, input: &str) -> super::PuzzleResult {
        let input = PuzzleInput::from_str(input)?;
        // println!("{:#?}", input);

        let mut locations = BTreeMap::new();
        for seed in input
            .seed_ranges
            .iter()
            .map(|r| r.clone().into_iter())
            .flatten()
        {
            let location = input.lookup_location(seed);
            locations.insert(location, seed);

            // Keep only the two lowest values
            while locations.len() > 2 {
                locations.pop_last();
            }
        }

        Ok(locations.pop_first().unwrap().0.to_string())
    }
}

impl PuzzleInput {
    fn lookup_location(&self, seed: usize) -> usize {
        let mut answer = self.seed_soil.lookup(seed);
        answer = self.soil_fertilizer.lookup(answer);
        answer = self.fertilizer_water.lookup(answer);
        answer = self.water_light.lookup(answer);
        answer = self.light_temp.lookup(answer);
        answer = self.temp_humidity.lookup(answer);
        self.humidity_location.lookup(answer)
    }

    fn parse_seed_ranges(vals: Vec<usize>) -> Vec<Range<usize>> {
        let mut vals = vals.into_iter();
        let mut ranges = Vec::new();
        loop {
            let Some(start) = vals.next() else {
                return ranges;
            };
            let len = vals.next().unwrap();
            ranges.push(start..(start + len));
        }
    }
}

impl Table {
    fn lookup(&self, input: usize) -> usize {
        self.mappings
            .iter()
            .find(|m| input >= m.from && input < m.from + m.len)
            .map(|m| m.to + (input - m.from))
            .unwrap_or(input)
    }
}

impl FromStr for PuzzleInput {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("\n\n").collect::<Vec<_>>();
        let seeds = parts[0]
            .split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect_vec();

        let seed_ranges = Self::parse_seed_ranges(
            parts[0]
                .split(' ')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect(),
        );

        Ok(Self {
            seeds,
            seed_ranges,
            seed_soil: parts[1].parse()?,
            soil_fertilizer: parts[2].parse()?,
            fertilizer_water: parts[3].parse()?,
            water_light: parts[4].parse()?,
            light_temp: parts[5].parse()?,
            temp_humidity: parts[6].parse()?,
            humidity_location: parts[7].parse()?,
        })
    }
}

impl FromStr for Table {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let name = lines.next().unwrap().to_string();
        let mappings = lines
            .map(|line| {
                let mut parts = line.split(' ');
                let to = parts.next().unwrap().parse::<usize>().unwrap();
                let from = parts.next().unwrap().parse::<usize>().unwrap();
                let len = parts.next().unwrap().parse::<usize>().unwrap();
                Mapping { from, to, len }
            })
            .collect::<Vec<_>>();

        Ok(Self { name, mappings })
    }
}
