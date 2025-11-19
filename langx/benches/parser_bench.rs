use criterion::{black_box, criterion_group, criterion_main, Criterion};
use langx::parser;

fn bench_simple_parsing(c: &mut Criterion) {
    let source = "Set x to 10. print x.";
    c.bench_function("parser_simple", |b| {
        b.iter(|| parser::parse(black_box(source)).unwrap())
    });
}

fn bench_arithmetic_parsing(c: &mut Criterion) {
    let source = "Set result to 10 + 20 * 30 - 5 / 2.";
    c.bench_function("parser_arithmetic", |b| {
        b.iter(|| parser::parse(black_box(source)).unwrap())
    });
}

fn bench_complex_parsing(c: &mut Criterion) {
    let source = r#"
        Set x to 10.
        Set y to 20.
        If x is greater than 5 then print "Hello".
        Define add with parameters a, b:
            Return a + b.
        End definition.
        Set result to Call add with x, y.
        print result.
    "#;
    c.bench_function("parser_complex", |b| {
        b.iter(|| parser::parse(black_box(source)).unwrap())
    });
}

fn bench_loop_parsing(c: &mut Criterion) {
    let source = r#"
        Set i to 0.
        While i is less than 100:
            Set i to i + 1.
        End while.
        Repeat 10 times:
            print i.
        End repeat.
    "#;
    c.bench_function("parser_loops", |b| {
        b.iter(|| parser::parse(black_box(source)).unwrap())
    });
}

fn bench_function_parsing(c: &mut Criterion) {
    let source = r#"
        Define factorial with parameters n:
            If n is equal to 0 then Return 1.
            Return n * Call factorial with n - 1.
        End definition.
        Set result to Call factorial with 5.
    "#;
    c.bench_function("parser_functions", |b| {
        b.iter(|| parser::parse(black_box(source)).unwrap())
    });
}

fn bench_list_map_parsing(c: &mut Criterion) {
    let source = r#"
        Set list to [1, 2, 3, 4, 5].
        Set map to {"key1": "value1", "key2": 42}.
        Set item to item 0 of list.
        Set value to map at "key1".
    "#;
    c.bench_function("parser_data_structures", |b| {
        b.iter(|| parser::parse(black_box(source)).unwrap())
    });
}

criterion_group!(
    benches,
    bench_simple_parsing,
    bench_arithmetic_parsing,
    bench_complex_parsing,
    bench_loop_parsing,
    bench_function_parsing,
    bench_list_map_parsing
);
criterion_main!(benches);

