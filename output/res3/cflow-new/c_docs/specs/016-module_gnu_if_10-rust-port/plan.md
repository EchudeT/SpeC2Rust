# Implementation Plan

## Summary

Port the logic currently contained in `gnu/vasnprintf.c` into a Rust module that preserves the existing formatting behavior and control flow relevant to the identified conditional paths. The Rust implementation should stay narrowly aligned with the existing C file rather than introducing broader formatting infrastructure.

The technical approach is to translate the file’s formatting and buffer-management logic into safe Rust first, using standard library types such as `String`, `Vec<u8>`, and slices to replace raw pointer arithmetic and manually managed buffers. Any C paths that depend on conditional formatting decisions should be represented directly with Rust `if` branches and helper functions extracted only where needed to keep the migrated code readable and testable.

Memory ownership should move from caller-managed buffers and realloc-style growth to Rust-owned buffers with explicit capacity management. Error handling should replace implicit failure signaling and null/error return patterns with `Result` and narrowly scoped internal error enums where needed.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain behavior comparable to the C implementation for dynamic string construction.
  - Avoid unnecessary intermediate allocations during formatted output assembly.
  - Preserve linear buffer growth characteristics where the C code expands output storage.
  - Keep branch structure close to the source to reduce migration risk.

## Module Mapping

| C Source File | Rust Target | Notes |
|---|---|---|
| `gnu/vasnprintf.c` | `src/module_gnu_if_10.rs` | Direct port of the file’s formatting and conditional buffer-writing logic. |
| conditional logic in `if` sites | internal Rust `if` branches/functions | Keep branch behavior local to the translated formatting flow; only extract helpers when they correspond to reusable blocks already present in the C flow. |

## Data Model

The analysis identifies only an anonymous data structure. The Rust port should map data by role rather than preserving anonymous C layout literally.

| C Construct | Rust Mapping | Notes |
|---|---|---|
| anonymous struct used for transient formatting/buffer state | private `struct` in `src/module_gnu_if_10.rs` | Name according to actual role discovered during port, such as output state or formatting state. |
| raw character buffer | `Vec<u8>` or `String` | Use `Vec<u8>` if the logic works at byte level; convert to `String` only when UTF-8 invariants are known. |
| pointer + length/capacity fields | slice/indexes plus `usize` length/capacity | Replace pointer arithmetic with bounds-checked indexing and explicit capacity tracking only when required by the original logic. |
| integer status/error codes | `Result<T, ModuleGnuIf10Error>` | Use a private error enum for allocation/formatting failures that are explicit in the translated code. |

### Memory Management Decisions

- Replace manual allocation and reallocation with `Vec`/`String` growth.
- Avoid exposing borrowed views that outlive internal buffer mutation.
- Keep ownership of temporary formatting buffers local to the main formatting routine.
- Where the C code writes into caller-provided memory, model the target as `&mut Vec<u8>` or return an owned buffer instead, depending on the original call pattern in the file.

### Error Handling Decisions

- Replace null returns and sentinel failure codes with `Result`.
- Preserve fallible allocation and conversion points as explicit error propagation with `?`.
- Keep the error surface minimal and private unless another existing Rust module must consume it.

## Implementation Phases

### Phase 1: File Skeleton and State Mapping

- Create `src/module_gnu_if_10.rs`.
- Identify the top-level entry points represented in `gnu/vasnprintf.c` and define Rust equivalents with signatures matching the surrounding Rust project conventions.
- Inventory anonymous state used by the C file and introduce a minimal private Rust struct for that state.
- Translate constants, local flags, counters, and capacity fields to Rust primitive types (`usize`, `bool`, integer types matching source width where relevant).

**Exit criteria**:
- Rust file exists with compiling type definitions and function skeletons.
- All anonymous/shared state from the C file has a clear Rust owner.

### Phase 2: Core Control-Flow Port

- Port the main formatting/buffer-building routine from `gnu/vasnprintf.c` into Rust.
- Translate each identified `if` path directly, preserving ordering and side effects.
- Replace pointer arithmetic with index-based writes or buffer extension operations.
- Keep helper extraction restrained: only split out chunks that correspond to repeated or naturally isolated logic from the C source.

**Exit criteria**:
- Main module logic compiles.
- Conditional branches from the C implementation are represented explicitly in Rust.
- No unsafe code is introduced unless a specific source construct proves impossible to express safely; if unavoidable, confine it to the smallest possible block.

### Phase 3: Buffer Growth and Error Semantics

- Implement buffer expansion behavior using `Vec::reserve`, `push`, `extend_from_slice`, or `String` operations as appropriate.
- Map C failure paths to `Result` returns and define a small private error enum.
- Verify that length/capacity updates and truncation/termination behavior from the C logic are preserved where relevant.
- Ensure byte-oriented vs text-oriented handling is consistent with the original file’s semantics.

**Exit criteria**:
- Allocation and write paths are fully migrated.
- Failure cases compile into explicit Rust error paths.
- Buffer ownership and lifetimes are clear and safe.

### Phase 4: Tests and Behavioral Validation

- Add unit tests in the same module or under `tests/` only for the translated behaviors visible from this file.
- Cover:
  - primary formatting/output path,
  - both identified conditional branches,
  - buffer growth behavior,
  - error propagation for fallible paths that can be exercised deterministically.
- Run `cargo test` and resolve mismatches caused by C-to-Rust integer, indexing, or string/byte semantics.

**Exit criteria**:
- `cargo test` passes.
- Tests exercise the migrated conditional logic and output assembly behavior.
- The Rust module remains narrowly scoped to the contents of `gnu/vasnprintf.c`.