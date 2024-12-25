use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::fmt::Display;

pub const DAY: u32 = 25;

fn pins_fit<'a>(lock: impl Iterator<Item = &'a u8>, key: impl Iterator<Item = &'a u8>) -> bool {
    lock.zip(key).all(|(l, k)| l + k <= 5)
}

fn solve_part1_impl(input: &Data) -> Result<usize> {
    let lock_key_combinations = input
        .locks
        .iter()
        .flat_map(|l| input.keys.iter().map(move |k| (l, k)));
    let fits = lock_key_combinations.filter(|(l, k)| pins_fit(l.iter(), k.iter()));
    Ok(fits.count())
}

fn solve_part2_impl(_input: &Data) -> Result<usize> {
    // XXX: Solving logic for part 2
    Ok(0)
}

/// Solution to part 1
#[aoc(day25, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day25, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

type Profile = Vec<u8>;

/// Problem input
#[derive(Debug)]
struct Data {
    // XXX: Change this to the actual data structure
    keys: Vec<Profile>,
    locks: Vec<Profile>,
}
impl Data {
    fn parse(s: &str) -> Result<Self> {
        let profiles = s.split("\n\n");

        let mut keys = Vec::new();
        let mut locks = Vec::new();

        for profile in profiles {
            let mut profile = profile.lines();
            let first_line = profile
                .next()
                .ok_or_else(|| anyhow::anyhow!("empty profile"))?;

            let mut code = vec![0u8; first_line.len()];

            for line in profile.take(5) {
                for (l, c) in line.chars().zip(code.iter_mut()) {
                    if l == '#' {
                        *c += 1;
                    }
                }
            }
            if first_line.starts_with('#') {
                locks.push(code);
            } else {
                keys.push(code);
            }
        }

        Ok(Data { keys, locks })
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
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&test_data(super::DAY).unwrap()).unwrap(),
            0 // XXX: Update this to the expected value for part 2 sample data.
        );
    }
}
