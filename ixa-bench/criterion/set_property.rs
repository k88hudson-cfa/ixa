use criterion::{criterion_group, criterion_main, Criterion};
use ixa::prelude::*;

define_rng!(SetPropertyRng);

define_entity!(Person);
define_property!(
    enum Status {
        A,
        B,
        C,
    },
    Person,
    default_const = Status::A
);
define_property!(struct Age(u8), Person, default_const = Age(0));

pub fn bench_set_property_no_dependents(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("set_property");

    for n in [1_000usize, 10_000] {
        group.bench_function(format!("enum_{n}_entities"), |b| {
            b.iter_with_large_drop(|| {
                let mut context = Context::new();
                let mut ids = Vec::with_capacity(n);
                for _ in 0..n {
                    ids.push(context.add_entity(()).unwrap());
                }
                let values = [Status::A, Status::B, Status::C];
                for (i, &id) in ids.iter().enumerate() {
                    context.set_property(id, values[i % 3]);
                }
                context
            });
        });

        group.bench_function(format!("u8_{n}_entities"), |b| {
            b.iter_with_large_drop(|| {
                let mut context = Context::new();
                let mut ids = Vec::with_capacity(n);
                for _ in 0..n {
                    ids.push(context.add_entity(()).unwrap());
                }
                for (i, &id) in ids.iter().enumerate() {
                    context.set_property(id, Age((i % 256) as u8));
                }
                context
            });
        });
    }

    group.finish();
}

pub fn bench_set_property_indexed(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("set_property_indexed");

    for n in [1_000usize, 10_000] {
        group.bench_function(format!("enum_{n}_entities"), |b| {
            b.iter_with_large_drop(|| {
                let mut context = Context::new();
                context.index_property::<Person, Status>();
                let mut ids = Vec::with_capacity(n);
                for _ in 0..n {
                    ids.push(context.add_entity(()).unwrap());
                }
                let values = [Status::A, Status::B, Status::C];
                for (i, &id) in ids.iter().enumerate() {
                    context.set_property(id, values[i % 3]);
                }
                context
            });
        });
    }

    group.finish();
}

criterion_group!(
    set_property_benches,
    bench_set_property_no_dependents,
    bench_set_property_indexed
);
criterion_main!(set_property_benches);
