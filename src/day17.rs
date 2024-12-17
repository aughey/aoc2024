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

    // let mut last = None;
    // let mut count = 0;
    // for reg_a in 0.. {
    //     let output = solve_part1_impl(&Data {
    //         a: reg_a,
    //         ..input.clone()
    //     })?;
    //     let last_output = output.iter().rev().take(1).copied().next().unwrap();
    //     count += 1;
    //     if let Some(l) = last {
    //         if last_output != l {
    //             println!("reg_a {}, output: {:?}, count: {}", reg_a, output, count);
    //             last = Some(last_output);
    //             count = 0;
    //         }
    //     } else {
    //         last = Some(last_output);
    //     }
    // }
    // return Ok(0);

    let mut exp = 15;
    let mut reg_a = 8u64.pow(exp);

    assert_eq!(
        solve_part1_impl(&Data {
            a: reg_a,
            ..input.clone()
        })?
        .len(),
        input.raw_program.len()
    );
    assert_eq!(
        solve_part1_impl(&Data {
            a: reg_a - 1,
            ..input.clone()
        })?
        .len(),
        input.raw_program.len() - 1
    );

    //    for index in 0..14 {
    let mut exps = [0; 14];
    loop {
        let mut reg_a = 8u64.pow(15);
        for (i, count) in exps.iter().enumerate() {
            reg_a += 64 * 8u64.pow(13 - i as u32) * count;
        }

        let output = solve_part1_impl(&Data {
            a: reg_a,
            ..input.clone()
        })?;
        assert_eq!(output.len(), input.raw_program.len());

        println!(
            "reg_a: {}, exps: {:?}, output: {:?}, program: {:?}",
            reg_a, exps, output, input.raw_program
        );

        let mut check = output.iter().rev().zip(input.raw_program.iter().rev());

        let bad_index = check.position(|(a, b)| a != b);
        if let Some(bad_index) = bad_index {
            if bad_index >= exps.len() {
                break;
            }
            exps[bad_index] += 1;
            for exp in exps.iter_mut().skip(bad_index + 1) {
                *exp = 0;
            }
        } else {
            break;
        }
    }

    let mut reg_a = 8u64.pow(15);
    for (i, count) in exps.iter().enumerate() {
        reg_a += 64 * 8u64.pow(13 - i as u32) * count;
    }

    //   }
    println!("Brute");
    // Brute force the last
    for _ in 0.. {
        let output = solve_part1_impl(&Data {
            a: reg_a,
            ..input.clone()
        })?;
        assert_eq!(output.len(), input.raw_program.len());
        // println!(
        //     "reg_a: {}, exp: {}, output: {:?}, input: {:?}",
        //     reg_a, exp, output, input.raw_program
        // );
        if output.as_slice() == input.raw_program.as_slice() {
            break;
        }

        reg_a += 1;
    }
    Ok(reg_a)
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
