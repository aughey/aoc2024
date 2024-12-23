use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use itertools::Itertools as _;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Display,
    hash::Hash,
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

    let longest_groups = nodes.into_iter().map(|n| {
        let mut longest = HashSet::new();
        longest_group(
            n,
            &mappings,
            &mut BTreeSet::new(),
            &mut longest,
            &mut HashSet::new(),
        );
        longest
    });
    let longest_group = longest_groups
        .max_by_key(|g| g.len())
        .ok_or_else(|| anyhow::anyhow!("No groups found"))?;

    let password = longest_group.iter().sorted().join(",");

    Ok(password)
}

fn longest_group<'a>(
    node: &'a str,
    mappings: &'a HashMap<&'a str, HashSet<&'a str>>,
    current_group: &mut BTreeSet<&'a str>,
    longest: &mut HashSet<&'a str>,
    seen: &mut HashSet<Vec<&'a str>>,
) -> bool {
    let key = current_group.iter().copied().collect::<Vec<_>>();
    if seen.contains(&key) {
        return false;
    }

    if current_group.contains(node) {
        return false;
    }
    // If all nodes are not connected to this node
    let my_connections = mappings.get(node).unwrap();
    if !current_group.iter().all(|n| my_connections.contains(n)) {
        return false;
    }

    current_group.insert(node);
    let children = mappings.get(node).unwrap();
    let mut added = false;
    for child in children {
        if longest_group(child, mappings, current_group, longest, seen) {
            added = true;
        }
    }
    if added == false {
        // See if we're now the longest
        if current_group.len() > longest.len() {
            longest.clear();
            longest.extend(current_group.iter().copied());
        }
    }
    current_group.remove(node);
    seen.insert(key);
    added
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
            "co,de,ka,ta"
        );
    }
}
