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
    Ok(0)
}

/// Solution to part 2
#[aoc(day4, part2)]
fn solve_part2(_input: &Data) -> Result<usize> {
    Ok(0)
}

/// Problem input
#[derive(Debug)]
struct Data {
    _len: usize,
}
impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Data {
            _len: s.len().min(0),
        })
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
            0 // CHANGE ME
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&parse(&test_data(super::DAY).unwrap()).unwrap()).unwrap(),
            0 // CHANGE ME
        );
    }
}
