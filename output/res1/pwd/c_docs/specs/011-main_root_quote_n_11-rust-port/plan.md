# Implementation Plan

## Summary

Port the `quotearg.c` responsibilities for `quote_n_mem` and `quote_n` into a Rust module that preserves the existing call shape and observable quoting behavior used by the `main_cluster` portion of `pwd`. The Rust implementation should stay narrowly scoped to these two functions and the data they directly depend on, avoiding broader reorganization of the quoting subsystem.

Technical approach:

- Migrate the relevant logic from `quotearg.c` into a single Rust source module under the crate’s normal `src/` layout.
- Represent C byte-oriented string handling with Rust byte slices (`&[u8]`) and owned buffers (`Vec<u8>` / `String`) as appropriate for the original behavior.
- Keep the implementation centered on index-based quote slot selection for `quote_n_mem` and the convenience wrapper behavior for `quote_n`.
- Replace implicit C memory lifetime patterns with explicit Rust ownership and borrowing, ensuring that any returned quoted result has a well-defined owner.
- Convert nullable and global-state-style C patterns into explicit Rust types, while keeping the migration limited to the minimum state required by these two functions.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on the provided input
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Preserve the original byte-processing behavior without unnecessary intermediate allocations
  - Keep per-call overhead close to the C implementation by using slice-based input and preallocated output buffers where practical
  - Avoid copying input more than required to produce the quoted output
  - Maintain deterministic behavior for repeated `quote_n` / `quote_n_mem` calls

## Module Mapping

| C File | C Functions | Rust File | Rust Items |
|---|---|---|---|
| `quotearg.c` | `quote_n_mem` | `src/main_root_quote_n_11.rs` | `pub fn quote_n_mem(...) -> ...` |
| `quotearg.c` | `quote_n` | `src/main_root_quote_n_11.rs` | `pub fn quote_n(...) -> ...` |

Notes:

- Keep both migrated functions in the same Rust file to mirror the original source locality.
- If the crate already has an existing module tree for `main_cluster`, expose this file through the smallest necessary `mod` declaration only.
- Do not split helper logic into extra modules unless required for compilation hygiene inside the same file.

## Data Model

The analysis lists only anonymous C data structures. Since no named public structs are identified for this function subset, the migration should introduce only the minimum Rust-owned internal representations needed to support the two functions.

| C Data Shape | Rust Mapping | Notes |
|---|---|---|
| anonymous byte buffer inputs | `&[u8]` | Direct replacement for pointer + length inputs in `quote_n_mem` |
| anonymous NUL-terminated string input | `&str` or `&CStr`-equivalent handling at module boundary | Prefer `&str` if callers are already Rust-native; otherwise convert from bytes conservatively |
| anonymous mutable output buffer/state | `String` or `Vec<u8>` | Choose `String` only if quoting output is guaranteed valid UTF-8; otherwise use `Vec<u8>` internally |
| anonymous indexed quote slot state | internal `Vec<...>` or fixed internal storage | Mirrors C’s per-`n` storage concept without exposing raw global mutable storage |
| anonymous option/config state referenced by the original implementation | private Rust `struct`/`enum` only if directly required | Introduce only the fields actually needed by `quote_n_mem` / `quote_n` |
| anonymous nullable pointers | `Option<T>` / `Option<&T>` | Replace null checks explicitly |
| anonymous length / index fields | `usize` | Replace C size and index arithmetic with bounds-checked Rust indices |

Rust-side design constraints:

- Prefer private module-local structs over public types unless another migrated module must call them.
- If the original functions rely on reusable per-index quoted results, represent that state with explicit ownership instead of static mutable buffers.
- Any internal enum should model quoting mode only if directly necessary for these two functions; do not generalize beyond the observed dependency surface.

## Implementation Phases

### Phase 1: Establish module skeleton and signatures

- Create `src/main_root_quote_n_11.rs`.
- Add Rust equivalents for `quote_n_mem` and `quote_n` with signatures matching the current crate conventions.
- Identify the exact input and output ownership model required by existing callers:
  - byte-slice input for memory-length form
  - string-like wrapper for plain form
- Add the minimal module export wiring so the rest of the branch can call these functions.
- Define any private placeholder types needed to replace anonymous C state referenced by the two functions.

Exit criteria:

- The crate compiles with stubbed but type-correct `quote_n_mem` and `quote_n`.
- No extra modules or public APIs are introduced beyond what is required for these functions.

### Phase 2: Port core quoting logic and state handling

- Translate the byte-oriented quoting path used by `quote_n_mem` into safe Rust logic.
- Implement the `quote_n` wrapper by forwarding to `quote_n_mem` with the correct length derivation.
- Replace C memory management patterns:
  - remove raw allocation/reallocation logic
  - use owned Rust buffers for generated quoted output
  - model per-`n` result retention with explicit owned storage if the original semantics require stable indexed results
- Replace null and sentinel handling with `Option` and standard Rust branching.
- Keep the implementation faithful to existing behavior, especially around:
  - input length handling
  - repeated calls with the same quote index
  - byte preservation and escaping rules from the original code path

Exit criteria:

- `quote_n_mem` and `quote_n` are fully implemented in Rust.
- No unsafe code is introduced unless strictly required by an existing crate boundary.
- Memory ownership is explicit and compilation succeeds.

### Phase 3: Validate behavioral equivalence with focused tests

- Add unit tests covering:
  - empty input
  - plain ASCII input
  - embedded special bytes through `quote_n_mem`
  - multiple `n` values to verify slot separation behavior
  - `quote_n` length forwarding behavior
- Add regression-style tests for boundary cases seen in the C logic, such as:
  - zero-length buffers
  - repeated invocations for the same index
  - large index values if supported by the original function
- Verify error-free behavior under Rust bounds checks and ownership rules.

Exit criteria:

- `cargo test` passes.
- The migrated functions demonstrate stable behavior for representative inputs and index usage.

### Phase 4: Cleanup and integration finalization

- Remove any leftover placeholders from the initial skeleton.
- Tighten visibility of helper types and functions to private scope.
- Confirm the file/module naming and exports align with the branch target `011-main_root_quote_n_11-rust-port`.
- Perform a final pass to ensure the implementation remains limited to the original module scope and does not introduce unrelated abstractions.

Exit criteria:

- Final Rust module is integrated cleanly into the crate.
- Implementation remains narrowly scoped to the migrated C functions and their direct supporting data only.