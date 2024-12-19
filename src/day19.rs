use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::fmt::Display;

pub const DAY: u32 = 19;

fn possible_counts<'a>(
    original_pattern: &str,
    towels: impl Iterator<Item = &'a str> + Clone,
) -> usize {
    pathfinding::directed::count_paths::count_paths(
        original_pattern,
        |sub_pattern| {
            towels
                .clone()
                // Create a tuple with the head of the pattern the same
                // length as the towel and the towel itself.
                .filter_map(|towel| Some((sub_pattern.get(..towel.len())?, towel)))
                // Filter to include tuples where the head and towel match.
                // i.e. the pattern starts with the towel and can be used.
                .filter(|(pattern_head, towel)| pattern_head == towel)
                // Create a tuple with the tail of the pattern and the towel.
                // Safety: Unwrap is safe here because we got the head of the
                // same length earlier.
                .map(|(_, towel)| sub_pattern.get(towel.len()..).unwrap())
        },
        |p| p.is_empty(),
    )
}

fn solve_part1_impl(input: &Data) -> Result<usize> {
    Ok(input
        .patterns
        .iter()
        .filter(|p| possible_counts(p, input.towels.iter().map(|t| *t)) > 0)
        .count())
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    Ok(input
        .patterns
        .iter()
        .map(|p| possible_counts(p, input.towels.iter().map(|t| *t)))
        .sum())
}

/// Solution to part 1
#[aoc(day19, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day19, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

/// Problem input
#[derive(Debug)]
struct Data<'a> {
    towels: Vec<&'a str>,
    patterns: Vec<&'a str>,
}
impl<'a> Data<'a> {
    fn parse(s: &'a str) -> Result<Self> {
        let (towels, patterns) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow::anyhow!("Invalid input split"))?;

        let towels = towels.split(',').map(|s| s.trim()).collect();
        let patterns = patterns.split('\n').map(|s| s.trim()).collect();

        Ok(Data { towels, patterns })
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
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&test_data(super::DAY).unwrap()).unwrap(), 16);
    }
}
