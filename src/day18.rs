use crate::{add_xy, Direction, GetCell as _, GetCellMut, Position, Result};
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::{cell::RefCell, collections::HashSet, fmt::Display, rc::Rc};

pub const DAY: u32 = 18;

#[cfg(test)]
const MAPSIZE: Position = (7, 7);
#[cfg(not(test))]
const MAPSIZE: Position = (71, 71);

#[cfg(test)]
const FALL_COUNT: usize = 12;
#[cfg(not(test))]
const FALL_COUNT: usize = 1024;

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
        |xy| {
            const DIRECTIONS: &[Direction] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
            DIRECTIONS
                .iter()
                .filter_map(|delta| {
                    let new_pos = add_xy(xy, delta)?;
                    if new_pos.0 >= MAPSIZE.0 || new_pos.1 >= MAPSIZE.1 {
                        return None;
                    }
                    if !map.contains(&new_pos) {
                        Some(new_pos)
                    } else {
                        None
                    }
                })
                .map(|pos| (pos, 1))
                .collect::<Vec<_>>()
        },
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
    let map = vec![vec![false; MAPSIZE.0]; MAPSIZE.1];

    let mut prev_path: Option<Vec<Position>> = None;

    // all of this just to avoid allocating a vec for the successors generation.
    // The purpose of the Rc is so that the iterator returned in the successor
    // call is wholely owned by the closure and the iterator doesn't need to be
    // bound to any lifetime.
    let map: Rc<RefCell<Vec<Vec<bool>>>> = Rc::new(RefCell::new(map));

    // The critical path now doesn't allocate and is fast.
    // Well... except for the generation of the vec that the pathfinding
    // library does.
    for (_count, cell) in next_cell {
        {
            // Drilling down into the RefCell is a bit verbose.
            let mut c = map.try_borrow_mut()?;
            let mut c = c.as_mut_slice();
            let c = c
                .get_cell_mut(&cell)
                .ok_or_else(|| anyhow::anyhow!("cell out of range"))?;
            *c = true;
        }

        if let Some(prev_path) = prev_path.as_ref() {
            // if this cell didn't block the path it won't change the
            // length of the shortest path so we don't need to check.
            if !prev_path.contains(&cell) {
                continue;
            }
        }

        let shortest = pathfinding::directed::dijkstra::dijkstra(
            &(0, 0),
            |xy| valid_map_steps(map.clone(), *xy),
            |coord| *coord == (MAPSIZE.0 - 1, MAPSIZE.1 - 1),
        );
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
        if let Some(shortest) = shortest {
            prev_path = Some(shortest.0);
        } else {
            return Ok(cell);
        }
    }
    anyhow::bail!("No solution found")
}

fn valid_map_steps(
    map: Rc<RefCell<Vec<Vec<bool>>>>,
    xy: Position,
) -> impl Iterator<Item = (Position, usize)> {
    const DIRECTIONS: &[Direction] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
    DIRECTIONS
        .iter()
        .filter_map(move |delta| add_xy(&xy, delta))
        .filter_map(move |xy| {
            let map = map.borrow();
            let map = map.as_slice();
            if *map.get_cell(&xy)? {
                None
            } else {
                Some(xy)
            }
        })
        .map(|pos| (pos, 1))
}

/// Solution to part 1
#[aoc(day18, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day18, part2)]
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
