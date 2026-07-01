# Implementation Plan

## Summary
This module ports `src/parseopt/help.c` into Rust with a narrow scope: preserve the existing help/option parsing behavior embodied by the `min` function and migrate any file-local data layout required to support it.

The Rust implementation should:
- translate the current control flow and data access patterns directly, rather than redesigning the option/help subsystem,
- replace C pointer/null handling with explicit references, slices, and `Option`,
- keep output generation and comparison logic behaviorally aligned with the C implementation,
- confine the port to the Rust equivalent of this source file and only the items needed for its compilation and tests.

The technical approach is a file-oriented migration:
- create one Rust module corresponding to `src/parseopt/help.c`,
- map C anonymous structs/unions used by this file into locally scoped named Rust structs/enums with minimal fields,
- port `min` first as the only required function surface from this analysis unit,
- add focused tests for boundary behavior and parity with the original integer/ordering semantics.

## Technical Context

### Language/Version
- Rust stable, edition 2021
- Minimum recommended toolchain: Rust 1.75+

### Primary Dependencies
- Rust standard library only

No third-party crates are recommended from the available evidence. The module appears small and procedural, and standard library facilities should be sufficient for:
- integer comparison,
- string handling,
- collection-free or slice-based traversal,
- formatting/output assembly if needed.

### Testing
- `cargo test`

Testing should cover:
- direct unit tests for `min`,
- edge cases around integer ordering and equal values,
- any helper behavior introduced solely to replace C-local data manipulation in this file.

### Performance Goals
- Match C module asymptotic behavior.
- Avoid unnecessary heap allocation when porting local help/option processing.
- Preserve constant-time behavior for `min`.
- Use borrowed data (`&str`, slices, references) where the C code used borrowed pointers, unless ownership transfer is clearly required.

## Module Mapping

### C to Rust File Mapping
- C: `src/parseopt/help.c`
- Rust: `src/parseopt/help.rs`

### Symbol Mapping
- C function: `min`
- Rust function: `pub(crate) fn min(...) -> ...` or private `fn min(...) -> ...`

Visibility should remain as narrow as possible:
- use private functions unless cross-module use in the current Rust crate requires `pub(crate)`,
- do not widen API surface beyond what the migrated callers need.

### Integration Mapping
If the original C file was included through a larger parseopt subsystem, wire the Rust file into the existing crate module tree with the minimal equivalent:
- `src/parseopt/mod.rs` exports `help`,
- no additional adapter modules unless already required by the existing Rust project layout.

## Data Model

The analysis identifies only anonymous data structures, so the Rust plan should treat them as file-local implementation details rather than stable public models.

### Data-Structure Mapping Strategy
For each anonymous C structure referenced from `help.c`:
- assign a descriptive Rust name based on role in this file,
- keep the type private to `parseopt::help`,
- translate field types mechanically from usage.

### C-to-Rust Type Mapping Rules
- `char *` used as borrowed text -> `&str` when UTF-8 input is guaranteed by the surrounding Rust code; otherwise `&[u8]` or `String` only if mutation/ownership is needed
- `const char *` -> `&str`
- nullable pointer -> `Option<&T>` / `Option<&str>`
- mutable pointer to owned state -> `&mut T`
- array plus length -> slice `&[T]` / `&mut [T]`
- integer flags/booleans -> `bool` if only boolean semantics are used; otherwise preserve exact integer type
- enum-like integer discriminants -> Rust `enum` only if all variants are known from this file; otherwise keep integer type
- C anonymous struct -> private named `struct`
- C anonymous union or variant-like payload -> private `enum` if the active variant is explicit; otherwise a conservative struct layout matching actual usage

### Proposed Rust Model Shape
Because the exact field inventory is not provided, use this restrained naming approach during implementation:
- anonymous record for help entry -> `HelpEntry`
- anonymous record for option/help formatting state -> `HelpState`
- anonymous record for range or width tracking -> `WidthInfo`
- anonymous callback/context carrier -> `HelpContext`

Only introduce the named structs actually required by `help.c`; do not predefine all twenty placeholders if some are unused after examining the source.

### Memory Management Notes
- Replace C ownership ambiguity with explicit borrowing first.
- Avoid `unsafe` unless the surrounding crate still exposes raw C-compatible buffers that cannot yet be refactored.
- If temporary concatenation or formatting is required, use local `String` values with clear ownership boundaries.
- Preserve stack allocation for small transient values where C used local structs.

### Error Handling Notes
- If `min` cannot fail semantically, keep it infallible.
- For helper routines created during the port, prefer:
  - plain return values for infallible logic,
  - `Option<T>` for nullable-result cases,
  - `Result<T, E>` only where the C code had explicit failure branches that must be represented.
- Do not introduce broad custom error frameworks for this module.

## Implementation Phases

## Phase 1: Source Audit and Rust Module Skeleton
- Inspect `src/parseopt/help.c` and identify:
  - the exact signature and call sites of `min`,
  - all anonymous structs/unions actually touched by this file,
  - any local macros or constants needed by `min`.
- Create `src/parseopt/help.rs`.
- Add the module to the crate tree with minimal visibility.
- Define placeholder private Rust structs/enums only for data shapes this file truly uses.

### Deliverables
- Rust module file exists and compiles in skeleton form.
- Initial type aliases/structs reflect the C file’s local data roles.
- No behavioral expansion beyond the original file.

## Phase 2: Port Core Logic from `help.c`
- Port `min` directly, preserving:
  - operand types or their safe Rust equivalents,
  - comparison semantics,
  - return-value behavior for equal and boundary cases.
- Port any tightly coupled local helpers or constants only if `min` depends on them.
- Replace C macros with:
  - `const`,
  - small private functions,
  - `match` or `if` expressions.

### Deliverables
- `min` implemented in Rust.
- The module builds without stubbed logic for the migrated function.
- Pointer/null-sensitive logic is expressed with references and `Option`.

## Phase 3: Data Layout Stabilization and Call-Site Alignment
- Refine the private Rust data structures based on actual field access in `help.c`.
- Remove unnecessary raw-pointer patterns introduced during initial translation.
- Align any existing Rust caller signatures with the new function signature while keeping behavior unchanged.
- Ensure integer widths (`i32`, `usize`, etc.) match the original semantics where comparisons or indexing are involved.

### Deliverables
- Private data model finalized for this file.
- Call sites compile cleanly with minimal API exposure.
- No unnecessary allocations or ownership churn remain in the migrated path.

## Phase 4: Verification and Cleanup
- Add unit tests for `min`:
  - lower/greater ordering,
  - equal operands,
  - boundary-relevant values for the chosen integer type.
- Run `cargo test`.
- Remove dead translation artifacts, such as unused temporary wrappers or redundant compatibility helpers.
- Confirm the module remains limited to the original C file responsibilities.

### Deliverables
- Passing `cargo test`.
- Clean Rust implementation of the `help.c` migration unit.
- Final code is idiomatic where safe to do so, but still structurally traceable to the C source.