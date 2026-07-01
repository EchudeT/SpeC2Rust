# Implementation Plan

## Summary

Port the `quotearg.c` subset for `quotearg_colon` and `quotearg_colon_mem` into a Rust module that preserves existing behavior and calling intent while narrowing the scope strictly to these two functions and the internal data they directly depend on.

The Rust implementation should:
- translate the existing byte-oriented quoting logic into safe Rust,
- preserve colon-specific escaping/quoting behavior,
- operate on arbitrary byte slices for the `_mem` variant,
- provide a string-facing wrapper for the non-`_mem` entry point,
- avoid introducing new formatting or quoting capabilities beyond what these functions already require.

The preferred technical approach is to implement the core logic over `&[u8]`, build output into `Vec<u8>` or `String` as appropriate, and isolate any C global-option style dependencies into minimal Rust equivalents only if the two target functions require them. Ownership and lifetime concerns should be resolved by returning owned Rust values rather than emulating C static buffers.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library only
  - No third-party crates recommended, since the input provides no evidence of external dependency requirements
- **Testing**:
  - `cargo test`
  - unit tests for direct function behavior
  - byte-oriented tests for embedded/non-UTF-8 input in `quotearg_colon_mem`
  - regression-style tests for colon handling and edge characters
- **Performance Goals**:
  - linear-time processing over input length
  - no unnecessary intermediate allocations beyond the output buffer
  - preallocate output conservatively when input length is known
  - remain close to C behavior for byte scanning and escaping cost

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` → `src/quotearg.rs`

### Function Mapping

- `quotearg_colon` → `pub fn quotearg_colon(input: &str) -> String`
- `quotearg_colon_mem` → `pub fn quotearg_colon_mem(input: &[u8]) -> Vec<u8>` or `String` depending on exact output requirements discovered during migration

### Rust Module Placement

Keep the implementation in a single Rust source file matching the C source responsibility:

- `src/quotearg.rs`

If the crate already has a root module layout, expose only the migrated functions needed by the current branch through the existing `mod` declarations. Do not split helpers into extra files unless the current repository structure already requires it.

## Data Model

The C analysis lists anonymous data structures but does not identify named structures used directly by the target functions. The Rust plan should therefore map only the structures proven necessary during migration.

### Data Structure Mapping Strategy

- **C anonymous structs used only as internal option/state carriers**
  → private Rust `struct` with named fields only if required by `quotearg_colon` / `quotearg_colon_mem`

- **C enum-like mode flags or integer constants**
  → private Rust `enum` or `const` values, whichever matches the minimal dependency surface

- **C pointer + length input pairs**
  → `&[u8]`

- **C NUL-terminated string inputs**
  → `&str` for `quotearg_colon` if the source interface is string-based in the Rust port

- **C mutable output buffers / static return buffers**
  → owned `Vec<u8>` or `String`

### Notes on Unknown Anonymous Structures

Because the analysis only reports repeated `anonymous` entries without field details:
- do not invent a broad Rust model for all of them,
- identify the exact state and constants referenced by these two functions during implementation,
- define the smallest private Rust representations needed to support migrated logic.

### Memory Management

- Replace C-managed output memory with owned Rust returns.
- Avoid borrowed return values tied to internal mutable globals.
- Use byte-slice processing for `_mem` to preserve non-UTF-8 behavior.
- Convert to `String` only where UTF-8 validity is guaranteed or where escaped ASCII output is explicitly constructed.

### Error Handling

These functions are expected to be pure formatting/quoting helpers rather than fallible I/O operations. Prefer infallible APIs:
- return owned quoted output directly,
- avoid `Result` unless migration reveals a true fallible conversion boundary,
- where UTF-8 conversion is uncertain, keep the core API byte-based instead of forcing fallible string conversion.

## Implementation Phases

## Phase 1: Inspect and Isolate Required C Logic

- Locate `quotearg_colon` and `quotearg_colon_mem` in `quotearg.c`.
- Trace only the helpers, constants, and option/state objects they directly depend on.
- Identify:
  - whether `quotearg_colon` is a thin wrapper over `_mem`,
  - whether output is fundamentally byte-oriented or string-oriented,
  - whether colon behavior is implemented via a quoting style, flag, or dedicated helper path.
- Record any anonymous C structs actually touched by these functions and reduce them to minimal named Rust equivalents.
- Settle the Rust signatures based on the discovered call pattern, favoring:
  - `&str -> String` for the plain variant,
  - `&[u8] -> Vec<u8>` or escaped ASCII `String` for the memory variant.

## Phase 2: Port Core Quoting Logic to `src/quotearg.rs`

- Implement the minimal private constants/enums/structs needed for colon quoting behavior.
- Port the byte-scanning logic first into a private helper operating on `&[u8]`.
- Ensure the helper preserves:
  - colon-specific escaping/quoting rules,
  - handling of embedded NUL or arbitrary bytes if present,
  - deterministic output matching the C function behavior.
- Implement `quotearg_colon_mem` on top of that helper.
- Implement `quotearg_colon` as a thin wrapper, reusing the same core path instead of duplicating logic.

## Phase 3: Replace C Memory Patterns with Rust Ownership

- Remove any dependence on C-style static buffers or caller-managed writable buffers in the Rust port.
- Preallocate output capacity based on input length and expected escaping expansion.
- Use `Vec<u8>` internally when byte preservation matters.
- Convert internal byte output to `String` only when the produced representation is guaranteed valid UTF-8; otherwise, keep the public API byte-based for `_mem`.
- Keep all helper visibility private except for the two migrated functions.

## Phase 4: Validate Behavior with Focused Tests

- Add unit tests in the module or crate test layout covering:
  - empty input,
  - strings without colons,
  - strings containing one or multiple colons,
  - inputs requiring escaping in addition to colon handling,
  - byte-slice inputs with non-UTF-8 data for `_mem`,
  - parity between `quotearg_colon` and `quotearg_colon_mem` for UTF-8-compatible inputs.
- Use expected outputs derived from current C behavior for regression confidence.
- Run `cargo test` and adjust only for behavior mismatches attributable to safe ownership conversion, not to new functionality.