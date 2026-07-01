# Implementation Plan: module_src_parseopt_parseopt_03

## Summary

This module ports the option parsing logic from `src/parseopt/parseopt.c` into Rust while preserving the existing parsing flow, option lookup behavior, initialization/free lifecycle, and state-query functions. The Rust implementation should remain narrowly aligned with the current C module responsibilities: initialize parser state, resolve option groups and definitions, parse command-line inputs, expose parsed option values, and report whether specific options were set.

The implementation approach is a direct structural migration of the existing file into a Rust module with explicit ownership of parser state and borrowed access to input arguments where appropriate. C-style manual allocation and cleanup patterns will be replaced by Rust-owned containers and `Drop`-safe field composition, while still keeping function boundaries recognizable from the source module. Error-prone sentinel/null-based control flow should be translated into `Option` and `Result` without introducing new capabilities beyond what the current module already performs.

## Technical Context

### Language / Version
- Rust 1.78 or newer
- Edition: 2021

### Primary Dependencies
- Rust standard library only:
  - `std::env` or borrowed slices for argument handling
  - `std::ffi` only if required by surrounding code interfaces
  - `std::collections` only if the C lookup behavior benefits from direct keyed access; otherwise prefer slices/vectors to stay close to the source layout

No third-party crates are recommended based on the provided module evidence.

### Testing
- `cargo test`

Test scope should cover:
- parser initialization and reset behavior
- option lookup by code
- option lookup by name
- parse flow for valid and invalid inputs
- option state querying after parse
- cleanup-equivalent behavior through ownership drop and explicit free/reset compatibility if retained

### Performance Goals
- Preserve linear or near-linear parse behavior comparable to the C implementation
- Avoid unnecessary string cloning during option scanning where borrowed `&str` is sufficient
- Keep option definition lookup costs aligned with the source behavior unless the C code already implies repeated indexed searches that can be safely represented without semantic change
- Minimize allocations to parser-state construction and parsed-value storage only

## Module Mapping

### Source to Target Mapping
- C source file:
  - `src/parseopt/parseopt.c`

- Rust target module:
  - `src/parseopt.rs`

If the project already uses directory modules, an equivalent standard Rust layout is also acceptable:
- `src/parseopt/mod.rs`

The migration should keep all listed functions within this single Rust module rather than splitting into additional helper modules.

### Function Mapping
- `set_version`
  - Port as an internal helper function/method for updating parser version-related state
- `_parseopt_optgroup`
  - Port as a private helper for option-group resolution
- `parseopt_init0`
  - Port as an internal initializer that establishes zero/default state
- `parseopt_init`
  - Port as the public constructor-style initialization entry point
- `parseopt_free`
  - Port as an explicit reset/clear method only if required by current call sites; otherwise Rust drop semantics should absorb raw cleanup duties
- `parseopt_parse`
  - Port as the main parse routine operating on parser state and argument input
- `parseopt_getopt`
  - Port as an accessor for parsed option values/results
- `parseopt_optdef_by_code`
  - Port as a lookup helper by numeric/enum code
- `parseopt_optdef_by_name`
  - Port as a lookup helper by option name
- `parseopt_is_set`
  - Port as a state-query helper returning whether an option was encountered/set

## Data Model

Because the analysis exposes only anonymous C data structures, the Rust plan should derive named types directly from actual usage sites in `parseopt.c` during implementation rather than inventing unrelated abstractions. The target model should keep a close one-to-one mapping with the C state layout.

### Expected Structural Mapping
- Anonymous parser state struct
  - Rust: `struct ParseOpt`
  - Holds parser configuration, option definitions/groups references, parse results, flags, counters, and version/help metadata as applicable from the C file

- Anonymous option definition struct
  - Rust: `struct OptDef`
  - Holds option code, long/short name fields, argument-kind metadata, group/category links, and descriptive text if present in the C source

- Anonymous option group struct
  - Rust: `struct OptGroup`
  - Holds group identity and references/slices to associated option definitions if such grouping exists in the source

- Anonymous parsed option/result record
  - Rust: `struct ParsedOpt` or fields embedded into `ParseOpt`
  - Holds whether set, argument value, occurrence count, or positional association according to the C implementation

- Anonymous flags / mode fields
  - Rust: `enum` or integer-backed fields
  - Use `enum` where the C code clearly models distinct states; use primitive integer/bitflag storage only where exact masking behavior is already central to the implementation

### C-to-Rust Type Guidelines
- `char *` used for owned mutable strings
  - Rust: `String`
- `const char *` borrowed input/definition text
  - Rust: `&str` where lifetimes are manageable, otherwise `String`
- Null pointer optional references
  - Rust: `Option<T>` / `Option<&T>` / `Option<String>`
- Arrays of definitions
  - Rust: `Vec<OptDef>` for owned runtime data, or slices `&[OptDef]` for static definitions
- Integer option codes and flags
  - Rust: `i32`, `u32`, or small enums depending on actual C usage
- Manual allocation/free temporary buffers
  - Rust: `Vec<T>` / `String`

### Memory Management Decisions
- Replace explicit allocation and `parseopt_free` cleanup logic with owned fields inside `ParseOpt`
- Keep an explicit `free`/`clear`-style method only to preserve the original lifecycle shape for migrated callers
- Eliminate null-state hazards by requiring initialized parser state before parse operations
- Avoid interior raw pointers unless the original design cannot be expressed with references/slices due to self-referential layout; in that case, redesign field ownership rather than carrying C pointer topology across

### Error Handling Decisions
- Translate integer status returns into `Result<_, ParseOptError>` where externally visible behavior permits
- If existing surrounding interfaces require C-like return codes, use a small internal error enum plus compatibility conversion methods
- Convert missing lookup results to `Option`
- Distinguish initialization errors, invalid option syntax, unknown option references, and missing argument cases only if the C code already distinguishes them

## Implementation Phases

### Phase 1: Inventory and State Skeleton Port
- Inspect `src/parseopt/parseopt.c` and identify the concrete anonymous structs behind:
  - parser state
  - option definitions
  - option groups
  - parsed value storage
- Create the Rust module file and declare the minimal structs/enums required to represent the same state
- Port `parseopt_init0` into an internal default/reset path
- Port `parseopt_init` into the public state-construction entry point
- Add `set_version` as an internal helper with the same state effect as the C code
- Decide whether parser state owns option definitions or borrows them from caller/context based strictly on the C file’s current usage

### Phase 2: Lookup and Group Resolution
- Port `_parseopt_optgroup` as a private helper
- Port `parseopt_optdef_by_code`
- Port `parseopt_optdef_by_name`
- Preserve lookup precedence, name matching semantics, and failure behavior from the C code
- Add unit tests for:
  - successful code lookup
  - successful name lookup
  - missing option cases
  - group resolution behavior

### Phase 3: Parse Execution and Result Access
- Port `parseopt_parse` as the central parser routine
- Port `parseopt_getopt`
- Port `parseopt_is_set`
- Preserve argument consumption rules, option state updates, and repeated-option behavior exactly as implemented in the C source
- Use borrowed argument slices where possible to avoid unnecessary copies, promoting to owned strings only when the C logic stores values beyond immediate parsing
- Add unit tests for representative parse scenarios from the original behavior:
  - flags without values
  - options with required values
  - unknown options
  - missing values
  - state queries after parse

### Phase 4: Cleanup Semantics and Compatibility Validation
- Port `parseopt_free` as a clear/reset method if existing migrated callers need an explicit lifecycle endpoint
- Ensure any C cleanup side effects are represented through field clearing rather than manual deallocation
- Review all functions for:
  - removal of sentinel/null assumptions
  - consistent `Option`/`Result` usage
  - no dangling borrowed data retained beyond input lifetime
- Finalize regression tests around init → parse → query → clear/reinit flows
- Confirm the module remains confined to the functionality currently provided by `parseopt.c` and does not introduce extra parser features