Project from Zero to Production in Rust book.

# Notes

- `rustup` is more than a Rust installer; main proposition is toolchain management (compilation target and release channel)
- a new version of the compiler is release on the stable channel every six weeks
- there are two release channels: beta/nightly
- nightly is not recommended for production
  - https://doc.rust-lang.org/beta/unstable-book/
- rust-analyzer: language server protocol for rust
- tests:
  - `cargo test`, build project before running tests
- code coverage:
  - [`cargo tarpaulin`](https://crates.io/crates/cargo-tarpaulin) (only supports x86_64 arch. running Linuxs)
- linting:
  - [`clippy`](https://github.com/rust-lang/rust-clippy), the official rust linter
  - included by default
  - clippy is named after the paperclip-shaped MSWord.
- formatting:
  - `rustfmt` is the official Rust formatter
  - included by default
  - `cargo fmt`
- security:
  - [`cargo audit`](https://github.com/RustSec/rustsec/tree/main/cargo-audit), sub-command to check for vulnerabilities
  - `cargo audit` to scan your dependency tree
- actix-web, web framework for rust
  - alternatives: tide, rocket, warp
  - actix's runtime is built on top of [`tokio`](https://crates.io/crates/tokio)
- `main` cannot be an async. function, reason:
  - async programming in rust is built on top of `Future` trait
  - all futures explose a poll method
  - Rust's standard library does not include an async. runtime.
- Rust macros operate at the token level
  - Take a stream of symbols and output a strem of new symbols which then gets passed to the compiler
  - main purpose of Rust macros is code generation
  - [`cargo expand`](https://crates.io/crates/cargo-expand) (requires nightly compiler):
    - expands all macros in your code without passing the output to the compiler
    - debug/inspect; allow you to step through it and understand what's going on
- tests can be written in three different places
  - next to your code/embedded test module
  - external tests folder
  - as part of public documentation (doc tests)
- anything under `tests` folder (and your documentation test)
  - are compiled in their own separate binaries (this has consequences when it comes to visibility rules)
  - used mostly for integration tests

