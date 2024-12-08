use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools as _;
use std::{collections::HashSet, fmt::Display, str::FromStr};
use tracing::info;

pub const DAY: u32 = 8;

/// Parsing logic uses the FromStr trait
#[aoc_generator(day8)]
fn parse(input: &str) -> Result<Data> {
    info!("Parsing input");
    Data::from_str(input).context("input parsing")
}

/// Solution to part 1
#[aoc(day8, part1)]
fn solve_part1(input: &Data) -> Result<usize> {
    // XXX: Solving logic for part 1
    let nodes = &input.nodes;
    let max_xy = &input.max_xy;

    let pairs = nodes
        .iter()
        .combinations(2)
        .map(|pair| (pair[0], pair[1]))
        .filter(|(a, b)| a.frequency == b.frequency);
    let antinodes = pairs.flat_map(|(a, b)| {
        let freq = a.frequency;
        let diff = a.xy - b.xy;
        let forward_xy = a.xy + diff;
        let backward_xy = b.xy - diff;
        [
            Node {
                frequency: freq,
                xy: forward_xy,
            },
            Node {
                frequency: freq,
                xy: backward_xy,
            },
        ]
        .into_iter()
        .filter(|node| {
            node.xy.x >= 0 && node.xy.y >= 0 && node.xy.x < max_xy.x && node.xy.y < max_xy.y
        })
    });

    let antinode_positions = antinodes.map(|node| node.xy).collect::<HashSet<_>>();

    Ok(antinode_positions.len())
}

/// Solution to part 2
#[aoc(day8, part2)]
fn solve_part2(input: &Data) -> Result<usize> {
    // XXX: Solving logic for part 1
    let nodes = &input.nodes;
    let max_xy = &input.max_xy;

    let pairs = nodes
        .iter()
        .combinations(2)
        .map(|pair| (pair[0], pair[1]))
        .filter(|(a, b)| a.frequency == b.frequency);
    let antinodes = pairs.flat_map(|(a, b)| {
        let freq = a.frequency;
        let diff = a.xy - b.xy;
        let xy = a.xy;

        let forward_locations = (0..).map(move |i| xy + diff * i);
        let backward_locations = (0..).map(move |i| xy - diff * i);

        let valid_forward_locations = forward_locations
            .take_while(|xy| xy.x >= 0 && xy.y >= 0 && xy.x < max_xy.x && xy.y < max_xy.y);
        let valid_backward_locations = backward_locations
            .take_while(|xy| xy.x >= 0 && xy.y >= 0 && xy.x < max_xy.x && xy.y < max_xy.y);

        let locations = valid_forward_locations.chain(valid_backward_locations);
        locations.map(move |xy| Node {
            frequency: freq,
            xy,
        })
    });

    let antinode_positions = antinodes.map(|node| node.xy).collect::<HashSet<_>>();
    info!("antinode_positions: {:?}", antinode_positions);

    Ok(antinode_positions.len())
}

#[derive(Debug, Clone)]
struct Node {
    frequency: char,
    xy: glam::IVec2,
}

/// Problem input
#[derive(Debug)]
struct Data {
    nodes: Vec<Node>,
    max_xy: glam::IVec2,
}
impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.lines();

        let nodes = s
            .clone()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c != &'.')
                    .map(move |(x, c)| {
                        Ok(Node {
                            frequency: c,
                            xy: glam::IVec2::new(x.try_into()?, y.try_into()?),
                        })
                    })
            })
            .collect::<Result<Vec<_>>>()?;

        let max_y = s.clone().count();
        let max_x = s.clone().next().unwrap().len();
        info!("max_x: {}, max_y: {}", max_x, max_y);

        Ok(Data {
            nodes,
            max_xy: glam::IVec2::new(max_x.try_into()?, max_y.try_into()?),
        })
    }
}

/// codspeed compatible function
pub fn part1(input: &str) -> impl Display {
    solve_part1(&parse(input).unwrap()).unwrap()
}

/// codspeed compatible function
pub fn part2(input: &str) -> impl Display {
    solve_part2(&parse(input).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::test_data;
    use test_log::test;

    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            solve_part1(&parse(&test_data(super::DAY).unwrap()).unwrap()).unwrap(),
            14
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&parse(&test_data(super::DAY).unwrap()).unwrap()).unwrap(),
            34
        );
    }
}
