use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};
use nolana::{allocator::Allocator, ast::Program, codegen::Codegen, parser::Parser};

fn codegen(program: &Program) {
    let _ = Codegen::default().build(program);
}

fn bench_codegen(c: &mut Criterion) {
    let allocator = Allocator::default();
    let source_code = fs::read_to_string("benches/sample.molang").unwrap();
    let ret = Parser::new(&allocator, &source_code).parse();
    c.bench_function("codegen", |b| {
        b.iter(|| {
            codegen(&ret.program);
        });
    });
}

criterion_group!(parser, bench_codegen);
criterion_main!(parser);
