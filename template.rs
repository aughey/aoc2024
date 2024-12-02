use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

/// Problem input
#[derive(Debug)]
struct Data {
    input: String,
}
impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Data {
            input: s.to_string(),
        })
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Result<Data> {
    Data::from_str(input)
}

#[aoc(day3, part1)]
fn solve_part1(input: &Data) -> String {
    input.input.clone()
}

#[aoc(day3, part2)]
fn solve_part2(input: &Data) -> String {
    input.input.clone()
}

/// codspeed compatible function
pub fn part1(input: &str) -> impl Display {
    solve_part1(&parse(input).unwrap())
}

/// codspeed compatible function
pub fn part2(input: &str) -> impl Display {
    solve_part2(&parse(input).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Result<String> {
        Ok(std::fs::read_to_string("test3.txt")?)
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            solve_part1(&parse(&test_data().unwrap()).unwrap()),
            "<RESULT>"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&parse(&test_data().unwrap()).unwrap()),
            "<RESULT>"
        );
    }
}
