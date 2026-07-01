# Implementation Plan: module_gnu_if_09

## Summary

This module ports the formatting-support logic currently implemented in `gnu/printf-parse.c` and `gnu/vasnprintf.c` into Rust, preserving the existing parsing and conditional formatting behavior without adding new capabilities. The implementation should focus on translating the existing control flow and data handling into safe Rust where possible, while keeping the structure close enough to the C sources to make validation and review straightforward.

The technical approach is to migrate the parsing and formatting paths into a Rust module that uses byte-slice and string-slice processing, `Vec` for dynamically sized buffers, and explicit `Result`-based error propagation in places where the C code relies on sentinel values or allocation failure checks. Memory ownership should be made explicit, replacing manual allocation and resizing with standard-library containers. Any anonymous C data aggregates should be converted into named Rust structs or small enums only as needed to represent the existing intermediate state.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear-time parsing behavior relative to format-string length.
  - Avoid unnecessary string copying during parse steps.
  - Use `Vec<u8>` or `String` growth patterns comparable to the original dynamic buffer behavior.
  - Preserve existing allocation boundaries as closely as practical during migration, without introducing extra abstraction layers.

## Module Mapping

### Source File Mapping

- `gnu/printf-parse.c`
  - Port to: `src/module_gnu_if_09/printf_parse.rs`
  - Responsibility:
    - Translate format-string scanning logic.
    - Preserve the existing branching structure around conditional parse decisions.
    - Extract any helper routines currently embedded in the C file into private Rust functions only when required by borrow-checking or readability during direct migration.

- `gnu/vasnprintf.c`
  - Port to: `src/module_gnu_if_09/vasnprintf.rs`
    - Translate dynamic output-buffer construction and formatting assembly logic.
    - Preserve formatting-path behavior and buffer growth semantics using Rust-owned buffers.
    - Keep the Rust implementation aligned with the original function boundaries where feasible.

### Rust Module Layout

- `src/module_gnu_if_09/mod.rs`
  - Re-export only the functions needed by the rest of the crate.
  - Declare:
    - `mod printf_parse;`
    - `mod vasnprintf;`

This layout keeps the port limited to the original file split and avoids introducing extra support modules not evidenced by the input.

## Data Model

Because the C analysis only identifies `anonymous` data structures, the Rust data model should be derived directly from actual local aggregates encountered during migration.

### Mapping Rules

- **Anonymous C struct used for parse state**
  - Map to a named private Rust `struct`, for example:
    - `ParseState`
  - Use fields with explicit Rust types:
    - indexes/lengths: `usize`
    - flags/booleans: `bool` or compact integer types only if required for exact behavior
    - optional positions/pointers: `Option<usize>` or slice offsets

- **Anonymous C struct used for output buffer state**
    - `OutputBuffer`
  - Prefer:
    - `Vec<u8>` if the original logic is byte-oriented
    - `String` only if all operations are valid UTF-8 text operations
  - Track capacity/length through container APIs rather than duplicated manual fields unless exact behavior requires explicit bookkeeping.

- **Pointer-based string references**
  - `const char *` / `char *` -> `&[u8]`, `&str`, or `&mut Vec<u8>` depending on usage
  - Use byte slices for parser fidelity when the original C logic operates on raw bytes.

- **C integer status codes**
  - Map to:
    - `Result<T, ModuleError>` for internal Rust paths
    - or `Option<T>` only for simple presence/absence cases
  - Define a small private error enum only if multiple distinct failure paths exist in the migrated functions.

### Memory Management Decisions

- Replace manual `malloc`/`realloc`/`free` patterns with `Vec` and `String`.
- Preserve allocation-failure semantics through standard Rust container behavior; do not add custom allocators or recovery layers.
- Eliminate raw-pointer lifetime coupling by expressing ownership through function parameters and return values.
- When exact in-place mutation is needed, use mutable slices or `Vec<u8>` with index-based access instead of pointer arithmetic.

### Error Handling Decisions

- Replace sentinel return patterns with `Result`.
- Convert parse failure, invalid format progression, or buffer construction failure into explicit error returns.
- Keep the error surface narrow and local to this module; do not introduce project-wide error frameworks.

## Implementation Phases

## Phase 1: Establish Module Skeleton and Port Parse-State Types

- Create:
  - `src/module_gnu_if_09/mod.rs`
  - `src/module_gnu_if_09/printf_parse.rs`
  - `src/module_gnu_if_09/vasnprintf.rs`
- Define minimal private Rust structs corresponding to the anonymous C aggregates discovered during translation.
- Identify the two target `if` functions in the source files and map each to its Rust destination file.
- Settle foundational type choices:
  - byte-oriented parsing with slices
  - dynamic buffer with `Vec<u8>` or `String`
  - local `Result` error handling
- Add placeholder unit tests that compile and establish module wiring.

## Phase 2: Port `gnu/printf-parse.c`

- Translate the format parsing logic into `printf_parse.rs`.
- Keep branch ordering and scanning rules close to the C implementation to reduce behavioral drift.
- Replace pointer increments with explicit indices over slices.
- Introduce private helper functions only where needed to express repeated parse steps or satisfy borrowing constraints.
- Validate:
  - parser progress on representative format strings
  - handling of malformed or incomplete parse inputs
  - correct preservation of parse-state fields and transitions

## Phase 3: Port `gnu/vasnprintf.c`

- Translate the dynamic formatting/output assembly logic into `vasnprintf.rs`.
- Replace manual buffer growth with `Vec<u8>` or `String` reserve/append operations.
- Preserve output sequencing and conditional branches from the C implementation.
- Connect parser output from `printf_parse.rs` to the formatter path using explicit Rust data structures rather than shared mutable globals.
- Validate:
  - output construction for normal cases
  - capacity growth behavior under repeated appends
  - error propagation from parse stage into formatting stage

## Phase 4: Integration Cleanup and Behavior Verification

- Finalize the public/private function boundaries in `mod.rs`.
- Remove migration-only scaffolding and ensure file/function names remain traceable to the original C sources.
- Add focused `cargo test` coverage for:
  - parse-to-format integration
  - edge cases around empty input, invalid format segments, and buffer extension
- Review for:
  - unnecessary cloning
  - incorrect UTF-8 assumptions where byte processing is required
  - index bounds safety and complete replacement of manual memory management