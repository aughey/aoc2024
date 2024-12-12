use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Display,
};
use tracing::info;

pub const DAY: u32 = 12;

fn solve_part1_impl(input: &Data) -> Result<usize> {
    // make a grid of nones
    let mut seen: HashSet<XY> = HashSet::new();

    let grid = &input.grid;

    let points = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (x, y)));

    let scores = points.clone().filter_map(move |point| {
        if !seen.insert(point) {
            return None;
        }
        seen.insert(point);

        let this_c = input.grid[point.1][point.0];
        info!("point: {:?} {}", point, this_c);

        let mut connected = vec![point];
        loop {
            let mut found = false;
            for p in points
                .clone()
                .filter(|p| grid[p.1][p.0] == this_c)
                .filter(|p| !seen.contains(p))
                .filter(|p| connected.iter().any(|c| next_to(c, p)))
                .collect::<Vec<_>>()
            {
                seen.insert(p);
                connected.push(p);
                found = true
            }
            if !found {
                break;
            }
        }
        info!("connected: {this_c} {:?}", connected);
        const DIRECTIONS: &[Direction] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
        let fence_count = connected
            .iter()
            .map(|p| {
                let fence_sides = DIRECTIONS.len()
                    - DIRECTIONS
                        .iter()
                        .filter_map(|d| {
                            let x = p.0.checked_add_signed(d.0)?;
                            let y = p.1.checked_add_signed(d.1)?;
                            let test_c = grid.get(y)?.get(x)?;
                            info!("  checking:  {:?}", (x, y));
                            if test_c == &this_c {
                                info!("    TRUE");
                                Some(())
                            } else {
                                None
                            }
                        })
                        .count();
                info!("  fence_count: {:?} {}", p, fence_sides);
                fence_sides
            })
            .sum::<usize>();

        Some((this_c, fence_count, connected.len()))
    });

    Ok(scores.map(|s| s.1 * s.2).sum())
}

/// Spec: directions that can be considered immediately adjacent
/// to a point.
/// Defined as left right up down.
const ADJ_DIRECTIONS: &[Direction] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

/// Are two points next to each other?
fn next_to(point0: &XY, point1: &XY) -> bool {
    ADJ_DIRECTIONS
        .iter()
        .any(|d| delta_xy(point0, d) == Some(*point1))
}

fn extract_connected_for_loop<'a, T>(
    point: XY,
    this_c: &'a T,
    points: impl Iterator<Item = (&'a T, XY)> + Clone,
    adjacent_directions: impl Iterator<Item = &'a Direction> + Clone + 'a,
) -> HashSet<XY>
where
    T: std::cmp::PartialEq + 'a,
{
    // Iteratively build up a set of connected points.
    let mut connected = HashSet::new();
    // Add the starting point to the connected set.
    connected.insert(point);

    // Keep looking for points that are our same color, haven't been
    // already added to our set, and are adjacent to any point in this connected list.
    loop {
        // Points that could be considered (not in connected and are the same color)
        let possible_points = points.clone().filter(|(c, _)| c == &this_c);

        let mut added = false;
        for (_, p) in possible_points {
            // It's already in the set, skip it.
            if connected.contains(&p) {
                continue;
            }

            // Find all the points that are adjacent to a point in our set
            // look at all of our adjacent points.
            let adjacent_points = coordinates_from(p, adjacent_directions.clone());
            for adj_p in adjacent_points {
                if connected.contains(&adj_p) {
                    connected.insert(p);
                    added = true;
                    break;
                }
            }
        }

        // If we didn't find any new points, we are done.
        if !added {
            break;
        }
    }
    connected
}

/// Given a point, find all the points that are connected to it.
///
/// Connected points are points that are the same color and are adjacent to
/// other points in the connected set.
///
/// This is the same as extract_connected_for_loop but uses iterators instead of
/// for loops.  A curious thing is because of the need to add to the connected set
/// while querying, this version is more complex than the for loop version because
/// of needing to use a RefCell to keep the borrow checker happy.
pub fn extract_connected<'a, T>(
    point: XY,
    this_c: &'a T,
    points: impl Iterator<Item = (&'a T, XY)> + Clone,
    adjacent_directions: impl Iterator<Item = &'a Direction> + Clone + 'a,
) -> HashSet<XY>
where
    T: std::cmp::PartialEq + 'a,
{
    // Iteratively build up a set of connected points.
    let mut connected = HashSet::new();
    // Add the starting point to the connected set.
    connected.insert(point);
    // Because we need to query and mutate this set during the generation of
    // the connected set, we use a RefCell to keep the borrow checker happy.
    let connected = RefCell::new(connected);

    // Keep looking for points that haven't been
    // seen and that are adjacent to points in this connected list.
    loop {
        // Points that could be considered (not in connected and are the same color)
        let possible_points = points
            .clone()
            .filter(|(_, p)| !connected.borrow().contains(p))
            .filter(|(c, _)| c == &this_c);

        // Find all the points that are adjacent to a point in our set
        let points_adjacented_to_connected = possible_points.filter_map(|(_, p)| {
            // look at all of our adjacent points.
            let mut adjacent_points = coordinates_from(p, adjacent_directions.clone());
            // this is in the set if any adjacent point is in the connected set
            let connected = connected.borrow();
            adjacent_points
                .any(|adj| connected.contains(&adj))
                .then_some(p)
        });

        // Add all the points we found to the connected set.
        let mut added = false;
        points_adjacented_to_connected.for_each(|p| {
            connected.borrow_mut().insert(p);
            added = true;
        });

        // If we didn't find any new points, we are done.
        if !added {
            break;
        }
    }
    connected.take()
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    // make a grid of nones
    let mut seen: HashSet<XY> = HashSet::new();

    let grid = &input.grid;

    let points = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (c, (x, y))));

    struct GroupScore {
        area: usize,
        permimeter: usize,
    }

    let scores = points.clone().filter_map(move |(this_c, point)| {
        if !seen.insert(point) {
            return None;
        }
        seen.insert(point);

        info!("point: {:?} {}", point, this_c);

        // Connected is a set of points that are connected to this point
        let connected =
            extract_connected_for_loop(point, this_c, points.clone(), ADJ_DIRECTIONS.iter());

        // Mark all of these points as seen.
        seen.extend(connected.iter());

        info!("connected: {this_c} {:?}", connected);
        // Find all the fence sides for each point in the connected set.
        let fences: HashMap<XY, HashSet<&Direction>> = connected
            .iter()
            .map(|p| {
                let fence_sides = ADJ_DIRECTIONS
                    .iter()
                    .filter(|d| {
                        // This is NOT a valid fence side if the cell in this direction
                        // is in our connected set.
                        delta_xy(p, d)
                            .map(|newxy| !connected.contains(&newxy))
                            .unwrap_or(true)
                    })
                    .collect::<HashSet<_>>();
                (*p, fence_sides)
            })
            .collect::<HashMap<_, _>>();

        // Count all the fence sides (part 1 answer)
        let fence_count = fences
            .values()
            .map(|fence_sides| fence_sides.len())
            .sum::<usize>();

        info!("   starting fence_count: {this_c} {}", fence_count);

        // Our subtract pattern considers a cell in an adjacent direction and
        // will subtract a fence for each fence side in this pattern that is
        // in both our cell and the adjacent cell.
        struct SubtractPattern {
            direction: Direction,
            fence_sides: &'static [Direction],
        }

        // The subtract patterns we will consider.
        const SUBTRACT_PATTERNS: &[SubtractPattern] = &[
            // For left, we don't repeat a count if both us and them have top and bottom fences.
            SubtractPattern {
                direction: (-1, 0),
                fence_sides: &[(0, 1), (0, -1)],
            },
            // For left, we don't repeat a count if both us and them have left and right fences.
            SubtractPattern {
                direction: (0, -1),
                fence_sides: &[(1, 0), (-1, 0)],
            },
        ];

        // Look at all the fences and see if we can subtract some.
        let subtract: usize = fences
            .iter()
            .flat_map(|(p, fence_sides)| {
                // (p,fence_sides) is a point in our collection and the fence sides for that point.

                // Look at all the subtract patterns see how many sides we can subtract.
                SUBTRACT_PATTERNS.iter().filter_map(|pattern| {
                    // get the adjacent cell xy coordinates
                    let adj_xy = delta_xy(p, &pattern.direction)?;
                    // Get the fences of this adjacent cell.
                    let adj_fences = fences.get(&adj_xy)?;
                    Some(
                        pattern
                            // look at the fence sides that are in our pattern
                            .fence_sides
                            .iter()
                            // Include this sides if it's in both our's and the adjacent fence sets.
                            .filter(|side| fence_sides.contains(side) && adj_fences.contains(side))
                            // We'll subtract the count of fence patterns that form straight lines.
                            .count(),
                    )
                })
            })
            .sum();

        // Subtract fences that contribute to the continuation of a fence line.
        let fence_count = fence_count - subtract;

        info!("  fence_count: {this_c} {}", fence_count);

        Some(GroupScore {
            area: connected.len(),
            permimeter: fence_count,
        })
    });

    Ok(scores.map(|s| s.area * s.permimeter).sum())
}

// Given an xy and direction, compute a new xy not going out of bounds.
fn delta_xy(xy: &XY, delta: &Direction) -> Option<XY> {
    Some((
        xy.0.checked_add_signed(delta.0)?,
        xy.1.checked_add_signed(delta.1)?,
    ))
}

/// Given an xy and an iterator of directions, return an iterator that provides
/// the valid coordinates that are in these delta directions.
fn coordinates_from<'a>(
    xy: XY,
    directions: impl Iterator<Item = &'a Direction> + 'a,
) -> impl Iterator<Item = XY> + 'a {
    directions.filter_map(move |d| delta_xy(&xy, d))
}

/// Solution to part 1
#[aoc(day12, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day12, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

type Grid = Vec<Vec<char>>;
type XY = (usize, usize);
type Direction = (isize, isize);

/// Problem input
#[derive(Debug)]
struct Data {
    grid: Grid,
}
impl Data {
    fn parse(s: &str) -> Result<Self> {
        // Create the generator for the grid
        let grid = s.lines().map(|line| line.chars());
        // Collect the grid into a 2D vector
        let grid = grid.map(|row| row.collect()).collect();

        Ok(Data { grid })
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
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&test_data(super::DAY).unwrap()).unwrap(), 1206);
    }
}
