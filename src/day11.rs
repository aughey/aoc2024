use crate::{Result, SumResults};
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::fmt::Display;
use tracing::info;

pub const DAY: u32 = 11;

fn blink_stone(stone: u64, steps_to_go: usize, cache: Cache) -> Result<Vec<u64>> {
    if let Some(res) = cache.get(&(stone, steps_to_go)) {
        return Ok(*res);
    }
    let newstone = if stone == 0 {
        vec![1]
    } else if stone.to_string().len() % 2 == 0 {
        let stone = stone.to_string();
        let (a, b) = stone.split_at(stone.to_string().len() / 2);
        vec![a.parse::<u64>()?, b.parse::<u64>()?]
    } else {
        vec![stone * 2024]
    };
    Ok(newstone)
}

type Key = (u64, usize);
type Cache = std::collections::HashMap<Key, usize>;

fn blink(stones: Vec<u64>, steps_to_go: usize, cache: &mut Cache) -> Result<Vec<u64>> {
    let mut newstones = vec![];
    for stone in stones {
        let newstone = blink_stone(stone)?;
        newstones.extend_from_slice(newstone.as_slice());
    }
    Ok(newstones)
}

fn solve_part1_impl(input: &Data) -> Result<usize> {
    let mut stones = input.stones.clone();

    for _ in 0..25 {
        stones = blink(stones)?;
        info!("stones: {:?}", stones);
    }

    Ok(stones.len())
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    let stones = &input.stones;

    // do each one by itself an sum
    Ok(stones
        .into_iter()
        .map(|s| {
            let mut stones = vec![*s];
            for _ in 0..75 {
                stones = blink(stones)?;
            }
            Ok(stones.len())
        })
        .sum_results()?)
}

/// Solution to part 1
#[aoc(day11, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day11, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

/// Problem input
#[derive(Debug)]
struct Data {
    // XXX: Change this to the actual data structure
    stones: Vec<u64>,
}
impl Data {
    fn parse(s: &str) -> Result<Self> {
        // XXX: Do actual parsing here.
        let stones = s.split_whitespace().map(|s| Ok(s.parse::<u64>()?));
        let stones = stones.collect::<Result<_>>()?;

        // XXX: Update the returned Data to include the parsed data.
        Ok(Data { stones })
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
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&test_data(super::DAY).unwrap()).unwrap(),
            0 // XXX: Update this to the expected value for part 2 sample data.
        );
    }
}
