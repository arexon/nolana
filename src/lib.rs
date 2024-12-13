#![doc = include_str!("../README.md")]

pub mod ast;
pub mod ast_builder;
pub mod codegen;
pub mod diagnostic;
pub mod parser;
pub mod semantic;
pub mod span;
mod token;
pub mod visit;
pub mod visit_mut;
pub mod allocator {
    pub use oxc_allocator::*;
}
