use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn add_test<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}
fn sub_test<T: std::ops::Sub<Output=T>>(a: T, b: T) -> T {
    a - b
}
fn mul_test<T: std::ops::Mul<Output=T>>(a: T, b: T) -> T {
    a * b
}
fn div_test<T: std::ops::Div<Output=T>>(a: T, b: T) -> T {
    a / b
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut floats = [0.0; 1000];
    let mut floats0 = [0.0; 1000];
    let mut rats = [maph::r32::new(1, 1).unwrap(); 1000];
    let mut rats0 = [maph::r32::new(1, 1).unwrap(); 1000];
    for i in 0..1000 {
        let af = i as f32 / (i + 1) as f32;
        let bf = (i + 2) as f32 / (i + 3) as f32;
        let ar = maph::r32::new_unchecked(i, i + 1);
        let br = maph::r32::new_unchecked(i + 2, i + 3);
        floats[i as usize] = af; floats0[i as usize] = bf;
        rats[i as usize] = ar; rats0[i as usize] = br;
    }
    let mut add = c.benchmark_group("Add");
    add.bench_function("add 100 f32", |b| b.iter(|| for i in 0..1000 {
        add_test(black_box(floats[i]), black_box(floats0[999 - i]));
    }));
    add.bench_function("add 100 r32", |b| b.iter(|| for i in 0..1000 {
        add_test(black_box(rats[i]), black_box(rats0[999 - i]));
    }));
    add.finish();
    let mut sub = c.benchmark_group("Sub");
    sub.bench_function("sub 100 f32", |b| b.iter(|| for i in 0..1000 {
        sub_test(black_box(floats[i]), black_box(floats0[999 - i]));
    }));
    sub.bench_function("sub 100 r32", |b| b.iter(|| for i in 0..1000 {
        sub_test(black_box(rats[i]), black_box(rats0[999 - i]));
    }));
    sub.finish();
    let mut mul = c.benchmark_group("Mul");
    mul.bench_function("mul 100 f32", |b| b.iter(|| for i in 0..1000 {
        mul_test(black_box(floats[i]), black_box(floats0[999 - i]));
    }));
    mul.bench_function("mul 100 r32", |b| b.iter(|| for i in 0..1000 {
        mul_test(black_box(rats[i]), black_box(rats0[999 - i]));
    }));
    mul.finish();
    let mut div = c.benchmark_group("Div");
    div.bench_function("div 100 f32", |b| b.iter(|| for i in 0..1000 {
        div_test(black_box(floats[i]), black_box(floats0[999 - i]));
    }));
    div.bench_function("div 100 r32", |b| b.iter(|| for i in 0..1000 {
        div_test(black_box(rats[i]), black_box(rats0[999 - i]));
    }));
    div.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);