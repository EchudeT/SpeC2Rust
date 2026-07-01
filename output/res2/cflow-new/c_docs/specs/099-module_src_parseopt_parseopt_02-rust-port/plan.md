# Implementation Plan: module_src_parseopt_parseopt_02

## Summary

This module ports the option parsing logic from `src/parseopt/parseopt.c` into Rust, preserving the existing parsing behavior and internal processing order rather than redesigning the API. The Rust implementation should remain centered on argument-vector traversal, short/long option lookup, option definition preparation, permutation of positional arguments, lookahead/skip behavior, and generation of usage/help-related metadata.

The implementation approach is to migrate the C module into a single Rust module with closely corresponding internal functions and compact supporting data types. Parsing state that is implicit in C pointer/index manipulation should be made explicit through Rust structs and slice/index fields. String handling should move from raw C strings to `String`/`&str` where owned text is required, and argument iteration should use `Vec<String>` or borrowed slices. Error reporting should use Rust enums/results internally while preserving the original module’s externally observable outcomes.

## Technical Context

### Language/Version
- Rust 1.78 or newer

### Primary Dependencies
- Rust standard library only
- No third-party crates are recommended at this stage because the input indicates a self-contained parser module and does not provide evidence for external dependency needs

### Testing
- `cargo test`

### Performance Goals
- Preserve linear traversal behavior over the argument vector for normal parsing paths
- Avoid unnecessary string allocation during option matching by preferring borrowed `&str` slices where possible
- Keep option lookup and permutation behavior close to the C implementation’s asymptotic cost
- Ensure no avoidable cloning of the full argv buffer during parse steps other than when permutation semantics require reordering

## Module Mapping

### Source Mapping
- C source: `src/parseopt/parseopt.c`
- Rust target: `src/parseopt.rs`

### Function Mapping
The Rust module should migrate the existing functions with near-1:1 responsibility mapping:

| C Function | Rust Target | Notes |
|---|---|---|
| `option_find_short` | `fn option_find_short(...)` | Internal helper for short option lookup |
| `negmatch` | `fn negmatch(...)` | Internal helper for negative/prefix matching logic |
| `option_find_long` | `fn option_find_long(...)` | Internal helper for long option lookup |
| `permute` | `fn permute(...)` | Internal helper for argv reordering |
| `parseopt_lookahead` | `fn parseopt_lookahead(...)` | Internal parser-state helper |
| `parseopt_skip` | `fn parseopt_skip(...)` | Internal parser-state helper |
| `parseopt_next_internal` | `fn parseopt_next_internal(...)` | Core parsing routine |
| `parseopt_next` | `pub fn parseopt_next(...)` or method | Public stepping entry point matching current module boundary |
| `parseopt_argv` | `pub fn parseopt_argv(...)` or method | Public argv parsing entry point |
| `parseopt_error` | `fn parseopt_error(...)` | Internal error construction/reporting helper |
| `optidx_slot` | `fn optidx_slot(...)` | Internal helper for option index bookkeeping |
| `collect_optdef` | `fn collect_optdef(...)` | Internal option definition collection |
| `prepare_optdef` | `fn prepare_optdef(...)` | Internal preparation/normalization pass |
| `set_help` | `fn set_help(...)` | Internal metadata assignment |
| `set_usage` | `fn set_usage(...)` | Internal metadata assignment |

### Rust Organization
Keep the port within one Rust module file to reflect the original C compilation unit:
- `src/parseopt.rs`

If the current crate already uses a nested module tree, expose it using the smallest conventional adaptation only:
- `src/lib.rs` or existing parent module: `mod parseopt;`

## Data Model

The analysis only identifies multiple anonymous C data structures, so the Rust plan should derive concrete types directly from usage in `parseopt.c` during migration. The mapping should remain minimal and local to this module.

### Data-Structure Mapping Strategy

| C Shape | Rust Mapping | Purpose |
|---|---|---|
| Anonymous parser state struct | `struct ParseOptState` | Holds argv slice/view, current index, option/argument parsing position, skip/lookahead/permutation state |
| Anonymous option definition struct | `struct OptDef` | Represents one short/long option definition and its metadata |
| Anonymous parse result/event struct | `struct ParseEvent` or equivalent | Represents the next parsed option/argument outcome if the C code uses a result carrier |
| Anonymous help/usage descriptor fields | Fields on `OptDef` or a compact `struct HelpText` | Stores usage/help strings without expanding feature scope |
| Anonymous option-index bookkeeping struct/array entry | `struct OptIndexEntry` or `Vec<Option<usize>>` | Supports `optidx_slot` / prepared lookup tables |
| Anonymous error/status values | `enum ParseOptError` and/or `enum ParseStatus` | Replaces integer error codes and sentinel returns |

### Concrete Rust Type Decisions
- C strings:
  - `const char *` inputs -> `&str` when borrowed
  - owned mutable textual fields -> `String`
- C arrays:
  - argv-style mutable arrays -> `Vec<String>` when reordering is required
  - borrowed read-only argv -> `&[String]` or `&[&str]` depending on surrounding crate usage
- C indexes/counters:
  - use `usize` for collection indexing
  - use signed integers only where the C logic depends on negative sentinels
- C booleans/flags:
  - use `bool`
- C nullable pointers:
  - use `Option<T>` / `Option<&T>` / `Option<usize>`

### Ownership and Memory Management
- Replace pointer arithmetic with explicit indices into slices or vectors
- Encapsulate mutable parser progress in `ParseOptState`
- Avoid self-referential storage; store indexes into argv/definition collections instead of references when lifetimes would become awkward
- Keep help/usage text ownership straightforward with `String` if text is synthesized or normalized during preparation
- Use borrowed `&str` only for transient matching during parse operations

### Error Handling
- Convert integer/sentinel error signaling into:
  - internal `Result<T, ParseOptError>` where failure should short-circuit, or
  - internal status enums where the original function distinguishes “no match”, “need argument”, “done”, and “error”
- Preserve the original outward control flow by translating Rust errors/statuses back into the public API shape required by the crate
- `parseopt_error` should become the single place that formats or records parser errors to avoid scattering message construction

## Implementation Phases

### Phase 1: Port core types and option definition preparation
- Create `src/parseopt.rs`
- Read `parseopt.c` and identify each anonymous struct’s actual role from field usage
- Define minimal Rust structs/enums for:
  - parser state
  - option definitions
  - option index bookkeeping
  - parse status/error values
- Port:
  - `optidx_slot`
  - `collect_optdef`
  - `prepare_optdef`
  - `set_help`
  - `set_usage`
- Establish tests for:
  - option definition normalization
  - help/usage field propagation
  - index slot behavior and duplicate/lookup preparation rules inferred from C behavior

### Phase 2: Port lookup and traversal helpers
- Port internal matching helpers:
  - `option_find_short`
  - `negmatch`
  - `option_find_long`
- Port parser movement helpers:
  - `parseopt_lookahead`
  - `parseopt_skip`
  - `permute`
- Replace C pointer walking with explicit index arithmetic and string slicing
- Add focused tests for:
  - short option resolution
  - long option resolution
  - ambiguous/negative match behavior
  - positional argument permutation semantics

### Phase 3: Port main parse execution path
- Port:
  - `parseopt_next_internal`
  - `parseopt_error`
  - `parseopt_next`
  - `parseopt_argv`
- Keep the Rust control flow close to the C branch structure so behavior remains comparable
- Route all parse-step outcomes through the Rust status/error types designed earlier
- Ensure end-of-input, missing-argument, unknown-option, and skip/lookahead cases match the original module’s behavior

### Phase 4: Stabilization and compatibility verification
- Compare Rust behavior against the original C implementation using module-level test cases derived from existing call patterns
- Verify:
  - argv consumption order
  - permutation results
  - help/usage preparation behavior
  - error text or error classification consistency as applicable
- Refine signatures only as needed to fit the existing Rust crate interfaces without expanding the module’s feature set

## Notes and Constraints

- Keep the migration constrained to the behavior present in `src/parseopt/parseopt.c`
- Do not split the port into extra helper modules unless required by the existing crate layout
- Prefer straightforward imperative Rust over abstraction-heavy rewrites so the mapping back to the C logic remains auditable
- Where the C code relies on mutation of argv contents, use `Vec<String>` or a mutable vector-like representation directly rather than introducing alternative parser designs