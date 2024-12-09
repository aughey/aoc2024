use crate::{CheckedSum as _, Result, SumResults as _};
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::fmt::Display;

pub const DAY: u32 = 9;

pub fn print_blocks(blocks: &[Option<u64>]) {
    for b in blocks {
        if let Some(b) = b {
            print!("{}", b);
        } else {
            print!(".");
        }
    }
    println!();
}

fn solve_part1_impl(input: &Data) -> Result<u64> {
    let len = input.blocks.len();
    let mut blocks = input.blocks.clone();
    let mut forward = 0usize;
    let mut backward = len - 1;

    'outer: loop {
        // Find the next block to move from the rear
        while backward > 0 {
            if blocks[backward].is_some() {
                break;
            }
            if backward == 1 {
                break 'outer;
            }
            backward -= 1
        }
        // Find the next empty block to move to from the front
        while forward < len {
            if blocks[forward].is_none() {
                break;
            }
            if forward == len - 1 {
                break 'outer;
            }
            forward += 1;
        }
        if forward >= backward {
            break;
        }

        // 593815965
        let from = &mut blocks[backward];
        if let Some(num) = from {
            let num = *num;
            let to = &mut blocks[forward];

            if to.is_none() {
                *to = Some(num);
                blocks[backward] = None;
            }
        }
    }

    Ok(blocks
        .iter()
        .enumerate()
        .filter(|(_, c)| c.is_some())
        .map(|(i, c)| (i as u64 * c.unwrap()))
        .checked_sum()
        .ok_or_else(|| anyhow::anyhow!("add overflowed"))?)
}

fn solve_part2_impl(input: &Data) -> Result<u64> {
    let mut blocks = input.blocks.clone();

    let last_id = blocks
        .iter()
        .rev()
        .find(|c| c.is_some())
        .ok_or_else(|| anyhow::anyhow!("no last"))?
        .unwrap();

    for id in (0..=last_id).rev() {
        // Find the span of this id
        let from = {
            let start_index = blocks
                .iter()
                .position(|c| c == &Some(id))
                .ok_or_else(|| anyhow::anyhow!("no start index for id {}", id))?;
            let end_index = blocks[start_index..]
                .iter()
                .position(|c| c != &Some(id))
                .map(|i| start_index + i)
                .unwrap_or_else(|| blocks.len());
            (start_index, end_index)
        };
        assert_eq!(blocks[from.0..from.1].iter().all(|c| c == &Some(id)), true);
        let from_len = from.1 - from.0;

        // Find span with empty blocks with the same length
        let mut to = None;
        for i in 0..blocks.len() {
            if blocks[i..]
                .iter()
                .take(from_len)
                .take_while(|c| c.is_none())
                .count()
                == from_len
            {
                to = Some(i);
                break;
            }
        }
        if let Some(to) = to {
            assert_eq!(blocks[to..to + from_len].iter().all(|c| c.is_none()), true);
            if to > from.0 {
                continue;
            }
            // Move the span
            for i in 0..from_len {
                blocks[to + i] = blocks[from.0 + i];
                blocks[from.0 + i] = None;
            }
        }
    }

    blocks
        .iter()
        .enumerate()
        .filter(|(_, c)| c.is_some())
        .map(|(i, c)| {
            (i as u64)
                .checked_mul(c.unwrap())
                .ok_or_else(|| anyhow::anyhow!("mul overflowed"))
        })
        .sum_results()
}

/// Solution to part 1
#[aoc(day9, part1)]
fn solve_part1(input: &str) -> Result<u64> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day9, part2)]
fn solve_part2(input: &str) -> Result<u64> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

/// Problem input
#[derive(Debug)]
struct Data {
    // XXX: Change this to the actual data structure
    blocks: Vec<Option<u64>>,
}

impl Data {
    fn parse(s: &str) -> Result<Self> {
        let mut input = s.chars();
        let mut blocks = vec![];
        let mut index = 0u64;
        loop {
            let count = if let Some(c) = input.next() {
                c.to_string().parse::<u64>()?
            } else {
                break;
            };
            for _ in 0..count {
                blocks.push(Some(index));
            }
            index += 1;
            let count = if let Some(c) = input.next() {
                c.to_string().parse::<u64>()?
            } else {
                break;
            };
            for _ in 0..count {
                blocks.push(None);
            }
        }
        Ok(Data { blocks })
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
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&test_data(super::DAY).unwrap()).unwrap(), 2858);
    }
}
