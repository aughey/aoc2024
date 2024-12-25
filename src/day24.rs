use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use itertools::Itertools as _;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub const DAY: u32 = 24;

fn sort_operations<'a>(start: &[&str], operations: &[Operation<'a>]) -> Result<Vec<Operation<'a>>> {
    // topological sort of operations
    let indicies = (0..(start.len() + operations.len())).collect::<Vec<_>>();
    let sorting =
        pathfinding::directed::topological_sort::topological_sort(indicies.as_slice(), |&i| {
            let output = if i < start.len() {
                start[i]
            } else {
                let op = &operations[i - start.len()];
                op.dest
            };
            operations
                .iter()
                .enumerate()
                .filter(move |(_, op)| op.a == output || op.b == output)
                .map(|(i, _)| i + start.len())
        })
        .map_err(|e| anyhow::anyhow!("topological sort failed: {:?}", e))?;
    Ok(sorting
        .into_iter()
        .filter_map(|i| i.checked_sub(start.len()))
        .map(|i| operations[i].clone())
        .collect())
}

fn solve_part1_impl(input: &Data) -> Result<usize> {
    let mut state = input.start.clone();

    let start = input.start.keys().copied().collect::<Vec<_>>();

    let operations = sort_operations(start.as_slice(), input.operations.as_slice())?;

    {
        // make sure all outputs are unique
        let op_outputs = operations.iter().map(|op| op.dest).collect::<HashSet<_>>();
        assert_eq!(op_outputs.len(), operations.len());
    }

    evaluate_z(operations.as_slice(), &mut state, &[])
}

fn evaluate_z<'a>(
    operations: &'a [Operation],
    state: &mut State<'a>,
    swaps: &[(&'a str, &'a str)],
) -> Result<usize> {
    evaluate_ops(operations, state, swaps)?;

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

fn evaluate_ops<'a>(
    operations: &'a [Operation],
    state: &mut State<'a>,
    swaps: &[(&'a str, &'a str)],
) -> Result<()> {
    for op in operations {
        op.try_op(state, swaps).ok_or_else(|| {
            anyhow::anyhow!("failed to evaluate operation: {op:?} with swaps {swaps:?}")
        })?;
    }
    Ok(())
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    let mut state = input.start.clone();

    let start = input.start.keys().copied().collect::<Vec<_>>();

    let operations = sort_operations(start.as_slice(), input.operations.as_slice())?;

    // // output operations as a mermaid graph
    // println!("graph TD;");
    // for (i, op) in operations.iter().enumerate() {
    //     println!("    {}({:?})", i, op.op);
    //     println!("    {} --> {}", i, op.dest);
    //     println!("    {} --> {}", op.a, i);
    //     println!("    {} --> {}", op.b, i);
    // }

    {
        // make sure all outputs are unique
        let op_outputs = operations.iter().map(|op| op.dest).collect::<HashSet<_>>();
        assert_eq!(op_outputs.len(), operations.len());
    }

    let outputs = operations.iter().map(|op| op.dest).collect::<Vec<_>>();

    // Make unique indicies of outputs
    let first_indicies = (0..outputs.len()).flat_map(|ax| {
        ((ax + 1)..outputs.len())
            .filter(move |&bx| ax != bx)
            .map(move |bx| (ax, bx))
    });
    fn pair_single_equality(&(a, b): &(usize, usize), other: usize) -> bool {
        a == other || b == other
    }
    fn pair_pair_equality(&(a, b): &(usize, usize), &(c, d): &(usize, usize)) -> bool {
        a == c || a == d || b == c || b == d
    }
    let second_indices = first_indicies.flat_map(|first| {
        ((first.1 + 1)..outputs.len())
            .filter(move |ax| !pair_single_equality(&first, *ax))
            .flat_map(|ax| ((ax + 1)..outputs.len()).map(move |bx| (ax, bx)))
            .filter(move |second| !pair_pair_equality(&first, second))
            .map(move |second| (first, second))
    });
    let third_indicies = second_indices.flat_map(|(first, second)| {
        ((second.1 + 1)..outputs.len())
            .filter(move |ax| {
                !pair_single_equality(&first, *ax) && !pair_single_equality(&second, *ax)
            })
            .flat_map(|ax| ((ax + 1)..outputs.len()).map(move |bx| (ax, bx)))
            .filter(move |third| {
                !pair_pair_equality(&first, third) && !pair_pair_equality(&second, third)
            })
            .map(move |third| (first, second, third))
    });
    let fourth_indices = third_indicies.flat_map(|(first, second, third)| {
        ((third.1 + 1)..outputs.len())
            .filter(move |ax| {
                !pair_single_equality(&first, *ax)
                    && !pair_single_equality(&second, *ax)
                    && !pair_single_equality(&third, *ax)
            })
            .flat_map(|ax| ((ax + 1)..outputs.len()).map(move |bx| (ax, bx)))
            .filter(move |fourth| {
                !pair_pair_equality(&first, fourth)
                    && !pair_pair_equality(&second, fourth)
                    && !pair_pair_equality(&third, fourth)
            })
            .map(move |fourth| (first, second, third, fourth))
    });

    println!(
        "number of wierd indicies things we have about is {}",
        fourth_indices.clone().count()
    );
    println!(
        "First sequences are: {:?}",
        fourth_indices.take(5).collect::<Vec<_>>()
    );

    let output_pairs = outputs
        .into_iter()
        .combinations(2)
        .map(|pair| (pair[0], pair[1]))
        .collect::<Vec<_>>();

    println!("number of output pairs: {}", output_pairs.len());

    // Now all combinations of 4 of these output pairs
    set_bits(5, &mut state, "x");
    set_bits(6, &mut state, "y");
    const EXPECTED: usize = 11;
    for swap_combo in output_pairs.into_iter().combinations(4) {
        let res = evaluate_z(operations.as_slice(), &mut state, swap_combo.as_slice())?;
        if res == EXPECTED {
            println!("found solution: {:?}", swap_combo);
            break;
        }
    }
    Ok(0)
}

fn set_bits(value: u64, state: &mut State, prefix: &str) {
    for i in 0..64 {
        let bit = (value >> i) & 1;
        let key = format!("{}{:02}", prefix, i);
        if let Some(cur) = state.get_mut(key.as_str()) {
            *cur = bit as u8;
        }
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
struct Operation<'a> {
    op: Op,
    a: &'a str,
    b: &'a str,
    dest: &'a str,
}
impl<'a> Operation<'a> {
    fn try_op(&self, state: &mut State<'a>, swaps: &[(&'a str, &'a str)]) -> Option<()> {
        let a = state.get(self.a)?;
        let b = state.get(self.b)?;
        let result = match self.op {
            Op::AND => a & b,
            Op::OR => a | b,
            Op::XOR => a ^ b,
        };
        let dest = swaps
            .iter()
            .copied()
            .find_map(|(a, b)| {
                if self.dest == a {
                    Some(b)
                } else if self.dest == b {
                    Some(a)
                } else {
                    None
                }
            })
            .unwrap_or(self.dest);
        state.insert(dest, result);
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
