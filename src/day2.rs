use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Report(Vec<i32>);
impl Report {
    pub fn is_valid(&self) -> bool {
        let mut state = State::Start;
        for value in &self.0 {
            state = match state.next(*value) {
                Some(s) => s,
                None => return false,
            }
        }
        state.is_directional()
    }
    pub fn removed_levels(&self) -> impl Iterator<Item = Report> + '_ {
        self.0.iter().enumerate().map(move |(i, _)| {
            let mut new = self.0.clone();
            new.remove(i);
            Report(new)
        })
    }
}

struct Data {
    reports: Vec<Report>,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Data {
    let lines = input.lines();
    let reports = lines
        .map(|l| {
            Report(
                l.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    Data { reports }
}

enum State {
    Start,
    Unknown(i32),
    Assending(i32),
    Decending(i32),
}
impl State {
    pub fn diff_bad(diff: i32) -> bool {
        diff.abs() > 3 || diff == 0
    }
    pub fn next(self, value: i32) -> Option<State> {
        match self {
            State::Start => Some(State::Unknown(value)),
            State::Unknown(prev) => {
                let diff = value - prev;
                if Self::diff_bad(diff) {
                    return None;
                }
                if diff > 0 {
                    Some(State::Assending(value))
                } else {
                    Some(State::Decending(value))
                }
            }
            State::Assending(prev) => {
                let diff = value - prev;
                if Self::diff_bad(diff) {
                    return None;
                }
                if diff > 0 {
                    Some(State::Assending(value))
                } else {
                    // Cannot change direction
                    None
                }
            }
            State::Decending(prev) => {
                let diff = value - prev;
                if Self::diff_bad(diff) {
                    return None;
                }
                if diff < 0 {
                    Some(State::Decending(value))
                } else {
                    // Cannot change direction
                    None
                }
            }
        }
    }
    fn is_directional(&self) -> bool {
        matches!(self, State::Assending(_) | State::Decending(_))
    }
}

#[aoc(day2, part1)]
fn solve_part1(input: &Data) -> usize {
    input.reports.iter().filter(|r| r.is_valid()).count()
}

#[aoc(day2, part2)]
fn solve_part2(input: &Data) -> usize {
    input
        .reports
        .iter()
        .filter(|r| r.is_valid() || r.removed_levels().any(|r| r.is_valid()))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day2::{parse, solve_part1, solve_part2};
    use anyhow::Result;

    fn test_data() -> Result<String> {
        Ok(std::fs::read_to_string("test2.txt")?)
    }

    #[test]
    fn part1_example() {
        assert_eq!(solve_part1(&parse(&test_data().unwrap())), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_part2(&parse(&test_data().unwrap())), 4);
    }
}
