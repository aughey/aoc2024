pub mod day1;
pub mod day10;
pub mod day10_video;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day9_iterators;

pub use anyhow::Result;
use aoc_runner_derive::aoc_lib;

type Position = (usize, usize);
type Direction = (isize, isize);

aoc_lib! { year = 2024 }

/// A trait for things that can provide a cell reference given a position.
pub trait GetCell<T> {
    /// Get a reference to the cell at the given position.
    fn get_cell(&self, xy: &Position) -> Option<&T>;
    /// Get a reference to the cell at the given position, returning an error if the cell does not exist.
    fn get_cell_result(&self, xy: &Position) -> Result<&T> {
        self.get_cell(xy)
            .ok_or_else(|| anyhow::anyhow!("no cell at {:?}", xy))
    }
}

/// A trait for things that can provide a mutable cell reference given a position.
pub trait GetCellMut<T> {
    fn get_cell(&mut self, xy: &Position) -> Option<&T> {
        self.get_cell_mut(xy).map(|c| &*c)
    }
    fn get_cell_result(&mut self, xy: &Position) -> Result<&T> {
        self.get_cell_mut_result(xy).map(|c| &*c)
    }
    fn get_cell_mut(&mut self, xy: &Position) -> Option<&mut T>;
    fn get_cell_mut_result(&mut self, xy: &Position) -> Result<&mut T> {
        self.get_cell_mut(xy)
            .ok_or_else(|| anyhow::anyhow!("no cell at {:?}", xy))
    }
}

impl<T, INNER> GetCell<T> for &[INNER]
where
    INNER: AsRef<[T]>,
{
    fn get_cell(&self, xy: &Position) -> Option<&T> {
        self.get(xy.1)?.as_ref().get(xy.0)
    }
}
impl<T, INNER> GetCellMut<T> for &mut [INNER]
where
    INNER: AsMut<[T]>,
{
    fn get_cell_mut(&mut self, xy: &Position) -> Option<&mut T> {
        self.get_mut(xy.1)?.as_mut().get_mut(xy.0)
    }
}
impl<T, INNER> GetCell<T> for &mut [INNER]
where
    INNER: AsRef<[T]>,
{
    fn get_cell(&self, xy: &Position) -> Option<&T> {
        self.get(xy.1)?.as_ref().get(xy.0)
    }
}

pub fn enumerate_grid<T, INNER>(
    grid: impl IntoIterator<Item = INNER>,
) -> impl Iterator<Item = (usize, usize, T)>
where
    INNER: IntoIterator<Item = T>,
{
    grid.into_iter()
        .enumerate()
        .flat_map(|(y, row)| row.into_iter().enumerate().map(move |(x, c)| (x, y, c)))
}

/// Read the problem sample test for a day from the input directory.
pub fn test_data(day: u32) -> Result<String> {
    Ok(std::fs::read_to_string(format!(
        "input/2024/day{}-test.txt",
        day
    ))?)
}

// checked_add functions on u16, i16, u32, i32, etc are not defined as a trait.
// This is our own definition of checked_add that is implemented for a few types used
// in the solutions.  Other types can be added as needed.
pub trait CheckedAdd<T> {
    fn checked_add(self, other: T) -> Option<T>;
}
impl CheckedAdd<u32> for u32 {
    fn checked_add(self, rhs: u32) -> Option<u32> {
        self.checked_add(rhs)
    }
}
impl CheckedAdd<u64> for u64 {
    fn checked_add(self, rhs: u64) -> Option<u64> {
        self.checked_add(rhs)
    }
}
impl CheckedAdd<usize> for usize {
    fn checked_add(self, rhs: usize) -> Option<usize> {
        self.checked_add(rhs)
    }
}

pub trait CountResults<T, E> {
    fn count_results(self) -> Result<usize, E>;
}
impl<T, I> CountResults<T, anyhow::Error> for I
where
    I: Iterator<Item = Result<T>>,
{
    fn count_results(self) -> Result<usize> {
        let mut count = 0;
        for v in self {
            _ = v?;
            count += 1;
        }
        Ok(count)
    }
}

/// Similar to the sum() function on iterators, but for results.
pub trait SumResults<T, E> {
    fn sum_results(self) -> Result<T, E>;
}

/// blanket implementation for all iterators of results that add.
/// This also checks the sum for overflow.
impl<T, I> SumResults<T, anyhow::Error> for I
where
    I: Iterator<Item = Result<T>>,
    T: std::ops::Add<Output = T> + Default,
    T: CheckedAdd<T>,
{
    /// Given an iterator of results, sums the inner value of the results and checks for overflow
    /// of the sum itself.
    fn sum_results(self) -> Result<T> {
        let mut sum = T::default();
        for v in self {
            sum = sum
                .checked_add(v?)
                .ok_or_else(|| anyhow::anyhow!("add overflowed"))?;
        }
        Ok(sum)
    }
}

/// Similar to the sum() function on iterators, but will check for overflow of
/// the sum itself.
pub trait CheckedSum<T> {
    /// Sums the values in an iterator and checks for overflow of the sum itself.
    /// Returns None if the sum overflows.
    fn checked_sum(self) -> Option<T>;
}
impl<T, I> CheckedSum<T> for I
where
    I: Iterator<Item = T>,
    T: std::ops::Add<Output = T> + Default + CheckedAdd<T>,
{
    fn checked_sum(self) -> Option<T> {
        let mut sum = T::default();
        for v in self {
            sum = sum.checked_add(v)?;
        }
        Some(sum)
    }
}

pub trait StopMap {
    fn stop_map<T, F>(self, f: F) -> impl Iterator<Item = T>
    where
        F: FnMut(Self::Item) -> Option<T>,
        Self: Iterator;
}
pub trait StopMapClone {
    fn stop_map<T, F>(self, f: F) -> impl Iterator<Item = T> + Clone
    where
        F: FnMut(Self::Item) -> Option<T> + Clone,
        Self: Iterator;
}
impl<I> StopMap for I
where
    I: Iterator,
{
    fn stop_map<T, F>(self, f: F) -> impl Iterator<Item = T>
    where
        F: FnMut(I::Item) -> Option<T>,
    {
        self.map(f).take_while(|c| c.is_some()).map(|c| c.unwrap())
    }
}

impl<I> StopMapClone for I
where
    I: Iterator + Clone,
{
    /// Similar to map, but stops the iterator when the closure returns None.
    /// Will unwrap all the values.
    /// Equilivant to map + take_while(Option::is_some) + map(Option::unwrap)
    fn stop_map<T, F>(self, f: F) -> impl Iterator<Item = T> + Clone
    where
        F: FnMut(I::Item) -> Option<T> + Clone,
    {
        self.map(f).take_while(|c| c.is_some()).map(|c| c.unwrap())
    }
}

fn add_xy_result(cur_cell: &Position, direction: &Direction) -> Result<Position> {
    Ok((
        cur_cell
            .0
            .checked_add_signed(direction.0)
            .ok_or_else(|| anyhow::anyhow!("invalid movement"))?,
        cur_cell
            .1
            .checked_add_signed(direction.1)
            .ok_or_else(|| anyhow::anyhow!("invalid movement"))?,
    ))
}

fn add_xy(xy: &Position, direction: &Direction) -> Option<Position> {
    Some((
        xy.0.checked_add_signed(direction.0)?,
        xy.1.checked_add_signed(direction.1)?,
    ))
}
