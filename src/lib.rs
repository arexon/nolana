#![doc = include_str!("../README.md")]

mod parser;
pub use parser::{ParseResult, Parser};

mod codegen;
pub use codegen::{Codegen, CodegenOptions};

mod compiler;
pub use compiler::Compiler;

pub mod ast;
pub mod diagnostic;
pub mod semantic;
pub mod span;
mod token;
pub mod traverse;
