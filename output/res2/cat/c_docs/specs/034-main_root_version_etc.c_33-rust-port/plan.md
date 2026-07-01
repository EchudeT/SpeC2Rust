# Implementation Plan

## Summary

Port the C module `version-etc.c` function `emit_bug_reporting_address` into a Rust module that preserves current output behavior and call shape as closely as practical within the Rust codebase. The implementation should focus on direct migration of the existing formatting and emission logic, using standard library I/O primitives rather than introducing broader reporting abstractions.

The Rust approach should:
- translate the function into a small, self-contained module,
- keep output generation deterministic and allocation-light,
- handle write failures explicitly through `std::io::Result` where the surrounding Rust code permits, or through a narrow compatibility wrapper if the existing call sites expect infallible behavior,
- avoid introducing additional utilities beyond what is needed to replace the current C function.

## Technical Context

- **Language/Version**: Rust 1.75+
- **Primary Dependencies**: Rust standard library only (`std::io`, `std::fmt`, `std::borrow` only if needed)
- **Testing**: `cargo test`
- **Performance Goals**:
  - Match the C implementation’s negligible runtime cost for a short fixed-format output routine.
  - Avoid unnecessary heap allocation where direct writes to an output sink are sufficient.
  - Preserve simple, predictable control flow and minimal formatting overhead.

## Module Mapping

| C File | C Function | Rust Module | Rust Item |
|---|---|---|---|
| `version-etc.c` | `emit_bug_reporting_address` | `src/version_etc.rs` or nearest existing migrated equivalent for `version-etc.c` | `pub(crate) fn emit_bug_reporting_address(...)` |

### Mapping Notes
- Keep the Rust file aligned to the original C file’s responsibility rather than splitting into additional helper modules.
- If the project already has a partially migrated `version_etc` module, add this function there instead of creating a new abstraction layer.
- Prefer a function signature that accepts a generic writer, such as `&mut impl std::io::Write`, if surrounding migrated code already uses explicit sinks. If not, keep the interface minimal and consistent with existing Rust call sites.

## Data Model

This module analysis lists no dedicated C structs for migration.

| C Data Type | Rust Mapping | Notes |
|---|---|---|
| string literals / `const char *` used for message fragments | `&'static str` | Direct mapping for fixed text segments |
| output stream target (if `FILE *` is involved in original code path) | `&mut dyn std::io::Write` or generic `W: Write` | Choose the narrowest form matching surrounding migrated code |
| integer status / write result conventions | `std::io::Result<()>` | Prefer explicit propagation over hidden global error state |

### Memory Management
- No manual memory management is needed for the migrated function.
- Use borrowed string slices for static message text.
- Avoid temporary owned `String` values unless exact formatting composition makes them necessary; prefer `write!`/`writeln!` directly to the sink.

### Error Handling
- Convert C-style implicit stream error handling into Rust `std::io::Result<()>`.
- If the surrounding main-cluster code expects no returned error, contain any compatibility adaptation at the call boundary rather than inside this module.
- Do not introduce custom error enums for this single-function migration.

## Implementation Phases

## Phase 1: Inspect and Define the Rust Function Boundary

- Review the original `emit_bug_reporting_address` implementation in `version-etc.c` to identify:
  - exact output text layout,
  - whether output is single-line or multi-line,
  - whether it writes to stdout, stderr, or an arbitrary stream,
  - whether it depends on static constants or macros from adjacent code.
- Locate the existing Rust destination for migrated `version-etc.c` functionality.
- Define the Rust signature to match current migrated call patterns with minimal adaptation:
  - preferred: `pub(crate) fn emit_bug_reporting_address<W: std::io::Write>(out: &mut W) -> std::io::Result<()>`
  - if required by existing structure, use a more specific sink or wrapper, but do not broaden scope.

## Phase 2: Port the Emission Logic Directly

- Translate the C formatting logic into direct Rust writes.
- Keep message fragments as `&'static str` constants if the C code uses fixed text.
- Preserve newline placement, spacing, and ordering exactly.
- Use `write!` / `writeln!` macros against the chosen writer instead of building a full intermediate buffer unless that is required for exact formatting.
- Remove any C-specific assumptions such as null-terminated strings or implicit stream state checks.

## Phase 3: Integrate with the Main Cluster

- Replace or connect the existing call site(s) in the Rust main-cluster code so they invoke the migrated function.
- Ensure the module is declared in the standard Rust project layout without adding extra facade modules.
- Propagate `std::io::Result` upward where surrounding code already returns a result; otherwise, add only the minimal local handling needed to preserve current behavior.
- Verify that no remaining references depend on the old C-style interface within the Rust branch.

## Phase 4: Add Focused Tests and Validate Behavior

- Add unit tests that capture the function’s emitted bytes using an in-memory buffer such as `Vec<u8>`.
- Assert exact output content, including:
  - line breaks,
  - punctuation,
  - any fixed addresses or surrounding wording,
  - behavior for the complete message sequence.
- If there are multiple valid formatting branches in the source, add one test per branch only.
- Run `cargo test` and confirm the migrated function behaves identically to the original formatting contract.