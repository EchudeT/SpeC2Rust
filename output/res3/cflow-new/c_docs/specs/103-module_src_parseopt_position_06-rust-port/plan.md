# Implementation Plan: module_src_parseopt_position_06

## Summary

Port the position-tracking and word-wrapping logic from `src/parseopt/wordwrap.c` into a Rust module that preserves the existing control flow and behavior of the C implementation without adding new capabilities. The Rust implementation should focus on translating the small position helper routines (`position_init`, `position_incr`, `position_add`, `position_eq`) and the wrapping helpers (`wordwrap_last_ws`, `flush_line`) into safe, idiomatic Rust using owned buffers and explicit mutable state instead of raw pointer arithmetic.

The implementation approach is to keep the module boundaries narrow: migrate the logic into a single Rust source file under the corresponding parse option area, model the C position state as a compact Rust struct, and represent line-buffer state with slices/`String`/`Vec<char>` or byte-oriented buffers as required by the original behavior. Memory ownership should be made explicit so that temporary line manipulation does not rely on aliasing or manual lifetime management. Error handling should remain minimal and local, using standard return types only where output operations can fail.

## Technical Context

### Language/Version
- Rust 1.78+ (stable)

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended based on the available module evidence

### Testing
- `cargo test`

### Performance Goals
- Preserve the current asymptotic behavior of position updates and line scanning
- Keep word-wrap whitespace search linear in the size of the current buffered line, matching the C routine expectations
- Avoid unnecessary string reallocations where mutable buffers can be reused
- Maintain low-overhead state updates for position accounting

## Module Mapping

### C to Rust File Mapping
- `src/parseopt/wordwrap.c` -> `src/parseopt/wordwrap.rs`

### Function Mapping
- `position_init` -> `Position::new(...)` or `Position::init(...)`
- `position_incr` -> `Position::incr(&mut self, ch: ...)`
- `position_add` -> `Position::add(&mut self, text: ...)`
- `position_eq` -> `Position::eq_pos(&self, other: &Position)` or `PartialEq` implementation
- `wordwrap_last_ws` -> `fn last_ws(...) -> Option<usize>`
- `flush_line` -> `fn flush_line(...) -> Result<..., ...>` or `fn flush_line(...)` depending on whether the surrounding output path is fallible

### Rust Module Placement
- Keep the implementation in a single module corresponding to the original C file
- If parent module declarations already exist, expose only the functions/types required by the current parseopt subsystem
- Do not split helpers into extra files unless existing Rust project structure already requires it

## Data Model

Because the C analysis exposes only anonymous data structures, the Rust data model should be derived narrowly from actual usage in `wordwrap.c` during porting.

### Core State Mapping
- C anonymous struct used for position tracking -> Rust `struct Position`
  - Likely fields:
    - current line number/count -> `usize`
    - current column/offset -> `usize`
    - any tab- or width-related counters -> `usize`
- C line or wrap buffer state held through local structs or ad hoc variables -> Rust local state using:
  - `String` for UTF-8 text buffers if original logic is character-oriented
  - `Vec<u8>` or `&[u8]` if original logic is byte-oriented and depends on direct byte scanning
- C whitespace search return values/pointers -> Rust `Option<usize>` index into the current buffer
- C equality helper over position struct -> Rust `PartialEq`/`Eq` where field-for-field comparison matches the original semantics

### Suggested Rust Structures
```rust
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Position {
    line: usize,
    column: usize,
    // add only fields confirmed by the C source during migration
}
```

If `flush_line` depends on accumulated formatting state beyond `Position`, define one additional private struct only if the C file already groups those fields logically. Otherwise, keep state as function-local variables to avoid inventing new abstractions.

### Memory Management Notes
- Replace pointer traversal with indexed access or iterators
- Use borrowing for read-only scans and `&mut` references for state updates
- Avoid storing references into buffers across mutations; store indices instead
- Prefer `usize` for indices and counters after validating no sentinel negative values are required by the C code

### Error Handling Notes
- Pure position helpers should remain infallible
- Output-emitting functions should return `std::io::Result<()>` only if they directly write to an output sink in the Rust port
- If the original C code writes into caller-owned buffers rather than files/streams, preserve that with mutable buffer parameters and infallible functions

## Implementation Phases

### Phase 1: Inspect and Map Existing C Logic
- Read `src/parseopt/wordwrap.c` and identify the exact fields used by the anonymous position-related structures
- Determine whether the implementation is byte-based or character-based
- Identify all callers of:
  - `position_init`
  - `position_incr`
  - `position_add`
  - `position_eq`
  - `wordwrap_last_ws`
  - `flush_line`
- Record any output-side fallibility to decide whether `flush_line` should return `Result`
- Create the Rust file skeleton at `src/parseopt/wordwrap.rs` and wire it into the existing module tree without adding extra modules

### Phase 2: Port Position Helpers
- Introduce `Position` with only the fields required by the C implementation
- Port:
  - initialization logic
  - single-step increment logic
  - multi-character accumulation logic
  - equality comparison
- Translate any C sentinel or macro-based behavior into explicit Rust conditionals
- Add unit tests for:
  - initial state
  - increment behavior across ordinary characters
  - newline or special-width handling if present
  - additive updates over strings/slices
  - equality comparisons

### Phase 3: Port Whitespace Search and Flush Logic
- Port `wordwrap_last_ws` using safe scanning over the current line buffer
- Port `flush_line` preserving original buffer mutation order and wrap decisions
- Replace C pointer slicing with Rust index boundaries checked against buffer length
- Keep output behavior aligned with the original function signatures as closely as possible
- Add focused tests for:
  - finding the last whitespace in representative buffers
  - no-whitespace cases
  - flush behavior at wrap boundaries
  - position updates after flushes if the C logic couples these behaviors

### Phase 4: Integrate and Validate Module Behavior
- Connect the Rust module to the existing parseopt flow on branch `103-module_src_parseopt_position_06-rust-port`
- Ensure all former C call sites use the Rust equivalents with no capability expansion
- Run `cargo test` and fix any mismatches caused by indexing, newline accounting, or ownership differences
- Perform a final pass to remove any unnecessary helper abstractions introduced during translation and keep the module close to the original scope