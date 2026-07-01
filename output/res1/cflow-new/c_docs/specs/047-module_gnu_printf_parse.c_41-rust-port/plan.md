# Implementation Plan

## Summary

Port `gnu/printf-parse.c` into an idiomatic Rust module that preserves the existing parsing behavior of `PRINTF_PARSE` without extending scope beyond the current C implementation.

The Rust work should focus on:
- translating the existing format-string parsing logic directly,
- preserving parsing order and control flow,
- mapping pointer/index-based scanning to slice/index iteration,
- replacing implicit C memory handling with explicit owned Rust data structures,
- expressing parse failures through `Result` or equivalent internal error types where needed, while keeping externally visible behavior aligned with the original module.

The implementation should stay narrowly scoped to the current file and function set. The preferred approach is a single Rust module containing the parser and any minimal internal helper types required to represent parse output.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain linear scanning behavior relative to input format length.
  - Avoid unnecessary intermediate allocations during format traversal.
  - Use `&str`/byte-slice indexing patterns carefully to match C-style character scanning efficiently.
  - Keep parse output allocation limited to structures that correspond to discovered directives or argument metadata already implied by the C implementation.

## Module Mapping

| C Source File | Rust Target | Notes |
|---|---|---|
| `gnu/printf-parse.c` | `src/module_gnu_printf_parse.rs` | Direct port of parser logic from the C file into a single Rust module. |
| `PRINTF_PARSE` | `pub(crate) fn printf_parse(...)` | Rust function name should use snake_case while preserving the original function’s role and argument/result structure as closely as practical. |

If the crate already has an existing module tree for migrated files, expose this file through the smallest necessary `mod` declaration only, without introducing extra abstraction layers.

## Data Model

No explicit C data structures were listed in the analysis input, so the Rust data model should be derived strictly from what `PRINTF_PARSE` requires during migration.

### Expected Mapping Strategy

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| `const char *` format input | `&str` or `&[u8]` | Prefer `&str` for API boundaries; use byte-level iteration internally if the parser relies on ASCII-class checks and positional scanning. |
| Pointer arithmetic over format text | `usize` index into byte slice | Preserve exact scan order and boundary checks explicitly. |
| Output buffers / dynamically grown parse arrays | `Vec<T>` | Replace manual allocation and resizing with `Vec`, keeping growth tied only to actual parsed items. |
| Integer status / error return | `Result<T, ParseError>` or small internal error enum | Use Rust error signaling internally; adapt at the outer boundary if compatibility requires status-style behavior. |
| C flag fields / classification codes | Rust `struct` fields and small `enum`s | Use enums only where they correspond directly to parser categories already present in the C logic. |

### Rust Internal Types

Introduce only the minimum internal types needed to support the port:
- a parse-result structure if `PRINTF_PARSE` populates multiple related outputs,
- a directive/specifier representation if the C function builds per-conversion records,
- a compact error enum for malformed format handling and bounds/consistency failures.

Do not invent new domain objects beyond what is needed to replace the C file’s local and output state.

## Implementation Phases

## Phase 1: Establish the Rust Module Skeleton

- Create `src/module_gnu_printf_parse.rs`.
- Add the smallest required module declaration/export in the crate so the migrated function is reachable by existing Rust code.
- Define the Rust signature for `printf_parse` based on the C function’s inputs and outputs.
- Add minimal internal placeholder types for parse state and return values, based only on the requirements of `PRINTF_PARSE`.
- Document any assumptions needed where C output parameters become Rust return values or mutable references.

### Deliverables
- Compiling Rust module skeleton.
- Function signature established.
- Minimal type placeholders added without full logic.

## Phase 2: Port Core Parsing Logic

- Translate the body of `PRINTF_PARSE` into Rust in the same processing order as the C source.
- Replace character-pointer traversal with indexed scanning over the format string.
- Port all directive-detection branches, width/precision/length/specifier parsing, and positional/argument-related handling that exists in the C function.
- Convert manual memory management for parsed elements into `Vec` growth.
- Preserve edge-case behavior for malformed or incomplete directives as closely as possible.

### Memory and Error Handling Focus
- Eliminate raw allocation patterns from the C implementation.
- Use explicit bounds checks before every indexed read.
- Use `Result` propagation or tightly scoped error returns instead of sentinel-based implicit failure paths, while preserving the original parser outcome semantics.

### Deliverables
- Full Rust translation of parsing control flow.
- Parse state stored in Rust-owned structures.
- No unsafe code unless the original interface absolutely requires it.

## Phase 3: Align Output Semantics and Complete Type Mapping

- Refine internal structs/enums so they accurately represent the C parser’s output fields and classification values.
- Ensure numeric parsing, flag accumulation, and argument indexing match the C implementation’s behavior.
- Reconcile any differences between C out-parameters and Rust return types/mutable references.
- Remove placeholder definitions and finalize concrete representations.

### Deliverables
- Completed data model matching the parser’s actual outputs.
- Output semantics aligned with the original function.
- Clean compile without placeholder logic.

## Phase 4: Verification and Regression Tests

- Add unit tests covering:
  - plain text without directives,
  - basic conversion directives,
  - flags, width, precision, and length combinations,
  - positional arguments if supported by the C parser,
  - malformed/incomplete format strings,
  - edge cases around trailing `%` and mixed literal/directive content.
- Use focused `cargo test` cases that compare Rust parser results to expected directive parsing outcomes inferred from the original C behavior.
- Validate that the parser does not panic on malformed inputs and instead returns the intended error/status behavior.

### Deliverables
- `cargo test` coverage for normal and edge-case parsing paths.
- Verified behavior for key parser branches.
- Final cleanup of comments and module visibility.