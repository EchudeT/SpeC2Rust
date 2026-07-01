# Implementation Plan: main_root_setlocale_null_05

## Summary

This module ports the C locale-query helpers in `setlocale_null.c` and `setlocale_null-unlocked.c` into a Rust implementation that preserves the existing call layering and behavior boundaries. The focus is limited to migrating the current functions:

- `setlocale_null_unlocked`
- `setlocale_null_r_unlocked`
- `setlocale_null_r_with_lock`
- `setlocale_null_r`
- `setlocale_null`

The Rust version should keep the same separation between the unlocked/internal path and the public wrapper path, while adapting C string and buffer handling to Rust’s ownership model. Since the C implementation likely interacts with locale state and nullable results, the Rust design should model these operations with explicit return types and carefully scoped string allocation, avoiding undefined behavior from raw pointers and manual buffer writes.

The implementation approach should:

- consolidate the duplicated C function variants into one Rust module with internal helpers and public entry points,
- preserve the original migration order from lower-level helper to wrapper functions,
- use standard-library string types and `Option`/`Result` to represent nullable and fallible outcomes,
- avoid introducing new abstractions beyond what is needed to map the existing files and functions.

## Technical Context

- **Language/Version**: Rust 1.77+ stable
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the current low-overhead wrapper structure from the C code.
  - Avoid unnecessary string copies beyond what is required to safely return locale names.
  - Keep helper layering shallow and avoid extra allocation-heavy abstractions.
  - Match existing behavior for null/empty/error cases without adding retry or synchronization layers not present in the source scope.

## Module Mapping

### Source File Mapping

| C File | Rust File | Notes |
|---|---|---|
| `setlocale_null-unlocked.c` | `src/main_root_setlocale_null_05.rs` | Migrate unlocked/internal helper logic first. |
| `setlocale_null.c` | `src/main_root_setlocale_null_05.rs` | Migrate public wrapper functions into the same Rust module to keep the port constrained. |

### Function Mapping

| C Function | Rust Function | Visibility | Notes |
|---|---|---|---|
| `setlocale_null_unlocked` | `setlocale_null_unlocked` | `pub(crate)` or private | Internal helper retaining unlocked semantics in naming. |
| `setlocale_null_r_unlocked` | `setlocale_null_r_unlocked` | private | Buffer-oriented/internal variant should be migrated before wrappers. |
| `setlocale_null_r_with_lock` | `setlocale_null_r_with_lock` | private | Preserve as an internal wrapper if distinct behavior exists in the C source. Duplicate entries in the analysis should be treated as one implementation target. |
| `setlocale_null_r` | `setlocale_null_r` | `pub(crate)` or `pub` per crate usage | Safe Rust-facing wrapper around the lower-level helper chain. |
| `setlocale_null` | `setlocale_null` | `pub(crate)` or `pub` per crate usage | Top-level convenience entry point. |

### Rust Module Placement

Use a single module file under standard Rust layout:

- `src/main_root_setlocale_null_05.rs`

If the crate already exposes modules from `src/lib.rs` or `src/main.rs`, add only the necessary `mod`/`pub mod` declaration for this file. Do not split this small port into additional submodules.

## Data Model

No explicit C structs were identified in the analysis input. The migration should therefore remain function-centered and use standard Rust value types for state passed through the call chain.

### Data-Structure Mapping

| C Representation | Rust Representation | Notes |
|---|---|---|
| `char *` locale string result | `String` | Use owned UTF-8 text when the implementation guarantees valid Rust strings. |
| nullable `char *` result | `Option<String>` | Represents absence of locale value without raw null pointers. |
| caller-provided character buffer | `&mut String` or internal temporary `String` | Prefer safe string accumulation instead of fixed raw buffers unless exact byte-buffer semantics are required by surrounding code. |
| status/error integer return conventions | `Result<T, E>` or `Option<T>` | Choose the narrowest type that matches the actual C control flow during migration. |
| locale category integer/constant | integer Rust type matching call sites, likely `i32` | Preserve current category representation until broader crate typing requires refinement. |

### Memory Management and Error Handling

- Replace C pointer lifetime assumptions with owned `String` returns or explicit mutable references.
- Do not expose raw pointers in Rust-facing APIs unless required by surrounding migrated code.
- Convert null-result branches into `Option`.
- Convert buffer-capacity or conversion failures into `Result` where the original function distinguished failure from empty output.
- Keep temporary allocations local to each helper; avoid global mutable storage.

## Implementation Phases

### Phase 1: Establish module skeleton and migrate unlocked helpers

- Create `src/main_root_setlocale_null_05.rs`.
- Port logic from `setlocale_null-unlocked.c` first.
- Implement:
  - `setlocale_null_r_unlocked`
  - `setlocale_null_unlocked`
- Preserve the internal call relationship from the C code rather than redesigning APIs.
- Translate any raw C string handling into safe Rust string construction with explicit null/empty checks.
- Add focused unit tests for:
  - successful locale retrieval path,
  - null or unavailable locale path,
  - empty-string handling if present in the C behavior.

### Phase 2: Migrate locked/wrapper path from the main C file

- Port the remaining logic from `setlocale_null.c`.
- Implement:
  - `setlocale_null_r_with_lock`
  - `setlocale_null_r`
  - `setlocale_null`
- Keep wrapper depth consistent with the original source, even if some functions become thin delegators.
- If the C file contains repeated declarations or macro-expanded duplicates, collapse them into one Rust implementation while preserving behavior.
- Ensure return-type conversions between internal helper output and public wrapper output are explicit and minimal.

### Phase 3: Normalize signatures and integrate with crate exports

- Confirm the narrowest practical Rust signatures for each function based on actual crate usage:
  - `Option<String>` for nullable getters,
  - `Result<String, _>` or `Result<(), _>` for `_r` variants if they encode failure.
- Add only the required module declaration/export in the crate root.
- Keep naming close to the C source for traceability during review.
- Verify no extra module files, compatibility layers, or utility abstractions were introduced.

### Phase 4: Complete behavioral tests and cleanup

- Add `cargo test` coverage for the full function chain:
  - unlocked helper to wrapper equivalence where applicable,
  - propagation of missing locale values,
  - consistent handling of repeated calls,
  - any category-specific dispatch present in the C source.
- Remove dead code introduced during translation.
- Check that ownership and borrowing are straightforward and that no unnecessary clones remain in wrapper layers.
- Finalize documentation comments only where they help map Rust functions back to the original C sources.