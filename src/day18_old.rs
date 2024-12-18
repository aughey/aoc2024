use crate::{add_xy, Direction, GetCell, GetCellMut, Position, Result};
use anyhow::Context as _;
use std::{collections::HashSet, fmt::Display, hash::Hash};

pub const DAY: u32 = 18;

#[cfg(test)]
const MAPSIZE: Position = (7, 7);
#[cfg(not(test))]
const MAPSIZE: Position = (71, 71);

#[cfg(test)]
const FALL_COUNT: usize = 12;
#[cfg(not(test))]
const FALL_COUNT: usize = 1024;

trait Occupied<T> {
    fn contains(&self, value: &T) -> bool;
}

impl<V> Occupied<V> for HashSet<V>
where
    V: Hash + Eq,
{
    fn contains(&self, value: &V) -> bool {
        HashSet::contains(self, value)
    }
}

impl<C> Occupied<Position> for C
where
    C: GetCell<bool>,
{
    fn contains(&self, value: &Position) -> bool {
        let cell = self.get_cell(value);
        if let Some(cell) = cell {
            *cell
        } else {
            // If it's out of bounds, we consider it occupied
            // because we cannot move there.
            true
        }
    }
}

impl<C> Occupied<Position> for (&C, Position)
where
    C: Occupied<Position>,
{
    fn contains(&self, value: &Position) -> bool {
        let bounds = &self.1;
        let chain = &self.0;
        if value.0 >= bounds.0 || value.1 >= bounds.1 {
            true
        } else {
            chain.contains(value)
        }
    }
}

fn solve_part1_impl(input: &Data) -> Result<usize> {
    // Simulate falling (use hash set this time for funzies)
    let map = input
        .coords
        .iter()
        .take(FALL_COUNT)
        .copied()
        .collect::<HashSet<_>>();

    // Do pathfinding
    let shortest = pathfinding::directed::dijkstra::dijkstra(
        &(0, 0),
        |xy| valid_map_steps(&(&map, MAPSIZE), *xy),
        |coord| *coord == (MAPSIZE.0 - 1, MAPSIZE.1 - 1),
    );

    // print_map(
    //     &map,
    //     &shortest.as_ref().unwrap().0.iter().copied().collect(),
    //     MAPSIZE,
    // );

    shortest
        .map(|(coords, _)| coords.len() - 1)
        .ok_or_else(|| anyhow::anyhow!("no path found"))
}

#[allow(dead_code)]
fn print_map(map: &HashSet<Position>, path: &HashSet<Position>, map_size: Position) {
    for y in 0..map_size.1 {
        for x in 0..map_size.0 {
            let c = if path.contains(&(x, y)) {
                'O'
            } else if map.contains(&(x, y)) {
                '#'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
}

fn solve_part2_impl(input: &Data) -> Result<Position> {
    let next_cell = input.coords.iter().copied().enumerate();

    // Simulate falling (use vector this time for funzies)
    // For part 2 the map is incrementally built
    let mut map = vec![vec![false; MAPSIZE.0]; MAPSIZE.1];
    let mut map = map.as_mut_slice();

    let mut prev_path: Option<Vec<Position>> = None;

    // The critical path now doesn't allocate and is fast.
    // Well... except for the generation of the vec that the pathfinding
    // library does.
    for (_count, cell) in next_cell {
        // Add the cell to the map
        {
            let c = map
                .get_cell_mut(&cell)
                .ok_or_else(|| anyhow::anyhow!("cell out of range"))?;
            *c = true;
        }

        // Check if this will change the best path.
        if let Some(prev_path) = prev_path.as_ref() {
            // if this cell didn't block the path it won't change the
            // length of the shortest path so we don't need to check.
            if !prev_path.contains(&cell) {
                continue;
            }
        }

        let shortest_path = pathfinding::directed::dijkstra::dijkstra(
            &(0, 0),
            |xy| valid_map_steps(&map, *xy),
            |coord| *coord == (MAPSIZE.0 - 1, MAPSIZE.1 - 1),
        );
        // astar_bag is quite slower than dijkstra for this map
        // let shortest = pathfinding::directed::astar::astar_bag(
        //     &(0, 0),
        //     |xy| valid_map_steps(map.clone(), *xy),
        //     |_| 0,
        //     |coord| *coord == (MAPSIZE.0 - 1, MAPSIZE.1 - 1),
        // );

        // if let Some(mut shortest) = shortest {
        //     prev_path = shortest.0.next();
        // } else {
        //     return Ok(cell);
        // }

        // If we found shortest path.
        if let Some(shortest_path) = shortest_path {
            prev_path = Some(shortest_path.0);
        } else {
            return Ok(cell);
        }
    }
    anyhow::bail!("No solution found")
}

fn valid_map_steps(
    map: &impl Occupied<Position>,
    xy: Position,
) -> impl Iterator<Item = (Position, usize)> {
    const DIRECTIONS: &[Direction] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut can_step = DIRECTIONS.iter().filter_map(move |delta| {
        let xy = add_xy(&xy, delta)?;
        if map.contains(&xy) {
            None
        } else {
            Some((xy, 1))
        }
    });

    // So we don't have to hold on to the map reference, we pre-calculate
    // the steps in the four directions.
    let steps = [
        can_step.next(),
        can_step.next(),
        can_step.next(),
        can_step.next(),
    ];

    // If we think about this, can_step will return valid steps until the
    // iterator runs out (because we use filter_map).  The first None
    // is the last none, so the pattern of take_while(is_some) and map(unwrap)
    // is safe and valid.
    // steps
    //     .into_iter()
    //     .take_while(Option::is_some)
    //     .map(Option::unwrap)

    // However, we're going to use flatten just to completely avoid unwrapping
    steps.into_iter().flatten()
}

/// Solution to part 1
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
fn solve_part2(input: &str) -> Result<String> {
    let input = Data::parse(input).context("input parsing")?;
    let ans = solve_part2_impl(&input)?;
    Ok(format!("{},{}", ans.0, ans.1))
}

/// Problem input
#[derive(Debug)]
struct Data {
    coords: Vec<Position>,
}
impl Data {
    fn parse(s: &str) -> Result<Self> {
        let s = s.lines();
        let coords = s
            .map(|line| {
                line.split_once(",")
                    .ok_or_else(|| anyhow::anyhow!("bad split"))
            })
            .map(|xy| {
                let (x, y) = xy?;
                Ok((x.parse()?, y.parse()?))
            })
            .collect::<Result<_>>()?;

        Ok(Data { coords })
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
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&test_data(super::DAY).unwrap()).unwrap(), "6,1");
    }
}
