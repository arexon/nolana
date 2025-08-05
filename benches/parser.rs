use std::fs;

use criterion::{Criterion, criterion_group, criterion_main};
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
