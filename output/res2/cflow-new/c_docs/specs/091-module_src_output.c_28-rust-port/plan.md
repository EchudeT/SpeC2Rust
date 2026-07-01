# Implementation Plan: module_src_output.c_28

## Summary

This module migration covers `src/output.c`, specifically the existing `set_active` and `output` functions, and ports their current behavior into Rust without adding new capabilities. The Rust implementation should preserve the original control flow and state transitions around output selection and emission, while replacing C global/stateful patterns, raw pointers, and implicit lifetime rules with explicit Rust ownership and borrowing.

The implementation approach is to create a narrowly scoped Rust module that mirrors the C file’s responsibilities only:
- represent the C module’s internal state with Rust structs/enums,
- translate `set_active` into explicit mutable state updates,
- translate `output` into safe formatted/output-writing logic,
- keep the call surface aligned with the original behavior as closely as practical for the surrounding port.

The migration should favor standard library facilities such as `Option`, `Vec`, `String`, and `std::io::Write`, and should preserve existing output ordering and side effects. Error handling should replace C sentinel/error-path patterns with `Result` where the caller can already accommodate propagation; otherwise, internal state checks should be made explicit without broadening behavior.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve the C module’s effective runtime characteristics for output-path operations.
  - Avoid unnecessary allocation beyond what is required to replace mutable C string handling safely.
  - Keep output-path overhead low by using borrowed data where possible and writing directly to sinks instead of building intermediate structures unless required by the original logic.

## Module Mapping

### C to Rust File Mapping

- `src/output.c` → `src/output.rs`

### Function Mapping

- `set_active` → `set_active(...)` in `src/output.rs`
  - Port as a focused state mutation function over explicit module state.
  - Replace C global/shared mutable state access with `&mut` access to the owning Rust state object where feasible.
- `output` → `output(...)` in `src/output.rs`
  - Port as the main emission routine using safe string/stream handling.
  - Preserve original ordering, branching, and active-target behavior.

### Rust Module Placement

Use a single Rust source file for this migration unit:
- `src/output.rs`

If the current Rust crate already uses a central module declaration, only add the minimal `mod output;` / `pub(crate) mod output;` wiring needed for this file. Do not introduce additional helper modules unless the existing crate layout already requires them.

## Data Model

Because the analysis reports only anonymous C data structures, the Rust port should derive its concrete types directly from the actual fields used by `src/output.c`. The mapping should remain local to this module unless another already-ported module owns the same state.

### Data-Structure Mapping Strategy

- **C anonymous struct used as module state** → named Rust `struct`
  - Assign a stable, role-based name derived from usage in `set_active`/`output`.
  - Convert nullable pointers to `Option<T>` or `Option<&T>` / `Option<&mut T>` depending on ownership.
  - Convert ownership-bearing pointer-linked buffers to `String`, `Vec<u8>`, or `Vec<T>` as dictated by actual usage.
- **C anonymous record used for output target/configuration** → named Rust `struct` or `enum`
  - Use `enum` if the C logic switches among output modes/types.
  - Use `struct` if the C layout is a plain field bundle with no tagged variants.
- **C integer flags / active selectors** → Rust `enum` when the values represent discrete modes, otherwise integer/bitflag-compatible primitive
  - Prefer `enum` for mode selection used by `set_active`.
  - Keep primitive integers only if the C logic depends on arithmetic/bitwise flag composition already present in the source.
- **C string pointers (`char *`)** → `String` or borrowed `&str`
  - Use `String` for owned mutable text retained in module state.
  - Use `&str` for transient input parameters where lifetimes remain local.
- **C output handles (`FILE *`-like or sink pointers)** → `std::io` writer abstraction or concrete owned handle already established by surrounding code
  - Prefer a concrete writer field if the surrounding Rust port already chose one.
  - If polymorphism is necessary, keep it minimal and local to the existing call pattern.

### Expected Rust Type Shapes

The exact names depend on field inspection, but the port should likely resolve the anonymous C structs into a small set of role-based Rust types such as:

- `OutputState`
- `ActiveOutput`
- `OutputTarget`
- `OutputMode`

These names are placeholders for implementation planning only; final naming should match the actual semantics visible in `src/output.c` and should not invent broader abstractions.

### Memory Management Notes

- Eliminate manual allocation/free patterns by moving owned buffers and state into Rust-owned fields.
- Replace null checks with `Option` matching.
- Where C code stores aliases into mutable global/module state, define a single owner and pass references explicitly.
- Avoid self-referential layouts; if the C code relies on pointer stability, redesign around indices, enums, or owned sibling fields instead of raw internal references.

### Error Handling Notes

- Replace C return-code error signaling with `Result` where callers can naturally propagate failure.
- If the original functions are effectively infallible in normal use and primarily mutate state, keep them simple and avoid introducing artificial error variants.
- For output writes, map I/O failure to `std::io::Result` if the operation truly writes to an external sink in the Rust port.
- Preserve original behavior for invalid/unset active state by expressing the same preconditions explicitly rather than silently broadening acceptance.

## Implementation Phases

## Phase 1: Inspect and Define Rust State Model

- Inspect `src/output.c` to identify:
  - all file-local/static variables used by `set_active` and `output`,
  - all anonymous struct field layouts referenced by these functions,
  - whether output routing is mode-based, sink-based, or both.
- Create `src/output.rs`.
- Define the minimal Rust structs/enums required to represent:
  - module state,
  - active output selection,
  - output target/configuration referenced by the two functions.
- Decide ownership per field:
  - owned strings/buffers become `String`/`Vec<_>`,
  - optional references or absent pointers become `Option<_>`,
  - mode selectors become enums where appropriate.

**Exit criteria**:
- All state used by the C functions is represented in Rust types.
- No placeholder raw pointers remain except where unavoidable during transitional porting inside the crate.

## Phase 2: Port `set_active`

- Translate `set_active` first, since it establishes the state consumed by `output`.
- Preserve the original selection logic exactly:
  - active target switching,
  - any reset/update of associated fields,
  - any validation or fallback behavior present in C.
- Convert direct global mutation into mutation through a single explicit Rust state object.
- Add focused unit tests for:
  - selecting each valid active target/mode,
  - replacing a prior active selection,
  - handling unset/invalid cases only to the extent the original behavior defines them.

**Exit criteria**:
- `set_active` compiles and updates Rust state equivalently to the C implementation.
- Tests confirm state transitions and active selection semantics.

## Phase 3: Port `output`

- Translate `output` against the Rust state model produced in earlier phases.
- Preserve:
  - branching by active mode/target,
  - text formatting and ordering,
  - any conditional suppression or redirection behavior.
- Replace C buffer and string assembly with safe Rust string handling only where needed.
- If writing to a sink, use `std::io::Write`-compatible logic or the concrete sink type already established in the port.
- Convert failure paths:
  - I/O-related failures to `Result`,
  - internal absent-state conditions to explicit checks matching original assumptions.

- Add unit tests covering:
  - output under each relevant active state,
  - formatting-sensitive cases visible from the C logic,
  - behavior when no active output is set, if such a state is reachable in the original module.

**Exit criteria**:
- `output` behavior matches the C logic in emitted content and control flow.
- Tests verify output text and state-dependent routing.

## Phase 4: Integrate and Clean Up

- Wire `src/output.rs` into the crate using the minimal existing module conventions.
- Remove any temporary compatibility code used during translation.
- Ensure signatures align with already-ported neighboring modules without introducing new adapter layers beyond what is necessary for compilation.
- Run `cargo test` and fix mismatches in:
  - ownership/borrowing,
  - error propagation,
  - formatting/output behavior.

**Exit criteria**:
- The Rust module is integrated cleanly.
- `cargo test` passes for the module and dependent code paths.
- The port remains limited to the original file’s responsibilities without extra abstractions or features.