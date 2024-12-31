use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

// Goal: Find the y,x position of a character in a 2D grid

#[inline(never)]
pub fn for_loops(grid: &[Vec<char>], to_find: char) -> Option<(usize, usize)> {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == to_find {
                return Some((y, x));
            }
        }
    }
    None
}

#[inline(never)]
pub fn iterators(grid: &[impl AsRef<[char]>], to_find: char) -> Option<(usize, usize)> {
    grid.into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.as_ref()
                .into_iter()
                .enumerate()
                .filter_map(move |(x, cell)| (*cell == to_find).then_some((y, x)))
        })
        .next()
}

#[inline(never)]
pub fn iterators_find(grid: &[impl AsRef<[char]>], to_find: char) -> Option<(usize, usize)> {
    let mut foo = grid.iter();
    foo.next();
    grid.iter().enumerate().find_map(|(y, row)| {
        row.as_ref()
            .iter()
            .enumerate()
            .find_map(move |(x, cell)| (*cell == to_find).then_some((y, x)))
    })
}

#[inline(never)]
pub fn iterators_position(grid: &[impl AsRef<[char]>], to_find: char) -> Option<(usize, usize)> {
    grid.iter().enumerate().find_map(|(y, row)| {
        row.as_ref()
            .iter()
            .position(|cell| *cell == to_find)
            .and_then(|x| Some((y, x)))
    })
}

#[inline(never)]
pub fn for_loops_row(grid: &[impl AsRef<[char]>], to_find: char) -> Option<(usize, usize)> {
    for y in 0..grid.len() {
        let row = unsafe { grid.get_unchecked(y) };
        let row = row.as_ref();
        for x in 0..row.len() {
            let cell = unsafe { row.get_unchecked(x) };
            if *cell == to_find {
                return Some((y, x));
            }
        }
    }
    None
}

#[inline(never)]
pub fn iterators_2d_generic<'a, INNER, T>(
    grid: impl IntoIterator<Item = INNER>,
    to_find: T,
) -> Option<(usize, usize)>
where
    INNER: IntoIterator<Item = &'a T> + 'a,
    T: PartialEq + 'a,
{
    grid.into_iter().enumerate().find_map(|(y, row)| {
        row.into_iter()
            .position(|cell| *cell == to_find)
            .and_then(|x| Some((y, x)))
    })
}

#[inline(never)]
pub fn for_loop_pointers<T: AsRef<[char]>>(grid: &[T], to_find: char) -> Option<(usize, usize)> {
    if grid.len() == 0 {
        return None;
    }
    let first_row_ptr = grid.as_ptr();
    let mut row_ptr = first_row_ptr;
    let lastrow_ptr = unsafe { row_ptr.add(grid.len()) };
    while row_ptr != lastrow_ptr {
        let row = unsafe { &*row_ptr };
        let row = row.as_ref();
        if row.len() == 0 {
            continue;
        }
        let first_cell_ptr = row.as_ptr();
        let mut cell_ptr = first_cell_ptr;
        let lastcell_ptr = unsafe { cell_ptr.add(row.len()) };
        while cell_ptr != lastcell_ptr {
            let cell = unsafe { *cell_ptr };
            if cell == to_find {
                let x = cell_ptr as usize - first_cell_ptr as usize;
                let y = row_ptr as usize - first_row_ptr as usize;
                return Some((y / size_of::<T>(), x / size_of::<char>()));
            }
            cell_ptr = unsafe { cell_ptr.add(1) };
        }
        row_ptr = unsafe { row_ptr.add(1) };
    }
    None
}

#[inline(never)]
pub fn for_loop_pointers_unrolled<T: AsRef<[char]>>(
    grid: &[T],
    to_find: char,
) -> Option<(usize, usize)> {
    let grid_len = grid.len();
    let mut ptr = grid.as_ptr();
    let grid_end: *const T = unsafe { ptr.add(grid_len) };
    let mut y = 0;
    while ptr != grid_end {
        let row = unsafe { &*ptr }.as_ref();
        let row_len = row.len();
        let mut row_ptr = row.as_ptr();
        let row_end = unsafe { row_ptr.add(row_len) };
        let mut x = 0;

        while row_ptr != row_end {
            let cell = unsafe { *row_ptr };
            if cell == to_find {
                return Some((y, x));
            }

            row_ptr = unsafe { row_ptr.add(1) };
            x += 1;
        }
        ptr = unsafe { ptr.add(1) };
        y += 1;
    }
    None
}

pub fn use_all() {
    const SIZE: usize = 10000;
    let mut grid = vec![vec!['.'; SIZE]; SIZE];
    grid[SIZE - 1][SIZE - 1] = '@';

    const ANSWER: Option<(usize, usize)> = Some((SIZE - 1, SIZE - 1));
    const TO_FIND: char = '@';

    assert_eq!(for_loops(&grid, TO_FIND), ANSWER);
    assert_eq!(for_loops_row(&grid, TO_FIND), ANSWER);
    assert_eq!(for_loop_pointers(&grid, TO_FIND), ANSWER);
    assert_eq!(for_loop_pointers_unrolled(&grid, TO_FIND), ANSWER);
    assert_eq!(iterators(&grid, TO_FIND), ANSWER);
    assert_eq!(iterators_find(&grid, TO_FIND), ANSWER);
    assert_eq!(iterators_position(&grid, TO_FIND), ANSWER);
    assert_eq!(iterators_2d_generic(&grid, TO_FIND), ANSWER);
}

fn criterion_benchmark(c: &mut Criterion) {
    const SIZE: usize = 10000;
    let mut grid = vec![vec!['.'; SIZE]; SIZE];
    grid[SIZE - 1][SIZE - 1] = '@';

    const ANSWER: Option<(usize, usize)> = Some((SIZE - 1, SIZE - 1));
    const TO_FIND: char = '@';

    c.bench_function("for_loop", |b| {
        b.iter(|| assert_eq!(black_box(for_loops(&grid, TO_FIND)), ANSWER))
    });
    c.bench_function("for_loop_row", |b| {
        b.iter(|| assert_eq!(black_box(for_loops_row(&grid, TO_FIND)), ANSWER))
    });
    c.bench_function("for_loop_pointers", |b| {
        b.iter(|| assert_eq!(black_box(for_loop_pointers(&grid, TO_FIND)), ANSWER))
    });
    c.bench_function("for_loop_pointers_unrolled", |b| {
        b.iter(|| {
            assert_eq!(
                black_box(for_loop_pointers_unrolled(&grid, TO_FIND)),
                ANSWER
            )
        })
    });
    c.bench_function("iterators", |b| {
        b.iter(|| assert_eq!(black_box(iterators(&grid, TO_FIND)), ANSWER))
    });
    c.bench_function("iterators_find", |b| {
        b.iter(|| assert_eq!(black_box(iterators_find(&grid, TO_FIND)), ANSWER))
    });
    c.bench_function("iterators_position", |b| {
        b.iter(|| assert_eq!(black_box(iterators_position(&grid, TO_FIND)), ANSWER))
    });
    c.bench_function("iterators_2d_generic", |b| {
        b.iter(|| assert_eq!(black_box(iterators_2d_generic(&grid, TO_FIND)), ANSWER))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
