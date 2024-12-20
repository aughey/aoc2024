use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

pub fn for_loops(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '@' {
                return Some((y, x));
            }
        }
    }
    None
}

pub fn iterators(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, cell)| (*cell == '@').then_some((y, x)))
        })
        .next()
}

fn criterion_benchmark(c: &mut Criterion) {
    const SIZE: usize = 10000;
    let mut grid = vec![vec!['.'; SIZE]; SIZE];
    grid[SIZE - 1][SIZE - 1] = '@';

    c.bench_function("for_loop", |b| {
        b.iter(|| assert_eq!(black_box(for_loops(&grid)), Some((SIZE - 1, SIZE - 1))))
    });
    c.bench_function("iterators", |b| {
        b.iter(|| assert_eq!(black_box(iterators(&grid)), Some((SIZE - 1, SIZE - 1))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
