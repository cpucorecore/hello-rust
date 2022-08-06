use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_sorts_rev(c: &mut Criterion) {
    let mut group = c.benchmark_group("array_sort_rev");

    for array_size in [1000, 5000, 8000, 10000, 15000].iter() {
        let mut array = vec![];
        for i in (0..*array_size).rev() {
            array.push(i);
        }
        // println!("before merge/{}: {:?}", array_size, &array);
        group.bench_function(
            BenchmarkId::new("merge", array_size),
            |b| b.iter(|| sort::merge_sort(&mut array))
        );
        // println!("after merge/{}: {:?}", array_size, &array);

        let mut array = vec![];
        for i in (0..*array_size).rev() {
            array.push(i);
        }
        // println!("before insert/{}: {:?}", array_size, &array);
        group.bench_function(
            BenchmarkId::new("insert", array_size),
            |b| b.iter(|| sort::insert_sort(&mut array))
        );
        // println!("after insert/{}: {:?}", array_size, &array);
    }

    group.finish();
}

fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("array_sort_sorted");

    for array_size in [1000, 5000, 8000, 10000, 15000].iter() {
        let mut array = vec![];
        for i in 0..*array_size {
            array.push(i);
        }
        // println!("before merge/{}: {:?}", array_size, &array);
        group.bench_function(
            BenchmarkId::new("merge", array_size),
            |b| b.iter(|| sort::merge_sort(&mut array))
        );
        // println!("after merge/{}: {:?}", array_size, &array);

        let mut array = vec![];
        for i in 0..*array_size {
            array.push(i);
        }
        // println!("before insert/{}: {:?}", array_size, &array);
        group.bench_function(
            BenchmarkId::new("insert", array_size),
            |b| b.iter(|| sort::insert_sort(&mut array))
        );
        // println!("after insert/{}: {:?}", array_size, &array);
    }

    group.finish();
}

criterion_group!(benches_merge, bench_sorts, bench_sorts_rev);
criterion_main!(benches_merge);