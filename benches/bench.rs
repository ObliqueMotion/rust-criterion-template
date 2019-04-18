use criterion::{criterion_group, criterion_main, Criterion, Fun};

fn function1() -> u32 { 5 } // usually not defined here.
fn function2() -> u32 { 5 } // usually not defined here.

// Benchmark a single function's performance.
fn benchmark_single_function(critreion: &mut Criterion) {
    critreion.bench_function(
        "Description of single function.",
        |bencher| {
            bencher.iter(move || assert_eq!(5, function1()));
        },
    );
}

// Compare multiple functions' performance.
fn benchmark_compare_functions(critreion: &mut Criterion) {
    let benchmark_function1 = Fun::new("FunctionName1", |bencher, _| {
        bencher.iter(move || assert_eq!(5, function1()));
    });
    let benchmark_function2 = Fun::new("FunctionName2", |bencher, _| {
        bencher.iter(move || assert_eq!(5, function2()));
    });
    let benchmarks = vec![benchmark_function1, benchmark_function2];
    critreion.bench_functions(
        "Description Of Overal Functions To Be Compared",
        benchmarks,
        &20,
    );
}

// Adjust the sample size dynamically
fn sample_size(n: usize) -> Criterion {
    Criterion::default().sample_size(n)
}

// Create a group of benchmarks to run.
criterion_group! {
    name = single_function;
    config = sample_size(1000);
    targets = benchmark_single_function, // From line 7
}

// Create a group of benchmarks to run.
criterion_group! {
    name = multiple_functions;
    config = sample_size(100);
    targets = benchmark_compare_functions, // From line 17
}

// Generate the main to run the benchmark groups.
criterion_main! {
    single_function,
    multiple_functions,
}
