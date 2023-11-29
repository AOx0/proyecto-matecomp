use criterion::{black_box, criterion_group, criterion_main, Criterion};
use proyecto_matecomp::{BM, MS, SR};

fn bench_root_finding(c: &mut Criterion) {
    let mut group = c.benchmark_group("RF 1");

    group.bench_function("secante", |b: &mut criterion::Bencher<'_>| b.iter(|| {
        let sr = MS::new(&|x| x.powf(5.) + 20. * x.powf(2.) + x + 1.5, 10.0, 11.);
        for s in sr.into_iter() {
            black_box(s);
        }
    }));

    group.bench_function("newton-raphson", |b: &mut criterion::Bencher<'_>| b.iter(|| {
        let sr = SR::new(
            &|x| x.powf(5.) + 20. * x.powf(2.) + x + 1.5,
            &|x| 5. * x.powf(4.) + 40. * x + 1.,
            10.,
        );
        for s in sr.into_iter() {
            black_box( s);
        }
    }));

    group.bench_function("biseccion", |b: &mut criterion::Bencher<'_>| b.iter(|| {
        let solver = BM::new(&|x| x.powf(5.) + 20. * x.powf(2.) + x + 1.5, (-3., 3.));
        for s in solver.into_iter() {
            black_box(s);
        }
    }));
}

fn bench_root_finding2(c: &mut Criterion) {
    let mut group = c.benchmark_group("RF 2");

    group.bench_function("secante", |b: &mut criterion::Bencher<'_>| b.iter(|| {
        let sr = MS::new(&|x| x.powf(x) -100., 10.0, 11.);
        for s in sr.into_iter() {
            black_box(s);
        }
    }));

    group.bench_function("newton-raphson", |b: &mut criterion::Bencher<'_>| b.iter(|| {
        let sr = SR::new(
            &|x| x.powf(x) - 100.,
            &|x| x.powf(x) +  x.powf(x) * x.ln(),
            10.,
        );
        for s in sr.into_iter() {
            black_box( s);
        }
    }));

    group.bench_function("biseccion", |b: &mut criterion::Bencher<'_>| b.iter(|| {
        let solver = BM::new(&|x| x.powf(x) - 100., (-4., 4.));
        for s in solver.into_iter() {
            black_box(s);
        }
    }));
}

criterion_group!(benches, bench_root_finding, bench_root_finding2);
criterion_main!(benches);