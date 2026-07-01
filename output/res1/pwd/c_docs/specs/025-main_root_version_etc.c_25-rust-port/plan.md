# Implementation Plan

## Summary
This module ports the C file `version-etc.c` functionality used here for `emit_bug_reporting_address` into a Rust implementation with equivalent observable behavior and a minimal surface area.

The Rust approach should:
- migrate the single function into a focused Rust module under the main executable crate,
- preserve output semantics as closely as practical using the Rust standard library,
- avoid introducing new abstractions beyond what is needed to represent the existing function,
- use safe Rust for string and stream handling, with explicit `io::Write`-based output where the C code writes to standard streams.

Because this module analysis identifies only one function and no standalone data structures, the implementation should remain small and direct. The main technical work is mapping C-style output behavior and nullable/string-pointer conventions into idiomatic Rust parameter handling and error propagation.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**:
  - Rust standard library only
  - No third-party crates are recommended, since the analyzed input does not require external parsing, localization, or CLI frameworks
- **Testing**:
  - `cargo test`
  - unit tests for emitted text content and formatting boundaries
- **Performance Goals**:
  - match C module intent with negligible overhead
  - avoid unnecessary heap allocations where simple formatted writes are sufficient
  - keep runtime cost effectively dominated by output I/O, as in the original C implementation

## Module Mapping

| C File | C Function | Rust Module | Rust Item |
|---|---|---|---|
| `version-etc.c` | `emit_bug_reporting_address` | `src/main_root_version_etc.rs` or `src/version_etc.rs` | `pub(crate) fn emit_bug_reporting_address(...) -> std::io::Result<()>` |

### Mapping Notes
- Keep the Rust module narrowly scoped to the migrated function from `version-etc.c`.
- Place the module in the existing main crate layout using standard Rust source organization.
- Do not split the function into extra helper modules unless required by borrow-checking or testability concerns inside the same file.

## Data Model

No explicit C structs were identified for this module.

### C-to-Rust Type Mapping
The main migration concern is function argument and output type conversion:

| C Concept | Rust Mapping |
|---|---|
| `const char *` input text | `&str` when required to be present |
| nullable `const char *` | `Option<&str>` if the original function accepts missing values |
| `FILE *` output target | `&mut dyn std::io::Write` or a generic `W: std::io::Write` |
| integer/status return for output success/failure | `std::io::Result<()>` |

### Memory Management
- C string lifetime concerns are replaced by borrowed Rust string slices.
- No manual allocation or deallocation should be introduced unless the original formatting behavior requires temporary owned `String` construction.
- Prefer direct `write!`/`writeln!` calls over building large intermediate buffers.

### Error Handling
- Replace implicit C stream error state handling with explicit `io::Result<()>`.
- If the wider crate expects infallible printing APIs, keep a thin internal wrapper at call sites rather than hiding errors inside this module.
- Do not introduce custom error enums for this single-function port unless required by existing crate conventions.

## Implementation Phases

## Phase 1: Establish Module Skeleton and Signature
- Create the Rust source file corresponding to `version-etc.c`.
- Add the migrated function with a restrained signature based on actual call-site needs:
  - output writer parameter,
  - string parameters mapped to `&str` or `Option<&str>`,
  - `io::Result<()>` return.
- Wire the module into the crate without adding unrelated public API.
- Document any C-to-Rust signature deviation in brief code comments near the function.

### Deliverables
- New Rust module file
- Compiling function stub
- Crate-level module inclusion

## Phase 2: Port Output Logic Exactly
- Translate the body of `emit_bug_reporting_address` from C output calls into Rust formatted writes.
- Preserve line structure, spacing, and conditional text emission from the C implementation.
- Keep logic in a single function unless a tiny private formatter helper is clearly necessary.
- Ensure all writes propagate `io::Error` via `?`.

### Deliverables
- Completed Rust implementation of `emit_bug_reporting_address`
- Removal of placeholder logic
- Output semantics aligned with the original C behavior

## Phase 3: Validate Behavior with Unit Tests
- Add unit tests that write into an in-memory buffer such as `Vec<u8>`.
- Verify:
  - expected full output for standard inputs,
  - handling of optional/missing text paths if applicable,
  - newline and formatting fidelity.
- Keep tests local to this module unless existing project test organization requires otherwise.

### Deliverables
- Module unit tests
- `cargo test` passing for the ported behavior

## Phase 4: Final Integration Review
- Confirm all existing call sites compile against the Rust function signature.
- Check that no unnecessary allocations, wrappers, or extra modules were introduced.
- Review for standard-library-only compliance and safe-Rust memory handling.
- Remove any temporary compatibility scaffolding left from the initial port.

### Deliverables
- Integrated module in branch `025-main_root_version_etc.c_25-rust-port`
- Clean compile and test pass
- Finalized minimal Rust replacement for the C module