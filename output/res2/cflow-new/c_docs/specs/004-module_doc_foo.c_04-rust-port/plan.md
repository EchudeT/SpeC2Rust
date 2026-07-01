# Implementation Plan: module_doc_foo.c_04

## Summary

Port `doc/foo.c` into an idiomatic Rust module with a narrow scope matching the existing C surface: one source file and one function, `f`. The Rust implementation should preserve the current behavior and call shape as closely as practical while replacing C-style memory handling, nullability assumptions, and integer/error conventions with explicit Rust types.

The implementation approach is to create a single Rust module corresponding to `doc/foo.c`, migrate function `f` first, and keep supporting logic local to that module unless the port strictly requires a small private helper. The plan should avoid introducing new abstractions or cross-cutting facilities not evidenced by the source module. Any C patterns involving raw pointers, sentinel values, or manual ownership should be converted to safe standard-library representations where possible, with a limited `unsafe` boundary only if the function signature or semantics require it.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behaviorally equivalent runtime characteristics for `f`.
  - Avoid unnecessary heap allocation unless the C logic inherently requires owned dynamic storage.
  - Prefer borrowing and slice-based processing over copying.
  - Keep control flow and data access close to the original implementation to reduce migration risk.

## Module Mapping

### C to Rust File Mapping

- `doc/foo.c` → `src/module_doc_foo_c_04.rs`

### Function Mapping

- `f` → `pub(crate)` or `pub` Rust function `f` in `src/module_doc_foo_c_04.rs`

Visibility should be set to the minimum required by the rest of the crate. If the function is only used internally, keep it non-public or `pub(crate)` rather than exposing a broader API.

### Migration Notes

- Keep the port constrained to the contents of `doc/foo.c`.
- Do not split logic into additional modules unless required by Rust compilation constraints.
- If `f` depends on file-local C helpers/macros from `doc/foo.c`, migrate them as private functions or `const` items in the same Rust module.

## Data Model

No explicit C data structures were identified in the module analysis.

### Data-Structure Mapping

- **C structs/enums/unions**: None identified
- **Rust equivalents**: None required unless discovered during source-level porting of `f`

### Scalar and Pointer Conversion Guidance

If `f` uses C scalar or pointer-based data internally, apply these mappings during implementation:

- `int`/status codes:
  - Prefer `Result<T, E>` when the C function signals success/failure.
  - Use `bool` only if the C function is logically boolean.
  - Use fixed-width integers (`i32`, `u32`, etc.) only when width matters for correctness.
- `char *` / `const char *`:
  - Use `&str` for validated text input.
  - Use `&[u8]` when data may be non-UTF-8 or byte-oriented.
  - Use `String`/`Vec<u8>` only for owned outputs or mutable buffers.
- Raw pointers:
  - Replace with references/slices where lifetime and nullability can be expressed safely.
  - If raw-pointer interaction is unavoidable, isolate it in a small `unsafe` section with documented preconditions.
- Sentinel/null semantics:
  - Map nullable values to `Option<T>`.
- Output parameters:
  - Replace with return values or tuples when possible, matching the original behavior without adding features.

## Implementation Phases

## Phase 1: Source Analysis and Rust Module Skeleton

- Inspect `doc/foo.c` and determine the exact signature, dependencies, and side effects of `f`.
- Identify any file-local constants, macros, or helper routines directly required by `f`.
- Create `src/module_doc_foo_c_04.rs` with the minimal module skeleton needed for the port.
- Define the Rust signature for `f` based on the C contract:
  - preserve call semantics,
  - convert error/status signaling into Rust types where feasible,
  - keep visibility minimal.

### Deliverables

- Rust module file created.
- `f` signature drafted.
- Required local constants/helpers identified for migration.

## Phase 2: Core Function Port

- Port the body of `f` into Rust with behavior-first fidelity.
- Translate C control flow directly before applying minor idiomatic cleanup.
- Replace manual memory handling with:
  - references/slices for borrowed inputs,
  - owned standard-library containers only where the C logic requires allocation,
  - automatic drop semantics instead of explicit frees.
- Convert null checks, sentinel checks, and status code branches into `Option`/`Result` or explicit conditionals as appropriate.
- Keep any unavoidable `unsafe` code narrowly scoped and documented.

### Deliverables

- Functional Rust implementation of `f`.
- Any required private helpers migrated into the same file.
- Memory ownership and error paths made explicit.

## Phase 3: Validation and Test Coverage

- Add unit tests for `f` derived from observable behavior in `doc/foo.c`.
- Cover:
  - normal-path behavior,
  - boundary conditions,
  - invalid or empty inputs if relevant to the C implementation,
  - error/status outcomes if the original function distinguishes them.
- Run `cargo test` and resolve behavioral mismatches against the C source expectations.

### Deliverables

- Unit tests for `f`.
- Verified compilation and passing `cargo test`.

## Phase 4: Cleanup and Parity Review

- Review the Rust port against `doc/foo.c` for missed edge cases or semantic drift.
- Remove unnecessary temporary compatibility code introduced during translation.
- Confirm the final module remains narrowly scoped:
  - one Rust module,
  - no unevidenced infrastructure,
  - no API expansion beyond the migrated function and required local items.

### Deliverables

- Final parity pass completed.
- Module ready on branch `004-module_doc_foo.c_04-rust-port`.