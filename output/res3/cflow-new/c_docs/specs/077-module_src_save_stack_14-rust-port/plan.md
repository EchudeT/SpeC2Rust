# Implementation Plan: module_src_save_stack_14

## Summary

This module ports the `save_stack` and `save_stack_is_empty` logic from `src/parser.c` into Rust with a minimal, direct translation approach. The Rust implementation should preserve the existing stack behavior used by the parser, including push/pop or emptiness-state handling implied by the original C code paths, while replacing raw memory handling with owned Rust containers and explicit state transitions.

The implementation approach is to isolate the migrated stack behavior into a Rust module that mirrors the parser-facing role of the original C code, keep the API surface narrow, and model any anonymous C data layouts as named Rust types only where required by these two functions. The port should favor `Vec<T>` or a similarly simple standard-library-backed representation for stack storage, avoid introducing broader parser redesign, and convert implicit C invariants into explicit Rust checks.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve amortized constant-time stack operations.
  - Avoid unnecessary heap reallocations beyond normal `Vec` growth behavior.
  - Maintain parser-compatible behavior without adding abstraction layers that change hot-path costs.
  - Keep data ownership simple so the port does not require clone-heavy flows.

## Module Mapping

| C Source | C Functions | Rust Target | Notes |
|---|---|---|---|
| `src/parser.c` | `save_stack`, `save_stack_is_empty` | `src/parser.rs` or the parser module file already hosting the Rust port of `parser.c` | Keep migration colocated with the parser logic instead of creating unrelated new subsystems. |
| `src/parser.c` | stack-related local/static state used by `save_stack*` | parser-owned Rust struct field(s) | Replace file-scope mutable state or manually managed arrays with owned Rust fields. |

### Proposed Rust Placement

If `parser.c` is being ported into a single Rust module, place this work in:

- `src/parser.rs`

If the parser port already uses an internal module split, place the stack-specific implementation in the existing parser-internal file only if such a file already exists; otherwise keep it in `src/parser.rs` to avoid expanding structure beyond the migration need.

## Data Model

The C analysis exposes only anonymous data structures, so the Rust plan should introduce named types only for structures directly touched by `save_stack` and `save_stack_is_empty`.

| C Representation | Rust Representation | Migration Decision |
|---|---|---|
| Anonymous stack storage struct or manual array/state combination | `struct SaveStack<T>` or parser field `Vec<SaveFrame>` plus state helpers | Use a named Rust type only if it improves direct translation of the two functions; otherwise store as parser fields. |
| Anonymous saved parser entry/frame | `struct SaveFrame` | Define only the fields actually read/written by the migrated functions. |
| Integer emptiness/count flags | `usize` length checks or `bool` where semantically exact | Prefer deriving emptiness from container length rather than duplicating state. |
| Nullable pointers to stack nodes/elements | `Option<T>` / `Option<usize>` / `Vec<T>` | Replace null-based state with explicit option or empty container semantics. |
| Raw allocated memory buffers | `Vec<T>` | Transfer memory management to Rust ownership. |

### Data Ownership and Memory Management

- Replace any C-managed dynamic stack memory with `Vec`.
- Avoid parallel “capacity/count/pointer” bookkeeping unless required for exact parser behavior.
- If the C code stores references into parser-owned data, prefer stable owned values in stack entries where possible; if borrowing is unavoidable, use lifetimes only within already-established parser structures.
- Remove manual free/reset logic in favor of scope-based drop, but preserve explicit clear/reset behavior if the parser expects stack reuse.

### Error Handling Strategy

- If the C functions cannot fail under normal operation and only inspect/update internal state, keep Rust signatures infallible.
- If the C implementation depended on allocation success or invalid-state tolerance, express internal assumptions with:
  - `debug_assert!` for invariants expected from parser control flow
  - `Option`/`bool` return values where emptiness or missing-state is part of normal behavior
- Do not add new public error enums unless the original call pattern clearly requires fallible behavior.

## Implementation Phases

## Phase 1: Recover Stack Shape and Define Rust Data Types

- Inspect the `save_stack` and `save_stack_is_empty` bodies in `src/parser.c` to determine:
  - whether the stack is array-backed, linked, or embedded in parser state
  - the exact entry payload saved on the stack
  - whether `save_stack` mutates, pushes, pops, or snapshots
- Identify all anonymous C structs/unions touched by these functions.
- Introduce named Rust types for only the required stack entry and owner state.
- Decide whether the stack should be:
  - a dedicated `SaveStack` struct, or
  - a small set of fields on the parser struct
- Convert null/zero-initialized C state into explicit Rust initialization.

**Deliverable**:
- Rust type definitions and parser field mapping sufficient to compile placeholder versions of the two functions.

## Phase 2: Port `save_stack_is_empty` and Core State Access

- Port `save_stack_is_empty` first because it establishes the canonical empty-state interpretation.
- Map C emptiness checks to:
  - `vec.is_empty()`, or
  - `option.is_none()`, or
  - explicit count comparison
  depending on the recovered representation.
- Preserve any special semantics where “empty” is not identical to “no allocated storage”.
- Add focused unit tests covering:
  - freshly initialized state
  - state after one saved entry
  - state after clearing or consuming entries, if applicable

**Deliverable**:
- Working Rust emptiness check with tests demonstrating parity of empty/non-empty behavior.

## Phase 3: Port `save_stack`

- Translate the main `save_stack` logic directly from C into Rust.
- Replace manual allocation/growth/linking with `Vec` operations or equivalent owned updates.
- Preserve ordering semantics exactly:
  - push direction
  - overwrite vs append behavior
  - any index/count adjustments
- Make state transitions explicit so hidden C side effects become readable Rust mutations.
- Keep the API restricted to the parser-internal call pattern already present in the C code.

**Deliverable**:
- Rust implementation of `save_stack` passing compile-time checks and matching the C control flow.

## Phase 4: Validation and Cleanup

- Add parity-oriented unit tests for the migrated stack behavior using representative parser-state fixtures derived from the C logic.
- Verify:
  - empty-state behavior before and after save operations
  - multiple-entry behavior if supported by the original implementation
  - reset/clear semantics if the C code reuses the stack
- Remove redundant translated state fields if Rust container length fully replaces them.
- Confirm no unsafe code is required; if any unsafe block is introduced, narrow it and document the exact invariant.

**Deliverable**:
- Finalized Rust module with tests, minimal data model, and no leftover C-style memory management patterns.