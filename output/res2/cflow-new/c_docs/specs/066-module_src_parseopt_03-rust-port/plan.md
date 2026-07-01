# Implementation Plan: module_src_parseopt_03

## Summary

This module covers a focused portion of command-line option parsing currently implemented in `src/main.c`. The listed functions are option setter and hook routines that convert parsed option inputs into internal program configuration updates, plus user-facing version/help hooks and one environment-related error path.

The Rust implementation should migrate these routines as a tightly scoped option-handling unit without adding new behavior. The preferred approach is to extract the relevant logic from `src/main.c` into a small Rust module responsible for:

- receiving option values in already-parsed form,
- validating and normalizing those values,
- updating a shared runtime/options state structure,
- producing explicit `Result`-based errors instead of implicit C error signaling,
- preserving current side effects and ordering semantics.

Because the source module is only `src/main.c`, the migration plan should keep the Rust port minimal and direct: one Rust module for these setters/hooks and one state type that mirrors the C global/program state touched by the functions. Any anonymous C structures involved should be replaced with named Rust structs or enums only where needed to represent the accessed fields.

## Technical Context

### Language/Version
- Rust 1.78 or newer

### Primary Dependencies
Use the Rust standard library by default.

Recommended crates:
- None required for this module migration.

### Testing
- `cargo test`

Testing focus:
- per-function unit tests for each option setter,
- validation of state mutations after valid inputs,
- validation of error returns for invalid inputs,
- regression tests for help/version hook output control flow where practical.

### Performance Goals
- Match the C implementation’s effective runtime characteristics for command-line option handling.
- Avoid unnecessary heap allocation except where C behavior inherently builds owned strings or path lists.
- Keep option processing O(1) per setter call, except for list/path concatenation cases that are naturally O(n) in input length.

## Module Mapping

### C Source Mapping
- `src/main.c`
  - Rust target: `src/parseopt.rs` or `src/main.rs` internal module `parseopt`

Given the narrow migration scope and the fact that all functions originate from `src/main.c`, the least-expansive Rust layout is:

- `src/main.rs`
  - application entry and top-level argument dispatch
- `src/parseopt.rs`
  - migrated implementations of:
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

If the existing Rust port keeps all migrated logic in `src/main.rs`, these functions may first be added there and only split into `src/parseopt.rs` if needed to preserve readability. The migration should not introduce extra modules beyond this boundary.

### Function Mapping
| C Function | Rust Target | Notes |
|---|---|---|
| `optset_include_classes` | `fn optset_include_classes(...) -> Result<(), OptError>` | Parse/update include-class selection state |
| `optset_output_driver` | `fn optset_output_driver(...) -> Result<(), OptError>` | Map driver name/value to internal enum/string field |
| `optset_xref` | `fn optset_xref(...) -> Result<(), OptError>` | Toggle/update cross-reference mode |
| `optset_symbol` | `fn optset_symbol(...) -> Result<(), OptError>` | Update symbol-related option state |
| `optset_preproc_option` | `fn optset_preproc_option(...) -> Result<(), OptError>` | Append/store preprocessor option |
| `optset_preprocess` | `fn optset_preprocess(...) -> Result<(), OptError>` | Toggle preprocessing mode/settings |
| `optset_level_indent` | `fn optset_level_indent(...) -> Result<(), OptError>` | Parse integer indentation level |
| `optset_main_symbol` | `fn optset_main_symbol(...) -> Result<(), OptError>` | Set main symbol string |
| `optset_clear_main_symbol` | `fn optset_clear_main_symbol(...) -> Result<(), OptError>` | Clear main symbol state |
| `optset_install_target` | `fn optset_install_target(...) -> Result<(), OptError>` | Set install/target related state |
| `optset_int_1` | `fn optset_int_1(...) -> Result<(), OptError>` | Shared integer setter helper with fixed value semantics |
| `optset_prepend_path` | `fn optset_prepend_path(...) -> Result<(), OptError>` | Prepend path into search path list/string |
| `version_hook` | `fn version_hook(...) -> Result<HookAction, OptError>` | Trigger version output and stop/continue as current behavior requires |
| `help_hook` | `fn help_hook(...) -> Result<HookAction, OptError>` | Trigger help output and stop/continue as current behavior requires |
| `po_env_error` | `fn po_env_error(...) -> OptError` or `fn po_env_error(...) -> Result<(), OptError>` | Convert environment-related failure to explicit error |

## Data Model

The analysis only reports anonymous C data structures, so the Rust plan should derive named types from actual field usage during migration rather than attempting a broad one-to-one struct recreation.

### Data-Structure Mapping Strategy

| C Representation | Rust Representation | Notes |
|---|---|---|
| anonymous option context struct | `struct ParseOptState` | Central mutable state passed to setter functions |
| anonymous program/global config struct | `struct ProgramOptions` | Holds option values updated by this module |
| anonymous string pointer fields (`char *`) | `Option<String>` or `String` | `Option<String>` when nullable/clearable in C |
| anonymous boolean/integer flags (`int`) | `bool` or `i32`/`u32` | Use `bool` only when semantics are binary |
| anonymous enum-like integer selectors | `enum OutputDriver` / `enum XrefMode` / `enum PreprocessMode` or `String` | Prefer enum only if C uses a closed set visible in these functions |
| anonymous list of options/paths | `Vec<String>` | Preserves ordered accumulation |
| anonymous callback result conventions | `enum HookAction { Continue, ExitSuccess, ExitFailure }` | Only if hooks need to signal control flow distinctly |
| C error signaling via return code / stderr side effect | `Result<(), OptError>` | Centralized explicit error propagation |

### Proposed Core Rust Types

```rust
struct ParseOptState {
    options: ProgramOptions,
}

struct ProgramOptions {
    include_classes: Option<String>,
    output_driver: Option<OutputDriver>,
    xref: bool,
    symbol: Option<String>,
    preproc_options: Vec<String>,
    preprocess: bool,
    level_indent: Option<i32>,
    main_symbol: Option<String>,
    install_target: Option<String>,
    search_paths: Vec<String>,
}

enum OutputDriver {
    Named(String),
}

enum HookAction {
    Continue,
    ExitSuccess,
    ExitFailure,
}

enum OptError {
    InvalidValue { option: &'static str, value: String },
    MissingValue { option: &'static str },
    EnvironmentError(String),
}
```

This is intentionally conservative. During implementation, fields should be reduced or adjusted to exactly match the data touched by the migrated functions. If the existing Rust port already defines a broader application state, these fields should be integrated into that existing struct rather than duplicated.

### Memory Management
- Replace C-owned mutable strings with `String` and `Option<String>`.
- Replace manual list growth and pointer ownership with `Vec<String>`.
- Eliminate lifetime ambiguity by making setter functions take owned or borrowed `&str` input and clone only when state must retain values.
- Avoid shared mutable globals; use `&mut ParseOptState` or the existing mutable application state object.

### Error Handling
- Replace C integer return/error conventions with `Result`.
- Convert parse failures, invalid enumerated values, and environment-related issues into `OptError`.
- Keep user-visible output behavior for help/version intact, but avoid direct process termination inside low-level setters unless the existing Rust entry flow requires it. Prefer returning a `HookAction` to the caller.

## Implementation Phases

## Phase 1: Extract State and Port Simple Setters

### Goal
Create the Rust state representation and migrate the straightforward option setters that map directly to booleans, strings, or fixed integer values.

### Scope
- Define `ParseOptState`/`ProgramOptions` in the destination Rust file.
- Port:
  - `optset_xref`
  - `optset_symbol`
  - `optset_preprocess`
  - `optset_main_symbol`
  - `optset_clear_main_symbol`
  - `optset_install_target`
  - `optset_int_1`

### Technical Notes
- Preserve exact defaulting and overwrite behavior from C.
- For functions that used nullable C strings, use `Option<String>`.
- For helper-style setters like `optset_int_1`, keep the semantics narrow and avoid generalizing beyond the original use.

### Exit Criteria
- State compiles and supports mutation through these functions.
- Unit tests confirm expected value changes and clear/reset behavior.

## Phase 2: Port Validating and Accumulating Option Setters

### Goal
Migrate setters that parse values, append repeated options, or manipulate path-like state.

### Scope
- Port:
  - `optset_include_classes`
  - `optset_output_driver`
  - `optset_preproc_option`
  - `optset_level_indent`
  - `optset_prepend_path`

### Technical Notes
- Introduce minimal validation logic matching C behavior.
- Use `Vec<String>` for repeated preprocessor options and prepended path entries unless the existing Rust code requires a single concatenated string. If output later expects a joined path string, defer joining until consumption time.
- `optset_level_indent` should parse to a concrete integer type and reject malformed input through `OptError`.
- If `optset_output_driver` accepts a closed set of names, encode as an enum; otherwise retain a string-backed representation.

### Exit Criteria
- All value-parsing and accumulation behavior is migrated.
- Tests cover invalid numeric input, invalid driver values if applicable, and ordering of prepended/appended entries.

## Phase 3: Port Hooks and Environment Error Path

### Goal
Migrate control-flow-affecting hooks and finalize error handling integration.

### Scope
- Port:
  - `version_hook`
  - `help_hook`
  - `po_env_error`

### Technical Notes
- Keep output formatting and termination semantics aligned with the C behavior.
- Prefer returning `HookAction` to the caller rather than exiting directly in the hook function, unless the surrounding Rust main flow already centralizes exit handling differently.
- `po_env_error` should become a small constructor or helper that produces a stable, testable error value and any required message text.

### Exit Criteria
- Help/version flows are reachable through Rust option handling.
- Environment-related failure path is explicit and testable.
- No remaining direct dependency on the original C implementations for this module scope.

## Phase 4: Integrate with Main Argument Processing

### Goal
Wire the migrated functions into the active Rust command-line processing path and remove the corresponding C-side dependency for this slice.

### Scope
- Connect the new Rust setters/hooks to the existing option dispatch in `src/main.rs`.
- Ensure the module uses the existing application state if one already exists.
- Remove duplicated transitional logic for the migrated functions.

### Technical Notes
- Keep integration local to current argument parsing flow; do not redesign parsing architecture.
- Preserve call order and last-write-wins semantics where repeated options are accepted.
- Confirm that all migrated functions operate through the same mutable state object.

### Exit Criteria
- The Rust branch handles this option-setter subset end-to-end.
- `cargo test` passes for module-level and integration-level cases relevant to these functions.
- The implementation remains confined to the original module scope without adding unevidenced features.