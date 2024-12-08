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
    let max_xy = &input.max_xy;

    let antinodes = input.resonate_pairs().flat_map(|(a, b)| {
        // We skip the first node because it's the same as the second node.
        // We only take 1 node because part one only considers the first antinode.
        let forward_locations = anitnode_generator(a, b).skip(1).take(1);
        let backward_locations = anitnode_generator(b, a).skip(1).take(1);

        // neat little trick to capture max_xy so take_while looks clean
        let on_map = |node: &Node| on_map(node, max_xy);

        let valid_forward_locations = forward_locations.take_while(on_map);
        let valid_backward_locations = backward_locations.take_while(on_map);

        valid_forward_locations.chain(valid_backward_locations)
    });

    let antinode_positions = antinodes.map(|node| node.xy).collect::<HashSet<_>>();

    Ok(antinode_positions.len())
}

/// Generates antinodes for a given pair of nodes in the direction of a->b
/// Given input that looks like `a - b`,
/// it will generate * nodes `a - * - * - *....`
/// Includes node b
fn anitnode_generator(a: &Node, b: &Node) -> impl Iterator<Item = Node> {
    let diff = b.xy - a.xy;
    let xy = b.xy;
    let frequency = b.frequency;
    (0..)
        .map(move |i| xy + diff * i)
        .map(move |xy| Node { frequency, xy })
}

fn on_map(node: &Node, max_xy: &glam::IVec2) -> bool {
    node.xy.x >= 0 && node.xy.y >= 0 && node.xy.x < max_xy.x && node.xy.y < max_xy.y
}

/// Solution to part 2
#[aoc(day8, part2)]
fn solve_part2(input: &Data) -> Result<usize> {
    let max_xy = &input.max_xy;

    let antinodes = input.resonate_pairs().flat_map(|(a, b)| {
        let forward_locations = anitnode_generator(a, b);
        let backward_locations = anitnode_generator(b, a);

        // neat little trick
        let on_map = |node: &Node| on_map(node, max_xy);

        let valid_forward_locations = forward_locations.take_while(on_map);
        let valid_backward_locations = backward_locations.take_while(on_map);

        valid_forward_locations.chain(valid_backward_locations)
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
struct Data<NI>
where
    NI: Iterator<Item = Node>,
{
    nodes: NI, //Vec<Node>,
    max_xy: glam::IVec2,
}

fn pair_combinations<T, IT>(iter: IT) -> impl Iterator<Item = (T, T)>
where
    IT: Iterator<Item = T> + Clone,
{
    let mut a = iter;
    let mut b = a.clone();
    b.next();
    std::iter::from_fn(move || loop {
        match (a.next(), b.next()) {
            (Some(left), Some(b)) => return Some((left, b)),
            (Some(_), None) => {
                a.next();
                b = a.clone();
                b.next();
            }
            (None, _) => return None,
        }
    })
}

impl<NI> Data<NI>
where
    NI: Iterator<Item = Node> + Clone,
{
    fn resonate_pairs(&self) -> impl Iterator<Item = (Node, Node)> {
        pair_combinations(self.nodes.clone())
            // .clone()
            // .combinations(2)
            // .map(|pair| (pair[0], pair[1]))
            .filter(|(a, b)| a.frequency == b.frequency)
    }
}

impl<IT> FromStr for Data<IT>
where
    IT: Iterator<Item = Node>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.lines();

        let nodes = s.clone().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c != &'.')
                .map(move |(x, c)| Node {
                    frequency: c,
                    xy: glam::IVec2::new(x.try_into().unwrap(), y.try_into().unwrap()),
                })
        });
        //            .collect::<Result<Vec<_>>>()?;

        let max_y = s.clone().count();
        let max_x = s.clone().next().unwrap().len();

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
