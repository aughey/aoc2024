pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub use anyhow::Result;
use aoc_runner_derive::aoc_lib;

aoc_lib! { year = 2024 }

pub fn test_data(day: u32) -> Result<String> {
    Ok(std::fs::read_to_string(format!(
        "input/2024/day{}-test.txt",
        day
    ))?)
}

/// Sums the given iterator of results.
/// This allows for early return of errors.
fn sum_results<T: std::ops::Add<Output = T> + Default>(
    iter: impl Iterator<Item = Result<T>>,
) -> Result<T> {
    let mut sum = T::default();
    for v in iter {
        sum = sum + v?;
    }
    Ok(sum)
}

// Create a mixin so that .sum_results() works on iterators of Results.
pub trait SumResults<T> {
    fn sum_results(self) -> Result<T>;
}
pub struct SumResultsMixin<I>(I);
impl<T, I> SumResults<T> for I
where
    I: Iterator<Item = Result<T>>,
    T: std::ops::Add<Output = T> + Default,
{
    fn sum_results(self) -> Result<T> {
        sum_results(self)
    }
}

pub trait CheckedSum<T> {
    fn checked_sum(self) -> Result<T>;
}
impl<I> CheckedSum<u32> for I
where
    I: Iterator<Item = u32>,
{
    fn checked_sum(self) -> Result<u32> {
        let mut sum = 0u32;
        for v in self {
            sum = sum + v;
        }
        Ok(sum)
    }
}

impl<I> CheckedSum<i32> for I
where
    I: Iterator<Item = i32>,
{
    fn checked_sum(self) -> Result<i32> {
        let mut sum = 0i32;
        for v in self {
            sum = sum + v;
        }
        Ok(sum)
    }
}

impl<I> CheckedSum<usize> for I
where
    I: Iterator<Item = usize>,
{
    fn checked_sum(self) -> Result<usize> {
        let mut sum = 0usize;
        for v in self {
            sum = sum + v;
        }
        Ok(sum)
    }
}
