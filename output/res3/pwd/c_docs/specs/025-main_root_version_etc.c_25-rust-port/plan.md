# Implementation Plan

## Summary

Port `version-etc.c` into an idiomatic Rust module that preserves the existing responsibility of emitting the bug-reporting address text. The Rust implementation should stay narrowly scoped to the current C surface area: a direct migration of `emit_bug_reporting_address` into a small module under the main binary/library crate, using standard library output facilities and explicit error propagation where practical.

The implementation approach is to:
- map the single C function to a single Rust function with equivalent behavior,
- keep formatting logic local to the migrated module,
- avoid introducing broader version/reporting abstractions not required by this module analysis,
- use borrowed string types where possible to avoid unnecessary allocation,
- rely on Rust ownership and `std::io` for safe output handling.

## Technical Context

### Language / Version
- Rust stable, edition 2021
- Minimum practical toolchain target: Rust 1.74+

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended based on the available module scope

### Testing
- `cargo test`

### Performance Goals
- Match the C module’s lightweight behavior for short text emission
- Avoid unnecessary heap allocation in normal formatting paths
- Keep I/O operations straightforward and bounded to the emitted message size
- Preserve predictable control flow with minimal abstraction overhead

## Module Mapping

### C to Rust File Mapping
- `version-etc.c` -> `src/version_etc.rs` or equivalent existing crate-local module file following current repository layout

### Function Mapping
- `emit_bug_reporting_address` -> `emit_bug_reporting_address`

### Integration Mapping
- Expose the migrated function only where the current main-cluster code path requires it
- Wire call sites to the Rust module without creating new helper subsystems unless a direct signature adaptation is necessary

## Data Model

No module-specific C structs were identified in the analysis.

### Data Mapping
- C string inputs (`const char *`-style expectations, if present in surrounding usage) -> Rust `&str`
- Output stream usage from C stdio patterns -> Rust `std::io::Write` target or direct standard output/error access, depending on existing call-site needs

### Ownership and Memory
- Prefer borrowed string slices for static or caller-owned text
- Use stack-based formatting via standard macros where possible
- Avoid manual buffer management; rely on Rust string and writer semantics

### Error Handling
- If the C function writes to a stream and can fail, represent this in Rust with `std::io::Result<()>`
- If existing higher-level module conventions require infallible emission, constrain any `.expect`/ignored-result behavior to the outermost compatibility boundary rather than inside formatting logic

## Implementation Phases

### Phase 1: Module Skeleton and Signature Port
- Create the Rust module file for `version-etc.c`
- Add the Rust function corresponding to `emit_bug_reporting_address`
- Choose the narrowest function signature that fits known usage:
  - prefer `&str` inputs for text arguments,
  - prefer a `Write`-based output parameter if the C function is stream-oriented
- Add the module declaration in the crate root or existing parent module as required by current project layout

### Phase 2: Output Logic Migration
- Port the formatting and emission behavior from the C function into Rust
- Replace C stdio calls with `write!`/`writeln!` or equivalent standard-library output operations
- Preserve observable text layout and newline behavior expected from the original implementation
- Ensure no unnecessary intermediate owned strings are introduced unless formatting requires them

### Phase 3: Call-Site Integration and Error Boundary Alignment
- Update existing callers in the main-cluster path to invoke the Rust function
- Align return/error behavior with the surrounding Rust code:
  - propagate `io::Result` where already compatible,
  - otherwise contain conversion at the immediate call boundary
- Remove dependence on the C implementation for this function in the Rust port branch

### Phase 4: Tests and Cleanup
- Add focused unit tests for emitted output content and formatting boundaries
- Use writer-backed tests (for example, byte buffers) to validate exact text emission
- Confirm `cargo test` passes
- Perform a final pass to ensure the module remains limited to the migrated function and does not introduce unrelated helpers or abstractions