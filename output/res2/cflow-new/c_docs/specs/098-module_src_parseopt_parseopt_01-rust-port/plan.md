# Implementation Plan: module_src_parseopt_parseopt_01

## Summary

This module ports the C parse-option support code into Rust, covering option value setters, usage/help/version text emission, and dash-option classification logic currently implemented across:

- `src/parseopt/help.c`
- `src/parseopt/optset.c`
- `src/parseopt/parseopt.c`

The Rust implementation should preserve the existing behavior and call structure as closely as practical, focusing on a direct migration of the current functions rather than redesigning the option system. The technical approach is to:

- translate mutable C state into explicit Rust structs with borrowed or owned string fields as needed,
- replace raw pointer/string mutation patterns with safe `Option`, `bool`, `String`, and slice-based APIs,
- keep formatting/output logic centralized in a small Rust module corresponding to the existing help/usage C code,
- represent the option-setting callbacks as plain Rust functions operating on mutable state, mirroring the current C responsibilities.

The implementation should remain narrowly scoped to the files and functions listed above and avoid introducing extra parser capabilities beyond what is required to port the existing module.

## Technical Context

- **Language/Version**: Rust 1.78+
- **Primary Dependencies**: Rust standard library only
- **Testing**: `cargo test`
- **Performance Goals**:
  - Maintain effectively equivalent runtime characteristics for command-line option handling and usage/help output.
  - Avoid unnecessary string allocations except where the C code currently performs copies or owned-string assignment.
  - Keep output generation linear in the number of options/lines emitted.
  - Preserve low overhead for simple boolean and counter option setters.

## Module Mapping

Proposed Rust file layout should mirror the C source split without creating extra architectural layers:

- `src/parseopt/help.c`
  - Rust target: `src/parseopt/help.rs`
  - Responsibility:
    - usage-variable initialization
    - usage/help rendering
    - version/help output to writers or file-descriptor equivalents

- `src/parseopt/optset.c`
  - Rust target: `src/parseopt/optset.rs`
    - callback-style option setters
    - counter increment
    - bool/string assignment helpers

- `src/parseopt/parseopt.c`
  - Rust target: `src/parseopt/parseopt.rs`
    - local parse-option helpers required by this module
    - `option_dash` migration

- Module root
  - Rust target: `src/parseopt/mod.rs`
    - expose only the migrated module items needed by the rest of the crate
    - keep names close to the C source for migration clarity

## Data Model

Because the input only exposes anonymous C data structures, the Rust data model should be reconstructed from usage at the function boundaries rather than inventing broad new types. The mapping should follow these rules.

### Core mapping rules

- `char *` used as mutable string storage target
  - Rust: `Option<String>` if nullable/optional
  - Rust: `String` if always initialized and owned
  - Rust: `&str` for borrowed static usage text inputs

- integer flag/counter fields
  - Rust: `bool`, `i32`, `u32`, or `usize` depending on observed C semantics
  - Prefer signed integer only if decrement/negative states are actually present

- C nullable pointers for state/output variables
  - Rust: `Option<T>`

- `FILE *` output targets
  - Rust: generic `W: std::io::Write` where practical
  - If the migration needs exact internal structure simplicity, use `&mut dyn Write`

### Reconstructed Rust structures

The following Rust-side structures are expected, with exact field sets determined from the existing C call sites.

#### `UsageVars`
Maps the mutable usage-text state currently managed by `set_usage_var` and `init_usage_vars`.

Possible Rust shape:
```rust
struct UsageVars<'a> {
    program_name: Option<&'a str>,
    usage: Option<&'a str>,
    description: Option<&'a str>,
    extra: Option<&'a str>,
}
```

Notes:
- Use borrowed string slices if the C code only references externally owned static/argv-backed text.
- Switch individual fields to `String` only if the C logic constructs owned text dynamically.

#### `OptValueTarget`
Represents the destination mutated by optset callbacks.

Possible Rust shape:
```rust
enum OptValueTarget<'a> {
    Counter(&'a mut usize),
    Bool(&'a mut bool),
    OptionalString(&'a mut Option<String>),
    StringBuffer(&'a mut String),
}
```

Notes:
- This may remain an internal implementation detail if direct function signatures are simpler.
- Do not generalize beyond the required setter cases in the C module.

#### `DashOptionKind`
Represents the classification result of `option_dash`.

Possible Rust shape:
```rust
enum DashOptionKind {
    NotDash,
    ShortDash,
    DoubleDash,
    Other,
}
```

Notes:
- Final variants should match the actual branch outcomes in the C code.
- If the C function returns only boolean/int classification, a small enum is still acceptable internally, but public API may stay integer/boolean compatible if required by callers.

#### Anonymous C structs
For the listed anonymous structures:
- infer each concrete Rust struct from the C file-local usage,
- give them narrow names tied to current purpose, such as `UsageLine`, `OptionDescriptor`, or `HelpContext`, only when a struct is actually needed by migrated code,
- avoid creating placeholder structs for anonymous C declarations that collapse naturally into tuples, slices, or direct parameters in Rust.

## Implementation Phases

## Phase 1: Port option setter helpers

Scope:
- `optset_incr`
- `optset_string_copy`
- `optset_string`
- `optset_string_alloc`
- `optset_true`
- `optset_false`
- `optset_bool`

Technical work:
- Create `src/parseopt/optset.rs`.
- Translate each setter into a direct Rust function with explicit mutable references instead of `void *` payloads.
- Distinguish borrowed assignment from owned copy/allocation behavior:
  - copy/alloc-style setters become `String`-producing paths,
  - direct string setters should only borrow or clone according to the original ownership behavior.
- Replace null-pointer checks with `Option`-based inputs where the C code permits absence.
- Keep return conventions compatible with the surrounding parser expectations:
  - use `Result<(), ParseOptError>` only if the C code reports actual failure conditions,
  - otherwise keep infallible functions.

Validation:
- unit tests for each setter covering:
  - increment behavior,
  - true/false/bool assignment,
  - empty and non-empty strings,
  - replacement of previously set string values.

## Phase 2: Port usage/help/version output

Scope:
- `set_usage_var`
- `init_usage_vars`
- `parseopt_usage_std`
- `parseopt_usage_sdash`
- `parseopt_usage_fd`
- `parseopt_help_fd`
- `parseopt_version_fd`

Technical work:
- Create `src/parseopt/help.rs`.
- Reconstruct the minimal usage-state struct required by the C implementation.
- Replace `FILE *`-based emission with `std::io::Write`.
- Preserve output ordering, spacing, and newline behavior from the C code as closely as possible.
- Keep separate functions for standard usage, short-dash usage, and file-targeted usage/help/version output, matching the existing function boundaries.
- Handle I/O fallibility explicitly:
  - output functions should return `std::io::Result<()>`.
- Avoid building large intermediate strings unless the C implementation clearly depends on assembled buffers; prefer streaming writes.

Validation:
- snapshot-style tests against expected usage/help/version text.
- tests for alternate usage variable initialization states.
- tests using in-memory buffers (`Vec<u8>`) as output sinks.

## Phase 3: Port parseopt local helper logic

Scope:
- `option_dash`
- any minimal supporting local definitions from `parseopt.c` required for compilation of this module only

Technical work:
- Create `src/parseopt/parseopt.rs`.
- Translate dash-option detection from C string inspection into safe Rust byte/string checks.
- Preserve the original distinction among:
  - non-option input,
  - single dash handling,
  - double dash handling,
  - any special cases present in the current C logic.
- Keep function signatures narrow and compatible with the rest of the parser module.

Validation:
- unit tests covering:
  - empty string,
  - `-`,
  - `--`,
  - `-x`,
  - `--name`,
  - non-dash tokens.

## Phase 4: Integration and compatibility pass

Scope:
- connect `help.rs`, `optset.rs`, and `parseopt.rs` through `src/parseopt/mod.rs`
- align naming, visibility, and call sites with the rest of `cflow-new`

Technical work:
- Export only the functions/types actually used by neighboring code.
- Adjust any remaining C-style shared mutable state into localized Rust ownership/borrowing patterns.
- Resolve any signature mismatches introduced during the phased port while keeping behavior stable.
- Remove migration-only placeholders once all call sites compile.

Validation:
- `cargo test`
- module-level integration tests exercising a representative path from option recognition through setter mutation and usage/help emission.
- confirm no unsafe code is introduced unless a specific surrounding interface makes it unavoidable; if unavoidable, isolate and document it narrowly.

## Notes on Memory Management and Error Handling

- Prefer owned `String` only where the C code implies copied or allocated storage.
- Prefer borrowed `&str` for static usage text and transient inputs that do not require retention.
- Use `Option` instead of null sentinel values.
- Use `std::io::Result<()>` for writer-based functions.
- Avoid introducing custom error hierarchies unless the existing module has distinct failure modes that require them.
- Do not emulate manual free/reset behavior directly; rely on Rust drop semantics and overwrite semantics for owned fields.