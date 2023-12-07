use super::Puzzle;
use std::cmp::Ordering;

pub struct Day06;

// Input is so small, not bothering with a parser
const INPUT: &'static [(usize, usize)] = &[(46, 347), (82, 1522), (84, 1406), (79, 1471)];

impl Puzzle for Day06 {
    fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
        Box::new(Self)
    }

    fn part_one(&self, _input: &str) -> super::PuzzleResult {
        let result = INPUT
            .iter()
            .cloned()
            .map(|(time, best)| {
                (1..=time)
                    .into_iter()
                    .filter(|x| distance(*x, time) > best)
                    .count()
            })
            .product::<usize>();
        Ok(result.to_string())
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
    }
}

/// Returns the begginning and end of the sequence which produces a winning score
fn winning_solutions(time: usize, best: usize) -> (usize, usize) {
    let mut start: usize;
    let mut end: usize;
    let mut mid = time / 2;
    // Binary search to find some point where the distance is greater than the best
    loop {
        match probe(time, best, mid) {
            Ordering::Equal => {
                break;
            }
            Ordering::Less => {
                mid = mid + (time - mid) / 2;
            }
            Ordering::Greater => {
                mid = mid / 2;
            }
        }
    }

    todo!()
}

fn search(best: usize, start: usize, end: usize) -> (usize, usize) {
    todo!()
}

/// Check to see if the specified distance is a winning one
/// If not, looks ahead and back to see which direction to check next
fn probe(time: usize, best: usize, check: usize) -> Ordering {
    let dist = distance(check, time);
    if dist > best {
        return Ordering::Equal;
    }

    if check > 1 {
        if dist < distance(check - 1, time) {
            return Ordering::Less;
        }
    }
    if check >= time {
        panic!("distance is greater than time");
    }
    return Ordering::Greater;
}

/// For each whole millisecond you spend at the beginning of the race holding down
/// the button, the boat's speed increases by *one millimeter per millisecond*.
fn distance(time_held: usize, total_time: usize) -> usize {
    let speed = time_held; // for clarity
    (total_time - time_held) * speed
}

#[cfg(test)]
mod test {
    fn test_speed() {
        assert_eq!(super::distance(1, 10), 9);
        assert_eq!(super::distance(2, 10), 16);
        assert_eq!(super::distance(3, 10), 21);
        assert_eq!(super::distance(4, 10), 24);
        assert_eq!(super::distance(5, 10), 25);
        assert_eq!(super::distance(6, 10), 24);
        assert_eq!(super::distance(7, 10), 21);
        assert_eq!(super::distance(8, 10), 16);
        assert_eq!(super::distance(9, 10), 9);
    }
}
