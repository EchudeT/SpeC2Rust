# Implementation Plan

## Summary
Port `version-etc.c` into a Rust module that preserves the existing behavior of `emit_bug_reporting_address` with a minimal, direct translation. The Rust implementation should keep output formatting logic close to the C flow, use standard-library I/O primitives, and avoid introducing additional abstraction beyond what is needed to represent the current function boundary and call sites.

The implementation approach is:
- migrate the function into a single Rust source module corresponding to the current C file;
- represent constant/reporting text with borrowed string slices where possible;
- route output through standard Rust write targets;
- convert C-style implicit output/error behavior into explicit Rust return handling at module boundaries, while keeping call-site changes minimal.

## Technical Context
- **Language/Version**: Rust 1.78+ stable
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended based on current module evidence
- **Testing**:
  - `cargo test`
  - unit tests for emitted text and newline/format behavior
- **Performance Goals**:
  - Match C implementation behavior with negligible overhead
  - Avoid unnecessary allocation when emitting static or preformatted text
  - Keep I/O path simple and synchronous using standard library writers

## Module Mapping
- **C source**: `version-etc.c`
- **Rust target**: `src/version_etc.rs`

Planned symbol migration:
- `emit_bug_reporting_address` -> `pub(crate) fn emit_bug_reporting_address(...)`

Integration notes:
- Keep this functionality in one Rust module file matching the source responsibility of `version-etc.c`.
- If the surrounding crate currently centralizes executable entry logic elsewhere, expose only the migrated function needed by the main cluster.
- Do not split formatting helpers into additional modules unless required by existing Rust crate organization.

## Data Model
This module analysis lists no dedicated C structs for migration.

C-to-Rust representation plan:
- C string literals / `const char *` inputs used for output -> `&str`
- C output stream handle usage -> standard Rust writer abstraction, preferably:
  - `&mut dyn std::io::Write` if the original function is stream-oriented, or
  - direct stdout/stderr usage only if the C function is fixed to a process-global stream
- C integer/status error conventions -> `std::io::Result<()>` internally or at the function boundary, depending on call-site needs

Memory management notes:
- Prefer borrowed string data and stack-local formatting.
- Avoid heap allocation unless formatting requirements make it unavoidable.
- Let Rust ownership and lifetimes replace C manual pointer and stream lifetime concerns.

Error-handling notes:
- Replace unchecked C stream writes with explicit `Result` propagation where feasible.
- If existing crate conventions require infallible public behavior, keep a small wrapper that handles `Result` locally and preserves the current outward behavior.

## Implementation Phases

### Phase 1: Inspect and map the existing C function
- Review `emit_bug_reporting_address` in `version-etc.c` for:
  - exact output text segments
  - destination stream behavior
  - parameter types and nullability assumptions
  - any conditional formatting branches
- Identify the nearest Rust call site in the main cluster so the function signature can be kept narrow and compatible.
- Define the Rust function signature based on actual C usage rather than introducing generalized interfaces.

### Phase 2: Implement the direct Rust port
- Create `src/version_etc.rs`.
- Port `emit_bug_reporting_address` with formatting logic kept in the same function unless a tiny local helper is necessary to mirror repeated C output steps.
- Use standard library write macros or `Write` methods to emit text.
- Map C string handling to `&str` inputs and remove any manual lifetime management.
- Handle output errors explicitly with `std::io::Result<()>` internally; adapt outward behavior only as required by existing crate conventions.

### Phase 3: Integrate with the crate structure
- Export the migrated function with the narrowest visibility that still supports the current main-cluster call path.
- Update module declarations and replace the previous C-backed or placeholder implementation path with the Rust module.
- Ensure naming remains traceable to the original source file/function for maintenance and review.

### Phase 4: Validate behavior with focused tests
- Add unit tests covering:
  - exact emitted bug-reporting text
  - line termination behavior
  - any branch behavior driven by optional/empty inputs if present in the C logic
- Use in-memory buffers (`Vec<u8>`) to validate writer output deterministically.
- Run `cargo test` and resolve any behavior mismatches against the original C output.