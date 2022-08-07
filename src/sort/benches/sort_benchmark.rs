use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("array_sort");

    for array_size in [100, 1000, 5000, 8000, 10000, 50000].iter() {
        let mut array = vec![];
        for i in (0..*array_size).rev() {
            array.push(i);
        }
        group.bench_function(
            BenchmarkId::new("merge", array_size),
            |b| b.iter(|| sort::merge_sort(&mut array))
        );
    }

    for array_size in [100, 1000, 5000, 8000].iter() {
        let mut array = vec![];
        for i in (0..*array_size).rev() {
            array.push(i);
        }
        group.bench_function(
            BenchmarkId::new("insert", array_size),
            |b| b.iter(|| sort::insert_sort(&mut array))
        );
    }

    for array_size in [100, 1000, 5000, 8000, 10000].iter() {
        let mut array = vec![];
        for i in (0..*array_size).rev() {
            array.push(i);
        }
        group.bench_function(
            BenchmarkId::new("max_heap", array_size),
            |b| b.iter(|| sort::max_heap_sort(&mut array))
        );
    }

    group.finish();
}

criterion_group!(benches_merge, bench_sorts);
criterion_main!(benches_merge);