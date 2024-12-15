use crate::enumerate_grid;

pub fn demo_fn_his() {
    let grid = [
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];

    #[allow(clippy::needless_range_loop)]
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            println!("{}: ({}, {})", grid[y][x], x, y);
        }
    }
}

pub fn demo_fn_his2() {
    let grid = [
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];

    let mut same_neighbors = 0usize;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            for direction in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let row = x as isize + direction.0;
                let col = y as isize + direction.1;
                if col < 0 || col >= grid.len() as isize {
                    continue;
                }
                if row < 0 || row >= grid[col as usize].len() as isize {
                    continue;
                }
                if grid[col as usize][row as usize] == grid[y][x] {
                    same_neighbors += 1;
                }
            }
        }
    }
    println!("Same neighbors: {}", same_neighbors);
}

pub fn demo_fn_mine() {
    let grid = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];

    let cells = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (x, y, c)));

    for (x, y, c) in cells {
        println!("{}: ({}, {})", c, x, y);
    }

    for (x, y, c) in enumerate_grid(&grid) {
        println!("{}: ({}, {})", c, x, y);
    }
}

pub fn demo_fn_mine2() {
    let grid = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];

    let grid = &grid;

    let delta_cell = |x: usize, y: usize, dx: isize, dy: isize| {
        let row = x.checked_add_signed(dx)?;
        let col = y.checked_add_signed(dy)?;
        grid.get(col)?.get(row)
    };

    let mut same_neighbors = 0usize;
    for (x, y, c) in enumerate_grid(grid) {
        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if let Some(adj_cell) = delta_cell(x, y, dir.0, dir.1) {
                if adj_cell == c {
                    same_neighbors += 1;
                }
            }
        }
        println!("{}: ({}, {})", c, x, y);
    }
    println!("Same neighbors: {}", same_neighbors);
}

pub fn demo_fn_mine3() {
    let grid = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];

    let grid = &grid;

    let delta_cell = |x: usize, y: usize, dx: isize, dy: isize| {
        let row = x.checked_add_signed(dx)?;
        let col = y.checked_add_signed(dy)?;
        grid.get(col)?.get(row)
    };

    let adj_cells = |cur_x: usize, cur_y: usize| {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .filter_map(move |(dx, dy)| delta_cell(cur_x, cur_y, *dx, *dy))
    };

    let same_adj_cells = |cur_x: usize, cur_y: usize, c: char| {
        adj_cells(cur_x, cur_y).filter(move |adj_c| **adj_c == c)
    };

    let sibling_count = enumerate_grid(grid)
        .map(|(x, y, c)| same_adj_cells(x, y, *c).count())
        .sum::<usize>();

    println!("Same neighbors: {}", sibling_count);

    let sibling_count = enumerate_grid(grid)
        .flat_map(|(x, y, c)| same_adj_cells(x, y, *c))
        .count();

    println!("Same neighbors: {}", sibling_count);

    let mut sibling_count = 0;
    for (x, y, c) in enumerate_grid(grid) {
        for _ in same_adj_cells(x, y, *c) {
            sibling_count += 1;
        }
    }
    println!("Same neighbors: {}", sibling_count);

    let mut sibling_count = 0;
    for (x, y, c) in enumerate_grid(grid) {
        sibling_count += same_adj_cells(x, y, *c).count();
    }

    println!("Same neighbors: {}", sibling_count);
}

pub fn demo_fn_mine4() {
    let grid = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];

    let grid = &grid;

    let delta_cell = |x: usize, y: usize, dx: isize, dy: isize| {
        let row = x.checked_add_signed(dx)?;
        let col = y.checked_add_signed(dy)?;
        grid.get(col)?.get(row)
    };

    let adj_cells = |x: usize, y: usize| {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .filter_map(move |(dx, dy)| delta_cell(x, y, *dx, *dy))
    };

    let common_adj_cells =
        |x: usize, y: usize, c: char| adj_cells(x, y).filter(move |their_c| **their_c == c);

    let mut sibling_count = 0;
    for (x, y, c) in enumerate_grid(grid) {
        for _ in common_adj_cells(x, y, *c) {
            sibling_count += 1;
        }
    }
    println!("Same neighbors: {}", sibling_count);
}

#[test]
fn test_demo_fn() {
    demo_fn_his();
    demo_fn_his2();
    demo_fn_mine();
    demo_fn_mine2();
    demo_fn_mine3();
    demo_fn_mine4();
}
