use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::{aoc, aoc_generator};
use std::{collections::HashSet, fmt::Display, str::FromStr};
use tracing::info;

pub const DAY: u32 = 8;

trait XY {
    fn xy(&self) -> &glam::IVec2;
    fn frequency(&self) -> char;
}

/// Parsing logic uses the FromStr trait
#[aoc_generator(day8)]
fn parse(input: &str) -> Result<Data> {
    info!("Parsing input");
    Data::from_str(input).context("input parsing")
}

/// Solution to part 1
#[aoc(day8, part1)]
fn solve_part1(input: &impl DataShape) -> Result<usize> {
    let max_xy = &input.max_xy();

    let antinodes = input.resonate_pairs().map(|ab| {
        let (a, b) = ab?;
        // We skip the first node because it's the same as the second node.
        // We only take 1 node because part one only considers the first antinode.
        let forward_locations = anitnode_generator(&a, &b).skip(1).take(1);
        let backward_locations = anitnode_generator(&b, &a).skip(1).take(1);

        // neat little trick to capture max_xy so take_while looks clean
        let on_map = |node: &Node| on_map(node, max_xy);

        let valid_forward_locations = forward_locations.take_while(on_map);
        let valid_backward_locations = backward_locations.take_while(on_map);

        Ok::<_, anyhow::Error>(valid_forward_locations.chain(valid_backward_locations))
    });

    // Because of the inner result, we need to unwrap the inner result manually.
    // There should be a flat_map_results or something similar in the future.
    let mut antinode_positions = HashSet::new();
    for antinode in antinodes {
        let antinode = antinode?;
        antinode_positions.extend(antinode.map(|n| n.xy));
    }
    // antinodes.map(|node| Ok(node?.map(|n| n.xy)).collect::<Result<HashSet<_>>>()?;

    Ok(antinode_positions.len())
}

/// Generates antinodes for a given pair of nodes in the direction of a->b
///
/// Given input that looks like `a - b`,
/// it will generate * nodes `a - * - * - *....`
/// where * are the new nodes geneated and includes node b.
///
/// If you want it in the other direction, call it with `b` and `a` instead.
fn anitnode_generator(a: &impl XY, b: &impl XY) -> impl Iterator<Item = Node> {
    let diff = b.xy() - a.xy();
    let xy = *b.xy();
    let frequency = b.frequency();
    (0..)
        // compute the new xy
        .map(move |i| xy + diff * i)
        // create the new node
        .map(move |xy| Node { frequency, xy })
}

/// Checks if a node is within the bounds of the map bounds
fn on_map(node: &Node, max_xy: &glam::IVec2) -> bool {
    node.xy.x >= 0 && node.xy.y >= 0 && node.xy.x < max_xy.x && node.xy.y < max_xy.y
}

/// Solution to part 2
#[aoc(day8, part2)]
fn solve_part2(input: &impl DataShape) -> Result<usize> {
    let max_xy = &input.max_xy();

    let antinodes = input.resonate_pairs().map(|ab| {
        let (a, b) = ab?;
        let forward_locations = anitnode_generator(&a, &b);
        let backward_locations = anitnode_generator(&b, &a);

        // neat little trick
        let on_map = |node: &Node| on_map(node, max_xy);

        let valid_forward_locations = forward_locations.take_while(on_map);
        let valid_backward_locations = backward_locations.take_while(on_map);

        Ok::<_, anyhow::Error>(valid_forward_locations.chain(valid_backward_locations))
    });

    let mut antinode_positions = HashSet::new();
    for antinode in antinodes {
        let antinode = antinode?;
        antinode_positions.extend(antinode.map(|n| n.xy));
    }
    info!("antinode_positions: {:?}", antinode_positions);

    Ok(antinode_positions.len())
}

#[derive(Debug, Clone)]
struct Node {
    frequency: char,
    xy: glam::IVec2,
}
impl XY for Node {
    fn xy(&self) -> &glam::IVec2 {
        &self.xy
    }

    fn frequency(&self) -> char {
        self.frequency
    }
}

impl XY for &Node {
    fn xy(&self) -> &glam::IVec2 {
        &self.xy
    }

    fn frequency(&self) -> char {
        self.frequency
    }
}

/// Problem input
#[derive(Debug)]
struct Data {
    nodes: Vec<Node>,
    max_xy: glam::IVec2,
}

/// Generates all pairs of a given iterator.
/// This is equivalent to `iter.combinations(2)`.
fn pair_combinations<T, IT>(iter: IT) -> impl Iterator<Item = (T, T)>
where
    T: Clone,
    IT: Iterator<Item = T> + Clone,
{
    let mut a = iter;
    let mut left = a.next();
    let mut b = a.clone();
    std::iter::from_fn(move || loop {
        match (left.clone(), b.next()) {
            (Some(left), Some(right)) => return Some((left, right)),
            (Some(_), None) => {
                left = Some(a.next()?);
                b = a.clone();
            }
            _ => return None,
        }
    })
}

trait DataShape {
    type RPNODE<'a>: XY
    where
        Self: 'a;
    fn resonate_pairs<'a>(
        &'a self,
    ) -> impl Iterator<Item = Result<(Self::RPNODE<'a>, Self::RPNODE<'a>)>> + 'a;

    fn max_xy(&self) -> glam::IVec2;
}

impl DataShape for Data {
    type RPNODE<'a>
        = &'a Node
    where
        Self: 'a;
    fn resonate_pairs<'a>(&'a self) -> impl Iterator<Item = Result<(&'a Node, &'a Node)>> + 'a {
        // impl Iterator<Item = Result<(&Node, &Node)>> {
        return pair_combinations(self.nodes.iter())
            .map(|(a, b)| Ok((a, b)))
            .filter(|ab| {
                ab.as_ref()
                    .is_ok_and(|(a, b)| a.frequency() == b.frequency())
            });
        //use itertools::Itertools as _;
        // self.nodes
        //     .iter()
        //     .combinations(2)
        //     .map(|pair| (pair[0], pair[1]))
        //     .filter(|(a, b)| a.frequency == b.frequency)
        //     .map(Ok)
    }

    fn max_xy(&self) -> glam::IVec2 {
        self.max_xy
    }
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.lines();

        let nodes = s.clone().enumerate().flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c != &'.')
                .map(move |(x, c)| {
                    Ok(Node {
                        frequency: c,
                        xy: glam::IVec2::new(x.try_into()?, y.try_into()?),
                    })
                })
        });
        //   .collect::<Result<Vec<_>>>()?;

        let max_y = s.clone().count();
        let max_x = s.clone().next().unwrap().len();

        Ok(Data {
            nodes: nodes.collect::<Result<Vec<_>>>()?,
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

    #[test]
    fn test_pair_combinators() {
        let res = super::pair_combinations([0, 1, 2, 3].into_iter()).collect::<Vec<_>>();
        assert_eq!(res, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)])
    }
}
