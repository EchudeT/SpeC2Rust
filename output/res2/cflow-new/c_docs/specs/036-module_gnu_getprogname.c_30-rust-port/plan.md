# Implementation Plan

## Summary

Port `gnu/getprogname.c` to a Rust module that preserves the existing module scope: exposing the program-name lookup behavior currently provided by `getprogname`. The Rust implementation should stay narrowly aligned with the original file and function boundary, using the Rust standard library for process argument access and string ownership management.

The implementation approach is to replace the C function with a Rust function in a correspondingly named module, deriving the program name from `std::env::args_os()` / `std::env::args()` and normalizing it into a Rust string type suitable for the surrounding project API. Because the original C code likely returns a process-global program name view, the Rust port should define a simple, explicit return contract and avoid unsafe global mutable state unless required by the wider codebase. Memory management should rely on Rust ownership and borrowing rather than replicated C-style static storage.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Constant-time access after any initial extraction, if cached by the module design already used in the project
  - Negligible overhead relative to normal process startup and argument parsing
  - No unnecessary heap allocation beyond what is needed to materialize the program name in Rust form

## Module Mapping

### C to Rust File Mapping

- `gnu/getprogname.c` -> `src/gnu/getprogname.rs`

### Function Mapping

- `getprogname` -> `pub fn getprogname(...) -> ...`

The exact Rust signature should be chosen to match current project usage during integration:
- Prefer `&'static str` only if the value is safely initialized and retained for the process lifetime without unsafe ownership gaps.
- Otherwise prefer an owned or borrow-friendly Rust type such as `Option<String>` or `Option<std::path::PathBuf>` depending on existing call expectations.
- If the C behavior is “best-effort name or null”, model that with `Option<_>` rather than sentinel values.

## Data Model

The analysis lists only anonymous data structures and no named persistent module state. The Rust port should therefore avoid inventing new public structs unless needed by the crate layout.

### Data Structure Mapping

- anonymous -> no direct Rust type required

### Value-Level Mapping

- C string pointer result -> Rust string representation:
  - `Option<String>` if ownership is simplest for call-site migration
  - or `Option<OsString>` / `Option<PathBuf>` if preserving non-UTF-8 executable names is required by surrounding code

### Memory Management Notes

- Eliminate raw pointer lifetime concerns from the C implementation.
- Use Rust-owned values or safely retained process-lifetime storage if a borrowed static result is required.
- Avoid leaking heap allocations solely to imitate C static-pointer semantics unless that is the only way to preserve the existing project API.

### Error Handling Notes

- Replace null-result semantics with `Option`.
- If path extraction or UTF-8 conversion can fail, prefer explicit fallback handling:
  - keep `OsString`/`PathBuf`, or
  - return `None` when the project expects string-only semantics.

## Implementation Phases

### Phase 1: Create the Rust Module Skeleton

- Add `src/gnu/getprogname.rs`.
- Register the module in the existing Rust crate module tree with the minimal corresponding `mod` declarations.
- Define the Rust `getprogname` function with a provisional signature based on current crate conventions.
- Keep the implementation focused on the single migrated function from `gnu/getprogname.c`.

### Phase 2: Port Function Logic

- Translate the C lookup behavior into Rust using `std::env` facilities.
- Normalize the program identifier to match expected semantics:
  - preserve full invocation path or
  - reduce to basename only, depending on the original C behavior verified during porting.
- Resolve ownership/lifetime cleanly without raw pointers.
- Preserve “absence” behavior using `Option` if the C function could yield no name.

### Phase 3: Integrate and Align Call Sites

- Update internal uses of `getprogname` to the final Rust signature.
- Remove any C-specific assumptions about null pointers, static buffers, or mutable global storage from migrated call boundaries.
- Ensure the module remains limited to the original file’s responsibility and does not absorb unrelated process metadata helpers.

### Phase 4: Add Tests and Final Validation

- Add unit tests for the Rust module covering:
  - successful retrieval of a non-empty program name in normal execution
  - basename/path behavior expected by the original implementation
  - absence or fallback behavior if argument retrieval is unavailable
- Run `cargo test`.
- Confirm no unsafe memory patterns or unnecessary allocations remain in the final implementation.