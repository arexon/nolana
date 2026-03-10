use std::fs;

use criterion::{Criterion, criterion_group, criterion_main};
use nolana::{Parser, Printer, ir::IrProgram, lowerer::Lowerer};

fn print(program: &IrProgram) {
    let _ = Printer::default().print(program);
}

fn bench_codegen(c: &mut Criterion) {
    let source_code = fs::read_to_string("benches/sample.molang").unwrap();
    let result = Parser::new(&source_code).parse();
    let ir = Lowerer::default().lower(&result.program);
    c.bench_function("codegen", |b| {
        b.iter(|| {
            print(&ir);
        });
    });
}

criterion_group!(parser, bench_codegen);
criterion_main!(parser);
