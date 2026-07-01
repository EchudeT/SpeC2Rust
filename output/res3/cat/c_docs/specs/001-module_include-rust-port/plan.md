# Implementation Plan: module_include

## Summary

This module ports the C file `include/safe-read.c` into a focused Rust module that preserves the existing behavior of `safe_rw` without adding new capabilities. The Rust implementation should keep the same operational role: performing read/write style I/O safely in the presence of interrupted system calls and partial transfers, while surfacing failures through Rust `Result` values instead of C-style integer/error-code handling.

The technical approach is to migrate the logic into a small Rust module that relies primarily on `std::io` traits and standard error types. The implementation should mirror the original retry and loop structure closely so that behavior remains aligned with the C source. Memory ownership becomes implicit through Rust borrowing, and error handling is expressed with `io::Result` and explicit handling for `ErrorKind::Interrupted`.

## Technical Context

- **Language/Version**: Rust stable, edition 2021, minimum supported toolchain: Rust 1.74+
- **Primary Dependencies**:
  - Rust standard library only
  - No third-party crates are recommended from the provided evidence
- **Testing**:
  - `cargo test`
  - Unit tests should cover interrupted-operation retry behavior, partial progress handling, EOF/zero-length cases as applicable to the migrated logic, and propagated error cases
- **Performance Goals**:
  - Match the C implementation’s operational profile closely
  - Avoid unnecessary allocation and copying
  - Preserve iterative I/O behavior over caller-provided buffers
  - Keep syscall/I/O call count equivalent to the original loop structure wherever practical

## Module Mapping

| C Source File | Rust Target | Notes |
|---|---|---|
| `include/safe-read.c` | `src/module_include.rs` | Port the `safe_rw` logic directly into a single Rust module |
| `safe_rw` | `module_include::safe_rw` | Preserve control flow and return semantics as closely as Rust I/O types allow |

## Data Model

No explicit C structs or custom data containers are identified for this module.

| C Construct | Rust Mapping | Notes |
|---|---|---|
| raw buffer pointer + length parameters | `&mut [u8]` or `&[u8]` as appropriate | Use slice types to encode bounds and borrowing safely |
| signed/size return values for byte counts | `io::Result<usize>` | Use `usize` for successful transfer counts |
| `errno`-style failure signaling | `std::io::Error` | Map interrupt handling explicitly with `ErrorKind::Interrupted` |

## Implementation Phases

### Phase 1: Inspect and Define Rust Surface

- Review `include/safe-read.c` and isolate the exact signature and control flow of `safe_rw`.
- Determine whether the original helper is logically read-oriented, write-oriented, or parameterized over both directions, and choose the closest Rust function signature without inventing extra abstractions.
- Create `src/module_include.rs`.
- Define the Rust function with:
  - standard-library I/O types,
  - slice-based buffer parameters,
  - `std::io::Result<usize>` return type.

**Exit criteria**:
- A compilable module skeleton exists.
- The Rust signature is fixed based on the original function’s actual usage pattern.

### Phase 2: Port Core Logic

- Translate the C loop structure into Rust with minimal behavioral drift.
- Preserve handling for:
  - interrupted operations,
  - partial transfers,
  - zero-byte completion behavior,
  - immediate hard-error propagation.
- Keep the implementation allocation-free and based on caller-owned buffers.
- Use explicit match handling on `io::ErrorKind` rather than introducing generic helper frameworks.

**Exit criteria**:
- `safe_rw` is fully implemented in Rust.
- The code compiles and mirrors the original operational behavior closely.

### Phase 3: Add Focused Tests

- Add unit tests alongside the module or in the crate test layout using standard Rust test conventions.
- Use small test doubles built from standard-library traits to simulate:
  - interruption before eventual success,
  - partial read/write progress across multiple calls,
  - non-interrupted error propagation,
  - zero-byte/EOF style termination where relevant to the original logic.
- Ensure tests validate both returned byte counts and buffer/state progression.

**Exit criteria**:
- `cargo test` passes.
- Tests cover the branch behavior that corresponds to the original C function.

### Phase 4: Final Alignment and Cleanup

- Compare the Rust implementation against `include/safe-read.c` one final time for behavioral parity.
- Confirm no unnecessary modules, wrappers, or dependencies were introduced.
- Verify documentation comments are brief and technical, limited to behavior and error semantics needed for maintainability.
- Ensure naming and file placement remain narrowly scoped to this migrated module.

**Exit criteria**:
- Final Rust code is minimal, idiomatic, and traceable to the original C implementation.
- The port is ready on branch `001-module_include-rust-port`.