use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::fmt::Display;

pub const DAY: u32 = 13;

fn solve_part1_impl(_input: &Data) -> Result<usize> {
    // XXX: Solving logic for part 1
    Ok(0)
}

fn solve_part2_impl(_input: &Data) -> Result<usize> {
    // XXX: Solving logic for part 2
    Ok(0)
}

/// Solution to part 1
#[aoc(day13, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day13, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

/// Problem input
#[derive(Debug)]
struct Data {
    // XXX: Change this to the actual data structure
    _len: usize,
}
impl Data {
    fn parse(s: &str) -> Result<Self> {
        // XXX: Do actual parsing here.
        let s = s.lines();
        // XXX: Update the returned Data to include the parsed data.
        Ok(Data { _len: s.count() })
    }
}

/// codspeed compatible function
pub fn part1(input: &str) -> impl Display {
    solve_part1(input).unwrap()
}

/// codspeed compatible function
pub fn part2(input: &str) -> impl Display {
    solve_part2(input).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::test_data;
    use test_log::test;

    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            solve_part1(&test_data(super::DAY).unwrap()).unwrap(),
            0 // XXX: Update this to the expected value for part 1 sample data.
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&test_data(super::DAY).unwrap()).unwrap(),
            0 // XXX: Update this to the expected value for part 2 sample data.
        );
    }
}
