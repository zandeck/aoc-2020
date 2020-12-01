#[path = "../src/challenges/mod.rs"]
mod challenges;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    use challenges::day1::{part1_1, part1_2, part1_3, part2_1, part2_2, part2_3};
    let data = challenges::utils::read_file("./resources/1_1.txt").unwrap();
    c.bench_function("part1_1", |b| b.iter(|| part1_1(black_box(&data))));
    c.bench_function("part1_2", |b| b.iter(|| part1_2(black_box(&data))));
    c.bench_function("part1_3", |b| b.iter(|| part1_3(black_box(&data))));

    // c.bench_function("part2_1", |b| b.iter(|| part2_1(black_box(&data))));
    // c.bench_function("part2_2", |b| b.iter(|| part2_2(black_box(&data))));
    // c.bench_function("part2_3", |b| b.iter(|| part2_3(black_box(&data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
