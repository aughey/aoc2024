use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use itertools::Itertools as _;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub const DAY: u32 = 23;

fn solve_part1_impl(input: &Data) -> Result<usize> {
    let mut mappings = HashMap::<&str, HashSet<&str>>::new();
    for (a, b) in &input.connections {
        mappings.entry(a).or_default().insert(b);
        mappings.entry(b).or_default().insert(a);
    }

    let nodes = mappings.keys().copied().collect::<Vec<_>>();

    let permutations = nodes.iter().combinations(3);
    let all_connected = permutations.filter(|p| {
        mappings.get(p[0]).unwrap().contains(p[1])
            && mappings.get(p[1]).unwrap().contains(p[2])
            && mappings.get(p[2]).unwrap().contains(p[0])
    });
    let with_t = all_connected.filter(|p| p.iter().any(|x| x.starts_with('t')));

    Ok(with_t.count())
}

fn solve_part2_impl(input: &Data) -> Result<String> {
    let mut mappings = HashMap::<&str, HashSet<&str>>::new();
    for (a, b) in &input.connections {
        mappings.entry(a).or_default().insert(b);
        mappings.entry(b).or_default().insert(a);
    }

    let nodes = mappings.keys().copied().collect::<Vec<_>>();

    let mut largest: Option<HashSet<&str>> = None;
    for node in nodes.iter().copied() {
        for child in mappings.get(node).unwrap().iter().copied() {
            let mut cluster = HashSet::new();
            cluster.insert(node);
            cluster.insert(child);

            for possible in nodes.iter().copied() {
                let possible_children = mappings.get(possible).unwrap();
            }
        }

        let nodes_children = mappings.get(node).unwrap();
        for child in nodes_children.iter().copied() {
            let grandchildren = mappings.get(child).unwrap();
            if grandchildren.contains(node) {
                cluster.insert(child);
            }
        }
        cluster.insert(node);
        if largest
            .as_ref()
            .map(|l| cluster.len() > l.len())
            .unwrap_or(true)
        {
            largest = Some(cluster);
        }
    }
    println!("largest: {:?}", largest);
    let largest = largest.ok_or_else(|| anyhow::anyhow!("no largest cluster"))?;

    let password = largest.into_iter().sorted().join(",");

    Ok(password)
}

/// Solution to part 1
#[aoc(day23, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day23, part2)]
fn solve_part2(input: &str) -> Result<String> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

/// Problem input
#[derive(Debug)]
struct Data<'a> {
    // XXX: Change this to the actual data structure
    connections: Vec<(&'a str, &'a str)>,
}
impl<'a> Data<'a> {
    fn parse(s: &'a str) -> Result<Self> {
        // XXX: Do actual parsing here.
        let s = s.lines();
        let connections = s
            .map(|line| {
                line.split_once('-')
                    .ok_or_else(|| anyhow::anyhow!("Invalid line {line}"))
            })
            .collect::<Result<Vec<_>>>()?;
        // XXX: Update the returned Data to include the parsed data.
        Ok(Data { connections })
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
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&test_data(super::DAY).unwrap()).unwrap(),
            "co,de,kd,ta"
        );
    }
}
