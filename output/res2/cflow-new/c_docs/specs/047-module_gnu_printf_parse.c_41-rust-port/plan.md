# Implementation Plan

## Summary

Port `gnu/printf-parse.c` into a Rust module that preserves the existing parsing behavior of `PRINTF_PARSE` without expanding scope beyond the current C file. The Rust implementation should translate the original format-string parsing logic into safe, idiomatic Rust where possible, while keeping the control flow and parse states close to the source for verification.

The technical approach is to:
- migrate the single C source file into one Rust module;
- implement the parser as a focused function operating on `&str` or `&[u8]`, depending on byte-level needs from the original logic;
- represent parsed state and parse outcomes with minimal Rust structs/enums only where required by the original C logic;
- avoid introducing additional abstraction layers not justified by the source module;
- use explicit bounds checking and `Result`-based error propagation instead of pointer arithmetic and implicit failure paths.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - preserve linear-time parsing over the format string;
  - avoid unnecessary string allocation during scanning;
  - prefer slice/index-based inspection or iterator-based scanning that remains close to the C parser’s cost model;
  - keep temporary allocations limited to data that the original parser must materialize.

## Module Mapping

- **C source**: `gnu/printf-parse.c`
- **Rust target**: `src/module_gnu_printf_parse.rs`

Function mapping:
- `PRINTF_PARSE` -> `printf_parse` (Rust function name in snake_case)

If the existing Rust crate already uses a module tree for clustered ports, expose this file through the nearest existing `mod` declaration only; do not introduce extra submodules unless required by the current project layout.

## Data Model

No explicit C data structures were provided in the analysis input. The port should therefore introduce only the minimal Rust data model required to support the behavior of `PRINTF_PARSE`.

Planned mappings:

- **C parser-local state variables** -> **Rust local variables**
  - counters, indices, flags, and width/precision tracking should remain function-local unless the original file clearly shares them across helper routines.

- **C integer flag fields / parse categories** -> **Rust primitive integers or small enums**
  - use `u32`, `usize`, `i32`, or `bool` according to the original semantic range;
  - introduce enums only for clearly distinct parser states or token categories that improve correctness without changing behavior.

- **C output buffers / arrays derived during parsing** -> **Rust `Vec<T>` or slices**
  - use `Vec<T>` only where the parser needs dynamic accumulation;
  - prefer borrowed slices when the parser can reference the original format string directly.

- **C string pointer traversal** -> **Rust `&[u8]` or `str` indexing via byte positions**
  - if the source logic is byte-oriented and expects ASCII format-specifier handling, use `&[u8]` internally to mirror the C scanning model safely;
  - expose `&str` at the API boundary when input is textual and valid UTF-8 is already assumed by the Rust crate.

- **C error signaling / failure return paths** -> **Rust `Result<T, ParseError>` or equivalent**
  - define a small module-local error type only if the original function has distinct failure modes that must be preserved;
  - otherwise use a narrow success/failure representation consistent with the surrounding crate.

## Implementation Phases

### Phase 1: Source Analysis and Rust Module Skeleton

- Inspect `gnu/printf-parse.c` and isolate the full behavior of `PRINTF_PARSE`, including:
  - input parameters;
  - output parameters or mutated storage;
  - parser state transitions;
  - failure conditions;
  - assumptions about character classes, widths, precisions, flags, and conversion handling.
- Create `src/module_gnu_printf_parse.rs`.
- Define the Rust function signature for `printf_parse` to mirror the original behavior as closely as possible within Rust conventions.
- Add minimal module-local types only if required by the function signature or parse result.

**Exit criteria**:
- Rust module exists with a compile-valid skeleton.
- The intended mapping from C parameters and return values to Rust types is fixed.

### Phase 2: Core Parsing Logic Port

- Translate the main scanning loop from C into Rust, keeping branch order and parse sequencing close to the original file.
- Replace pointer arithmetic with index-checked traversal over a byte slice or string bytes.
- Port all local parse-state handling:
  - literal text scanning versus conversion detection;
  - flag parsing;
  - width and precision parsing;
  - length/modifier handling if present in the C logic;
  - terminal conversion-specifier recognition.
- Implement error propagation for malformed or incomplete format sequences using `Result` or the narrowest equivalent required by the crate.

Memory and safety focus:
- eliminate unchecked pointer dereferences;
- guard all indexing with loop invariants and explicit bounds checks;
- avoid storing references that outlive the input slice.

**Exit criteria**:
- `printf_parse` behavior is implemented end-to-end in Rust.
- The module compiles and basic happy-path parsing tests pass.

### Phase 3: Result Mapping and Behavioral Verification

- Ensure all outputs produced by `PRINTF_PARSE` are represented correctly in Rust:
  - parsed descriptor records;
  - counts;
  - flags;
  - argument-index or width/precision metadata, if applicable.
- Add unit tests that mirror the original function’s expected behavior for:
  - plain text with no conversions;
  - simple `%` conversions;
  - mixed flags/width/precision cases;
  - malformed or truncated format strings;
  - edge cases around repeated `%`, numeric parsing boundaries, and unsupported combinations handled by the C code.
- Verify that the Rust implementation does not introduce semantic drift in ordering, default values, or failure handling.

**Exit criteria**:
- `cargo test` passes for representative parser cases.
- Output/state equivalence with the C logic is reviewed.

### Phase 4: Cleanup and Integration Finalization

- Reduce only those temporary compatibility shims that are no longer necessary after tests pass.
- Align naming, visibility, and module exposure with existing project conventions.
- Remove dead code and ensure the final implementation remains limited to the migrated responsibilities of `gnu/printf-parse.c`.
- Perform a final pass on:
  - integer conversions;
  - overflow-sensitive numeric parsing;
  - allocation minimization;
  - comments documenting any unavoidable deviation from C representation.

**Exit criteria**:
- Final Rust module is integrated on branch `047-module_gnu_printf_parse.c_41-rust-port`.
- The port is limited to the original module scope and ready for review.