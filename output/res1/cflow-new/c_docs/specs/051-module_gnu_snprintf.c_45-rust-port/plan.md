# Implementation Plan

## Summary

Port the C module `gnu/snprintf.c` to a Rust module that preserves the existing `snprintf` behavior boundary as closely as possible while using Rust’s standard formatting and buffer handling primitives internally.

The Rust implementation should focus on migrating the single existing function only, without introducing broader formatting infrastructure. The technical approach is:

- map the C `snprintf` logic to a Rust function in a directly corresponding module;
- implement bounded output writing with explicit buffer-capacity handling;
- preserve truncation-aware behavior and return-value semantics expected from `snprintf`;
- use safe Rust where possible, isolating any unavoidable low-level buffer manipulation to a small, auditable area;
- validate behavior with tests covering capacity limits, null-termination behavior where applicable, and formatted output length reporting.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::fmt`, `std::cmp`, slice/string utilities)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear behavior relative to produced output size.
  - Avoid unnecessary intermediate allocations when writing into fixed-size buffers.
  - Keep overhead close to the C implementation for typical bounded-formatting cases.
  - Limit unsafe memory access to minimal buffer write paths only if required by the public interface.

## Module Mapping

### C to Rust File Mapping

- `gnu/snprintf.c` → `src/gnu/snprintf.rs`

### Function Mapping

- `snprintf` → `pub(crate)` or `pub` Rust function named `snprintf` in `src/gnu/snprintf.rs`

### Module Organization

Use standard Rust module layout only as needed for the migrated file:

- `src/gnu/mod.rs`
- `src/gnu/snprintf.rs`

If the project already has a `gnu` module tree, place `snprintf.rs` into that existing structure and wire it through the current `mod` declarations rather than adding parallel abstractions.

## Data Model

This module exposes behavior primarily through a function and does not define standalone C structs in the analyzed input.

### Data-structure Mapping

- **C structs**: none identified
- **Rust structs/enums**: none required by default

### Operational Type Mapping

Where needed during migration:

- `char *` output buffer → `&mut [u8]` internally, or a narrowly scoped raw pointer interface if the surrounding port requires C-compatible signatures
- output size parameter (`size_t`) → `usize`
- formatted length / status return (`int`) → `i32` or project-standard integer type matching original semantics
- C string termination rules → explicit trailing `0` byte management in buffer-writing logic

### Memory and Error Handling Notes

- Treat the destination as a bounded byte buffer.
- Ensure writes never exceed the provided capacity.
- Handle the zero-length buffer case explicitly.
- Separate:
  - number of bytes that would have been written, and
  - number of bytes actually stored.
- If formatting failure states exist in the surrounding port design, convert them into a deterministic return path consistent with the C function’s established behavior rather than introducing new error types.

## Implementation Phases

## Phase 1: Create Module Skeleton and Signature Mapping

- Add `src/gnu/snprintf.rs`.
- Register the module in `src/gnu/mod.rs` if not already present.
- Define the Rust `snprintf` entry point with a signature aligned to the surrounding port’s calling conventions.
- Map C parameter types to Rust equivalents with particular attention to:
  - destination buffer representation,
  - size handling,
  - return type width.
- Document the intended return semantics and truncation behavior directly in code comments for migration clarity.

### Deliverables

- Rust module file created
- function signature established
- module wiring complete

## Phase 2: Implement Core Bounded Formatting Logic

- Implement the internal write path that formats content into a bounded destination.
- Enforce maximum write length from the provided buffer size.
- Reserve space for the terminating `NUL` byte when capacity is nonzero.
- Return the total formatted length independent of truncation, matching `snprintf`-style behavior.
- Keep implementation centered on standard library formatting support and minimal buffer-copy logic.
- If raw-pointer access is unavoidable due to surrounding interfaces, isolate unsafe operations to:
  - destination slice construction,
  - final byte writes for output and terminator.

### Deliverables

- core `snprintf` logic implemented
- truncation behavior handled
- null-termination rules implemented
- unsafe scope minimized and documented if used

## Phase 3: Add Behavioral Tests

- Add unit tests for:
  - exact-fit output,
  - truncating output,
  - zero-length destination,
  - one-byte destination containing only terminator,
  - correct reported total output length,
  - preservation of valid byte content up to the written limit.
- Compare expected return semantics against the intended C behavior from the original module.
- Keep tests local to this module unless the project already uses centralized test placement.

### Deliverables

- module-focused unit tests
- `cargo test` passing for migrated behavior

## Phase 4: Integration Review and Cleanup

- Verify the migrated module integrates with existing call sites without expanding API surface beyond what the C file required.
- Remove any temporary migration helpers not needed after implementation.
- Confirm naming and file placement follow the existing Rust project structure on branch `051-module_gnu_snprintf.c_45-rust-port`.
- Perform a final review of:
  - buffer bounds,
  - return value consistency,
  - minimal unsafe usage,
  - absence of extra formatting facilities beyond the original module scope.

### Deliverables

- final integrated module
- cleaned implementation
- reviewed memory-safety and error-handling boundaries