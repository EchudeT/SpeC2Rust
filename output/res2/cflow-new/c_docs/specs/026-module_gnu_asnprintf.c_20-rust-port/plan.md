# Implementation Plan

## Summary

Port `gnu/asnprintf.c` into an idiomatic Rust module that preserves the existing module scope: a single `asnprintf`-equivalent formatting routine centered on dynamically allocated output with explicit size handling. The Rust implementation should rely primarily on `String`, `Vec<u8>`, and standard formatting facilities to replace C buffer growth and manual memory management, while keeping behavior aligned with the original module’s allocation-oriented formatting role.

The implementation should migrate only the functionality represented by `asnprintf`, without introducing broader formatting abstractions or unrelated utility layers. The main technical focus is safe buffer construction, clear ownership of allocated output, and faithful handling of capacity/length outcomes and formatting errors.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C module’s practical purpose of efficient dynamic string construction for formatted output.
  - Avoid unnecessary intermediate allocations where standard formatting APIs allow direct writing into a growable buffer.
  - Keep allocation growth predictable through `String`/`Vec<u8>` capacity management rather than manual realloc-style logic.
  - Preserve linear behavior relative to produced output size.

## Module Mapping

- **C source file**
  - `gnu/asnprintf.c`

- **Rust target**
  - `src/module_gnu_asnprintf.rs` or the project’s existing equivalent module path for direct one-file migration

- **Function mapping**
  - `asnprintf` -> `asnprintf` Rust function with a Rust-native signature reflecting:
    - owned output allocation
    - produced length tracking
    - fallible result handling

If the surrounding project already exposes C-like naming or signatures, keep the Rust entry point narrowly aligned with that existing convention rather than introducing a new API family.

## Data Model

This module has no named C data structures to map.

### Data Representation Mapping

- **C dynamic character buffer (`char *` with allocated storage)**
  -> `String` when output is guaranteed text
  -> `Vec<u8>` only if exact byte-oriented handling is required by the existing project interface

- **C size tracking (`size_t`)**
  -> `usize`

- **C success/failure through pointer/null/error-style outcomes**
  -> `Result<..., std::fmt::Error>` or `Result<..., ModuleError>` only if the project already has a local error type
  -> use standard library error propagation by default

- **C mutable output parameters**
  -> returned owned value and explicit length in the Rust return type, avoiding out-parameters unless the existing Rust project patterns require them

## Implementation Phases

### Phase 1: Inspect and define the direct Rust surface

- Review `gnu/asnprintf.c` and identify the exact observable contract of `asnprintf`:
  - what inputs it receives
  - whether it accepts an existing buffer and size limit
  - how it reports produced length
  - what allocation and truncation semantics it follows
  - what constitutes failure
- Define the minimal Rust function signature needed to preserve that contract in Rust-native form.
- Choose the output representation:
  - prefer `String` if the function is text-formatting only
  - use `Vec<u8>` only if the original behavior is byte-buffer oriented in a way that must be preserved
- Document any unavoidable semantic adaptation from C varargs/manual formatting behavior to Rust formatting traits, keeping the adaptation local to this function only.

### Phase 2: Port buffer management and formatting logic

- Implement the `asnprintf` logic in the mapped Rust module.
- Replace manual allocation/reallocation with:
  - `String::with_capacity` or `Vec::with_capacity` for initial storage
  - standard growth behavior for larger formatted output
- Replace C write/append behavior with Rust formatting/writing APIs from `std::fmt`.
- Ensure length accounting mirrors the original function’s produced-output behavior.
- Handle failure paths explicitly:
  - allocation-related failure remains process-level in standard Rust unless the project already requires fallible allocation handling
  - formatting failure should be surfaced through `Result`
- Keep the implementation contained to the migrated file; do not add helper subsystems unless a small local helper function is necessary to mirror the original logic.

### Phase 3: Reconcile ownership, edge cases, and signature compatibility

- Verify that the Rust return shape fully replaces any C ownership-transfer semantics.
- Confirm behavior for:
  - empty formatted output
  - small and large outputs
  - exact-capacity boundaries
  - invalid or failed formatting paths, if applicable in the selected Rust interface
- If the surrounding codebase expects a more C-shaped interface, add only the thinnest compatibility layer in the same module, while keeping the core logic in a safe internal function.
- Remove any residual assumptions from the C implementation that are no longer valid under Rust ownership and borrowing rules.

### Phase 4: Add focused tests and finalize migration

- Add unit tests covering:
  - successful formatted output generation
  - correct reported length
  - growth beyond small initial capacity assumptions
  - empty result behavior
  - any preserved boundary semantics from the C implementation
- Run `cargo test` and fix any API or behavior mismatches found during integration.
- Perform a final pass to ensure the module remains a direct migration of `gnu/asnprintf.c` only, with no added formatting framework or unrelated utilities.