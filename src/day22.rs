#![allow(dead_code)]
use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use itertools::Itertools as _;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fmt::Display;

pub const DAY: u32 = 22;

fn secret_numbers(seed: usize) -> impl Iterator<Item = usize> {
    let mut seed = seed;
    [seed].into_iter().chain(std::iter::from_fn(move || {
        let part1 = {
            let result = seed * 64;
            let mix = result ^ seed;
            let prune = mix % 16777216;
            prune
        };
        let part2 = {
            let result = part1 / 32;
            let mix = result ^ part1;
            let prune = mix % 16777216;
            prune
        };
        let part3 = {
            let result = part2 * 2048;
            let mix = result ^ part2;
            let prune = mix % 16777216;
            prune
        };
        seed = part3;
        Some(seed)
    }))
}

fn solve_part1_impl(input: &Data) -> Result<usize> {
    Ok(input
        .numbers
        .iter()
        .map(|num| secret_numbers(*num).skip(2000).next().unwrap())
        .sum())
}

fn ones_digit(num: usize) -> usize {
    num % 10
}

fn part2_sequence(seed: usize) -> impl Iterator<Item = (usize, isize)> {
    secret_numbers(seed)
        .map(ones_digit)
        .tuple_windows()
        .map(|(a, b)| (b, b as isize - a as isize))
}

fn compute_sale_iter(
    seq: impl Iterator<Item = (usize, isize)>,
    sequence: &[isize; 4],
) -> Option<usize> {
    let windows = seq.tuple_windows();
    for ((_, a), (_, b), (_, c), (e, d)) in windows {
        if a == sequence[0] && b == sequence[1] && c == sequence[2] && d == sequence[3] {
            return Some(e);
        }
    }
    None
}

fn compute_sale(seed: usize, sequence: &[isize; 4]) -> Option<usize> {
    let seq = part2_sequence(seed).take(2000);
    compute_sale_iter(seq, sequence)
}

fn compute_sales_iter(
    numbers: impl ParallelIterator<Item = impl IntoIterator<Item = (usize, isize)>>,
    sequence: [isize; 4],
) -> usize {
    numbers
        .filter_map(|seq| compute_sale_iter(seq.into_iter(), &sequence))
        .sum()
}

fn compute_sales(numbers: &[usize], sequence: [isize; 4]) -> usize {
    numbers
        .par_iter()
        .filter_map(|num| compute_sale(*num, &sequence))
        .sum()
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    // pre-compute all numbers
    let numbers = input
        .numbers
        .iter()
        .map(|num| part2_sequence(*num).take(2000).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    //    let mut cache = HashMap::new();
    let all_sequences = input
        .numbers
        .iter()
        .flat_map(|num| {
            part2_sequence(*num)
                .take(2000)
                .tuple_windows()
                .map(|(a, b, c, d)| (a.1, b.1, c.1, d.1))
        })
        .take(2000);
    Ok(all_sequences
        //        .inspect(|sequence| println!("Sequence: {:?}", sequence))
        .map(|sequence| {
            (
                sequence,
                //     *cache.entry(sequence).or_insert_with(|| {
                compute_sales_iter(
                    numbers.par_iter().map(|seq| seq.iter().copied()),
                    [sequence.0, sequence.1, sequence.2, sequence.3],
                ), // compute_sales(
                   //     &input.numbers,
                   //     [sequence.0, sequence.1, sequence.2, sequence.3],
                   // ), //     }),
            )
        })
        .max_by_key(|(_, sales)| *sales)
        .inspect(|(sequence, sales)| println!("Sequence: {:?} Sales: {}", sequence, sales))
        .map(|(_, sales)| sales)
        .ok_or_else(|| anyhow::anyhow!("No sales found"))?)
}

/// Solution to part 1
#[aoc(day22, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day22, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

/// Problem input
#[derive(Debug)]
struct Data {
    // XXX: Change this to the actual data structure
    numbers: Vec<usize>,
}
impl Data {
    fn parse(s: &str) -> Result<Self> {
        let s = s.lines();
        let numbers = s
            .map(|line| Ok(line.parse::<usize>()?))
            .collect::<Result<Vec<_>>>()?;
        Ok(Data { numbers })
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
    fn test_sequence() {
        let mut seq = secret_numbers(123);
        assert_eq!(seq.next(), Some(123));
        assert_eq!(seq.next(), Some(15887950));
        assert_eq!(seq.next(), Some(16495136));
    }

    #[test]
    fn part1_example() {
        assert_eq!(
            solve_part1(&test_data(super::DAY).unwrap()).unwrap(),
            37327623
        );
    }

    #[test]
    fn part2_example() {
        // assert_eq!(
        //     solve_part2(&test_data(super::DAY).unwrap()).unwrap(),
        //     0 // XXX: Update this to the expected value for part 2 sample data.
        // );
    }

    #[test]
    fn test_compute_sales() {
        let numbers = &[1, 2, 3, 2024];
        assert_eq!(compute_sales(numbers, [-2, 1, -1, 3]), 23);
    }

    #[test]
    fn test_sequence_2() {
        let mut seq = part2_sequence(123);
        assert_eq!(seq.next(), Some((0, -3)));
        assert_eq!(seq.next(), Some((6, 6)));
        assert_eq!(seq.next(), Some((5, -1)));
        assert_eq!(seq.next(), Some((4, -1)));
        assert_eq!(seq.next(), Some((4, 0)));
        assert_eq!(seq.next(), Some((6, 2)));

        assert_eq!(compute_sale(123, &[-1, -1, 0, 2]), Some(6));
    }
}
