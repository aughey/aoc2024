use crate::Result;
use aoc_runner_derive::aoc;
use day8_impl::{DataShape, Node, XY};
use std::{fmt::Display, str::FromStr};

pub const DAY: u32 = 8;

#[aoc(day8, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::from_str(input)?;
    //let input = DataNoStd:j:new(input);
    day8_impl::solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day8, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::from_str(input)?;
    //let input = DataNoStd::new(input);
    day8_impl::solve_part2_impl(&input)
}

mod day8_impl;

impl DataShape for Data {
    type RPNODE<'a>
        = &'a Node
    where
        Self: 'a;
    fn resonate_pairs(&self) -> impl Iterator<Item = Result<(&Node, &Node)>> + '_ {
        // impl Iterator<Item = Result<(&Node, &Node)>> {
        pair_combinations(self.nodes.iter())
            .map(|(a, b)| Ok((a, b)))
            .filter(|ab| {
                ab.as_ref()
                    .is_ok_and(|(a, b)| a.frequency() == b.frequency())
            })
    }

    fn max_xy(&self) -> Result<glam::IVec2> {
        Ok(self.max_xy)
    }
}

fn parse_nodes(s: &str) -> impl Iterator<Item = Result<Node>> + '_ + Clone {
    s.lines().enumerate().flat_map(|(y, line)| {
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
}

fn parse_maxxy(s: &str) -> Result<glam::IVec2> {
    let max_y = s.lines().count();
    let max_x = s.lines().next().unwrap().len();
    Ok(glam::IVec2::new(max_x.try_into()?, max_y.try_into()?))
}

impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            nodes: parse_nodes(s).collect::<Result<_>>()?,
            max_xy: parse_maxxy(s)?,
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

/// Problem input
#[derive(Debug)]
pub struct Data {
    pub nodes: Vec<Node>,
    pub max_xy: glam::IVec2,
}

pub struct DataNoStd<'a> {
    pub s: &'a str,
}

impl<'a> DataNoStd<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { s }
    }
}

impl DataShape for DataNoStd<'_> {
    type RPNODE<'a>
        = Node
    where
        Self: 'a;
    fn resonate_pairs(
        &self,
    ) -> impl Iterator<Item = Result<(Self::RPNODE<'_>, Self::RPNODE<'_>)>> + '_ {
        let nodes = parse_nodes(self.s);
        let nodes = nodes.map(|n| n.unwrap());
        pair_combinations(nodes)
            .filter(|(a, b)| a.frequency == b.frequency)
            .map(Ok)
    }

    fn max_xy(&self) -> Result<glam::IVec2> {
        parse_maxxy(self.s)
    }
}

/// Generates all pairs of a given iterator.
/// This is equivalent to `iter.combinations(2)`.
pub fn pair_combinations<T, IT>(iter: IT) -> impl Iterator<Item = (T, T)> + Clone
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

#[cfg(test)]
mod tests {
    use crate::test_data;
    use test_log::test;

    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&test_data(super::DAY).unwrap()).unwrap(), 34);
    }

    #[test]
    fn test_pair_combinators() {
        let res = super::pair_combinations([0, 1, 2, 3].into_iter()).collect::<Vec<_>>();
        assert_eq!(res, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)])
    }
}
