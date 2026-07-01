# Implementation Plan: module_src_parseopt_03

## Summary

This module covers a focused subset of command-line option parsing currently implemented in `src/main.c`. The Rust port should migrate the option-setting functions and hooks that update process-wide parsing state for include-class selection, output driver choice, cross-reference behavior, symbol configuration, preprocessing options, indentation level, install target, path prepending, and help/version/error reporting.

The implementation approach should remain narrow: extract the relevant parsing state and handlers from the C file into a Rust module that preserves existing behavior and call order, while replacing C-style mutable global/string handling with explicit Rust-owned state and typed helper functions. The work should favor direct translation of existing routines over redesign. Memory ownership should move from implicit C allocation and borrowed string pointers to `String`, `PathBuf`, `Vec<String>`, and small enums where the C code currently uses textual or integer flags. Error handling should convert invalid option values and environment-related failures into explicit `Result` values or process-exit paths at the command-dispatch boundary, matching current observable behavior without introducing new abstractions.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain effectively equivalent startup/runtime cost for command-line parsing compared with the C implementation.
  - Avoid unnecessary string cloning beyond what is required to own parsed option values safely.
  - Keep option-handler execution O(1) per option, except for expected linear work when appending/prepending to argument/path collections.

## Module Mapping

### C Source to Rust Module

- **C file**: `src/main.c`
- **Rust target**: `src/main.rs` with a tightly scoped internal module or adjacent functions dedicated to this migrated option subset

Because the source functions originate from `src/main.c`, the Rust port should keep the migrated code close to the main argument-processing path rather than splitting it into extra architectural layers. A practical mapping is:

- `src/main.rs`
  - `ParseOptState` struct holding mutable option state formerly stored in C globals or shared mutable variables
  - direct Rust equivalents for:
    - `optset_include_classes`
    - `optset_output_driver`
    - `optset_xref`
    - `optset_symbol`
    - `optset_preproc_option`
    - `optset_preprocess`
    - `optset_level_indent`
    - `optset_main_symbol`
    - `optset_clear_main_symbol`
    - `optset_install_target`
    - `optset_int_1`
    - `optset_prepend_path`
    - `version_hook`
    - `help_hook`
    - `po_env_error`

### Function Mapping

| C Function | Rust Mapping | Notes |
|---|---|---|
| `optset_include_classes` | `fn optset_include_classes(state: &mut ParseOptState, value: &str) -> Result<(), OptError>` | Parse string option into owned state; preserve existing class-selection semantics. |
| `optset_output_driver` | `fn optset_output_driver(state: &mut ParseOptState, value: &str) -> Result<(), OptError>` | Prefer enum if C uses fixed known driver names; otherwise `String` with validation. |
| `optset_xref` | `fn optset_xref(state: &mut ParseOptState, enabled: bool) -> Result<(), OptError>` | Boolean/flag setter. |
| `optset_symbol` | `fn optset_symbol(state: &mut ParseOptState, value: &str) -> Result<(), OptError>` | Store or append symbol-related option according to C behavior. |
| `optset_preproc_option` | `fn optset_preproc_option(state: &mut ParseOptState, value: &str) -> Result<(), OptError>` | Preserve ordering for repeated options. |
| `optset_preprocess` | `fn optset_preprocess(state: &mut ParseOptState, mode: bool \| PreprocessMode) -> Result<(), OptError>` | Final type depends on whether C treats it as boolean or multi-state. |
| `optset_level_indent` | `fn optset_level_indent(state: &mut ParseOptState, value: &str) -> Result<(), OptError>` | Parse bounded integer/format value exactly as C does. |
| `optset_main_symbol` | `fn optset_main_symbol(state: &mut ParseOptState, value: &str) -> Result<(), OptError>` | Store designated main symbol. |
| `optset_clear_main_symbol` | `fn optset_clear_main_symbol(state: &mut ParseOptState)` | Clear optional main symbol. |
| `optset_install_target` | `fn optset_install_target(state: &mut ParseOptState, value: &str) -> Result<(), OptError>` | Store validated install target setting. |
| `optset_int_1` | `fn optset_int_1(target: &mut i32)` or integrated into state setters | Preserve exact “set integer to 1” side effect. |
| `optset_prepend_path` | `fn optset_prepend_path(state: &mut ParseOptState, value: &str) -> Result<(), OptError>` | Use owned path/string collection; preserve prepend order. |
| `version_hook` | `fn version_hook(...) -> !` or `fn version_hook(...)` | Print version text and exit/return according to current dispatch flow. |
| `help_hook` | `fn help_hook(...) -> !` or `fn help_hook(...)` | Print help text and exit/return according to current dispatch flow. |
| `po_env_error` | `fn po_env_error(var_name: &str, detail: &str) -> OptError` or print-and-exit helper | Convert environment parse/report path into explicit Rust error handling. |

## Data Model

The input only exposes anonymous C data structures, so the Rust plan should derive data mappings from usage in these functions rather than recreate unnamed C layout mechanically.

### State Consolidation

Unnamed C globals, static flags, and ad hoc shared fields used by these option handlers should be consolidated into one Rust state holder local to argument processing:

```rust
struct ParseOptState {
    include_classes: Option<String>,
    output_driver: Option<OutputDriver>,
    xref_enabled: bool,
    symbols: Vec<String>,
    preproc_options: Vec<String>,
    preprocess: bool, // or PreprocessMode if required by observed C logic
    level_indent: Option<u32>, // adjust signedness after source confirmation
    main_symbol: Option<String>,
    install_target: Option<String>,
    prepended_paths: Vec<std::path::PathBuf>, // or Vec<String> if not path-semantic in C
    // additional integer flags migrated directly from C usage
}
```

### C-to-Rust Type Mapping

| C Pattern | Rust Type | Rationale |
|---|---|---|
| `char *` option value | `String` | Safe ownership of parsed values. |
| borrowed C string input | `&str` | Handler inputs should avoid extra allocation until storing. |
| repeated option list | `Vec<String>` | Matches append semantics and preserves order. |
| path-like string list | `Vec<PathBuf>` or `Vec<String>` | Use `PathBuf` only if the C logic treats them as filesystem paths rather than generic tokens. |
| integer flag (`int`, `bool`-like) | `bool` or `i32` | Prefer `bool` when semantics are binary; retain integer only where values matter beyond truthiness. |
| optional configured string | `Option<String>` | Replaces null-pointer state. |
| fixed textual mode/driver | `enum` | Use only when the C code clearly restricts values to a finite set. |

### Supporting Enums and Errors

```rust
enum OutputDriver {
    // variants determined from existing accepted C values
}

enum OptError {
    InvalidValue { option: &'static str, value: String },
    MissingValue { option: &'static str },
    EnvError { var_name: String, detail: String },
}
```

Use enums sparingly and only where existing C behavior already defines a closed set of accepted values.

### Memory Management Notes

- Replace null checks with `Option`.
- Replace in-place C string mutation/assignment with owned replacement in `String` fields.
- For repeated options, preserve insertion order exactly; if C prepends, use front insertion or accumulate then reverse only if behavior is unchanged.
- Avoid retaining references into argument buffers after parsing; all persisted values must be owned.

### Error Handling Notes

- Parsing helpers should return `Result<(), OptError>`.
- Help/version hooks may remain process-terminating at the top-level command path if that matches current behavior.
- Environment-related reporting from `po_env_error` should not panic; convert to a structured error or one-shot printed failure consistent with the existing CLI flow.

## Implementation Phases

## Phase 1: Extract and Define Parsing State

- Inspect `src/main.c` usages of the listed functions and identify the exact mutable fields each one reads or writes.
- Introduce `ParseOptState` in `src/main.rs` containing only the state required by this function set.
- Replace C null/integer sentinel patterns with `Option<T>` and `bool` where behavior is strictly equivalent.
- Define a minimal `OptError` for invalid option values and environment-related failures.
- Establish direct function signatures in Rust for all listed handlers without changing the surrounding option-dispatch order.

## Phase 2: Port Option Setter Functions

- Implement the setter functions in migration order from simplest side-effect handlers to string/value parsers:
  - `optset_int_1`
  - `optset_xref`
  - `optset_preprocess`
  - `optset_clear_main_symbol`
  - `optset_main_symbol`
  - `optset_symbol`
  - `optset_preproc_option`
  - `optset_prepend_path`
  - `optset_include_classes`
  - `optset_output_driver`
  - `optset_level_indent`
  - `optset_install_target`
- Preserve current validation rules exactly as found in C, including accepted textual values, overwrite-vs-append behavior, and any ordering requirements.
- Keep helper logic local to `src/main.rs`; do not introduce extra crates or generalized parser frameworks.

## Phase 3: Port Hooks and Integrate with Dispatch

- Implement `version_hook` and `help_hook` with output matching current CLI behavior and integrate them into the Rust argument-processing path.
- Implement `po_env_error` as the Rust-side environment error reporting path used by this option subset.
- Wire all migrated handlers into the existing Rust branch’s main option dispatch, replacing the corresponding C-derived logic for this subset only.
- Confirm that exit behavior for help/version/error conditions matches current expectations.

## Phase 4: Validation and Behavioral Tests

- Add focused `cargo test` coverage for:
  - valid and invalid option values for each parser
  - overwrite/clear semantics for `main_symbol` and similar fields
  - accumulation order for symbol/preprocessor/path options
  - integer/boolean flag updates
  - environment-error formatting or propagation from `po_env_error`
- Add lightweight integration-style tests for help/version invocation if the current Rust structure allows capturing output deterministically.
- Compare results against the C behavior for representative argument combinations drawn from the migrated functions only.