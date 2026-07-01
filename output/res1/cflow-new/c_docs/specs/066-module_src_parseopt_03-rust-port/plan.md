# Implementation Plan: module_src_parseopt_03

## Summary

This module ports the option-setting portion currently implemented in `src/main.c` into Rust, limited to the functions listed for `module_src_parseopt_03`. The Rust implementation should preserve existing behavior for command-line option side effects, configuration mutation, and user-facing help/version/error hooks without introducing new option-processing features.

The implementation approach is to migrate the C logic into a focused Rust module that:
- centralizes mutable program option state in Rust structs,
- rewrites each `optset_*` routine as a small Rust function operating on that state,
- replaces C string and pointer handling with owned or borrowed Rust string types,
- converts integer/status return conventions into `Result` or explicit status enums where needed,
- keeps output behavior compatible for help/version/error reporting.

Because all listed functions currently reside in `src/main.c`, the migration should first isolate the corresponding logic into a Rust module with a narrow API rather than re-architecting unrelated startup code. The plan should preserve existing call ordering and mutation semantics from the C implementation.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain command-line parsing and option mutation cost at effectively constant overhead relative to the C implementation for typical invocation sizes.
  - Avoid unnecessary heap allocation beyond required `String`/`PathBuf` ownership conversions.
  - Preserve linear processing over user-supplied options and arguments.
  - Keep formatting/output hooks simple and allocation-light.

## Module Mapping

### C to Rust File Mapping

| C File | Rust File | Notes |
|---|---|---|
| `src/main.c` | `src/main.rs` or `src/parseopt.rs` | Keep the migrated functions together in one Rust module. If main entry logic already exists elsewhere in Rust, place only this module’s functions in `src/parseopt.rs` and call from `main.rs`. |

### Function Mapping

| C Function | Rust Function | Responsibility |
|---|---|---|
| `optset_include_classes` | `fn optset_include_classes(...) -> Result<(), OptError>` | Update include/class-selection option state. |
| `optset_output_driver` | `fn optset_output_driver(...) -> Result<(), OptError>` | Resolve and store output driver selection. |
| `optset_xref` | `fn optset_xref(...) -> Result<(), OptError>` | Apply cross-reference related option state. |
| `optset_symbol` | `fn optset_symbol(...) -> Result<(), OptError>` | Store symbol-related option input. |
| `optset_preproc_option` | `fn optset_preproc_option(...) -> Result<(), OptError>` | Append/store preprocessor option text. |
| `optset_preprocess` | `fn optset_preprocess(...) -> Result<(), OptError>` | Toggle or configure preprocessing mode. |
| `optset_level_indent` | `fn optset_level_indent(...) -> Result<(), OptError>` | Parse and store indentation level setting. |
| `optset_main_symbol` | `fn optset_main_symbol(...) -> Result<(), OptError>` | Set main symbol selection/filter. |
| `optset_clear_main_symbol` | `fn optset_clear_main_symbol(...)` | Clear main symbol state. |
| `optset_install_target` | `fn optset_install_target(...) -> Result<(), OptError>` | Set install target or destination-related option state. |
| `optset_int_1` | `fn optset_int_1(...) -> Result<(), OptError>` | Parse/store integer option with current C semantics. |
| `optset_prepend_path` | `fn optset_prepend_path(...) -> Result<(), OptError>` | Prepend path data into search-path/config state. |
| `version_hook` | `fn version_hook(...) -> !` or `fn version_hook(...) -> ExitCode` | Emit version text and terminate or signal early exit, matching surrounding control flow. |
| `help_hook` | `fn help_hook(...) -> !` or `fn help_hook(...) -> ExitCode` | Emit help text and terminate or signal early exit. |
| `po_env_error` | `fn po_env_error(...) -> OptError` or `fn po_env_error(...) -> !` | Produce environment-related option error with equivalent user-visible text. |

### Recommended Rust Module Layout

Use the smallest standard layout that supports migration:

```text
src/
  main.rs
  parseopt.rs   # optional, only if separating these functions from main improves direct mapping
```

If the branch already contains a Rust `main.rs`, place the migrated option-set functions into `src/parseopt.rs` and expose only the minimal functions/types required by `main.rs`. Otherwise keep them in `main.rs` to match the original file boundary closely.

## Data Model

The input only identifies anonymous C data structures, so the Rust plan should derive concrete Rust types directly from actual field usage in the listed functions rather than recreating opaque one-to-one placeholders.

### Core Type Mapping Strategy

| C Pattern | Rust Mapping | Notes |
|---|---|---|
| anonymous option/config struct | `struct ParseOptionsState` | Central mutable state touched by `optset_*` functions. Consolidate only fields actually used by the migrated functions. |
| anonymous parser context struct | `struct ParseContext` | Use only if the C functions require shared access to process-wide state beyond option values. |
| anonymous callback payload/config | `struct HookContext` | Only introduce if `help_hook`/`version_hook` need borrowed metadata. |
| integer flags | `bool`, `u8`, or small enums | Prefer `bool` for toggles, enums for named modes, integers only when semantics are numeric. |
| C string pointers (`char *`, `const char *`) | `String`, `&str`, `Option<String>` | Use owned `String` when values persist in state; borrowed `&str` for temporary inputs. |
| path strings | `PathBuf` or `Vec<PathBuf>` | Use only where path semantics are explicit, such as prepend/install target behavior. |
| linked list of strings/options | `Vec<String>` | Prefer contiguous ownership unless C logic depends on node identity. |
| function status `int` | `Result<(), OptError>` or `Result<T, OptError>` | Preserve failure points and message content while removing sentinel return values. |
| process exit in hook | `ExitCode` or diverging function `!` | Choose based on how surrounding control flow is migrated. |

### Proposed Rust Structures

These names are intentionally restrained and should be adjusted only to match actual C field usage.

```rust
struct ParseOptionsState {
    include_classes: Option<String>,
    output_driver: Option<OutputDriver>,
    xref_enabled: bool,
    symbol: Option<String>,
    preproc_options: Vec<String>,
    preprocess: PreprocessMode,
    level_indent: Option<u32>,
    main_symbol: Option<String>,
    install_target: Option<std::path::PathBuf>,
    int_1_value: Option<i32>,
    search_paths: Vec<std::path::PathBuf>,
}
```

```rust
enum OutputDriver {
    Named(String),
}
```

```rust
enum PreprocessMode {
    Disabled,
    Enabled,
    Custom(String),
}
```

```rust
enum OptError {
    InvalidValue { option: &'static str, value: String },
    MissingValue { option: &'static str },
    EnvironmentError(String),
    Message(String),
}
```

These are starting shapes only. During migration, fields should be trimmed or specialized to reflect actual semantics in `src/main.c`. For example:
- if `output_driver` maps to a fixed set of names, replace `Named(String)` with explicit variants;
- if `xref` is tri-state rather than boolean, use an enum;
- if `optset_int_1` writes directly into an external integer slot, model that slot explicitly in `ParseOptionsState`.

### Memory Management Decisions

- Replace all C-owned mutable string buffers with `String`.
- Replace manually managed path or option string chains with `Vec<String>` or `Vec<PathBuf>`.
- Avoid shared mutable aliasing from the C design; pass `&mut ParseOptionsState` to setters.
- Use `Option<T>` instead of null pointers for unset optional values.
- Keep borrowed lifetimes local to parsing calls; store owned values in state when they outlive the call.

### Error Handling Decisions

- Convert parse/validation failures into `OptError`.
- Keep user-visible wording close to current behavior, especially for `po_env_error`, `help_hook`, and `version_hook`.
- Use `Result` propagation instead of integer error codes.
- Only terminate the process in hooks if the migrated caller structure still expects direct exit there; otherwise return a distinct early-exit signal.

## Implementation Phases

## Phase 1: Extract and Model Option State

**Goal**: Create the Rust state and error types needed to host the migrated logic from `src/main.c`.

### Tasks
- Inspect the listed functions in `src/main.c` and identify every field they read or write.
- Define `ParseOptionsState` with only those fields.
- Define minimal enums for option modes that are currently expressed as integer constants or strings.
- Define `OptError` covering invalid value, missing value, and environment-related failures.
- Decide whether hooks return `ExitCode`, `Result`, or diverge based on existing Rust main/control-flow shape.

### Deliverables
- Rust module file created or updated for the migrated functions.
- Initial state structs/enums compiled.
- Unit tests for basic state defaults and error formatting.

### Migration Notes
- Do not migrate unrelated startup logic from `src/main.c`.
- Do not invent generalized parser abstractions; model only the state these functions touch.

## Phase 2: Port Setter Functions

**Goal**: Migrate the `optset_*` functions into direct Rust equivalents with preserved mutation semantics.

### Tasks
- Port `optset_include_classes`.
- Port `optset_output_driver`.
- Port `optset_xref`.
- Port `optset_symbol`.
- Port `optset_preproc_option`.
- Port `optset_preprocess`.
- Port `optset_level_indent`.
- Port `optset_main_symbol`.
- Port `optset_clear_main_symbol`.
- Port `optset_install_target`.
- Port `optset_int_1`.
- Port `optset_prepend_path`.

### Implementation Rules
- Each function should take explicit typed inputs rather than raw pointers.
- Preserve existing validation and overwrite/append behavior exactly.
- Use `String::from`, `parse::<i32>()`, `parse::<u32>()`, and `PathBuf::from` where applicable.
- Preserve order-sensitive behavior for repeated options, especially for prepend-path and repeated preprocessor options.
- Where the C code mutates an externally provided destination, express that as `&mut ParseOptionsState` or `&mut` field arguments rather than shared globals.

### Deliverables
- All setter functions compile and are wired to the Rust option-processing call sites.
- Unit tests for success/failure cases per setter, especially:
  - invalid integers,
  - empty/missing values,
  - repeated option behavior,
  - clear-after-set behavior for main symbol,
  - path prepend ordering.

## Phase 3: Port Output and Error Hooks

**Goal**: Migrate user-visible hook behavior and environment-related error creation.

### Tasks
- Port `version_hook`.
- Port `help_hook`.
- Port `po_env_error`.
- Match current output stream usage (`stdout` for normal help/version, `stderr` for errors if that is what C does).
- Preserve current termination or early-return behavior.

### Deliverables
- Hook functions callable from the Rust entry path.
- Snapshot-style or string comparison tests for produced output text where practical.
- Tests confirming error-path behavior for environment-related failures.

### Migration Notes
- Keep hook text sourcing simple; use static strings or directly formatted output matching existing behavior.
- Do not introduce new templating or localization layers.

## Phase 4: Integrate and Verify Against Existing Behavior

**Goal**: Connect the migrated Rust functions into the current branch and confirm parity with the C behavior for this module scope.

### Tasks
- Replace existing call sites for the covered functions with the Rust implementations.
- Remove or isolate obsolete C-side logic for these specific functions in the port branch.
- Run `cargo test` and add targeted regression tests based on representative command-line combinations handled by these setters/hooks.
- Verify that sequencing among setter calls matches the original `src/main.c` behavior.

### Deliverables
- Fully integrated Rust implementation for `module_src_parseopt_03`.
- Regression tests covering combined option interactions relevant to this function set.
- Final cleanup of temporary compatibility code used during migration.

### Exit Criteria
- All listed functions from `module_src_parseopt_03` are represented in Rust.
- No raw-pointer-style ownership remains in the migrated path.
- Error and output behavior are stable and test-covered within the scope of this module.