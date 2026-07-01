# Implementation Plan

## Summary

This module ports the C source file `version-etc.c` function `emit_bug_reporting_address` into Rust with behavior kept narrowly aligned to the existing implementation role in the main command cluster.

The Rust implementation should:
- preserve the existing output-oriented behavior of the C function,
- use standard library string and writer facilities instead of manual buffer and pointer handling,
- keep formatting logic localized to a single Rust module matching the original file’s responsibility,
- expose a small function interface suitable for use by the existing main-program flow.

The technical approach is a direct migration:
- replace C string handling with `&str` and owned `String` only where needed,
- replace raw output calls with `std::io::Write`,
- represent failure through `std::io::Result<()>` if the function writes to a stream, or through infallible return if it only builds text before the caller writes it,
- avoid introducing extra abstractions beyond what is needed to host the migrated function.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::io`, `std::fmt`, `std::borrow` as needed)
- **Testing**: `cargo test`
- **Performance Goals**:
  - equivalent practical performance to the C implementation for short formatted output,
  - no unnecessary heap allocation beyond unavoidable formatting needs,
  - linear-time formatting/output with no retained state,
  - zero unsafe code unless required by surrounding existing interfaces, which is not expected for this module.

## Module Mapping

| C File | C Function | Rust Location | Rust Item |
|---|---|---|---|
| `version-etc.c` | `emit_bug_reporting_address` | `src/version_etc.rs` | `pub(crate) fn emit_bug_reporting_address(...)` |

### Mapping Notes

- Keep the Rust module focused on the migrated contents of `version-etc.c`.
- Do not split this single-function migration into multiple helper modules unless required by compiler-visible reuse inside the same file.
- If the broader crate already has a version/help text module, place this function there only if it directly corresponds to the existing migration target; otherwise prefer `src/version_etc.rs` as the one-to-one port destination.
- Function naming should remain close to the C name for traceability during port review.

## Data Model

This module analysis reports no C structs or custom data types.

### Data-structure Mapping

| C Construct | Rust Construct | Notes |
|---|---|---|
| C string pointer inputs (`char *`, `const char *`) | `&str` | Preferred for static or borrowed textual inputs. |
| Output stream / stdio target | `&mut impl std::io::Write` or concrete writer | Use writer-based API if the function directly emits output. |
| Integer/status return conventions | `std::io::Result<()>` or `()` | Use `Result` for write failures; otherwise keep infallible. |

### Ownership and Memory

- Borrow input text with `&str` wherever possible.
- Avoid manual memory management entirely.
- Prefer direct writes to the destination writer to avoid intermediate allocation.
- If line assembly is clearer than multiple writes, use a local `String` with bounded short-lived scope.

### Error Handling

- Convert C-style implicit I/O failure handling into explicit `std::io::Result<()>` if output is performed inside the function.
- Avoid panics for expected runtime conditions such as writer failure.
- Keep the error surface minimal and local to the migrated function.

## Implementation Phases

## Phase 1: Establish the Rust module and function signature

### Goals
- Create the Rust file corresponding to `version-etc.c`.
- Define the Rust signature for `emit_bug_reporting_address`.
- Align call style with current crate conventions without broad refactoring.

### Tasks
- Add `src/version_etc.rs` if no one-to-one module exists yet.
- Declare `pub(crate) fn emit_bug_reporting_address(...)`.
- Choose the narrowest practical signature based on the C usage pattern:
  - writer parameter plus address text inputs if the C function emits directly, or
  - borrowed string inputs and returned formatted text only if existing Rust-side integration clearly expects that.
- Wire the module into the crate with `mod version_etc;` or equivalent existing module declaration.

### Acceptance Criteria
- The crate compiles with the new module stubbed in.
- The function name and placement clearly map back to `version-etc.c`.

## Phase 2: Port formatting and output behavior

### Goals
- Translate the function body from C to Rust without expanding scope.
- Preserve text formatting and output order.

### Tasks
- Replace C string/pointer logic with `&str` handling.
- Replace stdio-style emission with `write!`/`writeln!` against a Rust writer.
- Preserve any conditional formatting present in the C implementation, including punctuation, spacing, and newline behavior.
- Keep helper logic private and only introduce it if needed to mirror repeated formatting within this function.

### Acceptance Criteria
- The Rust implementation produces the same textual output as the C function for representative inputs.
- No unsafe code is introduced.
- I/O errors are surfaced explicitly if writing occurs in this function.

## Phase 3: Add focused tests for emitted output

### Goals
- Validate output fidelity and error behavior with minimal test scope.

### Tasks
- Add unit tests in `src/version_etc.rs` or under `tests/` only if crate layout requires it.
- Use in-memory buffers such as `Vec<u8>` to capture emitted output.
- Cover:
  - standard bug-reporting-address emission,
  - formatting edge cases visible from the C logic, such as empty or alternate input text if applicable,
  - writer error propagation only if the function returns `std::io::Result<()>`.

### Acceptance Criteria
- `cargo test` passes.
- Tests verify exact emitted text rather than approximate matching.

## Phase 4: Final integration review and cleanup

### Goals
- Ensure the port is minimal, traceable, and ready for use in the branch.

### Tasks
- Confirm any callers are updated to use the Rust function signature with no extra compatibility layer unless already required by existing Rust code.
- Remove temporary stubs or placeholder code.
- Review for unnecessary allocations, duplicated formatting logic, and over-generalized interfaces.
- Verify module/file naming remains consistent with the migration target.

### Acceptance Criteria
- The migrated function is integrated into the branch `025-main_root_version_etc.c_25-rust-port`.
- The implementation remains limited to the original module responsibility.
- All tests and compilation checks succeed.