use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use glam::I8Vec2;
use std::{collections::HashMap, fmt::Display};

pub const DAY: u32 = 21;

type Keypad = HashMap<char, I8Vec2>;

fn optimal_sequence(
    cur_pos: I8Vec2,
    key: char,
    my_keypad: &Keypad,
    inner_keypad: &Keypad,
    depth: usize,
) {
}

fn compute_sequence(
    keypad: &HashMap<char, I8Vec2>,
    input: impl IntoIterator<Item = char>,
) -> Result<Vec<I8Vec2>> {
    let mut cur_pos = *keypad
        .get(&'A')
        .ok_or_else(|| anyhow::anyhow!("Invalid start position"))?;

    let mut sequence = Vec::new();
    for key in input {
        let next_pos = keypad
            .get(&key)
            .ok_or_else(|| anyhow::anyhow!("Invalid key: {}", key))?;

        while cur_pos != *next_pos {
            let diff = *next_pos - cur_pos;

            let mut possible = [0, 1]
                .into_iter()
                .filter(|&i| diff[i] != 0)
                .map(|i| {
                    // Make a direction vector
                    let mut dir = I8Vec2::ZERO;
                    dir[i] = diff[i].signum();
                    dir
                })
                .filter(|&dir| {
                    let next_pos = cur_pos + dir;
                    // Find the next position in the keypad
                    let next_pos_valid = keypad.iter().any(|(_, &v)| v == next_pos);
                    next_pos_valid
                });
            let next_dir = possible.next().ok_or_else(|| {
                anyhow::anyhow!(
                    "Invalid move from {:?} to {:?} (diff: {:?})",
                    cur_pos,
                    next_pos,
                    diff
                )
            })?;
            sequence.push(next_dir);
            cur_pos += next_dir;
        }
        sequence.push(I8Vec2::ZERO);
    }

    Ok(sequence)
}

fn dir_to_key(dir: &I8Vec2) -> char {
    match dir {
        I8Vec2 { x: 1, y: 0 } => '>',
        I8Vec2 { x: -1, y: 0 } => '<',
        I8Vec2 { x: 0, y: 1 } => '^',
        I8Vec2 { x: 0, y: -1 } => 'v',
        I8Vec2 { x: 0, y: 0 } => 'A',
        _ => panic!("Invalid direction: {:?}", dir),
    }
}

fn keypad_to_xy<INNER>(keypad: &[INNER]) -> HashMap<char, I8Vec2>
where
    INNER: AsRef<[char]>,
{
    keypad
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let y = keypad.len() - y;
            row.as_ref()
                .iter()
                .enumerate()
                .filter_map(move |(x, &key)| {
                    if key == ' ' {
                        None
                    } else {
                        Some((key, I8Vec2::new(x as i8, y as i8)))
                    }
                })
        })
        .collect()
}

fn dir_to_keypad(dirs: impl IntoIterator<Item = I8Vec2>) -> impl Iterator<Item = char> {
    dirs.into_iter().map(|dir| dir_to_key(&dir))
}

fn solve_part1_impl(input: &Data) -> Result<usize> {
    let numeric_keypad = [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        ['_', '0', 'A'],
    ];
    let numeric_keypad = keypad_to_xy(numeric_keypad.as_slice());

    let directional_keypad = [[' ', '^', 'A'], ['<', 'v', '>']];
    let directional_keypad = keypad_to_xy(directional_keypad.as_slice());

    for &code in &input.codes {
        let robot0 = compute_sequence(&numeric_keypad, code.chars())?;
        let robot1 = compute_sequence(&directional_keypad, dir_to_keypad(robot0))?;
        println!(
            "robot1 sequence {}",
            dir_to_keypad(robot1.clone()).collect::<String>()
        );
        let robot2 = compute_sequence(&directional_keypad, dir_to_keypad(robot1))?;

        let sequence = dir_to_keypad(robot2).collect::<String>();

        println!("{}: {} {}", code, sequence, sequence.len());
    }

    Ok(1)
}

fn solve_part2_impl(_input: &Data) -> Result<usize> {
    // XXX: Solving logic for part 2
    Ok(0)
}

/// Solution to part 1
#[aoc(day21, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day21, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

/// Problem input
#[derive(Debug)]
struct Data<'a> {
    // XXX: Change this to the actual data structure
    codes: Vec<&'a str>,
}
impl<'a> Data<'a> {
    fn parse(s: &'a str) -> Result<Self> {
        let s = s.lines();
        let codes = s.collect();

        Ok(Data { codes })
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
        assert_eq!(
            solve_part1(&test_data(super::DAY).unwrap()).unwrap(),
            0 // XXX: Update this to the expected value for part 1 sample data.
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&test_data(super::DAY).unwrap()).unwrap(),
            0 // XXX: Update this to the expected value for part 2 sample data.
        );
    }
}
