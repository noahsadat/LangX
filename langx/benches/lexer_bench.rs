use criterion::{black_box, criterion_group, criterion_main, Criterion};
use langx::lexer;

fn bench_simple_tokenization(c: &mut Criterion) {
    let source = "Set x to 10.";
    c.bench_function("lexer_simple", |b| {
        b.iter(|| lexer::tokenize(black_box(source)))
    });
}

fn bench_complex_tokenization(c: &mut Criterion) {
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
    c.bench_function("lexer_complex", |b| {
        b.iter(|| lexer::tokenize(black_box(source)))
    });
}

fn bench_large_tokenization(c: &mut Criterion) {
    // Generate a large source with many tokens
    let mut source = String::new();
    for i in 0..100 {
        source.push_str(&format!("Set x{} to {}. ", i, i));
    }
    c.bench_function("lexer_large", |b| {
        b.iter(|| lexer::tokenize(black_box(&source)))
    });
}

fn bench_string_literals(c: &mut Criterion) {
    let source = r#"Set text to "This is a long string with many characters and some escape sequences like \n and \t"."#;
    c.bench_function("lexer_string_literals", |b| {
        b.iter(|| lexer::tokenize(black_box(source)))
    });
}

fn bench_multi_line_strings(c: &mut Criterion) {
    let source = r#"
        Set text to """This is a multi-line string
        that spans multiple lines
        and contains "quotes" without escaping""".
    "#;
    c.bench_function("lexer_multi_line_strings", |b| {
        b.iter(|| lexer::tokenize(black_box(source)))
    });
}

criterion_group!(
    benches,
    bench_simple_tokenization,
    bench_complex_tokenization,
    bench_large_tokenization,
    bench_string_literals,
    bench_multi_line_strings
);
criterion_main!(benches);

