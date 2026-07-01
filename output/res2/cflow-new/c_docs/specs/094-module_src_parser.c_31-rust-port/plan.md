# Implementation Plan: module_src_parser.c_31

## Summary

Port `src/parser.c` into an idiomatic Rust parser module that preserves the existing parsing flow and symbol-handling behavior without adding new capabilities. The Rust implementation should keep the original function boundaries recognizable where practical so migration and validation remain straightforward.

The technical approach is to translate the parser logic into a single Rust module first, with closely corresponding functions for:

- declaration parsing
- direct/declarator parsing
- typedef and parameter-list handling
- K&R argument parsing support
- symbol lookup and reference recording
- function-call handling within the parser flow

State that is implicit or global in C should be made explicit in Rust through parser-owned structs and mutable references. Memory ownership should rely on Rust’s standard ownership model, using `String`, `Vec`, and enums/structs instead of raw pointers and anonymous C aggregates. Error handling should convert parse failures and invalid states into `Result`-based returns where needed, while preserving the original control flow as closely as possible.

## Technical Context

### Language / Version

- Rust 1.78+ stable

### Primary Dependencies

- Rust standard library only

No third-party crates are recommended based on the provided module scope. The parsing and data-structure needs described here can be covered with:

- `String`
- `Vec`
- `Option`
- `Result`
- `HashMap` or `BTreeMap` from `std::collections` if existing symbol-table behavior requires keyed lookup

### Testing

- `cargo test`

Testing should focus on:
- function-level migration validation for parser behavior
- declaration and typedef parsing cases
- parameter/declarator parsing paths
- symbol lookup and reference addition paths
- function-body and call-related control flow

### Performance Goals

- Maintain behaviorally similar performance to the C parser for single-pass source parsing
- Avoid unnecessary string cloning during token/identifier handling where borrowing is practical
- Keep symbol lookup near constant or logarithmic time, depending on the chosen standard-library map type
- Do not introduce extra parsing passes beyond what the C implementation requires

## Module Mapping

### C to Rust File Mapping

- `src/parser.c` -> `src/parser.rs`

If the existing Rust crate already uses a module tree, expose it through the conventional declaration from the crate root or parent module:

- `src/lib.rs` or existing parent module -> `mod parser;`

### Function Mapping

Retain function intent and migration order close to the original C code:

- `skip_struct` -> `skip_struct`
- `parse_typedef` -> `parse_typedef`
- `parse_dcl` -> `parse_dcl`
- `dcl` -> `dcl`
- `getident` -> `getident`
- `dirdcl` -> `dirdcl`
- `parmdcl` -> `parmdcl`
- `maybe_parm_list` -> `maybe_parm_list`
- `func_body` -> `func_body`
- `get_knr_args` -> `get_knr_args`
- `declare` -> `declare`
- `declare_type` -> `declare_type`
- `get_symbol` -> `get_symbol`
- `add_reference` -> `add_reference`
- `call` -> `call`

Where Rust naming or signature adjustments are needed, prefer changing signatures rather than splitting behavior into new helper subsystems. Small private helpers are acceptable only when required to replace unsafe C idioms or shared mutable global state.

## Data Model

The C analysis reports only anonymous data structures, so the Rust plan should reconstruct named internal types directly from actual usage in `src/parser.c` during migration.

### Data-Structure Mapping Strategy

Anonymous C structs/unions used in parser state, declarators, symbols, or references should be converted into named Rust types in `src/parser.rs`, limited to what the file already uses.

Recommended Rust representations:

- C parser state aggregates -> `struct Parser`
- C symbol records -> `struct Symbol`
- C declaration/type records -> `struct Decl` / `struct DeclType` or a single existing-equivalent struct if the C code uses one shared representation
- C parameter list aggregates -> `struct ParamDecl` or `Vec<ParamDecl>`
- C reference/call-site records -> `struct Reference`
- C tag/type category flags -> `enum` when the source uses a closed set of categories; otherwise integer constants may be preserved temporarily during initial migration
- C optional linked associations -> `Option<T>` / `Option<Box<T>>`
- C dynamic lists or expandable arrays -> `Vec<T>`
- C strings / identifier buffers -> `String`
- borrowed token text where possible -> `&str`

### Ownership and Memory Management

- Replace raw pointer ownership with owned Rust values and borrowing.
- Use `Vec` for collections previously managed through manual allocation.
- Use `Box<T>` only when recursive declarator/type structures require indirection.
- Avoid `Rc`, `Arc`, or interior mutability unless the original structure absolutely forces shared ownership; this is not expected from the given module scope.
- Eliminate manual frees by structuring parser state to own all temporary and persistent parse data naturally.

### Error Handling

- Replace sentinel returns and null-pointer failure signaling with `Option<T>` or `Result<T, ParseError>`.
- Use a small internal `ParseError` enum for syntax and state errors encountered by these functions.
- If the surrounding project already has an error type, map parser errors into that existing type instead of introducing a broader new error framework.

## Implementation Phases

### Phase 1: Establish Rust Module Skeleton and Core State

Create `src/parser.rs` and move over the parser-local state model before translating behavior in depth.

Tasks:
- define the Rust module entry points corresponding to the C functions
- identify all parser-global and file-static C state used by `src/parser.c`
- convert that state into explicit Rust structs and fields
- reconstruct named Rust types for the anonymous C data structures actually referenced in this file
- define minimal enums/flags/constants needed to preserve existing parser decisions
- introduce parser-local error/result types sized only for this module

Exit criteria:
- module compiles with placeholder function bodies
- all C-side parser state has a clear Rust owner
- no raw allocation model is carried forward as a design dependency

### Phase 2: Port Declarator and Type Parsing Flow

Translate the core declaration parsing path first, keeping function boundaries aligned with the C source.

Tasks:
- implement `getident`
- implement `dcl`
- implement `dirdcl`
- implement `parse_dcl`
- implement `declare_type`
- implement `declare`
- implement `parse_typedef`
- implement `skip_struct`
- implement `parmdcl`
- implement `maybe_parm_list`

Technical notes:
- preserve the original parse order and side effects
- convert buffer mutation and pointer walking into index-based or iterator-based token access
- use enums and explicit state transitions instead of C flag mutation where this improves safety without changing behavior
- keep recursion only where the original declarator grammar requires it

Exit criteria:
- declaration-related functions compile and operate over the Rust state model
- typedef, struct-skipping, and parameter/declarator paths are covered by unit tests based on observed C behavior

### Phase 3: Port Function-Body, K&R Arguments, and Symbol Interactions

Add the remaining parser behaviors that depend on declarations and symbol handling.

Tasks:
- implement `get_knr_args`
- implement `func_body`
- implement `get_symbol`
- implement `add_reference`
- implement `call`

Technical notes:
- map symbol lookup to a standard-library map or existing project symbol table structure, depending on how `src/parser.c` currently interacts with shared state
- preserve distinction between declaration-time symbol creation and reference-time lookup
- model call/reference recording with owned Rust records rather than pointer-linked nodes
- keep K&R argument handling strictly limited to existing source behavior

Exit criteria:
- function-body and call/reference paths compile
- symbol lookup and reference addition are validated with focused tests
- no unresolved dependence remains on C-style global mutation or null-pointer conventions

### Phase 4: Validation, Cleanup, and Behavioral Alignment

Finish migration by tightening signatures, reducing unsafe compatibility patterns, and validating behavior against the original file.

Tasks:
- compare each migrated Rust function against the original C control flow for edge-case parity
- remove temporary integer flags where a narrow enum is now safe and already justified by usage
- reduce unnecessary cloning in identifier and symbol handling
- normalize `Option`/`Result` returns to match actual failure modes
- add regression tests for mixed declaration/function-body scenarios that exercise the module end-to-end

Exit criteria:
- `cargo test` passes
- the Rust module fully replaces the behavior of `src/parser.c` within project expectations
- implementation remains contained to the migrated module and required existing crate wiring only