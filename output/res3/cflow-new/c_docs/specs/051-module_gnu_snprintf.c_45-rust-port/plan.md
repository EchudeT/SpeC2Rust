# Implementation Plan

## Summary

Port `gnu/snprintf.c` into an idiomatic Rust formatting module that preserves the observable behavior expected from the existing `snprintf` implementation while limiting scope to the current module boundary. The Rust work should focus on reproducing bounded string formatting semantics, especially output truncation to the destination capacity and return-value behavior, using standard-library formatting machinery where possible.

The implementation should migrate the single C entry point into a single Rust module with a narrow public API. Internally, formatting should be handled through `core::fmt`/`std::fmt` style writing into a bounded buffer abstraction so that memory safety is guaranteed without manual pointer arithmetic. Error handling should remain explicit and minimal, with behavior shaped around the original C function contract rather than introducing broader abstractions.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library only
  - No third-party crates recommended from the available evidence
- **Testing**:
  - `cargo test`
  - Unit tests covering truncation, zero-length buffers, exact-fit cases, and return-length semantics
- **Performance Goals**:
  - Keep formatting overhead close to direct standard-library formatting
  - Avoid unnecessary intermediate allocations where bounded writes can be done directly
  - Preserve linear behavior with respect to produced output length
  - Do not introduce heap allocation unless required by the chosen formatting path for compatibility

## Module Mapping

| C Source | Rust Target | Notes |
|---|---|---|
| `gnu/snprintf.c` | `src/module_gnu_snprintf.rs` | Single-module migration for the existing implementation |
| `snprintf` | `pub(crate)` or `pub` function in `src/module_gnu_snprintf.rs` | Visibility should match actual crate integration needs; do not widen API beyond current usage |

Suggested crate integration:

| Rust File | Purpose |
|---|---|
| `src/module_gnu_snprintf.rs` | Port of the C module logic |
| `src/lib.rs` or existing module registry | Re-export or declare the migrated module only as needed for current project structure |

## Data Model

This module analysis reports no custom C data structures.

C-to-Rust data mapping is therefore limited to function-level argument and buffer semantics:

| C Concept | Rust Mapping | Notes |
|---|---|---|
| `char *` destination buffer | `&mut [u8]` or dedicated bounded output buffer type | Prefer slice-based safe access over raw pointers |
| buffer size parameter | `usize` | Natural Rust size type |
| NUL termination expectation | Explicit write of trailing `0` when capacity permits | Must be handled deliberately |
| formatted output length return | `usize` or `Result<usize, _>` depending on crate conventions | If exact C compatibility is required internally, preserve “required length” semantics even on truncation |
| variadic formatting inputs | Rust formatting arguments (`std::fmt::Arguments`) | Use a Rust-facing equivalent rather than reproducing C varargs |

If an internal helper is needed, keep it local to the module:

| Helper | Rust Form | Purpose |
|---|---|---|
| bounded writer | private `struct` implementing `fmt::Write` or byte-oriented helper | Tracks written length, truncation, and terminator placement |

## Implementation Phases

### Phase 1: Module Skeleton and API Mapping

- Create `src/module_gnu_snprintf.rs`.
- Define the Rust function signature that best matches current crate call sites while preserving bounded-formatting behavior.
- Register the module in `src/lib.rs` or the existing module tree without introducing new package structure.
- Identify the expected return semantics from the original C function and encode them in the Rust signature and documentation comments.
- Keep the initial implementation minimal, compiling with placeholder logic if needed before full behavior is added.

### Phase 2: Core Formatting Port

- Implement a private bounded-buffer writer that:
  - writes only within the provided destination capacity,
  - tracks the total number of bytes that would have been produced,
  - ensures trailing NUL insertion when buffer length is nonzero.
- Build the `snprintf` port on top of Rust formatting primitives rather than manual unsafe byte manipulation where possible.
- Handle edge cases directly in the writer:
  - zero-capacity destination,
  - one-byte destination,
  - exact-fit output,
  - truncated output.
- Restrict `unsafe` usage to cases where integration absolutely requires raw buffer interoperability; otherwise remain fully safe.

### Phase 3: Behavioral Validation

- Add focused unit tests in the module or crate test area for:
  - empty format result,
  - output shorter than buffer,
  - output exactly filling available content space,
  - truncation with correct terminator handling,
  - zero-sized destination behavior,
  - reported required length independent of truncation.
- Verify byte-level output and return values together, not separately.
- Confirm that invalid states are represented through explicit errors only if the surrounding crate contract requires them; otherwise preserve direct return-style behavior.

### Phase 4: Integration Cleanup

- Replace any temporary scaffolding with final implementation details.
- Align naming, visibility, and documentation with the project’s existing Rust module conventions.
- Remove any unnecessary allocation-based fallback if direct bounded writing is sufficient.
- Run `cargo test` and resolve integration mismatches without expanding functionality beyond the original C module scope.