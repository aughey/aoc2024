use std::str::FromStr;

use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};

pub const DAY: u32 = 2;

/// A report is a list of integers that represent a series of levels.
#[derive(Debug)]
struct Report(Vec<i32>);
impl Report {
    /// Returns true if this report is valid given the rules:
    /// - The difference between each value is less than 3
    /// - The values are either all increasing or decreasing
    /// - The values are not all the same
    pub fn is_valid(&self) -> bool {
        let mut state = State::Start;
        for value in &self.0 {
            state = match state.next(*value) {
                Some(s) => s,
                None => return false,
            }
        }
        state.is_directional()
    }

    /// Permutates this report by creating an iterator where each
    /// item is a report with one level removed.
    pub fn removed_levels(&self) -> impl Iterator<Item = Report> + '_ {
        // (0..self.0.len()) works too, but this is more explicit that the indicies
        // had to have come from the container.
        self.0.iter().enumerate().map(|(i, _)| {
            // Could build new with an iterator, enumerator, and filter that
            // removes the value at the index.  That would save a shift of the
            // values in the vector, but would only be useful for a huge Vec.
            let mut new = self.0.clone();
            new.remove(i);
            Report(new)
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

impl State {
    /// Given a difference between two levels, it's bad if it's 0 or greater than 3.
    pub fn diff_bad(diff: i32) -> bool {
        diff.abs() > 3 || diff == 0
    }

    /// Given our current state, compute the next state given the next value.
    pub fn next(self, value: i32) -> Option<State> {
        match self {
            // If we're starting, we don't know the direction
            State::Start => Some(State::Unknown(value)),
            // If we're unknown, we can determine the direction
            State::Unknown(prev) => {
                let diff = value - prev;
                if Self::diff_bad(diff) {
                    return None;
                }
                if diff > 0 {
                    Some(State::Assending(value))
                } else {
                    Some(State::Decending(value))
                }
            }
            // Assending or decending, we filter out bad differences and
            // make sure we're traveling in the same direction.
            State::Assending(prev) => {
                let diff = value - prev;
                if Self::diff_bad(diff) {
                    return None;
                }
                if diff > 0 {
                    Some(State::Assending(value))
                } else {
                    // Cannot change direction
                    None
                }
            }
            State::Decending(prev) => {
                let diff = value - prev;
                if Self::diff_bad(diff) {
                    return None;
                }
                if diff < 0 {
                    Some(State::Decending(value))
                } else {
                    // Cannot change direction
                    None
                }
            }
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
    input.reports.iter().filter(|r| r.is_valid()).count()
}

/// The second part returns the number of reports that are valid, or
/// valid if one level is removed.
#[aoc(day2, part2)]
fn solve_part2(input: &Data) -> usize {
    input
        .reports
        .iter()
        .filter(|r| r.is_valid() || r.removed_levels().any(|r| r.is_valid()))
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
}
