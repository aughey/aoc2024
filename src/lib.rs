pub mod day1;
pub mod day10;
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

aoc_lib! { year = 2024 }

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
