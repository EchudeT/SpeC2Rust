# Implementation Plan

## Summary

Port `src/output.c` into a single Rust module that preserves the existing output-selection and text-emission behavior without adding new abstractions beyond what is required for safe ownership and explicit error handling.

The Rust implementation should keep the C module’s responsibilities together:
- registration and selection of output drivers,
- lifecycle calls for output formatting,
- text and separator emission,
- symbol/type-oriented print helpers,
- cross-reference output entry point.

The technical approach is:
- migrate the current C file into one Rust source file with closely corresponding functions,
- replace implicit global mutable state with a narrowly scoped module-owned state structure,
- represent C function-pointer-style output drivers with a Rust enum or struct of callbacks, choosing the smallest mapping that matches the existing call sites,
- use borrowed string slices and explicit writer interfaces to avoid manual memory management,
- convert sentinel/error-code flows into `Result` where failure is possible, while keeping pure formatting helpers infallible.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only (`std::io`, `std::cmp`, `std::fmt`, `std::collections` only if required by the existing registration logic)
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve current asymptotic behavior of driver lookup, output dispatch, and text formatting,
  - avoid unnecessary string allocation during print paths,
  - keep per-call overhead close to the C implementation by using direct dispatch and borrowed data where possible.

## Module Mapping

### C to Rust File Mapping

- `src/output.c` -> `src/output.rs`

### Function Mapping

Keep function names close to the original module naming, using idiomatic Rust visibility and signatures only where needed for safety.

- `print_level` -> `print_level`
- `register_output` -> `register_output`
- `select_output_driver` -> `select_output_driver`
- `output_init` -> `output_init`
- `newline` -> `newline`
- `begin` -> `begin`
- `end` -> `end`
- `separator` -> `separator`
- `print_text` -> `print_text`
- `compare` -> `compare`
- `is_var` -> `is_var`
- `symbol_is_function` -> `symbol_is_function`
- `clear_active` -> `clear_active`
- `print_type` -> `print_type`
- `xref_output` -> `xref_output`

### Internal Organization

Use one Rust module file corresponding to the C file. Inside `src/output.rs`, organize code into restrained sections only:

1. driver/state definitions,
2. registration and selection functions,
3. formatting/output lifecycle functions,
4. symbol/type helper functions,
5. xref emission entry point,
6. unit tests.

No additional architectural layers should be introduced unless the existing C dependencies force them.

## Data Model

Because the source analysis reports anonymous C structures only, define Rust data types based on role rather than inferred names, and keep them local to this module unless other migrated files require wider visibility.

### Data-Structure Mapping

| C construct | Rust mapping | Notes |
|---|---|---|
| anonymous driver descriptor struct | `struct OutputDriver` | Holds driver identity and the callable operations currently stored in C function pointers or equivalent tables. |
| anonymous global/current-output state struct | `struct OutputState` | Tracks registered drivers, selected driver, indentation/level state, and any active flags now held in globals/statics. |
| anonymous formatting/options struct | `struct OutputOptions` or fields inside `OutputState` | Only create if the C file clearly groups options; otherwise keep as state fields. |
| anonymous symbol-related record | borrowed reference to existing migrated symbol type, or `struct SymbolRef<'a>` shim | Do not duplicate the project’s symbol model; use a local shim only until the upstream symbol module is ported. |
| anonymous type-related record | borrowed reference to existing migrated type node, or `struct TypeRef<'a>` shim | Same approach as symbols: temporary thin mapping only if required to compile in migration order. |
| anonymous active-flag container | `enum ActiveState` or boolean fields | Prefer booleans if the C state is simple; use enum only if there are multiple mutually exclusive states. |
| C string pointers | `&str`, `String`, or `Option<&str>` | Borrow where input-only; own only when registration requires storage beyond call scope. |
| C integer flags | `bool`, `u32`, or small enums | Replace bitwise flags with enums only when directly supported by the current C semantics. |
| C function pointer table | function items or boxed closures avoided; use plain fn pointers in `OutputDriver` | Choose plain function pointers if dispatch is static and callback signatures are uniform. |
| C comparison return values | `std::cmp::Ordering` or `i32` wrapper | Use `Ordering` internally; convert only if compatibility with existing sorting interfaces requires integer results. |

### Ownership and Lifetime Decisions

- Registered driver names should be owned by the module (`String`) only if they must outlive registration calls.
- Emitted text should be passed as `&str` and written directly to the selected output target.
- Symbol and type inputs should be borrowed references; this module should not take ownership of upstream semantic data.
- Module state that was global in C should become a single explicit state holder. If the wider project still relies on process-global behavior, keep a minimal global container and isolate mutability carefully, but prefer passing `&mut OutputState` through migrated call paths.

### Error Handling Mapping

- C null checks for missing driver/current output -> `Option` with explicit error return.
- C status/error codes from initialization/selection -> `Result<(), OutputError>`.
- Pure helper predicates such as `is_var` and `symbol_is_function` -> return `bool`.
- Comparison helper -> return `Ordering` internally unless an existing caller contract requires a numeric comparator.

## Implementation Phases

## Phase 1: Establish Rust module skeleton and state mapping

### Goals
Create `src/output.rs` with the minimal state and type definitions needed to replace the C module’s globals and driver records.

### Tasks
- Add `src/output.rs`.
- Define `OutputDriver`.
- Define `OutputState`.
- Define a minimal `OutputError` enum for:
  - unknown driver,
  - duplicate registration if applicable,
  - uninitialized output state,
  - write failure.
- Implement initial versions of:
  - `register_output`
  - `select_output_driver`
  - `output_init`
  - `clear_active`
- Translate any C global/static variables into fields on `OutputState`.
- Keep signatures close to original usage to reduce churn in dependent migrated files.

### Notes
- If the current C code uses static driver arrays, start with `Vec<OutputDriver>` unless the original fixed-capacity behavior is semantically important.
- Do not introduce trait objects unless callback signatures differ enough to require them; plain function pointers are preferred for a close port.

## Phase 2: Port formatting and output dispatch functions

### Goals
Migrate the direct output operations and lifecycle calls while preserving formatting order and minimizing allocation.

### Tasks
- Implement:
  - `print_level`
  - `newline`
  - `begin`
  - `end`
  - `separator`
  - `print_text`
- Map C output writes onto a concrete writer strategy already used by the project; if none exists yet, use a module-local writer target based on `std::io::Write`.
- Preserve indentation/level semantics exactly.
- Replace manual C string handling with `&str` and bounded formatting through `write!`/`writeln!` as needed.
- Keep lifecycle functions no-op-capable if the selected driver omits certain operations in C.

### Notes
- Where the C code conditionally invokes driver callbacks, represent those operations as `Option<fn(...) -> ...>` inside `OutputDriver`.
- Avoid building intermediate `String`s unless formatting requires composition across multiple writes.

## Phase 3: Port comparison and semantic helper logic

### Goals
Move non-emission helper logic needed by output generation into Rust with exact predicate behavior.

### Tasks
- Implement:
  - `compare`
  - `is_var`
  - `symbol_is_function`
  - `print_type`
- Translate pointer/null and integer-classification logic into pattern matches and option checks.
- For `print_type`, use borrowed access to the project’s migrated type representation; if unavailable at this stage, add the smallest temporary shim needed to compile and mark it for replacement when the type module lands.
- Add focused unit tests for helper behavior, especially ordering and predicate edge cases.

### Notes
- Keep helper logic colocated in `src/output.rs`; do not split into separate utility modules.
- If `compare` is used by sorting code expecting a C-style comparator, provide a thin adapter around an internal `Ordering`-based implementation.

## Phase 4: Port xref output path and finalize tests

### Goals
Complete the main output entry point and verify behavior across driver selection, formatting, and semantic helpers.

### Tasks
- Implement `xref_output`.
- Connect `xref_output` to:
  - selected driver state,
  - lifecycle calls,
  - text/separator/newline helpers,
  - symbol/type helpers.
- Add unit tests covering:
  - driver registration and selection,
  - missing-driver error path,
  - indentation/newline behavior,
  - representative text output flow,
  - helper predicate behavior,
  - xref output on a small deterministic input fixture.
- Remove temporary shims if dependent migrated types are available by this point.

### Completion Criteria
- `src/output.c` functionality is represented in `src/output.rs`.
- No manual memory management remains in this module.
- All fallible external-facing operations return explicit Rust errors.
- `cargo test` passes for the module’s unit tests and integration points relevant to this migration.

## Testing Strategy

- Add unit tests directly in `src/output.rs` for helper and formatting functions.
- Prefer deterministic writer-backed tests using in-memory buffers.
- Validate exact output strings for:
  - indentation level handling,
  - separator placement,
  - begin/end ordering,
  - type-print fragments where behavior is stable.
- Validate state transitions for:
  - registration,
  - reselection,
  - clear/reset behavior.
- Keep tests scoped to migrated behavior only; do not introduce new behavioral expectations beyond the C module.

## Migration Notes

- Preserve function order in `src/output.rs` as much as practical to ease review against `src/output.c`.
- Preserve naming correspondence for easier diff-based validation during the port.
- Convert hidden global coupling into explicit state access incrementally; avoid large signature changes outside the direct call graph needed for this file.
- Where external C modules are not yet ported, use the smallest temporary borrowed shim types and remove them once their canonical Rust types exist.