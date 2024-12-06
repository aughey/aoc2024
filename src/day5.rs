use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::{aoc, aoc_generator};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
use tracing::{debug, info};

pub const DAY: u32 = 5;

/// Parsing logic uses the FromStr trait
#[aoc_generator(day5)]
fn parse(input: &str) -> Result<Data> {
    info!("Parsing input");
    Data::from_str(input).context("input parsing")
}

/// Solution to part 1
#[aoc(day5, part1)]
fn solve_part1(input: &Data) -> Result<usize> {
    Ok(input
        .updates
        .iter()
        .filter(|update| input.order_rules.as_slice().validate(update))
        .map(|update| update[update.len() / 2])
        .sum())
}

/// Solution to part 2
#[aoc(day5, part2)]
fn solve_part2(input: &Data) -> Result<usize> {
    Ok(input
        .updates
        .iter()
        .filter(|update| !input.order_rules.as_slice().validate(update))
        .map(|update| {
            let mut update = update.clone();
            while input.order_rules.as_slice().fix(&mut update) {
                // Keep fixing until we can't fix anymore.
            }
            update
        })
        .map(|update| update[update.len() / 2])
        .sum())
}

type PageNumber = usize;

trait UpdateFixer {
    /// Given an update, fix it in place and return true if any changes were made.
    fn fix(&self, update: &mut Update) -> bool;
}
trait UpdateValidator {
    /// Given an update, return true if it is valid.
    fn validate(&self, update: &Update) -> bool;
}

/// A nice blanket implementation for slices of validators.
impl<V> UpdateValidator for &[V]
where
    V: UpdateValidator + Debug,
{
    fn validate(&self, update: &Update) -> bool {
        // If any rule fails, the update is invalid.
        for rule in self.iter() {
            if !rule.validate(update) {
                debug!("Rule {:?} failed for update {:?}", rule, update);
                return false;
            }
        }
        true
    }
}

/// Similar blank implementation for slices of fixers.
impl<V> UpdateFixer for &[V]
where
    V: UpdateFixer + Debug,
{
    fn fix(&self, update: &mut Update) -> bool {
        let mut fixed = false;
        for rule in self.iter() {
            if rule.fix(update) {
                fixed = true;
            }
        }
        fixed
    }
}

#[derive(Debug)]
struct OrderRule {
    first: PageNumber,
    second: PageNumber,
}
impl FromStr for OrderRule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut s = s.split('|');
        let first = s.next().ok_or_else(|| anyhow::anyhow!("no first page"))?;
        let second = s.next().ok_or_else(|| anyhow::anyhow!("no second page"))?;
        Ok(OrderRule {
            first: first.parse().context("first")?,
            second: second.parse().context("second")?,
        })
    }
}

impl UpdateValidator for OrderRule {
    fn validate(&self, update: &Update) -> bool {
        // Validate uses the order_validation which returns which two page indices are in the wrong order.
        // Is this returns None, it's valid.
        self.order_validate(update).is_none()
    }
}
impl UpdateFixer for OrderRule {
    fn fix(&self, update: &mut Update) -> bool {
        // If the order is invalid, swap the two pages.
        if let Some((first, second)) = self.order_validate(update) {
            update.swap(first, second);
            true
        } else {
            false
        }
    }
}

impl OrderRule {
    /// Internal method like validate, but returns which two indices are
    /// in the wrong order.
    fn order_validate(&self, update: &Update) -> Option<(usize, usize)> {
        let mut seen_second = None;
        for (index, &page) in update.iter().enumerate() {
            if page == self.first {
                if let Some(second) = seen_second {
                    return Some((index, second));
                } else {
                    return None;
                }
            }
            if page == self.second {
                seen_second = Some(index)
            }
        }
        None
    }
}

type Update = Vec<PageNumber>;

/// Problem input
#[derive(Debug)]
struct Data {
    order_rules: Vec<OrderRule>,
    updates: Vec<Update>,
}
impl FromStr for Data {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // XXX: Do actual parsing here.
        let lines = s.lines();

        let mut order_rules = Vec::new();

        for rule in lines {
            if rule.is_empty() {
                break;
            }
            order_rules.push(OrderRule::from_str(rule)?);
        }
        let mut updates = Vec::new();
        let lines = s.lines().skip(order_rules.len() + 1);
        for update in lines {
            let update = update
                .split(',')
                .map(|s| s.parse::<PageNumber>().context("update"))
                .collect::<Result<Update, _>>()?;
            updates.push(update);
        }

        // XXX: Update the returned Data to include the parsed data.
        Ok(Data {
            order_rules,
            updates,
        })
    }
}

/// codspeed compatible function
pub fn part1(input: &str) -> impl Display {
    solve_part1(&parse(input).unwrap()).unwrap()
}

/// codspeed compatible function
pub fn part2(input: &str) -> impl Display {
    solve_part2(&parse(input).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::test_data;
    use test_log::test;

    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            solve_part1(&parse(&test_data(super::DAY).unwrap()).unwrap()).unwrap(),
            143
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&parse(&test_data(super::DAY).unwrap()).unwrap()).unwrap(),
            123
        );
    }
}
