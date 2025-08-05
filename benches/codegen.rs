use std::fs;

use criterion::{Criterion, criterion_group, criterion_main};
use nolana::{Codegen, Parser, ast::Program};

fn codegen(program: &Program) {
    let _ = Codegen::default().build(program);
}

fn bench_codegen(c: &mut Criterion) {
    let source_code = fs::read_to_string("benches/sample.molang").unwrap();
    let ret = Parser::new(&source_code).parse();
    c.bench_function("codegen", |b| {
        b.iter(|| {
            codegen(&ret.program);
        });
    });
}

criterion_group!(parser, bench_codegen);
criterion_main!(parser);
