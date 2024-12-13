## 🌺 Nolana

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

For more examples, check the [examples](./examples) directory.

## 📖 License

Nolana is free and open-source software distributed under the [MIT License](./LICENSE).
