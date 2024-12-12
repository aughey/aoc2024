use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

pub const DAY: u32 = 12;

/// Solve part 1 by iterator through all of the plots, getting
/// the number of fences and area, adding up all of those values.
fn solve_part1_impl(input: &Data) -> Result<usize> {
    Ok(all_plot_fences(input.plots())
        .map(|fences| {
            // Count all the fence sides (part 1 answer)
            let fence_count = fences
                .values()
                .map(|fence_sides| fence_sides.len())
                .sum::<usize>();

            let area = fences.keys().len();
            fence_count * area
        })
        .sum())
}

/// Spec: directions that can be considered immediately adjacent to a point.
/// Defined as left right up down.
const ADJ_DIRECTIONS: &[Direction] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

/// Given a point and color, find all the points that are connected to it.
/// - point: The starting point
/// - this_c: The color of the starting point
/// - points: An iterator of all the points and their colors
/// - adjacent_directions: An iterator of all the directions that are considered adjacent
fn expand_plot_to_region<'a, T>(
    point: XY,
    this_c: &'a T,
    points: impl Iterator<Item = (T, XY)> + Clone,
    adjacent_directions: impl Iterator<Item = Direction> + Clone + 'a,
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
        // Points that could be considered (are the same color)
        let possible_points = points.clone().filter(|(c, _)| c == this_c);

        // A flag of whether we added any points to the connected set.
        let mut added = false;
        for (_, p) in possible_points {
            // It's already in the set, skip it.
            if connected.contains(&p) {
                continue;
            }

            // Find all the points that are adjacent to a point in our set
            // look at all of our adjacent points.
            let adjacent_points = coordinates_from(p, adjacent_directions.clone());

            // If any of these adjacent points are in the connected set, add this point.
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

/// Give our list of plots, provide an iterator of all the regions (connected plots).
fn all_regions(
    plots: impl Iterator<Item = (char, XY)> + Clone,
) -> impl Iterator<Item = (char, HashSet<XY>)> {
    // Keep track of all the points we've seen.
    let mut seen: HashSet<XY> = HashSet::new();

    plots.clone().filter_map(move |(this_c, point)| {
        // Skip if we've already seen this point.
        if !seen.insert(point) {
            return None;
        }
        seen.insert(point);

        // Take this point and expand to all connected points.
        let region = expand_plot_to_region(
            point,
            &this_c,
            plots.clone(),
            ADJ_DIRECTIONS.iter().copied(),
        );

        // Mark all of these points as seen.
        seen.extend(region.iter());
        Some((this_c, region))
    })
}

/// Given a region of connected points, provide the fence sides for each point.
fn wrap_fence(region: &HashSet<XY>) -> impl Iterator<Item = (XY, FenceSet)> + '_ {
    // Find all the fence sides for each point in the connected set.
    region.iter().map(|p| {
        // Consider all sides of this plot, and filter out the sides that are
        // connected to another plot.
        let fence_sides = ADJ_DIRECTIONS
            .iter()
            .copied()
            .filter(|d| {
                // This is NOT a valid fence side if the cell in this direction
                // is in our connected set.  Tough to mentally parse, but it's right.
                delta_xy(p, d)
                    .map(|newxy| !region.contains(&newxy))
                    .unwrap_or(true)
            })
            .collect();
        (*p, fence_sides)
    })
}

/// Given a grid, provide all of the connected plots and their fence sides.
fn all_plot_fences(
    plots: impl Iterator<Item = (char, XY)> + Clone,
) -> impl Iterator<Item = HashMap<XY, FenceSet>> {
    // Given a plot, determine how the fence is connected.
    all_regions(plots).map(|(_this_c, connected)| wrap_fence(&connected).collect())
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    struct GroupScore {
        area: usize,
        permimeter: usize,
    }

    let scores = all_plot_fences(input.plots()).map(|fences| {
        // Count all the fence sides (permimeter)
        let fence_count = fences
            .values()
            .map(|fence_sides| fence_sides.len())
            .sum::<usize>();

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

        GroupScore {
            area: fences.keys().len(),
            permimeter: fence_count,
        }
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
    directions: impl Iterator<Item = Direction> + 'a,
) -> impl Iterator<Item = XY> + 'a {
    directions.filter_map(move |d| delta_xy(&xy, &d))
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
type FenceSet = HashSet<Direction>;

/// Problem input
#[derive(Debug)]
struct Data {
    grid: Grid,
}
impl Data {
    fn plots(&self) -> impl Iterator<Item = (char, XY)> + Clone + '_ {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (*c, (x, y))))
    }
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
