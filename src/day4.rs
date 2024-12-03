use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::{aoc, aoc_generator};
use std::{fmt::Display, str::FromStr};
use tracing::info;

pub const DAY: u32 = 4;

/// Parsing logic uses the FromStr trait
#[aoc_generator(day4)]
fn parse(input: &str) -> Result<Data> {
    info!("Parsing input");
    Data::from_str(input).context("input parsing")
}

/// Solution to part 1
#[aoc(day4, part1)]
fn solve_part1(_input: &Data) -> Result<usize> {
    // XXX: Solving logic for part 1
    Ok(0)
}

/// Solution to part 2
#[aoc(day4, part2)]
fn solve_part2(_input: &Data) -> Result<usize> {
    // XXX: Solving logic for part 2
    Ok(0)
}

/// Problem input
#[derive(Debug)]
struct Data {
    // XXX: Change this to the actual data structure
    _len: usize,
}
impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // XXX: Do actual parsing here.
        let s = s.lines();
        // XXX: Update the returned Data to include the parsed data.
        Ok(Data { _len: s.count() })
    }
}

/// codspeed compatible function
pub fn part1(input: &str) -> impl Display {
    solve_part1(&parse(input).unwrap()).unwrap()
}

/// codspeed compatible function
pub fn part2(input: &str) -> impl Display {
    solve_part2(&parse(input).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::test_data;
    use test_log::test;

    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            solve_part1(&parse(&test_data(super::DAY).unwrap()).unwrap()).unwrap(),
            0 // XXX: Update this to the expected value for part 1 sample data.
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&parse(&test_data(super::DAY).unwrap()).unwrap()).unwrap(),
            0 // XXX: Update this to the expected value for part 2 sample data.
        );
    }
}
