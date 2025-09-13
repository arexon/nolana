use std::fs;

use nolana::{
    Codegen, CodegenOptions,
    semantic::SemanticChecker,
    {ParseResult, Parser},
};

fn main() {
    let source_text = fs::read_to_string("examples/sample.molang").unwrap();

    let ParseResult { mut program, errors } = Parser::new(&source_text).parse();

    if !errors.is_empty() {
        for error in errors {
            let error = error.with_source_code(source_text.clone());
            print!("{error:?}");
        }
        return;
    }

    let errors = SemanticChecker::default().check(&mut program);
    if !errors.is_empty() {
        for error in errors {
            let error = error.with_source_code(source_text.clone());
            print!("{error:?}");
        }
        return;
    }

    println!("AST: {program:#?}");

    let output = Codegen::default().with_options(CodegenOptions { minify: true }).build(&program);

    println!("Printed Molang: {output}");
}
