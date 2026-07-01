# Implementation Plan

## Summary

Port `gnu/msvc-inval.c` into a single Rust module that preserves the existing module scope and behavior around the current invalid-parameter handling state on MSVC targets. The Rust implementation should stay narrowly aligned with the C source layout: migrate the handler-related functions into one Rust source file, keep any platform-specific branching explicit, and represent the current handler state using Rust types that model nullable function pointers and opaque C-compatible data without adding broader abstractions.

The implementation approach is:
- map the C file to one Rust module under the existing crate layout,
- preserve function-level responsibilities for reading or exposing the current invalid-parameter handler state,
- use standard-library types for raw pointers, optional callbacks, and conditional compilation,
- keep unsafe code limited to the minimum needed for C-compatible callback representation and any platform interaction,
- validate the port with unit tests that focus on API-level behavior and build-time target gating.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on the provided module evidence
- **Testing**:
  - `cargo test`
- **Performance Goals**:
  - Maintain constant-time access to current handler state
  - Avoid heap allocation in normal handler lookup or forwarding paths
  - Keep the Rust port operationally equivalent to the C implementation with negligible overhead beyond necessary Rust safety boundaries

## Module Mapping

### C to Rust File Mapping

- `gnu/msvc-inval.c` → `src/module_gnu_gl_msvc_inval_per_thread_07.rs`

### Function Mapping

Because the input repeats `gl_msvc_invalid_parameter_handler`, the Rust plan treats this as one logical function entry plus any internal helper distinctions found during migration.

- `gl_msvc_invalid_parameter_handler` → `pub(crate)` or private Rust function with C-compatible callback signature as required by the original call sites
- `gl_msvc_inval_current` → Rust function returning the current invalid-parameter handler representation for the active build target
- repeated `gl_msvc_invalid_parameter_handler` entries → consolidated during port; preserve only source-justified variants if the C file contains declarations plus definition or conditional forms

### Rust Module Scope

Keep the port in a single Rust module corresponding to the original C file. Do not split callback logic, target detection, or handler-state access into extra modules unless required by existing crate organization.

## Data Model

The analysis only reports anonymous C data structures, so the Rust plan should avoid inventing named public models unless the source file requires them for compilation.

### C Structure Mapping Strategy

- `anonymous` callback/context records → private Rust `type` aliases, tuple structs, or private `struct`s only if the C source uses actual aggregate storage that must be represented
- anonymous raw pointer fields → `*mut core::ffi::c_void` or `*const core::ffi::c_void`
- anonymous callback function pointer types → `Option<unsafe extern "C" fn(...) -> ...>` or the exact ABI/signature required by the migrated code
- nullable handler state → `Option<...>` rather than sentinel pointer values where behavior remains equivalent
- compile-time platform distinctions → `#[cfg(...)]` branches rather than runtime tagging structures

### Memory Management Notes

- No ownership expansion beyond what exists in the C file
- Prefer borrowed/raw representations for platform handler state rather than allocating wrapper objects
- Keep all unsafe blocks local to callback type conversion, raw pointer handling, and any low-level platform interaction
- Document nullability and lifetime assumptions at each unsafe boundary

### Error Handling Notes

- Preserve the C module’s low-level behavior rather than introducing `Result`-heavy APIs where the original logic does not report recoverable errors
- Use `Option` for absent handler state
- If a platform-specific branch is unsupported outside MSVC-related targets, keep behavior explicit through conditional compilation and minimal fallback behavior consistent with existing crate conventions

## Implementation Phases

## Phase 1: Source Inspection and Module Skeleton

- Inspect `gnu/msvc-inval.c` to identify:
  - the exact signature of `gl_msvc_invalid_parameter_handler`,
  - whether `gl_msvc_inval_current` returns a function pointer, opaque state, or both,
  - any conditional compilation tied to MSVC, per-thread behavior, or CRT variants,
  - any anonymous struct/union definitions that need direct representation.
- Create `src/module_gnu_gl_msvc_inval_per_thread_07.rs`.
- Add the minimal module declaration/export in the crate’s existing module tree.
- Define Rust callback type aliases and opaque pointer aliases matching the C signatures as closely as possible.

## Phase 2: Function Porting

- Port `gl_msvc_invalid_parameter_handler` first, keeping:
  - original control flow,
  - original null-handling behavior,
  - original visibility restricted to existing call needs.
- Port `gl_msvc_inval_current` next, mapping:
  - current-handler retrieval logic,
  - per-thread/platform conditional behavior,
  - any static or thread-local access exactly as required by the C source.
- Consolidate repeated function entries from the analysis into the actual set of Rust definitions present in the source file.
- Keep unsafe code narrowly scoped and annotate each block with the C behavior being preserved.

## Phase 3: Data Representation and Target Gating

- Implement any required private structs or aliases for anonymous C data encountered in the source.
- Add `#[cfg(...)]` guards for MSVC-specific or Windows-specific code paths reflected in the original C file.
- Ensure non-applicable targets still compile cleanly using the smallest source-compatible fallback permitted by the original module behavior.
- Verify there is no accidental ownership transfer, heap allocation, or widened API surface.

## Phase 4: Tests and Verification

- Add unit tests covering:
  - callback type presence/absence handling,
  - current-handler retrieval behavior on supported compilation paths,
  - compile-time validity of non-MSVC fallback branches where applicable.
- Use `cargo test` to validate the module in the crate context.
- Resolve any mismatches in ABI, pointer mutability, or conditional compilation until the Rust implementation remains a direct technical port of the original C file.