# Implementation Plan: module_gnu_if_11

## Summary

Port the relevant conditional control-flow portions of `gnu/vasnprintf.c` into a Rust module within the `cflow-new` project, preserving existing behavior and migration scope without introducing new capabilities. The source analysis identifies conditional branches only, so the implementation plan focuses on translating the associated formatting-path logic already present in the C file into idiomatic Rust using standard-library string and buffer handling.

The Rust implementation should stay tightly aligned with the original file’s responsibilities: internal formatting/buffer construction support, conditional output path selection, and bounded memory growth. The technical approach is to migrate logic function-by-function into a single Rust module that mirrors the original file boundary, replacing raw pointer arithmetic and manual allocation with `Vec<u8>`, `String`, slices, and explicit `Result`-based error propagation where failure conditions exist in the C logic.

## Technical Context

- **Language/Version**: Rust 1.76+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the current asymptotic behavior of buffer-building paths from `gnu/vasnprintf.c`
  - Avoid unnecessary intermediate allocations during formatted buffer assembly
  - Keep copying bounded to the same major transition points as the C implementation
  - Maintain predictable memory growth using `Vec` capacity management rather than repeated realloc-like behavior where avoidable

## Module Mapping

### C to Rust File Mapping

- `gnu/vasnprintf.c` → `src/module_gnu_if_11.rs`

### Rust Module Scope

- Create one Rust module corresponding directly to the analyzed C file segment
- Keep all migrated logic in this file unless the existing Rust crate layout requires a `mod` declaration entry in `src/lib.rs`
- Do not split helpers into extra files unless required by compilation structure

### Function Mapping

Because the analysis identifies conditional branches but not stable exported function names beyond duplicated `if` markers, migration should proceed by locating the enclosing C routines in `gnu/vasnprintf.c` and porting only the logic that belongs to this module slice.

Planned mapping approach:

- C conditional branch block #1 in `gnu/vasnprintf.c` → Rust internal function or inlined branch within the corresponding migrated formatter routine
- C conditional branch block #2 in `gnu/vasnprintf.c` → Rust internal function or inlined branch within the corresponding migrated formatter routine

### Integration Mapping

- If the project already has an equivalent formatting module, wire `module_gnu_if_11` into the existing call path with the narrowest possible change
- If not yet present, expose only the minimum crate-visible function(s) needed by adjacent migrated code

## Data Model

The analysis reports only an `anonymous` data structure. For planning purposes, treat this as an unnamed C aggregate local to `gnu/vasnprintf.c`.

### C to Rust Type Mapping

- Anonymous C struct/aggregate → private Rust `struct` with a file-local name based on its role in the surrounding function
- Raw character buffer fields (`char *`, `unsigned char *`) → `Vec<u8>` or `&[u8]` / `&mut [u8]` depending on ownership
- Length/capacity fields (`size_t`) → `usize`
- Signed status/result fields (`int`) → `i32` or preferably a dedicated Rust enum / `Result` where behavior is clearer and does not widen scope
- Optional or nullable pointers → `Option<&T>`, `Option<&mut T>`, or `Option<NonNull<T>>` only if direct pointer semantics are unavoidable

### Memory Management Decisions

- Replace manual allocation/reallocation patterns with `Vec<u8>` growth and checked capacity handling
- Convert pointer-range operations to slice indexing with explicit bounds checks
- Use `String` only when UTF-8 validity is guaranteed by the corresponding C path; otherwise keep byte-oriented storage as `Vec<u8>`
- Preserve any explicit truncation or terminator behavior from the C code as data-level logic rather than relying on C-style sentinel memory layout

### Error Handling Decisions

- Convert allocation failure and invalid-state branches into `Result<T, ModuleError>`
- Keep the error enum private to the module unless cross-module use is required by existing crate structure
- Preserve branch-specific failure outcomes rather than collapsing distinct conditions into generic panics
- Do not use `unwrap`/`expect` in migrated runtime paths

## Implementation Phases

## Phase 1: Source Extraction and Rust Module Skeleton

- Inspect `gnu/vasnprintf.c` and identify the exact enclosing functions for the two analyzed conditional branches
- Determine whether the relevant logic belongs to one formatter routine or multiple adjacent helpers
- Create `src/module_gnu_if_11.rs`
- Define the minimum private type aliases, structs, and error enum needed to represent the C local state
- Add the module to the crate with the smallest required integration change
- Establish placeholder tests for compilation and module wiring

## Phase 2: Port Core Conditional Logic and Buffer State Handling

- Translate the first identified conditional block into Rust, preserving branch order and side effects
- Translate the second identified conditional block into Rust in the same style
- Port surrounding local state manipulations necessary for those branches to compile correctly
- Replace pointer arithmetic with slice/offset logic and checked indexing
- Replace realloc-style growth with `Vec<u8>` capacity expansion while preserving observable behavior
- Encode C failure returns into `Result` values and propagate them through the migrated routine

## Phase 3: Complete Function-Level Migration Around the Branches

- Port the minimal enclosing function logic needed so the migrated conditional branches execute in their real context
- Ensure all temporary buffers, counters, and state flags from the C implementation are represented directly in Rust
- Reconcile differences between null-terminated C buffer handling and Rust length-tracked buffers
- Keep helper logic private and colocated in `src/module_gnu_if_11.rs`
- Remove any placeholder stubs introduced during Phase 1

## Phase 4: Validation and Behavioral Tests

- Add `cargo test` coverage for the migrated paths, focusing on:
  - branch selection behavior
  - buffer growth behavior
  - boundary conditions around empty and small buffers
  - failure/invalid-state propagation where represented in the original C logic
- Add regression tests for outputs produced by the migrated enclosing function(s), based on examples derived from the C behavior
- Verify no panics occur on normal error paths
- Perform final pass for ownership, borrowing, and allocation behavior consistency with the source intent