use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use rayon::iter::{ParallelBridge, ParallelIterator as _};
use std::{fmt::Display, u64};

pub const DAY: u32 = 17;

fn solve_part1_impl(input: &Data) -> Result<Vec<u64>> {
    let mut reg_a = input.a;
    let mut reg_b = input.b;
    let mut reg_c = input.c;

    let program = &input.program;

    let mut output = Vec::new();

    let mut pc = 0;
    loop {
        let opcode = match program.get(pc) {
            Some(Memory::Opcode(opcode)) => opcode,
            None => break,
            other => anyhow::bail!("Invalid opcode at {pc}: {:?}", other),
        };
        pc += 1;
        let operand = match program.get(pc) {
            Some(Memory::Operand(operand)) => u64::from(*operand),
            _ => anyhow::bail!("Invalid operand"),
        };
        pc += 1;
        let combo = || {
            Ok(match operand {
                0 | 1 | 2 | 3 => u64::from(operand),
                4 => reg_a,
                5 => reg_b,
                6 => reg_c,
                _ => anyhow::bail!("Invalid combo: {operand} at pc {pc}"),
            })
        };
        match opcode {
            Opcode::Adv => reg_a = reg_a / (2u64.pow(combo()?.try_into()?)),
            Opcode::Bxl => reg_b = reg_b ^ operand,
            Opcode::Bst => reg_b = combo()? % 8,
            Opcode::Jnz => {
                if reg_a != 0 {
                    pc = operand.try_into()?;
                }
            }
            Opcode::Bxc => reg_b = reg_b ^ reg_c,
            Opcode::Out => output.push(combo()? % 8),
            Opcode::Bdv => reg_b = reg_a / (2u64.pow(combo()?.try_into()?)),
            Opcode::Cdv => reg_c = reg_a / (2u64.pow(combo()?.try_into()?)),
        }
    }

    Ok(output)
}

fn solve_part2_impl(_input: &Data) -> Result<usize> {
    // XXX: Solving logic for part 2
    Ok(0)
}

/// Solution to part 1
#[aoc(day17, part1)]
fn solve_part1(input: &str) -> Result<String> {
    let input = Data::parse(input).context("input parsing")?;
    let output = solve_part1_impl(&input)?;
    Ok(output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(","))
}

/// Solution to part 2
#[aoc(day17, part2)]
fn solve_part2(input: &str) -> Result<u64> {
    let input = Data::parse(input).context("input parsing")?;
    let mut min = 0u64;
    let mut max = u64::MAX;
    loop {
        let reg_a = u64::try_from((min as u128 + max as u128) / 2)?;
        let reg_a = min;
        println!("{min} {max} {reg_a}");
        let output = solve_part1_impl(&Data {
            a: reg_a,
            ..input.clone()
        })
        .unwrap();

        // let num = output
        //     .iter()
        //     .rev()
        //     .enumerate()
        //     .fold(0, |acc, (i, &x)| acc + x * 4u64.pow(i as u64));

        println!("{reg_a} {:?} {:?}", output, input.raw_program);

        if output.as_slice() == input.raw_program {
            return Ok(reg_a);
        }
        min += 1;
        continue;

        if output.len() < input.raw_program.len() {
            min = reg_a;
        } else if output.len() > input.raw_program.len() {
            max = reg_a;
        } else {
            println!("Equal length");
            // Walk backward and find the first difference
            let first_diff = output
                .iter()
                .zip(input.raw_program.iter())
                .rev()
                .find(|(a, b)| a != b)
                .unwrap();
            if first_diff.0 < first_diff.1 {
                max = reg_a;
            } else {
                min = reg_a;
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}
impl TryFrom<u64> for Opcode {
    type Error = anyhow::Error;

    fn try_from(value: u64) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Adv),
            1 => Ok(Opcode::Bxl),
            2 => Ok(Opcode::Bst),
            3 => Ok(Opcode::Jnz),
            4 => Ok(Opcode::Bxc),
            5 => Ok(Opcode::Out),
            6 => Ok(Opcode::Bdv),
            7 => Ok(Opcode::Cdv),
            _ => Err(anyhow::anyhow!("Invalid opcode {value}")),
        }
    }
}

#[derive(Debug, Clone)]
enum Memory {
    Opcode(Opcode),
    Operand(u64),
}

/// Problem input
#[derive(Debug, Clone)]
struct Data {
    // XXX: Change this to the actual data structure
    program: Vec<Memory>,
    raw_program: Vec<u64>,
    a: u64,
    b: u64,
    c: u64,
}
impl Data {
    fn parse(s: &str) -> Result<Self> {
        let (registers, program) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow::anyhow!("Invalid input"))?;
        let mut registers = registers.lines().map(|line| {
            Ok::<_, anyhow::Error>(
                line.split_once(": ")
                    .ok_or_else(|| anyhow::anyhow!("Invalid input"))?
                    .1
                    .parse::<u64>()?,
            )
        });

        let (_, program) = program
            .split_once(": ")
            .ok_or_else(|| anyhow::anyhow!("Invalid program"))?;
        let raw_program: Vec<u64> = program
            .split(",")
            .map(|num| Ok::<_, anyhow::Error>(num.parse::<u64>()?))
            .collect::<Result<_>>()?;

        let program = raw_program
            .iter()
            .enumerate()
            .map(|(i, &num)| {
                Ok(if i % 2 == 0 {
                    Memory::Opcode(Opcode::try_from(num)?)
                } else {
                    Memory::Operand(num.into())
                })
            })
            .collect::<Result<_>>()?;

        // XXX: Update the returned Data to include the parsed data.
        Ok(Data {
            a: registers
                .next()
                .ok_or_else(|| anyhow::anyhow!("No Reg"))??,
            b: registers
                .next()
                .ok_or_else(|| anyhow::anyhow!("No Reg"))??,
            c: registers
                .next()
                .ok_or_else(|| anyhow::anyhow!("No Reg"))??,
            raw_program,
            program,
        })
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
            solve_part1(&test_data(super::DAY).unwrap())
                .unwrap()
                .as_str(),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }

    #[test]
    fn part2_example() {
        let data = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
        assert_eq!(solve_part2(data).unwrap(), 117440);
    }
}
