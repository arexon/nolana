use std::fs;

use nolana::{Parser, Printer, PrinterOptions, lowerer::Lowerer, semantic::SemanticChecker};

fn main() {
    let source_text = fs::read_to_string("examples/sample.molang").unwrap();

    let mut ast = Parser::new(&source_text).parse();

    if !ast.errors.is_empty() {
        for error in ast.errors {
            let error = error.with_source_code(source_text.clone());
            print!("{error:?}");
        }
        return;
    }

    let errors = SemanticChecker::default().check(&mut ast.program);
    if !errors.is_empty() {
        for error in errors {
            let error = error.with_source_code(source_text.clone());
            print!("{error:?}");
        }
        return;
    }

    println!("AST: {:#?}", ast.program);

    let ir_program = Lowerer::default().lower(&ast.program);

    println!("IR: {:#?}", ir_program);

    let output =
        Printer::default().with_options(PrinterOptions { minify: false }).print(&ir_program);

    println!("Printed Molang:\n{output}");
}
