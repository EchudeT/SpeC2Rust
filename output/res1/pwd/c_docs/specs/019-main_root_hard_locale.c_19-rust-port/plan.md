# Implementation Plan

## Summary

Port the C module `hard-locale.c` into a small Rust module that preserves the existing responsibility of `hard_locale` without adding new behavior. The Rust implementation should provide a direct, minimal translation of the locale-checking logic used by the `pwd` project’s main cluster.

The technical approach is to:
- migrate the single C function into a focused Rust source file,
- keep the API surface narrow and aligned with current call patterns,
- rely primarily on the Rust standard library,
- isolate any unavoidable environment/locale inspection behind a small function boundary,
- represent success paths with plain Rust return values and avoid manual memory management entirely.

Because this module contains one function and no declared data structures, the migration should remain compact and implementation-driven rather than introducing abstraction layers.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Preserve constant-time, low-allocation behavior appropriate for a simple locale check.
  - Avoid heap allocation where practical except where standard-library environment access requires owned strings.
  - Keep startup/runtime overhead negligible relative to the original C helper.

## Module Mapping

| C File | C Symbol | Rust File | Rust Symbol | Notes |
|---|---|---|---|---|
| `hard-locale.c` | `hard_locale` | `src/hard_locale.rs` | `hard_locale` | Direct functional port with minimal API adaptation |
| `hard-locale.c` | internal locale/environment access | `src/hard_locale.rs` | private helpers if needed | Only introduce private helpers when necessary to mirror C branching cleanly |

If the existing Rust branch already has a central module file, expose this module there with the smallest possible change:
- `src/lib.rs` or `src/main.rs`: `mod hard_locale;`
- optional re-export only if existing call sites require it

## Data Model

This module declares no dedicated C structs in the provided analysis.

| C Data | Rust Mapping | Notes |
|---|---|---|
| locale category argument / integer constants | integer type or small Rust enum as needed by call sites | Prefer the narrowest type compatible with existing usage |
| C string pointers used for locale names | `&str` / `String` / `OsString` as appropriate | Use borrowed string slices internally where possible; convert from environment values only at boundaries |
| boolean-like return value | `bool` | Prefer idiomatic Rust boolean return |

### Memory Management

- No manual allocation/free logic is needed in Rust.
- Any temporary environment-derived value should be held in owned standard-library string types only as long as required by the check.
- Avoid storing global mutable state.

### Error Handling

- If the original C function effectively reduces all lookup issues to a boolean result, keep the Rust API as `bool`.
- Treat absent or invalid environment/locale values according to the original behavior rather than surfacing new error types.
- Do not introduce custom error enums unless required by an existing Rust call interface.

## Implementation Phases

### Phase 1: Inspect and define the Rust-facing API

- Review the original `hard_locale` function signature and all current call sites in the target branch.
- Determine the exact Rust signature needed to match current project usage with minimal adaptation.
- Create `src/hard_locale.rs` and wire it into the crate module tree.
- Document any C-specific inputs that need straightforward Rust equivalents, especially locale category handling.

**Deliverable**:
- Compiling module stub with final function signature and module inclusion.

### Phase 2: Port the core locale decision logic

- Translate the branching logic from `hard-locale.c` into idiomatic Rust.
- Keep the implementation flat and local; only add private helper functions if they directly simplify the original control flow.
- Replace C string handling with standard Rust string comparison logic.
- Preserve original fallback behavior for unset, empty, or default locale values.
- Ensure the implementation does not introduce broader locale facilities beyond the original helper.

**Deliverable**:
- Functional Rust implementation of `hard_locale` matching the C behavior.

### Phase 3: Add targeted tests for behavioral parity

- Add unit tests covering the known decision cases implied by the C logic:
  - default/plain locale values,
  - non-default locale values,
  - unset or empty environment-derived values if relevant to the implementation path.
- Keep tests local to the module unless the existing crate test layout requires otherwise.
- Use `cargo test` to validate both compilation and behavior.

**Deliverable**:
- Passing unit tests for the translated logic.

### Phase 4: Integrate and remove dependency on the C implementation

- Update existing Rust-side callers to use the new module function directly.
- Confirm no duplicate locale helper remains active in the branch.
- Verify final naming and placement remain consistent with the rest of the ported main cluster modules.
- Perform a final review for unnecessary allocations, signature drift, and behavior changes.

**Deliverable**:
- Fully integrated Rust replacement for `hard-locale.c` with no extra module expansion.

## Notes and Constraints

- Prefer standard library facilities throughout; no third-party crates are warranted by the provided module scope.
- Keep the migration limited to the existing file/function responsibility.
- Do not introduce additional locale abstractions, caching, thread-local state, or generalized i18n support.
- Match existing project structure and naming conventions in the branch rather than reorganizing unrelated code.