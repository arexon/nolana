_default:
    just --list --unsorted

# Make sure you have cargo-binstall installed.
# Install tools required for developing the project.
init:
    cargo binstall cargo-insta

# When ready, run the same tasks as CI
ready:
    just fmt
    just lint
    just test
    just doc

# Format all files
fmt:
    cargo fmt

# Run all the tests with cargo-insta
test:
    cargo insta test --review

# Lint the whole project
lint:
    cargo clippy -- --deny warnings

# Run all benchmarks
bench:
    cargo bench --bench parser -- --output-format bencher
    cargo bench --bench codegen -- --output-format bencher

[unix]
doc:
  RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --document-private-items

[windows]
doc:
  $Env:RUSTDOCFLAGS='-D warnings'; cargo doc --no-deps --document-private-items
