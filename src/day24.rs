use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use itertools::Itertools as _;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
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

fn evaluate_z_multiple<'a>(
    operations: &'a [Operation],
    state: &mut State<'a>,
    swaps: &[(&'a str, &'a str)],
) -> Result<u64> {
    evaluate_ops_multiple(operations, state, swaps)?;

    let mut result = 0u64;
    for i in 0..64 {
        let z = format!("z{:02}", i);
        if let Some(&value) = state.get(z.as_str()) {
            result |= (value as u64) << i;
        }
    }
    Ok(result)
}

fn evaluate_ops_multiple<'a>(
    operations: &'a [Operation],
    state: &mut State<'a>,
    swaps: &[(&'a str, &'a str)],
) -> Result<()> {
    loop {
        let mut keep_going = false;
        for op in operations {
            if op.try_op(state, swaps).is_none() {
                keep_going = true;
            }
        }
        if !keep_going {
            break;
        }
    }
    Ok(())
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

    const VALID_BITS: u8 = 6;
    let valid_endpoints = (0..VALID_BITS)
        .map(|i| format!("z{:02}", i))
        .collect::<HashSet<_>>();

    // println!("op size before pruning: {}", operations.len());
    // // Remove operations that do not end in a valid endpoint
    // let operations = operations
    //     .iter()
    //     .filter(|&op| {
    //         let mut decendents = pathfinding::directed::dfs::dfs_reach(op, |op| {
    //             operations
    //                 .iter()
    //                 .filter(move |other| other.a == op.dest || other.b == op.dest)
    //                 .collect::<Vec<_>>()
    //         });
    //         decendents.any(|op| valid_endpoints.contains(op.dest))
    //     })
    //     .cloned()
    //     .collect::<Vec<_>>();

    // println!("op size after pruning: {}", operations.len());

    // let decendents = pathfinding::directed::dfs::dfs_reach("x00", |wire| {
    //     operations
    //         .iter()
    //         .filter(move |other| other.a == *wire || other.b == *wire)
    //         .map(|op| op.dest)
    //         .collect::<Vec<_>>()
    // })
    // .collect::<HashSet<_>>();
    // let x00ops = operations
    //     .iter()
    //     .filter(|op| decendents.contains(&op.dest))
    //     .cloned();

    // // output operations as a graphviz
    let wire_swaps: &[(&str, &str)] = &[
        ("qbw", "z14"),
        ("wcb", "z34"),
        ("wjb", "cvp"),
        ("mkk", "z10"),
    ];
    {
        use std::io::Write;
        let mut dotfile = std::fs::File::create("/tmp/graph.dot")?;
        writeln!(dotfile, "digraph G {{")?;
        // println!("graph TD;");
        let color = |op: &Op| match op {
            Op::AND => "green",
            Op::OR => "white",
            Op::XOR => "white", //"red",
        };

        fn wire_value<'a>(wire: &'a str, wire_swaps: &[(&'a str, &'a str)]) -> &'a str {
            // if the wire is a swap wire, return the swapped value
            wire_swaps
                .iter()
                .copied()
                .find_map(|(a, b)| {
                    if wire == a {
                        Some(b)
                    } else if wire == b {
                        Some(a)
                    } else {
                        None
                    }
                })
                .unwrap_or(wire)
        }

        // Color all z wires yellow
        for i in 0..=45 {
            writeln!(
                dotfile,
                "   z{:02}[label=\"z{:02}\" style=filled fillcolor=\"yellow\"]",
                i, i
            )?;
        }
        for i in 0..=44 {
            writeln!(
                dotfile,
                "   x{:02}[label=\"x{:02}\" style=filled fillcolor=\"brown\"]",
                i, i
            )?;
            writeln!(
                dotfile,
                "   y{:02}[label=\"y{:02}\" style=filled fillcolor=\"orange\"]",
                i, i
            )?;
        }
        for (i, op) in operations.iter().enumerate() {
            writeln!(
                dotfile,
                "   {}[label=\"{:?}\" style=filled fillcolor=\"{}\"]",
                i,
                op.op,
                color(&op.op)
            )?;
            let output = wire_value(op.dest, wire_swaps);
            // Find the position of the operations that use this output
            for dependent in operations.iter().enumerate().filter_map(|(j, op)| {
                if op.a == output || op.b == output {
                    Some(j)
                } else {
                    None
                }
            }) {
                writeln!(dotfile, "    {} -> {} [label=\"{}\"]", i, dependent, output)?;
            }
            if output.starts_with("z") {
                writeln!(dotfile, "    {} -> {} ", i, output)?;
            }
            if op.a.starts_with("x") || op.a.starts_with("y") {
                writeln!(dotfile, "    {} -> {}", op.a, i)?;
            }
            if op.b.starts_with("x") || op.b.starts_with("y") {
                writeln!(dotfile, "    {} -> {}", op.b, i)?;
            }
            //            writeln!(dotfile, "    {} -> {}", i, wire_value(op.dest, wire_swaps))?;

            //writeln!(dotfile, "    {} -> {}", op.a, i)?;
            //writeln!(dotfile, "    {} -> {}", op.b, i)?;
        }
        writeln!(dotfile, "}}")?;
    }

    let ops_with_x_parents = operations
        .iter()
        .filter(|op| op.a.starts_with("x") || op.b.starts_with("x"));
    // This must all be xor or and
    for op in ops_with_x_parents {
        if op.op != Op::XOR && op.op != Op::AND {
            return Err(anyhow::anyhow!("invalid operation: {:?}", op));
        }
    }
    let ops_with_y_parents = operations
        .iter()
        .filter(|op| op.a.starts_with("y") || op.b.starts_with("y"));
    // This must all be xor or and
    for op in ops_with_y_parents {
        if op.op != Op::XOR && op.op != Op::AND {
            return Err(anyhow::anyhow!("invalid operation: {:?}", op));
        }
    }

    fn num_as_bits(num: u64) -> String {
        (0..64)
            .map(|i| ((num >> i) & 1).to_string())
            .collect::<Vec<_>>()
            .join("")
    }

    let s = 2u64.pow(43) - 10;
    (s..s + 20).for_each(|num| {
        let mut state = input.start.clone();
        set_bits(num, &mut state, "x");
        set_bits(num, &mut state, "y");
        let res = evaluate_z_multiple(operations.as_slice(), &mut state, wire_swaps).expect("ans");
        println!("res: {}", num_as_bits(res));
        println!("exp: {}", num_as_bits(num + num));
    });

    todo!();

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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
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
