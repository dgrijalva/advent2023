use super::Puzzle;

pub struct Day06;

// Input is so small, not bothering with a parser
const INPUT: &'static [(usize, usize)] = &[(46, 347), (82, 1522), (84, 1406), (79, 1471)];
const INPUT_PART_2: (usize, usize) = (46828479, 347152214061471);

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
        let (time, best) = INPUT_PART_2;
        let result = (1..=time)
            .into_iter()
            .filter(|x| distance(*x, time) > best)
            .count();
        Ok(result.to_string())
    }
}

/// For each whole millisecond you spend at the beginning of the race holding down
/// the button, the boat's speed increases by *one millimeter per millisecond*.
fn distance(time_held: usize, total_time: usize) -> usize {
    let speed = time_held; // for clarity
    (total_time - time_held) * speed
}

#[cfg(test)]
mod test {
    #[test]
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
