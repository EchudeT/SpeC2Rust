# Implementation Plan: module_gnu_stdio-write.c_49

## Summary

This module ports the write-oriented stdio surface from `gnu/stdio-write.c` into Rust, covering the existing function set only: `printf`, `fprintf`, `vprintf`, `vfprintf`, `putchar`, `fputc`, `fputs`, `puts`, and `fwrite`.

The Rust implementation should map these routines onto the standard library’s output traits and stream types, using `std::io::{Write, stdout, stderr}` as the primary foundation. The port should preserve the existing module boundary and function-level behavior as closely as practical, with explicit handling for byte counts, newline behavior, stream-targeted writes, and I/O error propagation. Because the source module is C stdio-oriented and variadic in part, the implementation approach should focus on migrating the current file and functions into a single Rust module with narrow compatibility shims where needed, rather than redesigning formatting or introducing new abstractions.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library only
  - `std::io::{self, Write}`
  - `std::fmt` for formatting interoperability where applicable
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C module’s practical write behavior without adding avoidable allocations beyond what formatting requires.
  - Use direct writes to `Write` implementors for byte-oriented functions.
  - Keep per-call overhead low and avoid unnecessary buffering layers unless already implied by the target writer.
  - Preserve linear write cost with respect to input size for `fwrite`/string output paths.

## Module Mapping

### Source-to-target file mapping

- `gnu/stdio-write.c` → `src/module_gnu_stdio_write.rs`

### Function migration mapping

- `printf` → Rust function in `src/module_gnu_stdio_write.rs`
  - Implemented as formatted write to standard output.
- `fprintf` → Rust function in `src/module_gnu_stdio_write.rs`
  - Implemented as formatted write to a provided writer/stream abstraction.
- `vprintf` → Rust function in `src/module_gnu_stdio_write.rs`
  - Implemented via shared internal formatting path targeting standard output.
- `vfprintf` → Rust function in `src/module_gnu_stdio_write.rs`
  - Implemented via shared internal formatting path targeting a provided writer.
- `putchar` → Rust function in `src/module_gnu_stdio_write.rs`
  - Single-byte/character write to standard output.
- `fputc` → Rust function in `src/module_gnu_stdio_write.rs`
  - Single-byte/character write to a provided writer.
- `fputs` → Rust function in `src/module_gnu_stdio_write.rs`
  - String write without implicit trailing newline.
- `puts` → Rust function in `src/module_gnu_stdio_write.rs`
  - String write with appended newline to standard output.
- `fwrite` → Rust function in `src/module_gnu_stdio_write.rs`
  - Raw byte-slice block write to a provided writer.

### Internal organization

Keep all migrated logic in a single Rust module corresponding to the original C file. Shared helper routines are acceptable only when they directly eliminate duplication among the listed functions, such as:

- a helper for writing formatted content to a `Write` target
- a helper for converting Rust I/O results into the chosen C-like return convention
- a helper for block-write count handling in `fwrite`

No additional modules should be introduced unless required by the crate’s existing structure.

## Data Model

The input analysis does not identify module-specific C structs. The port therefore centers on function and stream mappings rather than custom data structure migration.

### C-to-Rust type mapping

- `FILE *` → mutable reference to a writer-compatible Rust type
  - Preferred concrete/trait form: `&mut dyn std::io::Write` or the narrowest existing crate-local stream type if one already exists.
- `char *` / `const char *` → `&str` for text-oriented APIs when valid UTF-8 is guaranteed by the surrounding crate contract; otherwise `&[u8]` or `&CStr` at the compatibility edge.
- `void *` / raw buffer in `fwrite` → `&[u8]`
- `int` return values for character/text functions → `Result<usize, std::io::Error>` internally, with compatibility conversion at the public boundary if the crate preserves C-style returns
- `size_t` → `usize`
- variadic argument state used by `vprintf`/`vfprintf` → Rust formatting arguments representation through `std::fmt::Arguments` where feasible

### Return-value handling model

Because the original C functions use integer/count return conventions while Rust uses `Result`, the implementation should separate:

- **internal Rust-native layer**: returns `io::Result<usize>` or `io::Result<()>`
- **public compatibility layer**: converts results into the exact crate-required signature, including error sentinel values if this port is preserving C-like APIs

This keeps memory safety and error propagation explicit while still allowing compatibility with the original module contract.

### Memory management considerations

- Replace all raw pointer writes with safe slice/string writes.
- Avoid retaining borrowed output buffers beyond the call.
- Use stack-local formatting state only.
- Ensure partial writes are handled explicitly for `fwrite`; use `write_all` only when the intended semantic is full completion or error, otherwise count actual bytes/elements written according to the original function’s contract.

## Implementation Phases

## Phase 1: Create module skeleton and byte-oriented write paths

### Goals
Port the non-variadic, direct-output functions first so the module has a working write core.

### Tasks
- Create `src/module_gnu_stdio_write.rs`.
- Implement stream-targeted byte/text helpers using `std::io::Write`.
- Port:
  - `putchar`
  - `fputc`
  - `fputs`
  - `puts`
  - `fwrite`
- Define the module’s return-type strategy:
  - Rust-native `io::Result`
  - compatibility conversion layer only if required by the crate API
- Add unit tests for:
  - single-character output
  - string output with and without newline
  - block writes and returned counts
  - empty input handling
  - I/O error propagation from a failing test writer

### Exit Criteria
- All direct write functions compile and pass tests.
- No unsafe code is introduced unless required by an already-existing crate boundary.

## Phase 2: Port formatted output functions

### Goals
Implement the formatted write surface while reusing the Phase 1 write foundation.

### Tasks
- Add shared formatted-write helper(s) targeting `Write`.
- Port:
  - `printf`
  - `fprintf`
  - `vprintf`
  - `vfprintf`
- Represent variadic formatting through Rust mechanisms, preferably `std::fmt::Arguments`.
- Keep stdout-targeted and writer-targeted paths thin and reuse common logic.
- Ensure returned counts follow the crate’s selected compatibility convention.

### Testing
- Add tests covering:
  - formatted stdout-targeted output through injectable writer logic where practical
  - formatted writer-targeted output
  - zero-length formatted output
  - formatting count correctness
  - formatting-path I/O failures

### Exit Criteria
- All listed functions are present in the Rust module.
- Formatted and unformatted paths share common error and count handling.

## Phase 3: Align semantics with C behavior and finalize integration

### Goals
Resolve edge semantics and complete module-level parity for this file.

### Tasks
- Verify newline behavior for `puts`.
- Verify character conversion behavior for `putchar`/`fputc` matches the intended byte-oriented contract.
- Confirm `fwrite` element-count semantics versus byte-count semantics and encode the correct conversion.
- Review all functions for:
  - partial write handling
  - count overflow assumptions
  - consistent error sentinel mapping if public signatures are C-like
- Integrate the module into the crate’s existing exports without adding new public surface beyond the migrated functions.

### Testing
- Add final regression tests for:
  - partial-write scenarios in `fwrite`
  - exact return values on success/failure
  - newline and no-newline distinctions
  - invalid or boundary-sized inputs as allowed by the chosen Rust signatures

### Exit Criteria
- The Rust file fully replaces the behavior of `gnu/stdio-write.c` within the scope of the listed functions.
- Tests pass under `cargo test`.
- The migrated implementation remains limited to the original file and function set.