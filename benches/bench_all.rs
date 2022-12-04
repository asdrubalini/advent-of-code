use advent_of_code_2021::aoc::{days::*, Solution};
use criterion::{criterion_group, criterion_main, Criterion};

/// Generate benches for day n
macro_rules! bench_day {
    ($c: ident, $struct_name:ident) => {
        $c.bench_function(stringify!($struct_name), |b| {
            b.iter(|| $struct_name::assert_solutions())
        });
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_day!(c, One);
    //bench_day!(c, Two);
    //bench_day!(c, Three);
    // bench_day!(c, Four);
    // bench_day!(c, Five);
    // bench_day!(c, Six);
    // bench_day!(c, Seven);
    // bench_day!(c, Eight);
    // bench_day!(c, Nine);
    // bench_day!(c, Ten);
    // bench_day!(c, Eleven);
    // bench_day!(c, Twelve);
    // bench_day!(c, Thirteen);
    // bench_day!(c, Fourteen);
    // bench_day!(c, Fifteen);
    // bench_day!(c, Sixteen);
    // bench_day!(c, Seventeen);
    // bench_day!(c, Eighteen);
    // bench_day!(c, Nineteen);
    // bench_day!(c, Twenty);
    // bench_day!(c, TwentyOne);
    // bench_day!(c, TwentyTwo);
    // bench_day!(c, TwentyThree);
    // bench_day!(c, TwentyFour);
    // bench_day!(c, TwentyFive);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);