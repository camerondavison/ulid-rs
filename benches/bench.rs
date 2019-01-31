use criterion::{criterion_group, criterion_main, Criterion, Bencher};
use chrono::Utc;
use ulid::{Ulid, Generator};

fn bench_new(b: &mut Bencher) {
    b.iter(|| Ulid::new());
}

fn bench_generator_generate(b: &mut Bencher) {
    let mut gen = Generator::new();
    b.iter(|| gen.generate().unwrap());
}

fn bench_from_time(b: &mut Bencher) {
    let time = Utc::now();
    b.iter(|| Ulid::from_datetime(time));
}

fn bench_to_string(b: &mut Bencher) {
    let ulid = Ulid::new();
    b.iter(|| ulid.to_string());
}

fn bench_from_string(b: &mut Bencher) {
    let s = Ulid::new().to_string();
    b.iter(|| Ulid::from_string(&s).unwrap());
}

fn bench(c: &mut Criterion) {
    c.bench_function("bench_new", bench_new);
    c.bench_function("bench_generator_generate", bench_generator_generate);
    c.bench_function("bench_from_time", bench_from_time);
    c.bench_function("bench_to_string", bench_to_string);
    c.bench_function("bench_from_string", bench_from_string);
}

criterion_group!(
    ulid_perf,
    bench
);

criterion_main!(ulid_perf);
