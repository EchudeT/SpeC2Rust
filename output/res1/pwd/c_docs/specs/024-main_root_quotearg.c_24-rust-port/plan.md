# Implementation Plan

## Summary

Port the `quotearg.c` functionality into a single Rust module that preserves the existing quoting behavior and call patterns used by the `pwd` project. The scope is limited to migrating the functions listed for `main_root_quotearg.c_24` and the internal state they require, without adding new quoting features or broader API layers.

The Rust implementation should mirror the C file closely:
- keep the logic concentrated in one Rust source module,
- translate buffer-oriented formatting into safe byte-slice and `String`/`Vec<u8>` based routines,
- replace C global/manual allocation patterns with owned Rust storage and explicit helper state where needed,
- preserve externally visible semantics of the quoted output as used by callers in this project.

The preferred approach is:
1. implement the core quoting routine first (`quotearg_buffer_restyled` equivalent),
2. layer the convenience wrappers (`quotearg`, `quotearg_mem`, `quotearg_char`, `quote_mem`, `quote`) on top of it,
3. migrate cleanup behavior (`quotearg_free`) using Rust ownership so that explicit freeing becomes minimal or a no-op unless compatibility storage is required.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s asymptotic behavior for quoting over input length.
  - Avoid unnecessary intermediate allocations where the C code writes into caller-provided buffers.
  - Use byte-oriented processing for non-UTF-8-safe paths and argument data.
  - Keep wrapper APIs thin over the core formatting routine.

## Module Mapping

| C File | Rust File | Notes |
|---|---|---|
| `quotearg.c` | `src/main_root_quotearg.rs` | Single-file migration of the module logic and its internal helper types/constants. |

| C Function | Rust Mapping | Notes |
|---|---|---|
| `gettext_quote` | `fn gettext_quote(...) -> ...` | Keep internal helper shape close to original role; use borrowed string/byte data as appropriate. |
| `quotearg_buffer_restyled` | `fn quotearg_buffer_restyled(...) -> usize` | Core implementation; return produced/required length in Rust form. |
| `quotearg_free` | `fn quotearg_free()` | Compatibility function; likely clears module-local reusable storage if such storage is retained, otherwise no-op. |
| `quotearg` | `fn quotearg(...) -> String` or module-equivalent wrapper | Wraps the core routine for full-input quoting. |
| `quotearg_mem` | `fn quotearg_mem(input: &[u8], ...) -> String/Vec<u8>` | Preserve byte-oriented behavior for non-UTF-8 input. |
| `quotearg_char` | `fn quotearg_char(...) -> String` | Thin wrapper adjusting quoting options for one character. |
| `quote_mem` | `fn quote_mem(input: &[u8]) -> String/Vec<u8>` | Convenience wrapper built on `quotearg_mem`. |
| `quote` | `fn quote(...) -> String` | Convenience wrapper built on `quotearg`. |

## Data Model

The analysis reports only anonymous C data structures, so the Rust plan should map them conservatively based on actual usage encountered in `quotearg.c`, not by inventing broader abstractions.

### Structure Mapping Strategy

| C Data Shape | Rust Mapping | Notes |
|---|---|---|
| Anonymous option/config structs | Named `struct` types scoped to `main_root_quotearg` | Introduce names only as needed to represent quoting options and flags used in the file. |
| Anonymous enums / mode constants | `enum` or `const` set | Use `enum` when the C code expresses exclusive modes; use constants/bitflags-like fields only if the source clearly relies on bitwise composition. |
| Anonymous static storage for cached quoted results | `static`/module-local storage only if required by original API behavior; otherwise owned return values | Prefer eliminating manual lifetime management where callers do not depend on shared static buffers. |
| Raw C character buffers | `Vec<u8>`, `&[u8]`, `String` | Choose `Vec<u8>`/`&[u8]` for core quoting logic; convert to `String` only when valid and expected by call sites. |
| Pointer + length pairs | `&[u8]` / `&str` | Preserve explicit length semantics for `*_mem` functions. |

### Expected Rust Types

The final names should follow the actual source logic, but the module will likely need only a small set of internal types:
- a quoting options struct,
- possibly a quoting style enum,
- optional reusable slot storage if the original wrappers depend on rotating/static buffers.

### Memory Management

- Replace `malloc`/`free` style buffer ownership with `Vec<u8>` and `String`.
- Avoid storing raw pointers.
- If the C module rotates through static buffers for returned strings, model that only if required by existing callers; otherwise prefer direct owned returns.
- `quotearg_free` should only release explicitly retained module-local storage. If no retained storage remains after migration, keep it as an empty compatibility function.

### Error Handling

- The core logic should avoid panics on arbitrary byte input.
- Functions that operate on raw memory should use byte slices and not assume UTF-8.
- If a public wrapper must return text, perform lossless escaping during quoting so the result remains representable; do not use unchecked UTF-8 conversion.
- Preserve infallible behavior where the C code expects formatting rather than recoverable errors.

## Implementation Phases

## Phase 1: Port the core quoting engine

### Goal
Create the Rust module and migrate the internal quoting logic from `quotearg.c` with minimal structural change.

### Tasks
- Add `src/main_root_quotearg.rs`.
- Identify the internal constants, flags, and anonymous structs used by `quotearg_buffer_restyled`.
- Introduce minimal named Rust equivalents for those internal data shapes.
- Implement `gettext_quote` in Rust using borrowed data.
- Implement `quotearg_buffer_restyled` over byte slices and mutable output buffers / owned vectors as needed.
- Keep the processing byte-oriented to preserve behavior for non-UTF-8 inputs.

### Completion Criteria
- The module compiles with the core helper and main buffer-restyling routine in place.
- The implementation does not rely on unsafe code unless a narrow source-specific need is found during migration.
- The core routine can format representative inputs into quoted output.

## Phase 2: Add wrapper functions and compatibility behavior

### Goal
Port the public wrapper functions directly on top of the core routine.

### Tasks
- Implement `quotearg`.
- Implement `quotearg_mem`.
- Implement `quotearg_char`.
- Implement `quote_mem`.
- Implement `quote`.
- Implement `quotearg_free` according to the final storage model:
  - clear retained reusable storage if present,
  - otherwise leave as a no-op compatibility function.
- Keep wrapper signatures and behavior aligned with existing project usage rather than introducing generalized APIs.

### Completion Criteria
- All listed functions exist in Rust.
- Wrapper functions delegate to the core quoting routine rather than duplicating logic.
- Any former C-managed retained buffers are either safely modeled or intentionally removed where unnecessary.

## Phase 3: Integrate with callers and normalize type boundaries

### Goal
Connect the migrated module to the rest of the `pwd` Rust branch without expanding scope.

### Tasks
- Replace references to the C-side quoting helpers with imports from `main_root_quotearg`.
- Normalize caller inputs to `&str` or `&[u8]` depending on each wrapper’s original semantics.
- Adjust return handling so callers consume owned Rust results safely.
- Ensure there is no remaining dependence on C allocation or pointer lifetime conventions.

### Completion Criteria
- The `pwd` branch builds with the Rust quoting module wired into its existing call sites.
- No caller depends on raw pointer results from the former C implementation.
- Memory ownership is explicit and contained within Rust types.

## Phase 4: Validate behavior with focused tests

### Goal
Lock in parity for the migrated functions using module-level tests.

### Tasks
- Add `cargo test` coverage for:
  - empty input,
  - plain ASCII input,
  - embedded special characters requiring escaping/quoting,
  - explicit-length input for `quotearg_mem`/`quote_mem`,
  - single-character quoting adjustments for `quotearg_char`,
  - repeated calls to wrappers to validate any retained-buffer replacement behavior,
  - `quotearg_free` compatibility behavior if storage is retained.
- Compare output behavior against known expectations derived from the original module logic and existing project usage.

### Completion Criteria
- Tests cover the listed Rust functions directly.
- Repeated wrapper use shows stable ownership and no stale-buffer issues.
- `cargo test` passes on the branch.