use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::{aoc, aoc_generator};
use std::{fmt::Display, str::FromStr};
use tracing::{debug, info};

pub const DAY: u32 = 3;

/// Parsing logic uses the FromStr trait
#[aoc_generator(day3)]
fn parse(input: &str) -> Result<Data> {
    info!("Parsing input");
    Data::from_str(input).context("input parsing")
}

/// Solution to part 1
#[aoc(day3, part1)]
fn solve_part1(input: &Data) -> Result<usize> {
    // XXX: Solving logic for part 1
    Ok(input.numbers.iter().map(|(a, b, _)| a * b).sum())
}

/// Solution to part 2
#[aoc(day3, part2)]
fn solve_part2(input: &Data) -> Result<usize> {
    // XXX: Solving logic for part 2
    Ok(input.numbers_with_dos().map(|(a, b, _)| a * b).sum())
}

/// Problem input
#[derive(Debug)]
struct Data {
    // XXX: Change this to the actual data structure
    numbers: Vec<(usize, usize, usize)>,
    dos: Vec<usize>,
    donts: Vec<usize>,
}
impl Data {
    fn numbers_with_dos(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        self.numbers
            .iter()
            .filter(|possible| {
                let pos = possible.2;
                // index of do before pos
                let doindex = self.dos.iter().rev().find(|&&d| d < pos);
                // index of dont before pos
                let dontindex = self.donts.iter().rev().find(|&&d| d < pos);
                debug!(
                    "pos: {:?} doindex: {:?}, dontindex: {:?}",
                    pos, doindex, dontindex
                );
                match (doindex, dontindex) {
                    // if there is a do and it is after the last dont
                    (Some(d), Some(dont)) => d > dont,
                    // if there is a do and no dont
                    (Some(_), None) => true,
                    // if there is a dont and no do
                    (None, Some(_)) => false,
                    // if there is no do or dont
                    (None, None) => true,
                }
            })
            .copied()
    }
}
impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // regex that looks for many statements of mul(##,##)
        let r = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let doit = regex::Regex::new(r"do()").unwrap();
        let dont = regex::Regex::new(r"don't()").unwrap();
        let numbers = r
            .find_iter(s)
            .map(|f| {
                let c = r.captures(f.as_str()).unwrap();
                (
                    c.get(1).unwrap().as_str().parse().unwrap(),
                    c.get(2).unwrap().as_str().parse().unwrap(),
                    f.start(),
                )
            })
            .collect();
        let dos = doit.find_iter(s).map(|f| f.start()).collect();

        let donts = dont.find_iter(s).map(|f| f.start()).collect();

        // XXX: Update the returned Data to include the parsed data.
        Ok(Data {
            numbers,
            dos,
            donts,
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
            161
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&parse(&test_data(super::DAY).unwrap()).unwrap()).unwrap(),
            48 // XXX: Update this to the expected value for part 2 sample data.
        );
    }
}
