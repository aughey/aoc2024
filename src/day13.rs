use std::fmt::Display;

use crate::Result;
use anyhow::Context as _;
use aoc_runner_derive::aoc;
use tracing::info;
use z3::ast::Ast as _;

pub const DAY: u32 = 13;

fn solve_part1_impl(input: &Data) -> Result<usize> {
    Ok(input
        .machines
        .iter()
        .filter_map(|m| solve_machine(m).ok())
        .map(|(a, b)| a * 3 + b)
        .sum())
}

fn solve_part2_impl(input: &Data) -> Result<usize> {
    const ADD_PRICE: usize = 10000000000000;
    Ok(input
        .machines
        .iter()
        .map(|m| Machine {
            button_a: m.button_a,
            button_b: m.button_b,
            prize: (m.prize.0 + ADD_PRICE, m.prize.1 + ADD_PRICE),
        })
        .filter_map(|m| solve_machine(&m).ok())
        .map(|(a, b)| a * 3 + b)
        .sum())
}

fn solve_machine(m: &Machine) -> Result<(usize, usize)> {
    use z3::{Config, Context};
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = z3::Optimize::new(&ctx);
    //let solver = z3::Solver::new(&ctx);

    let (apress, bpress) = (
        z3::ast::Int::new_const(&ctx, "apress"),
        z3::ast::Int::new_const(&ctx, "bpress"),
    );

    let axmovement = z3::ast::Int::from_i64(&ctx, m.button_a.0 as i64);
    let aymovement = z3::ast::Int::from_i64(&ctx, m.button_a.1 as i64);
    let bxmovement = z3::ast::Int::from_i64(&ctx, m.button_b.0 as i64);
    let bymovement = z3::ast::Int::from_i64(&ctx, m.button_b.1 as i64);

    let aprize = z3::ast::Int::from_i64(&ctx, m.prize.0 as i64);
    let bprize = z3::ast::Int::from_i64(&ctx, m.prize.1 as i64);

    let acost = z3::ast::Int::from_i64(&ctx, 3);
    let bcost = z3::ast::Int::from_i64(&ctx, 1);

    let first = apress.clone() * axmovement + bpress.clone() * bxmovement;
    let second = apress.clone() * aymovement + bpress.clone() * bymovement;

    let total_cost = apress.clone() * acost + bpress.clone() * bcost;

    solver.assert(&first._eq(&aprize));
    solver.assert(&second._eq(&bprize));
    solver.minimize(&total_cost);

    info!("solver: {:?}", solver);

    let check = solver.check(&[]);
    info!("check: {:?}", check);
    match check {
        z3::SatResult::Sat => {
            let model = solver
                .get_model()
                .ok_or_else(|| anyhow::anyhow!("no model"))?;
            info!("model: {:?}", model);
            info!("stats: {:?}", solver.get_statistics());
            let aans = model.eval(&apress, true).unwrap().as_i64().unwrap();
            let bans = model.eval(&bpress, true).unwrap().as_i64().unwrap();
            Ok((aans as usize, bans as usize))
        }
        z3::SatResult::Unsat => {
            anyhow::bail!("unsat")
        }
        z3::SatResult::Unknown => {
            anyhow::bail!("unknown")
        }
    }
}

/// Solution to part 1
#[aoc(day13, part1)]
fn solve_part1(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part1_impl(&input)
}

/// Solution to part 2
#[aoc(day13, part2)]
fn solve_part2(input: &str) -> Result<usize> {
    let input = Data::parse(input).context("input parsing")?;
    solve_part2_impl(&input)
}

#[derive(Debug)]
struct Machine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

/// Problem input
#[derive(Debug)]
struct Data {
    machines: Vec<Machine>,
}
impl Data {
    fn parse(s: &str) -> Result<Self> {
        let s = s.lines();

        let mut machines = vec![];
        _ = read_machines(s, &mut machines);

        info!("Parsed data: {:?}", machines);

        Ok(Data { machines })
    }
}

fn read_machines<'a>(
    mut lines: impl Iterator<Item = &'a str>,
    machines: &mut Vec<Machine>,
) -> Result<()> {
    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400
    let buttonre = regex::Regex::new(r"Button ([AB]): X\+(\d+), Y\+(\d+)").unwrap();
    let prizere = regex::Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    loop {
        let a = if let Some(a) = lines.next() {
            a
        } else {
            break;
        };
        let b = lines.next().ok_or_else(|| anyhow::anyhow!("no b"))?;
        let prize = lines.next().ok_or_else(|| anyhow::anyhow!("no prize"))?;
        let a = buttonre
            .captures(a)
            .ok_or_else(|| anyhow::anyhow!("no a recapture"))?;
        let b = buttonre
            .captures(b)
            .ok_or_else(|| anyhow::anyhow!("no b recapture"))?;
        let prize = prizere
            .captures(prize)
            .ok_or_else(|| anyhow::anyhow!("no prize recapture"))?;
        machines.push(Machine {
            button_a: (a[2].parse()?, a[3].parse()?),
            button_b: (b[2].parse()?, b[3].parse()?),
            prize: (prize[1].parse()?, prize[2].parse()?),
        });
        // eat a line maybe
        _ = lines.next();
    }
    Ok(())
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
        assert_eq!(solve_part1(&test_data(super::DAY).unwrap()).unwrap(), 480);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            solve_part2(&test_data(super::DAY).unwrap()).unwrap(),
            875318608908
        );
    }
}
