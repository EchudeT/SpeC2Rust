# Implementation Plan: module_src_output.c_28

## Summary

This plan covers the Rust port of `src/output.c`, limited to the existing responsibilities represented by `set_active` and `output`. The Rust implementation should preserve the current module boundaries and behavior while translating C-style global state, pointer-oriented data flow, and implicit ownership into explicit Rust module state and safe function interfaces.

The implementation approach is to migrate the C file into a single Rust source module with a narrow internal state model. Any C anonymous data structures used by this file should be converted into named Rust structs or enums only as required by the referenced fields and control flow in `set_active` and `output`. The port should prefer standard-library types, model nullable pointers as `Option`, and replace integer error signaling or unchecked writes with explicit `Result` returns only where the original behavior requires fallible operations. The migration should stay focused on parity with the existing file and not introduce extra abstraction layers.

## Technical Context

- **Language/Version**: Rust 1.78+ (stable)
- **Primary Dependencies**:
  - Rust standard library
  - No third-party crates recommended from the available evidence
- **Testing**:
  - `cargo test`
  - Unit tests localized to the Rust replacement for `src/output.c`
- **Performance Goals**:
  - Maintain behaviorally equivalent runtime characteristics to the C implementation
  - Avoid unnecessary heap allocation during output-path execution
  - Preserve sequential output flow and low-overhead state switching in `set_active`
  - Keep copies minimal; prefer borrowing and stack-local formatting where possible

## Module Mapping

### C to Rust File Mapping

- `src/output.c` -> `src/output.rs`

### Function Mapping

- `set_active` -> `pub(crate) fn set_active(...) -> ...`
- `output` -> `pub(crate) fn output(...) -> ...`

The exact Rust signatures should be derived from the existing C call sites and parameter types. During migration:
- raw pointer inputs that are required and non-null in practice should become references
- nullable pointer inputs should become `Option<&T>` / `Option<&mut T>`
- writable buffers or streams should be represented using concrete standard-library writer/state types already implied by the project
- C integer status returns should map to `Result` only if the function can actually fail due to I/O or invalid state; otherwise preserve simple return types

### State Mapping

If `src/output.c` currently uses file-local statics or mutable globals to track active output state, these should become:
- file-local `static` immutable constants where possible
- otherwise, explicit module state encapsulated in a Rust struct local to `src/output.rs`
- if mutation is required across calls, keep it narrowly scoped and consistent with existing single-threaded assumptions rather than introducing broader synchronization machinery

## Data Model

The analysis only identifies anonymous C data structures. The Rust port should introduce named types only for structures directly touched by `set_active` and `output`.

### Data-Structure Mapping Rules

- `anonymous` C struct used as stable record with named fields
  - -> `struct <DomainName> { ... }`
- `anonymous` C union used for tagged alternatives
  - -> `enum <DomainName>` when behavior depends on variant
  - -> dedicated struct with optional fields only if the C usage is field-overlap-free and simpler
- `anonymous` pointer-linked record
  - -> `Option<Box<T>>`, `Option<&T>`, or index/reference into an owning collection, depending on actual ownership
- `char *` textual field
  - -> `String` when Rust owns the text
  - -> `&str` when borrowed
  - -> `Option<String>` / `Option<&str>` when nullable
- output target/file handle abstractions
  - -> standard-library writer/file handle types already used by the surrounding Rust port
- C flags / mode integers
  - -> `bool` for single-bit meaning
  - -> small Rust `enum` for mutually exclusive modes
  - -> integer type only if arithmetic/bitmask compatibility is required by the existing logic

### Planned Named Rust Types

Because the source analysis does not expose field layouts, introduce only provisional names during implementation and collapse or refine them based on the actual C definitions:

- `OutputState`
  - holds the mutable module-level active-output selection and any persistent configuration needed by both functions
- `ActiveTarget`
  - enum or struct representing the currently selected output destination/mode if `set_active` switches among alternatives
- `OutputConfig`
  - struct for static formatting/output options if these are grouped in the C anonymous records
- `OutputItem` / `OutputRecord`
  - struct for the data unit emitted by `output`, if the function operates on a compound record
- Additional named structs for the remaining anonymous C records only if their fields are directly read or written by this module

### Memory Management and Error Handling

- Replace nullable ownership patterns with explicit ownership and borrowing
- Eliminate manual allocation/free logic by using RAII-managed Rust values
- Convert any transient C string building to `String` or formatting directly into the destination writer
- Preserve existing silent/non-failing behavior where appropriate; do not over-generalize all functions into fallible APIs
- Where `output` performs actual I/O, use `std::io::Result<()>` or a project-local equivalent already in use by adjacent ports
- Validate assumptions previously implicit in pointer checks using `Option` matching and narrow precondition enforcement

## Implementation Phases

## Phase 1: Read-through and Rust Skeleton for `src/output`

- Create `src/output.rs`
- Inspect `src/output.c` and identify:
  - exact function signatures for `set_active` and `output`
  - file-local static variables
  - anonymous struct/union definitions referenced by this file
  - call-site expectations from neighboring translated modules
- Define the minimal Rust module skeleton:
  - placeholder named structs/enums for each anonymous type actually used here
  - preliminary function signatures matching the surrounding Rust codebase
  - internal state container if file-global mutable state exists in C
- Decide per parameter whether each C pointer becomes:
  - reference
  - mutable reference
  - `Option`
  - owned value

## Phase 2: Port `set_active` and State Representation

- Translate the C active-selection logic into Rust with explicit state transitions
- Replace global mutable C state with the smallest viable Rust representation inside `src/output.rs`
- Map mode/flag integers to booleans or enums where semantically clear from the C code
- Preserve ordering and side effects exactly, especially if `set_active` updates shared output configuration used immediately by `output`
- Add unit tests covering:
  - default/initial active state
  - switching active targets or modes
  - null/absent optional inputs if they exist in the C API
  - repeated `set_active` calls and state overwrite behavior

## Phase 3: Port `output` Logic

- Translate the main output path into Rust using the state and data models defined earlier
- Preserve formatting, branching, and record traversal order from the C implementation
- Replace raw buffer writes and pointer stepping with safe string/byte handling
- Keep allocation behavior restrained:
  - borrow existing data where possible
  - avoid building intermediate collections unless required by the original algorithm
- If `output` writes to a file/stream, expose fallibility via standard Rust I/O results only at the boundary where writing occurs
- Add unit tests covering:
  - representative output formatting cases
  - interaction with prior `set_active` state
  - edge conditions from the C logic such as empty input, null-linked fields, or mode-dependent suppression

## Phase 4: Parity Cleanup and Integration Validation

- Compare Rust behavior against the C implementation for the reachable paths in `set_active` and `output`
- Remove provisional type names only after confirming the smallest stable set of structs/enums
- Ensure no unnecessary public API has been introduced beyond what callers require
- Run `cargo test` and fix signature or ownership mismatches at integration boundaries
- Final review points:
  - no leaked C allocation patterns remain
  - no unchecked null semantics remain
  - output ordering and formatting remain stable
  - module scope remains limited to the original `src/output.c` responsibilities