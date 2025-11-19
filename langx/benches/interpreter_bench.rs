use criterion::{black_box, criterion_group, criterion_main, Criterion};
use langx::parser;
use langx::interpreter::Interpreter;

fn bench_simple_execution(c: &mut Criterion) {
    let source = "Set x to 10. print x.";
    let program = parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    
    c.bench_function("interpreter_simple", |b| {
        b.iter(|| {
            interpreter = Interpreter::new();
            interpreter.interpret(black_box(&program)).unwrap()
        })
    });
}

fn bench_arithmetic_execution(c: &mut Criterion) {
    let source = "Set result to 10 + 20 * 30 - 5 / 2. print result.";
    let program = parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    
    c.bench_function("interpreter_arithmetic", |b| {
        b.iter(|| {
            interpreter = Interpreter::new();
            interpreter.interpret(black_box(&program)).unwrap()
        })
    });
}

fn bench_loop_execution(c: &mut Criterion) {
    let source = r#"
        Set sum to 0.
        Set i to 0.
        While i is less than 100:
            Set sum to sum + i.
            Set i to i + 1.
        End while.
    "#;
    let program = parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    
    c.bench_function("interpreter_loop", |b| {
        b.iter(|| {
            interpreter = Interpreter::new();
            interpreter.interpret(black_box(&program)).unwrap()
        })
    });
}

fn bench_repeat_execution(c: &mut Criterion) {
    let source = r#"
        Set sum to 0.
        Repeat 1000 times:
            Set sum to sum + 1.
        End repeat.
    "#;
    let program = parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    
    c.bench_function("interpreter_repeat", |b| {
        b.iter(|| {
            interpreter = Interpreter::new();
            interpreter.interpret(black_box(&program)).unwrap()
        })
    });
}

fn bench_function_call_execution(c: &mut Criterion) {
    let source = r#"
        Define add with parameters a, b:
            Return a + b.
        End definition.
        Set result to Call add with 10, 20.
    "#;
    let program = parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    
    c.bench_function("interpreter_function_call", |b| {
        b.iter(|| {
            interpreter = Interpreter::new();
            interpreter.interpret(black_box(&program)).unwrap()
        })
    });
}

fn bench_recursive_function_execution(c: &mut Criterion) {
    let source = r#"
        Define factorial with parameters n:
            If n is equal to 0 then Return 1.
            Return n * Call factorial with n - 1.
        End definition.
        Set result to Call factorial with 10.
    "#;
    let program = parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    
    c.bench_function("interpreter_recursive", |b| {
        b.iter(|| {
            interpreter = Interpreter::new();
            interpreter.interpret(black_box(&program)).unwrap()
        })
    });
}

fn bench_list_operations(c: &mut Criterion) {
    let source = r#"
        Set list to [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].
        Set sum to 0.
        For each item in list:
            Set sum to sum + item.
        End for.
    "#;
    let program = parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    
    c.bench_function("interpreter_list_operations", |b| {
        b.iter(|| {
            interpreter = Interpreter::new();
            interpreter.interpret(black_box(&program)).unwrap()
        })
    });
}

fn bench_map_operations(c: &mut Criterion) {
    let source = r#"
        Set map to {"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}.
        Set sum to 0.
        Set sum to sum + map at "a".
        Set sum to sum + map at "b".
        Set sum to sum + map at "c".
        Set sum to sum + map at "d".
        Set sum to sum + map at "e".
    "#;
    let program = parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    
    c.bench_function("interpreter_map_operations", |b| {
        b.iter(|| {
            interpreter = Interpreter::new();
            interpreter.interpret(black_box(&program)).unwrap()
        })
    });
}

fn bench_string_operations(c: &mut Criterion) {
    let source = r#"
        Set text to "Hello, World!".
        Set len to Call string_length with text.
        Set sub to Call substring with text, 0, 5.
        Set parts to Call split with "a,b,c,d,e", ",".
        Set joined to Call join with parts, "-".
    "#;
    let program = parser::parse(source).unwrap();
    let mut interpreter = Interpreter::new();
    
    c.bench_function("interpreter_string_operations", |b| {
        b.iter(|| {
            interpreter = Interpreter::new();
            interpreter.interpret(black_box(&program)).unwrap()
        })
    });
}

criterion_group!(
    benches,
    bench_simple_execution,
    bench_arithmetic_execution,
    bench_loop_execution,
    bench_repeat_execution,
    bench_function_call_execution,
    bench_recursive_function_execution,
    bench_list_operations,
    bench_map_operations,
    bench_string_operations
);
criterion_main!(benches);

