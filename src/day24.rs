use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::{collections::HashMap, fmt::Display};

pub const DAY: u32 = 24;

fn solve_part1_impl(input: &Data) -> Result<usize> {
    let mut state = input.start.clone();

    loop {
        let mut missed_one = false;
        for op in &input.operations {
            if op.try_op(&mut state).is_none() {
                missed_one = true;
            }
        }
        if !missed_one {
            break;
        }
    }

    let mut result = 0usize;
    for i in 0..64 {
        let z = format!("z{:02}", i);
        if let Some(&value) = state.get(z.as_str()) {
            println!("{}: {}", z, value);
            result |= (value as usize) << i;
        }
    }

    Ok(result)
}

fn solve_part2_impl(_input: &Data) -> Result<usize> {
    // XXX: Solving logic for part 2
    Ok(0)
}

/// Solution to part 1
#[aoc(day24, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day24, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

type State<'a> = HashMap<&'a str, u8>;

#[derive(Debug, PartialEq, Eq)]
enum Op {
    XOR,
    OR,
    AND,
}
impl TryFrom<&str> for Op {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self> {
        match s {
            "AND" => Ok(Op::AND),
            "OR" => Ok(Op::OR),
            "XOR" => Ok(Op::XOR),
            _ => Err(anyhow::anyhow!("invalid operation")),
        }
    }
}

#[derive(Debug)]
struct Operation<'a> {
    op: Op,
    a: &'a str,
    b: &'a str,
    dest: &'a str,
}
impl<'a> Operation<'a> {
    fn try_op(&self, state: &mut State<'a>) -> Option<()> {
        let a = state.get(self.a)?;
        let b = state.get(self.b)?;
        let result = match self.op {
            Op::AND => a & b,
            Op::OR => a | b,
            Op::XOR => a ^ b,
        };
        state.insert(self.dest, result);
        Some(())
    }
}

/// Problem input
#[derive(Debug)]
struct Data<'a> {
    start: HashMap<&'a str, u8>,
    operations: Vec<Operation<'a>>,
}
impl<'a> Data<'a> {
    fn parse(s: &'a str) -> Result<Self> {
        let (start, ops) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow::anyhow!("no newline separator"))?;

        let start = start
            .lines()
            .map(|line| {
                let (key, value) = line
                    .split_once(": ")
                    .ok_or_else(|| anyhow::anyhow!("no colon separator"))?;
                let value = value.trim().parse::<u8>()?;
                Ok((key.trim(), value))
            })
            .collect::<Result<_>>()?;

        let operations = ops
            .lines()
            .map(|line| {
                let (op, dest) = line
                    .split_once(" -> ")
                    .ok_or_else(|| anyhow::anyhow!("no arrow separator"))?;
                let dest = dest.trim();
                let mut op = op.split_whitespace();
                let a = op
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("no first operand"))?;
                let operation = op.next().ok_or_else(|| anyhow::anyhow!("no operation"))?;
                let b = op
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("no second operand"))?;
                let operation = Op::try_from(operation)?;
                Ok(Operation {
                    op: operation,
                    a,
                    b,
                    dest,
                })
            })
            .collect::<Result<_>>()?;

        // XXX: Update the returned Data to include the parsed data.
        Ok(Data { operations, start })
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
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 2024);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&test_data(super::DAY).unwrap()).unwrap(),
            0 // XXX: Update this to the expected value for part 2 sample data.
        );
    }
}
