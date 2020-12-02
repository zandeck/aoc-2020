#[path = "../src/challenges/mod.rs"]
mod challenges;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    use challenges::day1::{
        part1_1, part1_2, part1_3, part1_4, part1_4smart, part1_4smartvec, part1_5, part1_5p,
        part2_1, part2_2, part2_3,
    };
    let data = challenges::utils::read_file("./resources/1_1.txt").unwrap();
    c.bench_function("part1_1 (itertools combinations)", |b| {
        b.iter(|| part1_1(black_box(&data)))
    });
    c.bench_function("part1_2 (custom iterator)", |b| {
        b.iter(|| part1_2(black_box(&data)))
    });
    c.bench_function("part1_3 (hashset)", |b| {
        b.iter(|| part1_3(black_box(&data)))
    });
    c.bench_function(
        "part1_4 (replace hashset with stack allocated array)",
        |b| b.iter(|| part1_4(black_box(&data))),
    );
    c.bench_function(
        "part1_4smart (replace hashset with stack allocated array)",
        |b| b.iter(|| part1_4smart(black_box(&data))),
    );
    c.bench_function("part1_5 (replace array with bit-vec)", |b| {
        b.iter(|| part1_5(black_box(&data)))
    });
    c.bench_function("part1_5p (replace array with bitvec)", |b| {
        b.iter(|| part1_5p(black_box(&data)))
    });
    c.bench_function("part1_4smartvec (static array with vec)", |b| {
        b.iter(|| part1_4smartvec(black_box(&data)))
    });

    // c.bench_function("part2_1", |b| b.iter(|| part2_1(black_box(&data))));
    // c.bench_function("part2_2", |b| b.iter(|| part2_2(black_box(&data))));
    // c.bench_function("part2_3", |b| b.iter(|| part2_3(black_box(&data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
