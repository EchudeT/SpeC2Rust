# Implementation Plan: module_gnu_stdio-write.c_49

## Summary

Port `gnu/stdio-write.c` into a Rust module that provides the same write-oriented stdio surface covered by this unit: `printf`, `fprintf`, `vprintf`, `vfprintf`, `putchar`, `fputc`, `fputs`, `puts`, and `fwrite`.

The Rust implementation should stay narrow and migration-focused:

- map C stdio writing behavior onto `std::io::Write` where possible,
- keep formatting paths centered on Rust’s standard formatting machinery,
- preserve byte-count and error-result conventions through explicit adapters,
- avoid introducing new abstractions beyond what is needed to replace the current C file and its functions.

Because C varargs and `FILE *` do not map directly to safe Rust interfaces, the implementation should define internal Rust entry points around:
- generic `Write` targets for stream output,
- formatted argument handling via `std::fmt`,
- byte-slice based writing for `fwrite`/`fputs`-style operations,
- explicit result translation to C-like return values.

## Technical Context

### Language / Version

- Rust stable, edition 2021
- Minimum recommended compiler: `rustc 1.76+`

### Primary Dependencies

Use the Rust standard library by default.

Recommended dependencies:
- No third-party crates required for this module port.

Standard library facilities expected:
- `std::io::{self, Write}`
- `std::fmt`
- `std::fs::File` only if existing project stream mapping already uses it
- `std::ffi::{CStr, CString}` only if surrounding code still passes C strings into this module

### Testing

- `cargo test`

Testing focus:
- formatted writes to stdout/stderr-like sinks via test writers,
- byte count and return value behavior,
- newline behavior for `puts`,
- partial and full writes for `fwrite`,
- error propagation translated into C-compatible integer results.

### Performance Goals

- Match typical buffered write performance of the C implementation for equivalent sinks.
- Avoid unnecessary intermediate allocations for direct byte and string writes.
- Use stack-based formatting paths where supported by `std::fmt`.
- Keep `fwrite`-style operations linear and single-pass over the provided buffer.

## Module Mapping

### Source File Mapping

- C source: `gnu/stdio-write.c`
- Rust target: `src/module_gnu_stdio_write.rs`

If the project already groups ports by original path, an acceptable alternative is:

- Rust target: `src/gnu/stdio_write.rs`

The implementation should remain a single Rust module corresponding to the single C source file.

### Function Mapping

| C Function | Rust Implementation Plan |
|---|---|
| `printf` | Rust wrapper writing formatted output to `stdout`-equivalent target |
| `fprintf` | Rust wrapper writing formatted output to a provided stream abstraction |
| `vprintf` | Internal/shared formatted write path for stdout using prebuilt format arguments or equivalent internal representation |
| `vfprintf` | Internal/shared formatted write path for arbitrary stream target |
| `putchar` | Single-byte / single-char write to stdout target |
| `fputc` | Single-byte / single-char write to provided writer |
| `fputs` | String write to provided writer without implicit newline |
| `puts` | String write to stdout target followed by newline |
| `fwrite` | Raw byte-slice write routine over provided writer, returning item count per C semantics |

### Internal Rust API Shape

To keep the port practical in Rust, implementation should be structured around a small set of internal helpers:

- formatted write helper for any `Write`,
- single-byte write helper,
- string write helper,
- bulk byte write helper.

These helpers should support the exported module functions and centralize:
- return value conversion,
- write loop behavior,
- error handling.

## Data Model

This C file does not define module-owned structs, so the Rust port should avoid inventing persistent data structures unless required for stream adaptation.

### Data-Structure Mapping

| C Concept | Rust Mapping |
|---|---|
| `FILE *` | Borrowed stream abstraction, likely `&mut dyn std::io::Write` or project-local stream wrapper if one already exists |
| `char *` / `const char *` | `&str` when valid UTF-8 is guaranteed by caller context; otherwise `&[u8]` or `&CStr` for byte-preserving behavior |
| `void *` buffer for `fwrite` | `&[u8]` |
| `int` return code | `i32` |
| `size_t` | `usize` |
| variadic argument state (`va_list`) | Rust internal formatting representation via `std::fmt`; exact external parity may require project-local adaptation rather than direct language-level varargs |

### Ownership and Lifetime Decisions

- Writers should be borrowed, not owned, mirroring C’s non-owning `FILE *` usage.
- String and byte inputs should be borrowed slices.
- No heap allocation should be introduced unless required by unavoidable format adaptation.
- Any C-string boundary handling should validate input once and then write borrowed bytes directly.

### Error Handling Mapping

- C-style write functions usually signal failure through negative integer results or reduced item counts.
- Rust helpers should first produce `io::Result<_>`, then convert at the module boundary:
  - `printf`/`fprintf`/`vprintf`/`vfprintf`/`putchar`/`fputc`/`fputs`/`puts`: return `i32`, using `-1` on write failure where appropriate.
  - `fwrite`: return `usize` count of fully written items, allowing short-count behavior on partial write or error.
- Do not use panics for normal I/O failure paths.

## Implementation Phases

## Phase 1: Create the Rust module skeleton and core write helpers

### Goal

Establish the direct Rust replacement file and the minimal internal helper functions needed to support all migrated operations.

### Tasks

- Add `src/module_gnu_stdio_write.rs` or the project-standard equivalent path.
- Define module-local helpers for:
  - writing formatted content to a `Write`,
  - writing a single byte,
  - writing borrowed string/byte slices,
  - converting `io::Result` into C-like return values.
- Decide and document the exact stream abstraction used in this project branch:
  - `&mut dyn Write` if fully Rust-native,
  - or an existing project-local stream type if one already represents C `FILE *`.
- Keep helper visibility restricted to this module unless another already-migrated file requires reuse.

### Deliverables

- Compiling module file with helper signatures.
- Basic unit tests for helper return conversion and simple byte writes.

## Phase 2: Port character, string, and bulk-write functions

### Goal

Migrate the non-variadic write functions first, since they map directly onto Rust I/O and establish the module’s stream semantics.

### Tasks

- Implement `putchar`.
- Implement `fputc`.
- Implement `fputs`.
- Implement `puts`.
- Implement `fwrite`.

Implementation notes:
- `putchar`/`fputc` should write exactly one byte/character and return the written byte value or failure sentinel according to the chosen compatibility rule.
- `fputs` should write the string bytes exactly as provided, with no appended newline.
- `puts` should write the string then a single newline.
- `fwrite` should:
  - accept byte input as a raw buffer equivalent,
  - compute total bytes as `size * nmemb`,
  - write in a loop if needed,
  - return completed element count, not raw byte count.

### Testing

- Write success/failure cases using in-memory writers.
- Validate newline behavior for `puts`.
- Validate zero-length `fwrite` behavior.
- Validate short-write handling for `fwrite` using a custom test writer that simulates partial progress or failure.

### Deliverables

- Non-formatting functions migrated and tested.
- Clear return-value behavior documented in code comments where C semantics are non-obvious.

## Phase 3: Port formatted output paths

### Goal

Implement the formatting-based functions while reusing the stream and error-conversion behavior established earlier.

### Tasks

- Implement `fprintf`.
- Implement `printf`.
- Implement internal/shared logic for `vfprintf`.
- Implement internal/shared logic for `vprintf`.

Implementation notes:
- Prefer a single internal formatting helper taking a generic `Write` target.
- Use `std::fmt` and `write_fmt` as the primary mechanism.
- Since Rust does not expose C-style variadic functions in safe, idiomatic form, represent `vfprintf`/`vprintf` through whichever project-local formatting adapter already exists or, if this branch is purely Rust-facing, as internal functions consuming prepared formatting arguments.
- Do not broaden the design to support arbitrary new formatting engines.

### Testing

- Verify formatted string emission for stdout-like and arbitrary writer targets.
- Verify byte-count or success/failure return conversion.
- Verify formatting errors are treated as write failure at the public boundary if applicable.

### Deliverables

- Formatted output functions compiled and covered by unit tests.
- Shared formatting path with no duplicated write logic.

## Phase 4: Final semantic alignment and module integration

### Goal

Align edge-case behavior with the original C module and finish project integration without extending scope.

### Tasks

- Review original `gnu/stdio-write.c` for:
  - exact return conventions,
  - newline and terminator handling,
  - any assumptions about error indicators,
  - any distinction between byte and character handling.
- Adjust Rust function boundaries to preserve those semantics as closely as feasible.
- Integrate the module into the crate’s existing exports and call sites.
- Remove dead code or duplicate helper paths created during migration.

### Testing

- Run `cargo test`.
- Add regression tests for any edge cases discovered during source review.

### Deliverables

- Integrated Rust replacement for `gnu/stdio-write.c`.
- Passing tests for the module and any affected call sites.

## Risks and Handling

### Variadic Function Parity

C `printf`/`fprintf`/`vprintf`/`vfprintf` rely on varargs semantics that do not directly translate to Rust. The plan is to keep parity by routing through Rust formatting only to the extent supported by the surrounding project API, without adding a separate formatting subsystem.

### Stream Type Mismatch

If the larger port already uses a custom representation for `FILE *`, this module should adapt to that existing type rather than forcing `std::io::Write` at all boundaries. `Write` should remain the internal implementation model where possible.

### String Encoding Assumptions

C string inputs may be non-UTF-8. For byte-preserving behavior, implementation should prefer byte-slice or `CStr` handling at the boundary and avoid assuming `&str` unless guaranteed by caller context.