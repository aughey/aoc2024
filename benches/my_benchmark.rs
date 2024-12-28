use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

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

pub fn iterators(grid: &[impl AsRef<[char]>], to_find: char) -> Option<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.as_ref()
                .iter()
                .enumerate()
                .filter_map(move |(x, cell)| (*cell == to_find).then_some((y, x)))
        })
        .next()
}

pub fn iterators_find(grid: &[impl AsRef<[char]>], to_find: char) -> Option<(usize, usize)> {
    grid.iter().enumerate().find_map(|(y, row)| {
        row.as_ref()
            .iter()
            .enumerate()
            .find_map(move |(x, cell)| (*cell == to_find).then_some((y, x)))
    })
}

pub fn iterators_position(grid: &[impl AsRef<[char]>], to_find: char) -> Option<(usize, usize)> {
    grid.iter().enumerate().find_map(|(y, row)| {
        row.as_ref()
            .iter()
            .position(|cell| *cell == to_find)
            .and_then(|x| Some((y, x)))
    })
}

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
        b.iter(|| assert_eq!(black_box(for_loops(&grid, TO_FIND)), ANSWER))
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
