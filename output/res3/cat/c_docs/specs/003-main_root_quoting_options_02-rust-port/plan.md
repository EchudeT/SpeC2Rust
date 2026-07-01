# Implementation Plan: main_root_quoting_options_02

## Summary

This module ports the `quotearg_n_custom_mem` functionality from `quotearg.c` into Rust for the `cat` project branch `003-main_root_quoting_options_02-rust-port`.

The Rust implementation should preserve the existing behavior and call shape as closely as practical while replacing C-style memory handling with owned Rust buffers and explicit result handling. The work should stay narrowly focused on migrating the logic needed for `quotearg_n_custom_mem` and the immediate internal state it depends on, without introducing broader quoting abstractions beyond what is required to match the current module boundary.

The technical approach is:

- translate the relevant quoting logic from `quotearg.c` into a Rust module under standard crate layout
- represent C configuration/state structures as Rust structs/enums with explicit ownership
- replace raw output buffers and implicit allocation with `Vec<u8>` and/or `String` where UTF-8 is guaranteed; prefer `Vec<u8>` for byte-exact behavior
- model failure points explicitly using `Result` only where allocation or internal validation can fail; keep externally visible behavior aligned with existing calling expectations
- keep the implementation local to the migrated module and avoid adding generalized infrastructure not evidenced by this module analysis

## Technical Context

### Language/Version

- Rust stable, edition 2021
- Minimum recommended toolchain: Rust 1.74 or newer

### Primary Dependencies

- Rust standard library only
- No third-party crates are recommended based on the provided module scope

### Testing

- `cargo test`

Testing should include:

- direct unit tests for `quotearg_n_custom_mem`
- byte-preservation tests for non-UTF-8 input cases if the function operates on arbitrary memory
- regression-style tests covering custom quoting delimiters/behavior derived from the C implementation
- edge-case tests for empty input, embedded special bytes, and repeated calls with different slot indices if slot-based behavior is preserved

### Performance Goals

- preserve linear-time processing relative to input size
- avoid unnecessary intermediate allocations beyond the output buffer needed for quoted content
- maintain behavior suitable for CLI utility use, with performance comparable to the C implementation for typical argument sizes
- keep per-call state handling lightweight; if indexed storage from the C implementation is retained, ensure amortized reuse where it is part of current behavior

## Module Mapping

### C to Rust File Mapping

- `quotearg.c` -> `src/quotearg.rs`

### Function Mapping

- `quotearg_n_custom_mem` -> `pub(crate) fn quotearg_n_custom_mem(...) -> ...`

The Rust signature should be chosen to preserve byte-oriented semantics. Prefer a form based on byte slices and owned output, for example:

- input as `&[u8]`
- custom quoting markers as byte slices or fixed byte values, depending on the original C usage
- output as `Vec<u8>` or a borrowed reference into per-slot storage if the original contract requires stable storage across calls

If the surrounding port requires compatibility with a slot-based returned buffer model, keep the slot-management internal to `src/quotearg.rs` rather than expanding the API surface.

## Data Model

The analysis reports multiple anonymous C data structures. Since only `quotearg_n_custom_mem` is in scope, data modeling should be limited to the structures directly required by this function and the static/internal state it accesses.

### Mapping Principles

- anonymous C structs used only as internal option/state carriers -> named private Rust structs
- anonymous C unions or flag groupings -> private Rust enums or integer flag fields, whichever most directly preserves behavior
- pointer-plus-length memory ranges -> `&[u8]` for inputs, `Vec<u8>` for owned outputs
- static slot arrays or resizable storage -> `Vec<Vec<u8>>` or equivalent private buffer store
- nullable pointers representing optional configuration -> `Option<T>` / `Option<&[u8]>`

### Expected Rust Data Structures

Because the source analysis does not provide concrete struct names, the implementation should introduce only the minimum private named types needed, such as:

- `QuotingOptions`
  - Rust replacement for the anonymous option/config structures consumed by quoting logic
  - stores custom left/right quote markers and any flags required by `quotearg_n_custom_mem`

- `QuoteSlot`
  - private storage for one indexed output buffer if the C function uses numbered persistent slots

- `QuoteSlotStore`
  - private container for all slot buffers if `quotearg_n_custom_mem` retains `n`-indexed semantics across calls

- `enum QuotingStyle` or equivalent private flag representation
  - only if required by the migrated function’s internal branching
  - do not introduce additional styles not already needed by the current function path

### Memory Management Decisions

- replace manual allocation/reallocation with `Vec<u8>`
- avoid exposing raw pointers in Rust APIs unless required by surrounding already-ported interfaces
- if the function must return data tied to internal slot storage, use internal owned buffers and return references with controlled lifetimes inside the module boundary
- preserve byte-exact output; do not assume UTF-8 unless proven by usage

### Error Handling Decisions

- C allocation-failure behavior should be translated into Rust allocation via standard containers; no custom recovery layer should be added
- internal invalid-state conditions should be represented explicitly, preferably through assertions for invariants or `Result` for user-reachable invalid inputs
- do not broaden error taxonomy beyond what is needed for this function migration

## Implementation Phases

## Phase 1: Extract and Map Required Quoting State

### Goal

Identify and isolate only the `quotearg_n_custom_mem` logic and the internal data/state it directly depends on from `quotearg.c`.

### Tasks

- create `src/quotearg.rs`
- locate the exact control flow and helper state referenced by `quotearg_n_custom_mem`
- identify which anonymous C structures are actually used by this function path
- define minimal private Rust structs/enums to replace those structures
- decide the Rust function signature based on actual call expectations in the current port branch

### Deliverables

- Rust module skeleton for quoting logic
- private Rust data type definitions for required option/state mappings
- documented function signature and ownership model for the migrated function

## Phase 2: Port Core `quotearg_n_custom_mem` Logic

### Goal

Translate the byte-processing and custom-quoting behavior into Rust with matching semantics.

### Tasks

- port the main quoting loop from C to Rust
- replace pointer arithmetic and manual buffer sizing with indexed slice access and `Vec<u8>`
- implement custom quote delimiter insertion exactly as required by the C logic
- preserve any slot-indexed storage behavior if the original function returns per-index persistent buffers
- keep helper logic private and limited to what this function requires

### Deliverables

- working Rust implementation of `quotearg_n_custom_mem`
- internal buffer/state management matching current module behavior
- removal of unsafe code unless a small, clearly justified compatibility boundary is unavoidable

## Phase 3: Integrate With Existing Crate Structure

### Goal

Wire the migrated Rust module into the current `cat` project branch without extending functionality.

### Tasks

- connect `src/quotearg.rs` into the crate module tree
- update call sites that rely on `quotearg_n_custom_mem` to use the Rust implementation
- align argument and return types with the surrounding ported code while preserving byte semantics
- ensure no duplicate or parallel legacy implementation remains active in the Rust path

### Deliverables

- module included in the crate build
- call sites updated to the Rust implementation
- successful compilation for the branch

## Phase 4: Validate Behavior With Focused Tests

### Goal

Confirm that the Rust port matches expected quoting behavior and handles memory safely.

### Tasks

- add unit tests for representative custom quoting cases
- add tests for empty input, arbitrary byte input, and delimiter edge cases
- add tests for repeated indexed calls if slot semantics are preserved
- verify output stability and absence of unintended UTF-8 assumptions
- run `cargo test` and fix behavioral mismatches

### Deliverables

- passing unit tests for `quotearg_n_custom_mem`
- validated behavior for byte-oriented and edge-case inputs
- completed module migration for this branch scope