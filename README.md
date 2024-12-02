# aoc2024

```sh
RUST_LOG=info cargo watch -x 'nextest r day3 --test-threads 1'
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