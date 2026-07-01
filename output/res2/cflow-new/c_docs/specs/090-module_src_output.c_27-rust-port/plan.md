# Implementation Plan: module_src_output.c_27

## Summary

Port `src/output.c` into a focused Rust module that preserves the existing output-selection and symbol-printing behavior without adding new capabilities. The Rust implementation should migrate the current procedural C logic into a single module-oriented design centered on:

- output driver registration and selection,
- formatting helpers for separators, line starts/ends, and indentation,
- symbol/type/classification helpers used by output generation,
- the cross-reference output entrypoint.

The technical approach should stay close to the C layout: keep one Rust source module corresponding to `src/output.c`, translate global mutable state into explicitly owned module state, and replace ad hoc C string and pointer handling with borrowed references, owned `String`s where required, and enums/structs for driver and symbol state. Error handling should be explicit with `Result` where initialization or driver lookup can fail; pure formatting helpers can remain infallible.

## Technical Context

### Language/Version
- Rust 1.78+ edition 2021

### Primary Dependencies
- Rust standard library only:
  - `std::fmt` for text formatting
  - `std::cmp` for ordering/comparison helpers
  - `std::collections` only if the current C registration flow benefits from keyed driver lookup

No third-party crates are recommended from the provided evidence.

### Testing
- `cargo test`

### Performance Goals
- Preserve near-C behavior for output generation paths.
- Avoid unnecessary heap allocation during repeated printing; prefer writing into a provided sink or reusable `String` buffer.
- Keep driver selection and helper classification operations constant-time or linear only where the C design already implies small fixed tables.
- Maintain predictable ownership and eliminate leaks/invalid accesses present in manual C memory management patterns.

## Module Mapping

### C to Rust File Mapping
- `src/output.c` → `src/output.rs`

### Function Mapping
The Rust module should retain a close one-to-one migration path for the existing functions:

- `print_level` → `output::print_level`
- `register_output` → `output::register_output`
- `select_output_driver` → `output::select_output_driver`
- `output_init` → `output::output_init`
- `newline` → `output::newline`
- `begin` → `output::begin`
- `end` → `output::end`
- `separator` → `output::separator`
- `print_text` → `output::print_text`
- `compare` → `output::compare`
- `is_var` → `output::is_var`
- `symbol_is_function` → `output::symbol_is_function`
- `clear_active` → `output::clear_active`
- `print_type` → `output::print_type`
- `xref_output` → `output::xref_output`

### Rust Module Scope
Keep all migrated logic inside one Rust module for this port. If internal organization is needed, use private helper types inside `src/output.rs` rather than creating extra modules.

## Data Model

Because the input exposes only anonymous C data structures, the Rust plan should introduce minimal named internal types based on usage, not on speculative redesign.

### Data-Structure Mapping

| C shape | Rust mapping | Notes |
|---|---|---|
| anonymous driver descriptor struct | `struct OutputDriver` | Holds driver identity and function hooks or equivalent behavior handlers. |
| anonymous registration list/table | `Vec<OutputDriver>` or fixed slice | Use `Vec` if runtime registration is required by current control flow; use static slice only if all drivers are known at compile time in the existing code. |
| anonymous active-driver/global state struct | `struct OutputState` | Owns selected driver, formatting flags, indentation/level, and any active markers currently kept in globals. |
| anonymous output formatting state | fields inside `OutputState` | Avoid splitting unless required by direct translation. |
| anonymous symbol-like record | borrowed reference to existing project symbol type | Do not create duplicate symbol models if a canonical Rust port already exists elsewhere in the project. |
| anonymous type-like record | borrowed reference to existing project type type | `print_type` should consume references, not raw pointers. |
| anonymous string buffers | `String` / `&str` | Convert C mutable char buffers to owned or borrowed UTF-8 text as appropriate; if byte-preserving behavior is needed, use `Vec<u8>` internally with controlled conversion at write points. |
| anonymous boolean/int flags | `bool`, `usize`, or small enums | Replace sentinel integers with typed flags where directly inferable. |
| anonymous compare/category constants | `enum` | Use enums for output kind or symbol classification when C used integer tags. |
| anonymous linked list nodes | `Vec` or references | Prefer contiguous ownership unless the existing algorithm truly depends on pointer-linked mutation. |

### Ownership and Memory Mapping
- Replace C global mutable storage with a single owned `OutputState`.
- Replace raw nullable pointers with `Option<&T>`, `Option<&mut T>`, or owned values.
- Replace manually managed arrays of strings/records with `Vec<T>` or slices.
- Keep borrow scopes short in formatting paths to avoid aliasing complexity.
- If driver callbacks are required, prefer function pointers over boxed trait objects unless the C behavior clearly requires dynamic extensibility beyond a fixed signature.

### Error Handling Mapping
- `output_init`, `register_output`, and `select_output_driver` should return `Result<_, OutputError>` if failure was previously signaled by status codes or null checks.
- Formatting helpers such as `newline`, `separator`, and `print_level` should be infallible unless they write into an external sink that can fail.
- If writing to a generic sink is adopted, use `std::fmt::Result` or `std::io::Result<()>` consistently based on the destination type chosen during migration.

## Implementation Phases

## Phase 1: Establish the Rust Output Module Skeleton
- Create `src/output.rs` as the direct port target for `src/output.c`.
- Identify all current C globals, static tables, and anonymous records used by the listed functions.
- Introduce minimal Rust named types:
  - `OutputDriver`
  - `OutputState`
  - `OutputError`
- Define Rust signatures for all migrated functions with close correspondence to current call sites.
- Convert C integral flags and nullable selections into typed Rust fields (`bool`, `Option`, small enums).
- Decide the sink model used by the module:
  - either accumulate into `String`,
  - or write to an existing project output abstraction,
  based strictly on the surrounding project APIs already present.

### Deliverables
- Compiling Rust module skeleton with placeholders.
- Type definitions covering all former anonymous C state involved in `src/output.c`.
- Explicit ownership plan for active driver state and formatting state.

## Phase 2: Port Driver Registration and Formatting Primitives
- Implement `register_output`, `select_output_driver`, and `output_init`.
- Migrate any C driver table/list logic into `Vec<OutputDriver>` or equivalent static storage, depending on existing behavior.
- Implement formatting helpers:
  - `print_level`
  - `newline`
  - `begin`
  - `end`
  - `separator`
  - `print_text`
- Preserve current formatting order and separator/newline semantics exactly; do not normalize output formatting during port.
- Replace C string concatenation and buffer writes with safe Rust formatting operations.
- Add unit tests for:
  - driver registration order/selection behavior,
  - indentation and separator behavior,
  - repeated begin/end/newline sequences.

### Deliverables
- Working registration/selection path.
- Working primitive output formatting functions.
- Tests covering deterministic text output behavior.

## Phase 3: Port Symbol and Type Classification Helpers
- Implement `compare`, `is_var`, `symbol_is_function`, `clear_active`, and `print_type`.
- Map C comparison and classification logic into typed Rust conditionals/enums without changing sorting or inclusion rules.
- Where these functions depend on external symbol/type definitions, integrate against the already-ported Rust equivalents by reference instead of copying data.
- Replace pointer traversal and null checks with slice/reference iteration and `Option`.
- Add unit tests for:
  - comparison ordering,
  - variable/function classification,
  - active-state clearing,
  - representative type-printing cases already implied by the C behavior.

### Deliverables
- Ported helper logic with no raw-pointer-style state handling.
- Tests validating behavior-sensitive classification and formatting helpers.

## Phase 4: Port the Main Output Entry Path
- Implement `xref_output` using the completed driver/state/helper layer.
- Preserve the original processing order, filtering, and emission structure from `src/output.c`.
- Ensure interactions among selected driver, formatting helpers, symbol classification, and type printing follow the current C execution path.
- Remove temporary placeholders and confirm no residual unsafe/manual-memory patterns remain unless strictly required by surrounding interfaces.
- Add end-to-end tests using small fixture inputs that exercise:
  - output initialization,
  - driver selection,
  - symbol emission,
  - final formatted cross-reference output.

### Deliverables
- Complete Rust replacement for `src/output.c`.
- End-to-end tests for the module’s primary output path.
- Final verification that all listed C functions are migrated and reachable from Rust call sites.

## Notes on Migration Constraints
- Keep the Rust port behaviorally aligned with `src/output.c`; do not introduce new output modes, plugin systems, or abstractions beyond what is needed to replace existing C constructs.
- Prefer safe Rust throughout; use `unsafe` only if demanded by unavoidable interaction with already-ported low-level project interfaces.
- Keep file and module expansion restrained: this port should center on `src/output.rs` and reuse existing project types where available.
- Preserve externally visible function ordering and semantics as much as practical to simplify review against the C source.