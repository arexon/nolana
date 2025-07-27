use std::fs;

use nolana::parser::{Parser, ParserReturn};

fn main() {
    let source_text = fs::read_to_string("examples/sample.molang").unwrap();

    let ParserReturn { program, errors, panicked } = Parser::new(&source_text).parse();

    if !errors.is_empty() {
        for error in errors {
            let error = error.with_source_code(source_text.clone());
            print!("{error:?}");
        }
        if panicked {
            println!("Encountered an unrecoverable error");
        }
        return;
    }

    println!("AST: {:#?}", program);
}
