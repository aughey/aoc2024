use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use std::{collections::HashSet, fmt::Display};
use tracing::info;

pub const DAY: u32 = 12;

fn solve_part1_impl(input: &Data) -> Result<usize> {
    // make a grid of nones
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

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
        const DIRECTIONS: &[(isize, isize)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
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

fn next_to(point0: &(usize, usize), point1: &(usize, usize)) -> bool {
    let diff = (point0.0.abs_diff(point1.0), point0.1.abs_diff(point1.1));
    (diff.0 <= 1 && diff.1 <= 1) && diff.0 != diff.1
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    // make a grid of nones
    let mut seen: HashSet<(usize, usize)> = HashSet::new();

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
        const DIRECTIONS: &[(isize, isize)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
        let fences = connected
            .iter()
            .map(|p| {
                let invalid_fence_sides = DIRECTIONS
                    .iter()
                    .filter_map(|d| {
                        let x = p.0.checked_add_signed(d.0)?;
                        let y = p.1.checked_add_signed(d.1)?;
                        let test_c = grid.get(y)?.get(x)?;
                        info!("  checking:  {:?}", (x, y));
                        if test_c == &this_c {
                            Some(d)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                let fence_sides = DIRECTIONS
                    .iter()
                    .filter(|d| !invalid_fence_sides.contains(d))
                    .collect::<Vec<_>>();
                (*p, fence_sides)
            })
            .collect::<Vec<_>>();

        // need to walk again.
        let fence_count = fences
            .iter()
            .map(|(_, fence_sides)| fence_sides.len())
            .sum::<usize>();

        info!("   starting fence_count: {this_c} {}", fence_count);

        let subtract = fences.iter().filter_map(|(p, fence_sides)| {
            // look to left
            let leftp = (p.0.checked_sub(1)?, p.1);
            let left = fences.iter().find(|(p, _)| p == &leftp)?.1.clone();

            let mut subtract = 0;
            // look for a top and bottom
            if fence_sides.contains(&&(0, 1)) && left.contains(&&(0, 1)) {
                info!("sub: (0,1) at {:?}", p);
                subtract += 1;
            }
            if fence_sides.contains(&&(0, -1)) && left.contains(&&(0, -1)) {
                info!("sub: (0,-1) at {:?}", p);
                subtract += 1;
            }

            Some(subtract)
        });

        let fence_count = fence_count - subtract.sum::<usize>();

        // now back one
        let subtrace = fences.iter().filter_map(|(p, fence_sides)| {
            // look to left
            let abovep = (p.0, p.1.checked_sub(1)?);
            let above = fences.iter().find(|(p, _)| p == &abovep)?.1.clone();

            let mut subtract = 0;
            // look for a left and right
            if fence_sides.contains(&&(1, 0)) && above.contains(&&(1, 0)) {
                info!("sub: (1,0) at {:?}", p);
                subtract += 1;
            }
            if fence_sides.contains(&&(-1, 0)) && above.contains(&&(-1, 0)) {
                info!("sub: (-1,0) at {:?}", p);
                subtract += 1;
            }

            Some(subtract)
        });

        let fence_count = fence_count - subtrace.sum::<usize>();

        info!("  fence_count: {this_c} {}", fence_count);

        Some((this_c, fence_count, connected.len()))
    });

    Ok(scores.map(|s| s.1 * s.2).sum())
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
