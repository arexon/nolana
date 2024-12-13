use std::fs;

use nolana::{
    allocator::Allocator,
    ast::{CallExpression, CallKind, Program},
    parser::{Parser, ParserReturn},
    visit::{walk::walk_call_expression, Visit},
};

#[derive(Debug)]
struct MolangStats {
    pub math_functions: u32,
    pub queries: u32,
}

impl MolangStats {
    pub fn new(program: &Program) -> Self {
        let mut stats = Self {
            math_functions: 0,
            queries: 0,
        };
        stats.visit_program(program);
        stats
    }
}

impl<'a> Visit<'a> for MolangStats {
    fn visit_call_expression(&mut self, it: &CallExpression<'a>) {
        match it.kind {
            CallKind::Math => self.math_functions += 1,
            CallKind::Query => self.queries += 1,
        }
        walk_call_expression(self, it);
    }
}

fn main() {
    let source_text = fs::read_to_string("examples/sample.molang").unwrap();

    let allocator = Allocator::default();

    let ParserReturn {
        program,
        errors,
        panicked,
    } = Parser::new(&allocator, &source_text).parse();

    if !errors.is_empty() {
        for error in errors {
            let error = error.with_source_code(source_text.clone());
            print!("{error:?}");
        }
        if panicked {
            println!("The encountered errors were unrecoverable");
        }
        return;
    }

    let molang_stats = MolangStats::new(&program);
    println!("{molang_stats:?}");
}
