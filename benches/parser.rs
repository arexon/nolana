use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};
use nolana::Parser;

fn parse(source: &str) {
    let _ = Parser::new(source).parse();
}

fn bench_parser(c: &mut Criterion) {
    let source_code = fs::read_to_string("benches/sample.molang").unwrap();
    c.bench_function("parser", |b| {
        b.iter(|| {
            parse(&source_code);
        });
    });
}

criterion_group!(parser, bench_parser);
criterion_main!(parser);
