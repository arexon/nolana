## 🌺 Nolana

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![MIT licensed][license-badge]][license-url]
[![Build Status][ci-badge]][ci-url]

> Nolana is an extremely fast parser for [Molang](https://bedrock.dev/docs/stable/Molang).

Project goals, in roughly descending priority:

- Be fully compliant with Molang
- Optimize for speed while maintaining a user-friendly AST
- Ensure the parser can recover from most syntax errors
- Provide extra tools for code generation (printing), simple semantic analysis, and AST traversal

## ⚡ Performance

Run `just bench` to try the benchmarks.

```norust
test parser  ... bench:         593 ns/iter (+/- 5)
test codegen ... bench:         182 ns/iter (+/- 1)
```

> CPU: Intel Core i5-12400F

Nolana achieves this performance by leveraging [logos](https://github.com/maciejhirsz/logos) as its lexer, avoiding unnecessary allocations by using an arena allocator, and ensuring the memory size of each AST node is small.

## 📝 Example

```rust
use nolana::{
    allocator::Allocator,
    codegen::{Codegen, CodegenOptions},
    parser::{Parser, ParserReturn},
    semantic::SemanticChecker,
};

let source_text = "math.cos(q.anim_time * 38) * v.rotation_scale + v.x * v.x * q.life_time";

// Create an arena allocator to store the AST nodes.
let allocator = Allocator::default();

// Parse the provided Molang source code.
let ParserReturn {
    program,
    errors,
    panicked,
} = Parser::new(&allocator, source_text).parse();

// Check for syntax errors.
if !errors.is_empty() {
    for error in errors {
        let error = error.with_source_code(source_text);
        print!("{error:?}");
    }
    if panicked {
        println!("Encountered an unrecoverable error");
    }
    return;
}

// Pretty print the AST.
println!("AST: {:#?}", program);
```

For more info, check the [examples](./examples) directory or read the [documentation](https://docs.rs/nolana).

## 📖 License

Nolana is free and open-source software distributed under the [MIT License](./LICENSE).

[crates-url]: https://crates.io/crates/nolana
[crates-badge]: https://img.shields.io/crates/d/nolana?label=crates.io
[docs-url]: https://docs.rs/nolana
[docs-badge]: https://img.shields.io/docsrs/nolana
[license-url]: https://github.com/arexon/nolana/blob/main/LICENSE
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[ci-badge]: https://github.com/arexon/nolana/actions/workflows/ci.yml/badge.svg?event=push&branch=main
[ci-url]: https://github.com/arexon/nolana/actions/workflows/ci.yml?query=event%3Apush+branch%3Amain
