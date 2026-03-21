use criterion::{criterion_group, criterion_main, Criterion};
use ixa::plan::Queue;

fn bench_add_and_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("plan_queue");

    for n in [1_000, 10_000, 100_000] {
        group.bench_function(format!("add_get_{n}"), |b| {
            b.iter(|| {
                let mut queue: Queue<u64, ()> = Queue::new();
                for i in 0..n {
                    queue.add_plan(i as f64, i, ());
                }
                while queue.get_next_plan().is_some() {}
            });
        });
    }

    group.finish();
}

fn bench_add_cancel_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("plan_queue_cancel");

    for n in [1_000, 10_000, 100_000] {
        group.bench_function(format!("cancel_30pct_{n}"), |b| {
            b.iter(|| {
                let mut queue: Queue<u64, ()> = Queue::new();
                let mut ids = Vec::with_capacity(n as usize);
                for i in 0..n {
                    ids.push(queue.add_plan(i as f64, i, ()));
                }
                // Cancel ~30% of plans (every 3rd)
                for i in (0..n as usize).step_by(3) {
                    queue.cancel_plan(&ids[i]);
                }
                while queue.get_next_plan().is_some() {}
            });
        });
    }

    group.finish();
}

fn bench_interleaved(c: &mut Criterion) {
    let mut group = c.benchmark_group("plan_queue_interleaved");

    for n in [1_000, 10_000, 100_000] {
        group.bench_function(format!("interleaved_{n}"), |b| {
            b.iter(|| {
                let mut queue: Queue<u64, ()> = Queue::new();
                let batch = n / 10;
                let mut time = 0.0f64;
                for _ in 0..10 {
                    for _ in 0..batch {
                        queue.add_plan(time, 0, ());
                        time += 1.0;
                    }
                    for _ in 0..batch / 2 {
                        queue.get_next_plan();
                    }
                }
                while queue.get_next_plan().is_some() {}
            });
        });
    }

    group.finish();
}

criterion_group!(
    plan_queue_benches,
    bench_add_and_get,
    bench_add_cancel_get,
    bench_interleaved
);
criterion_main!(plan_queue_benches);
