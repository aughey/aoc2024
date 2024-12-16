# aoc2024

```sh
RUST_LOG=debug cargo watch -x 'nextest r day3 --test-threads 1'
```

This year using pattern from https://github.com/gobanos/cargo-aoc?tab=readme-ov-file

also running codspeed

test_codspeed.sh script created to quickly validate that the project will work for a given day

template.rs created after day 2.  Current capabilities are:

- day{#}.rs file for each day
- const DAY at the top of the file to indicate which day (for testing)
- `Data` struct defined to be a parsed version of the problem input
- `Data` implements `FromStr` to parse the input data
- Ideally, any sub-input (lines, etc) also use FromStr to parse
- aoc_generator pre-built to use this from_str method
- Need to implment `solve_part1` and `solve_part2`
    - Change return type to be some impl Display value (usize often)
    - Implement problem logic
- codspeed public functions part1 and part2 defined generically enough to use the above signatures without change.
- tests written for testing against sample input provided
    - uses `lib.rs` defined `test_data` function to read the data.
    - Only the CHANGE ME line needs to be touched to provde the provided sample solution.

# Videos to make

- General
    - template.rs
- day1.rs
    - SumResults
    - CheckedSum
- day2.rs
    - Representing State while iterating
        - State Graph
    - Changing state transition to use diff_direction
    - Testing diff_direction
    - Using iterator to skip
- day4.rs
    - Not using any loops or conditionals
    - xy computation with bounds and overflow
- day6.rs
    - Creating a grid of a different type with the same dimensionality of another.

```rust
    let mut curr = (0,0);
    for i in 0..data.grid.len() {
        for j in 0..data.grid[0].len() {
            if data.grid[i][j] == '@' {
                curr = (i as i64,j as i64);
                break
            }
        }
    }

    let mut curr = None;
    for i in 0..data.grid.len() {
        for j in 0..data.grid[0].len() {
            if data.grid[i][j] == '@' {
                curr = Some((i as i64,j as i64));
                break
            }
        }
    }

    let mut curr = None;
    for i in 0..data.grid.len() {
        for j in 0..data.grid[0].len() {
            if data.grid[i][j] == '@' {
                curr = Some((i as i64,j as i64));
                break
            }
        }
    }
    let curr = curr.ok_or_else(|| anyhow::anyhow!("no current"));

    let mut curr = None;
    'top: for i in 0..data.grid.len() {
        for j in 0..data.grid[0].len() {
            if data.grid[i][j] == '@' {
                curr = Some((i as i64,j as i64));
                break 'top;
            }
        }
    }
    let curr = curr.ok_or_else(|| anyhow::anyhow!("no current"))?;

    let curr = {
        let mut curr = None;
        'top: for i in 0..data.grid.len() {
            for j in 0..data.grid[0].len() {
                if data.grid[i][j] == '@' {
                    curr = Some((i as i64,j as i64));
                    break 'top;
                }
            }
        }
        curr.ok_or_else(|| anyhow::anyhow!("no current"))?
    };

    let curr = {
        let mut curr = None;
        'top: for i in 0..data.grid.len() {
            for j in 0..data.grid[i].len() {
                if data.grid[i][j] == '@' {
                    curr = Some((i as i64,j as i64));
                    break 'top;
                }
            }
        }
        curr.ok_or_else(|| anyhow::anyhow!("no current"))?
    };

    let curr = map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, cell)| {
                if let Cell::Player = cell {
                    Some((y, x))
                } else {
                    None
                }
            })
        })
        .next()
        .ok_or_else(|| anyhow::anyhow!("no current"))?

```