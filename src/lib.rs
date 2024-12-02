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
