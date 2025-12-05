# Copilot Instructions for promql-parser-js

## Project Overview

This is a PromQL parsing WebAssembly module based on the Rust crate [promql-rs](https://github.com/detailyang/promql-rs). It compiles Rust code to WebAssembly for use in Node.js applications.

## Technology Stack

- **Rust**: Core parsing logic using the `promql-parser` crate
- **WebAssembly**: Compiled target using `wasm-bindgen` and `wasm-pack`
- **Node.js**: JavaScript bindings and runtime
- **Jest**: JavaScript testing framework

## Project Structure

- `src/lib.rs` - Main Rust library with WASM bindings and JSON serialization
- `js/` - JavaScript files including tests and entry point
- `pkg/` - Generated WASM package (build output)
- `Cargo.toml` - Rust dependencies and configuration
- `package.json` - Node.js dependencies and scripts

## Build Commands

```bash
# Install dependencies
npm install

# Build the WASM package (requires wasm-pack and Rust toolchain)
npm run build

# Run JavaScript tests
npm test

# Run Rust tests
npm run test-rust

# Clean build artifacts
npm run clean
```

## Key Functions

- `promql_parse(query: String)` - Main exported function that parses a PromQL query and returns a JSON AST

## Coding Conventions

### Rust
- Use the `ToSerde` trait for converting PromQL AST types to JSON
- Implement `ToSerde` for new types by adding an `impl ToSerde for TypeName` block
- Use `json!` macro from `serde_json` for creating JSON values
- Return `Result<JsValue, JsError>` from WASM-exported functions

### JavaScript
- Use CommonJS `require()` for imports
- Tests use Jest with snapshot testing
- The main module is exported from `pkg/promql_parser_js.js` after building

## Testing

- Add Rust tests using `#[test]` attribute in `src/lib.rs`
- Add JavaScript tests in `js/index.test.js` using Jest
- Use snapshot testing for complex AST output validation

## Important Notes

- The project requires building the WASM module before JavaScript tests can run
- The build process requires Docker on some platforms (see `build.sh`)
- Version numbers should be synchronized between `package.json` and `Cargo.toml`
