use crate::{Result, SumResults};
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::fmt::Display;
use tracing::debug;

pub const DAY: u32 = 11;

fn blink_stone(stone: u64) -> Result<Vec<u64>> {
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

/// Key is stone id and the depth
type Key = (u64, usize);
/// Value is how many stones
type Cache = std::collections::BTreeMap<Key, usize>;

/// Given a stone, blink it according to the rules, and return how many stones are created.
/// This is a recursive function that will call itself for each stone created.
/// depth is how many times to blink the stones with 0 depth being the base case (Don't blink).
fn recurse_count(stone: u64, depth: usize, cache: &mut Cache) -> Result<usize> {
    // base case, no blinking, it's just one stone.
    debug!("stone: {}, depth: {}", stone, depth);
    if depth == 0 {
        return Ok(1);
    }
    // Our key in the cache is the stone number and the depth
    let key = (stone, depth);
    // If we have already calculated this, return the cached value.
    if let Some(depth) = cache.get(&key) {
        return Ok(*depth);
    }
    let stones = blink_stone(stone)?;
    let count = stones
        .into_iter()
        .map(|s| recurse_count(s, depth - 1, cache))
        .sum_results()?;
    cache.insert(key, count);
    Ok(count)
}

pub fn blink(stones: Vec<u64>) -> Result<Vec<u64>> {
    let mut newstones = vec![];
    for stone in stones {
        let newstone = blink_stone(stone)?;
        newstones.extend_from_slice(newstone.as_slice());
    }
    Ok(newstones)
}

fn solve_part1_impl(input: &Data) -> Result<usize> {
    solve_depth(input.stones.iter().copied(), 25)
    // let mut stones = input.stones.clone();

    // for _ in 0..25 {
    //     stones = blink(stones)?;
    //     info!("stones: {:?}", stones);
    // }

    // Ok(stones.len())
}

fn solve_depth(stones: impl Iterator<Item = u64>, depth: usize) -> Result<usize> {
    let mut cache = Cache::new();
    let count = stones
        .map(|s| recurse_count(s, depth, &mut cache))
        .sum_results()?;
    debug!("Cache size: {}", cache.len());

    Ok(count)
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    solve_depth(input.stones.iter().copied(), 75)
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
        let s = test_data(super::DAY).unwrap();
        let parsed = Data::parse(&s).unwrap();
        assert_eq!(
            solve_depth(parsed.stones.iter().copied(), 25).unwrap(),
            55312
        );
    }
}
