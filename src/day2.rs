use std::str::FromStr;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

pub const DAY: u32 = 2;

/// A report is a list of integers that represent a series of levels.
#[derive(Debug)]
struct Report(Vec<i32>);
impl<'a> IntoIterator for &'a Report {
    type Item = &'a i32;
    type IntoIter = std::slice::Iter<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// Returns true if this report is valid given the rules:
/// - The difference between each value is less than 3
/// - The values are either all increasing or decreasing
/// - The values are not all the same
fn valid_report<'a>(report: impl IntoIterator<Item = &'a i32>) -> bool {
    let mut state = State::Start;
    for value in report.into_iter() {
        state = match state.next(*value) {
            Some(s) => s,
            None => return false,
        }
    }
    state.is_directional()
}

impl Report {
    /// Permutates this report by creating an iterator where each
    /// item is a report with one level removed.
    pub fn removed_levels(&self) -> impl Iterator<Item = impl Iterator<Item = &'_ i32>> + '_ {
        let reports = &self.0;
        (0..reports.len()).map(|i| {
            reports
                .iter()
                .enumerate()
                .filter(move |(thisi, _v)| *thisi != i)
                .map(|(_, v)| v)
        })
    }
}
impl FromStr for Report {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let reports = s
            .split_whitespace()
            .map(|n| n.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Report(reports))
    }
}

/// Our data consists of a list of reports.
struct Data {
    reports: Vec<Report>,
}
impl Data {
    /// This could validate that the reports are all the same length, but
    /// that requirement isn't necessarily levied in the problem.
    pub fn new(reports: Vec<Report>) -> Self {
        Self { reports }
    }
    pub fn each_report(&self) -> impl Iterator<Item = &'_ Report> + '_ {
        self.reports.iter()
    }
}

/// Parse the input according to the spec.
/// - Each line is a report
/// - Each report is a series of integers (size unspecified)
/// - The integers are separated by whitespace
#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Data> {
    let lines = input.lines();
    let reports = lines
        .map(Report::from_str)
        // Filter out empty reports because our input might be sus.
        .filter(|r| !r.as_ref().is_ok_and(|r| r.0.is_empty()))
        .collect::<Result<Vec<_>>>()?;
    Ok(Data::new(reports))
}

/// When walking across the reports, we need to keep track of the state.
enum State {
    // The initial state, no previous
    Start,
    // We've started, but we haven't assended or decended, so we don't know the direction
    Unknown(i32),
    // We're assending
    Assending(i32),
    // We're decending
    Decending(i32),
}

#[derive(PartialEq, Debug)]
enum Direction {
    Assending,
    Decending,
}

impl State {
    /// Given two levels, determine the direction of the difference.
    /// Returns None if the difference is bad (> 3 or == 0).
    /// The ordering is relative to the prev, so if current is > it's assending
    fn direction(prev: i32, current: i32) -> Option<Direction> {
        let diff = current - prev;
        if diff.abs() > 3 || diff == 0 {
            return None;
        }

        if diff > 0 {
            Some(Direction::Assending)
        } else {
            Some(Direction::Decending)
        }
    }

    /// Given our current state, compute the next state given the next value.
    pub fn next(self, current: i32) -> Option<State> {
        // This match is designed to allow for visual pattern matching to show correct implementation.
        match self {
            // If we're starting, we don't know the direction
            State::Start => Some(State::Unknown(current)),
            // If we're unknown, we can determine the direction
            State::Unknown(prev) => match Self::direction(prev, current)? {
                Direction::Assending => Some(State::Assending(current)),
                Direction::Decending => Some(State::Decending(current)),
            },
            // Assending or decending, we filter out bad differences and
            // make sure we're traveling in the same direction.
            State::Assending(prev) => match Self::direction(prev, current)? {
                Direction::Assending => Some(State::Assending(current)),
                Direction::Decending => None,
            },
            State::Decending(prev) => match Self::direction(prev, current)? {
                Direction::Assending => None,
                Direction::Decending => Some(State::Decending(current)),
            },
        }
    }

    /// This state is directional if it's assending or decending.
    pub fn is_directional(&self) -> bool {
        matches!(self, State::Assending(_) | State::Decending(_))
    }
}

/// codspeed compatible function
pub fn part1(input: &str) -> Result<usize> {
    let data = parse(input)?;
    Ok(solve_part1(&data))
}

/// codspeed compatible function
pub fn part2(input: &str) -> Result<usize> {
    let data = parse(input)?;
    Ok(solve_part2(&data))
}

/// The first part returns the number of reports that are valid.
#[aoc(day2, part1)]
fn solve_part1(input: &Data) -> usize {
    input.each_report().filter(|&r| valid_report(r)).count()
}

/// The second part returns the number of reports that are valid, or
/// valid if one level is removed.
#[aoc(day2, part2)]
fn solve_part2(input: &Data) -> usize {
    input
        .reports
        .iter()
        .filter(|&r| valid_report(r) || r.removed_levels().any(valid_report))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::{
        day2::{parse, solve_part1, solve_part2},
        test_data,
    };

    #[test]
    fn part1_example() {
        assert_eq!(
            solve_part1(&parse(&test_data(super::DAY).unwrap()).unwrap()),
            2
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&parse(&test_data(super::DAY).unwrap()).unwrap()),
            4
        );
    }

    #[test]
    fn test_diff_direction() {
        assert_eq!(
            super::State::direction(1, 2),
            Some(super::Direction::Assending)
        );
        assert_eq!(
            super::State::direction(2, 1),
            Some(super::Direction::Decending)
        );
        // Right before going bad +-
        assert_eq!(
            super::State::direction(0, 3),
            Some(super::Direction::Assending)
        );
        assert_eq!(
            super::State::direction(3, 0),
            Some(super::Direction::Decending)
        );
        // Two equal
        assert_eq!(super::State::direction(1, 1), None);
        // Just +- than 3
        assert_eq!(super::State::direction(0, 4), None);
        assert_eq!(super::State::direction(4, 0), None);
    }
}
