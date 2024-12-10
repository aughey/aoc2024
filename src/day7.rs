use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools as _;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator as _};
use std::{fmt::Display, str::FromStr};
use tracing::info;

pub const DAY: u32 = 7;

/// Parsing logic uses the FromStr trait
#[aoc_generator(day7)]
fn parse(input: &str) -> Result<Data> {
    info!("Parsing input");
    Data::from_str(input).context("input parsing")
}

/// Solution to part 1
#[aoc(day7, part1)]
fn solve_part1(input: &Data) -> Result<i64> {
    const OPERATORS: [Operators; 2] = [Operators::Add, Operators::Multiply];
    Ok(input
        .equations
        .iter()
        .filter(|e| {
            let count = valid_equation(e, &OPERATORS);
            count > 0
        })
        .map(|e| e.result)
        .sum())
}

/// Solution to part 2
#[aoc(day7, part2)]
fn solve_part2(input: &Data) -> Result<i64> {
    const OPERATORS: [Operators; 3] = [Operators::Add, Operators::Multiply, Operators::Concat];
    Ok(input
        .equations
        .par_iter()
        .filter(|e| {
            let count = valid_equation(e, &OPERATORS);
            count > 0
        })
        .map(|e| e.result)
        .sum())
}

#[derive(Debug)]
enum Operators {
    Add,
    Multiply,
    Concat,
}

fn valid_equation(e: &Equation, operators: &[Operators]) -> usize {
    let combinations = (0..e.terms.len() - 1)
        .map(|_| operators)
        .multi_cartesian_product();

    info!(
        "Combinations for size {}: {:?}",
        e.terms.len() - 1,
        combinations.clone().collect::<Vec<_>>()
    );
    let valid_combinations = combinations.map(|combo| {
        let mut combo = combo.into_iter();
        let result = e.terms.iter().fold(None, |acc, term| match (acc, term) {
            (None, _) => Some(*term),
            (Some(acc), term) => {
                let operator = combo.next().unwrap();

                match operator {
                    Operators::Add => Some(acc + term),
                    Operators::Multiply => Some(acc * term),
                    Operators::Concat => {
                        let left = acc.to_string();
                        let right = term.to_string();
                        let result = format!("{}{}", left, right).parse().unwrap();
                        Some(result)
                    }
                }
            }
        });
        result.unwrap()
    });
    valid_combinations.filter(|&r| r == e.result).count()
}

#[derive(Debug)]
struct Equation {
    result: i64,
    terms: Vec<i64>,
}
impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut s = s.split(':');
        let result = s
            .next()
            .ok_or_else(|| anyhow::anyhow!("No result"))?
            .parse()?;

        let terms = s.next().ok_or_else(|| anyhow::anyhow!("No terms"))?;
        let terms = terms
            .split_whitespace()
            .map(|t| Ok(t.parse()?))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { result, terms })
    }
}

/// Problem input
#[derive(Debug)]
struct Data {
    equations: Vec<Equation>,
}
impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // XXX: Do actual parsing here.
        let s = s.lines();
        let equations = s.map(Equation::from_str).collect::<Result<Vec<_>>>()?;
        // XXX: Update the returned Data to include the parsed data.
        Ok(Data { equations })
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
            3749
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&parse(&test_data(super::DAY).unwrap()).unwrap()).unwrap(),
            11387
        );
    }
}
