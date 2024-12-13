use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};
use nolana::{allocator::Allocator, parser::Parser};

fn parse(allocator: &Allocator, source: &str) {
    let _ = Parser::new(allocator, source).parse();
}

fn bench_parser(c: &mut Criterion) {
    let mut allocator = Allocator::default();
    let source_code = fs::read_to_string("benches/sample.molang").unwrap();
    c.bench_function("parser", |b| {
        b.iter(|| {
            parse(&allocator, &source_code);
            allocator.reset();
        });
    });
}

criterion_group!(parser, bench_parser);
criterion_main!(parser);
