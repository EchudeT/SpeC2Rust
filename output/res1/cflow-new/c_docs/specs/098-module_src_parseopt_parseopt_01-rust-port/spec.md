# spec.md

## Title

Functional Specification: `module_src_parseopt_parseopt_01` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_parseopt_01`
- Category: `module_cluster`
- Target Rust branch: `098-module_src_parseopt_parseopt_01-rust-port`
- Source basis:
  - `src/parseopt/help.c`
  - `src/parseopt/optset.c`
  - `src/parseopt/parseopt.c`
- Generation date: 2026-06-11

## Overview

This module provides support functionality for a command-line option parsing subsystem in three areas:

1. rendering usage text,
2. rendering help and version output,
3. applying parsed option values to destination storage.

The Rust rewrite must preserve the observable behavior evidenced by the source module: it must format usage/help/version output from option definitions and parse context, and it must provide the same option-setting behaviors used when parsed options are committed to program state.

This specification is limited to functionality evidenced by the listed source files and named functions. It does not define broader parsing behavior beyond what these files expose.

## Feature Specification

### 1. Usage text generation

The module must generate usage text for a parse context using the option definitions available in that context.

Observed capabilities include:

- initialization of usage-related substitution variables from the parse context,
- replacement or extraction of usage variables from text used for usage rendering,
- support for at least two usage rendering styles:
  - a standard usage layout,
  - a short-dash-oriented usage layout,
- writing rendered usage output to a caller-supplied file descriptor.

The Rust version must preserve the distinction between the usage formatting paths evidenced by:
- `parseopt_usage_std`
- `parseopt_usage_sdash`
- `parseopt_usage_fd`

The module must derive the visible option prefix string through the same functional boundary as `option_dash`, so usage output reflects the option style configured in the parse context and option definition.

### 2. Help text generation

The module must generate help output for a parse context and write it to a caller-supplied file descriptor.

The generated help must be based on option definitions associated with the parse context and must present option-oriented explanatory output rather than only the compact usage line. The source evidence in `help.c` shows that help generation performs option-aware organization and formatting using helper context and sorting structures; therefore the Rust version must preserve the resulting functional behavior that help output is derived from the defined options and their metadata, not a fixed template.

The Rust version must implement behavior equivalent in scope to `parseopt_help_fd`.

### 3. Version output generation

The module must generate version output for a parse context and write it to a caller-supplied file descriptor.

The Rust version must implement behavior equivalent in scope to `parseopt_version_fd`: emitting version-oriented output from parse context data without requiring the caller to manually assemble the output text.

### 4. Option value application helpers

The module must provide option-setting helpers that apply a parsed argument to storage designated by an option definition.

The Rust rewrite must support the behaviors evidenced by these functions:

- incrementing a numeric destination value (`optset_incr`),
- copying a string into caller-provided storage (`optset_string_copy`),
- storing a string reference/value (`optset_string`),
- allocating and storing a copied string (`optset_string_alloc`),
- setting a boolean destination to true (`optset_true`),
- setting a boolean destination to false (`optset_false`),
- toggling or assigning a boolean destination through a generic boolean setter (`optset_bool`).

These helpers must remain tied to parse context and option definition inputs, since each setter is invoked with `struct parseopt *` and `struct optdef *`.

### 5. Option prefix selection

The module must provide logic that determines the displayed dash prefix for an option, based on parse context and option definition inputs.

The Rust version must preserve the externally visible effect of `option_dash`: usage/help formatting that depends on whether an option is represented with a particular dash style must continue to select the correct textual prefix.

## User Scenarios & Testing

### Scenario 1: Print compact usage to a specific output target

A program has initialized a parse context with option definitions and wants to print a usage summary to a chosen file descriptor, such as standard output or standard error.

The Rust module must support:
- taking the parse context,
- rendering usage text according to the context’s configured usage style,
- writing that output to the supplied descriptor.

**Test focus**
- usage output is produced without requiring the caller to manually iterate options,
- output differs appropriately when the context selects the alternate usage style,
- option prefixes shown in usage reflect the option dash style.

### Scenario 2: Print full help for a command

A program receives a help request and needs to emit a more descriptive help listing for all relevant options.

The Rust module must support:
- reading option metadata from the parse context,
- formatting option entries into a help document,
- writing the result to a supplied descriptor.

**Test focus**
- help output includes option-driven content rather than only a single usage line,
- options are represented consistently with their configured names/prefixes,
- output is generated solely from module inputs and reaches the requested descriptor.

### Scenario 3: Print version information

A program receives a version request and needs to emit version text.

The Rust module must support:
- obtaining version-related output content from the parse context,
- writing version output to a supplied descriptor.

**Test focus**
- version output is non-empty when version data is present in the parse context,
- output goes to the specified descriptor.

### Scenario 4: Apply an incrementing option

A parsed option corresponds to a counter-style flag such as repeated verbosity flags.

The Rust module must support:
- receiving the parse context, option definition, and parsed argument,
- incrementing the destination numeric value associated with that option.

**Test focus**
- repeated application increments the stored value deterministically,
- the helper returns the same success/failure class as the C behavior for valid inputs.

### Scenario 5: Apply string-valued options in different storage modes

A parsed option carries a string argument. Different option definitions may require different assignment strategies.

The Rust module must support:
- assigning a direct string value,
- copying into existing caller-provided storage,
- allocating/storing an owned copy.

**Test focus**
- each setter updates the destination according to its mode,
- copied/allocated modes preserve argument text content,
- direct-string mode preserves the intended assigned value behavior.

### Scenario 6: Apply boolean options

A parsed option toggles a boolean destination.

The Rust module must support:
- setting true,
- setting false,
- applying the generic boolean setter behavior.

**Test focus**
- destination boolean state matches the invoked setter,
- repeated calls remain consistent with the source function semantics.

## Requirements

### Functional Requirements

#### FR-1: Usage variable preparation
The module shall derive and initialize usage-related substitution state from the parse context before usage rendering.

**Traceability:** `set_usage_var`, `init_usage_vars` in `src/parseopt/help.c`

#### FR-2: Standard usage rendering
The module shall render usage output in the standard formatting path supported by the source module.

**Traceability:** `parseopt_usage_std` in `src/parseopt/help.c`

#### FR-3: Alternate short-dash usage rendering
The module shall render usage output in the alternate short-dash formatting path supported by the source module.

**Traceability:** `parseopt_usage_sdash` in `src/parseopt/help.c`

#### FR-4: Usage emission to caller-selected descriptor
The module shall provide an entry point that writes usage output to a supplied file descriptor.

**Traceability:** `parseopt_usage_fd` in `src/parseopt/help.c`

#### FR-5: Help emission to caller-selected descriptor
The module shall provide an entry point that writes help output, derived from parse context and option definitions, to a supplied file descriptor.

**Traceability:** `parseopt_help_fd` in `src/parseopt/help.c`

#### FR-6: Version emission to caller-selected descriptor
The module shall provide an entry point that writes version output to a supplied file descriptor.

**Traceability:** `parseopt_version_fd` in `src/parseopt/help.c`

#### FR-7: Numeric increment option setter
The module shall provide an option setter that increments the option’s destination value.

**Traceability:** `optset_incr` in `src/parseopt/optset.c`

#### FR-8: Fixed-buffer string copy option setter
The module shall provide an option setter that copies an argument string into destination storage designated by the option definition.

**Traceability:** `optset_string_copy` in `src/parseopt/optset.c`

#### FR-9: Direct string assignment option setter
The module shall provide an option setter that stores the parsed string value according to the direct string assignment behavior evidenced by the source module.

**Traceability:** `optset_string` in `src/parseopt/optset.c`

#### FR-10: Allocating string assignment option setter
The module shall provide an option setter that stores an owned copy of the parsed string value.

**Traceability:** `optset_string_alloc` in `src/parseopt/optset.c`

#### FR-11: Boolean true setter
The module shall provide an option setter that sets the destination boolean state to true.

**Traceability:** `optset_true` in `src/parseopt/optset.c`

#### FR-12: Boolean false setter
The module shall provide an option setter that sets the destination boolean state to false.

**Traceability:** `optset_false` in `src/parseopt/optset.c`

#### FR-13: Generic boolean setter
The module shall provide a generic boolean-setting helper consistent with the source module’s boolean option behavior.

**Traceability:** `optset_bool` in `src/parseopt/optset.c`

#### FR-14: Option dash prefix selection
The module shall determine the display prefix used for an option from the parse context and option definition.

**Traceability:** `option_dash` in `src/parseopt/parseopt.c`

### Key Entities

#### `parseopt`
The central parse context for this module.

Role evidenced by source:
- supplies data used for usage, help, and version rendering,
- is passed to all option-setting helpers,
- participates in option dash prefix selection.

Relationships:
- owns or references a collection of `optdef` entries,
- provides source data for usage variable initialization,
- is used together with `optdef` to render option text and apply parsed values.

**Traceability:** referenced throughout `help.c`, `optset.c`, `parseopt.c`

#### `optdef`
An option definition entry associated with the parse context.

Role evidenced by source:
- supplies option-specific metadata for usage/help formatting,
- identifies destination storage or setter behavior for parsed values,
- participates in display prefix selection.

Relationships:
- belongs to or is referenced from `parseopt`,
- is consumed by option setters and help/usage formatting logic.

**Traceability:** referenced throughout `help.c`, `optset.c`, `parseopt.c`

#### `parseopt_help_format`
A help/usage formatting structure local to help generation.

Role evidenced by source:
- supports formatting decisions for help-related output.

Relationships:
- used within help rendering logic together with `parseopt` and `optdef`.

**Traceability:** `src/parseopt/help.c`

#### `usage_var_def`
A usage-variable definition structure used during usage text preparation.

Role evidenced by source:
- identifies usage variables that can be initialized or substituted during usage rendering.

Relationships:
- used by `set_usage_var` and `init_usage_vars`,
- depends on `parseopt` as the source of variable values.

**Traceability:** `src/parseopt/help.c`

#### `optsort`
A helper structure used during help organization.

Role evidenced by source:
- supports ordering or grouping of option entries for help output.

Relationships:
- derived from `optdef` data during help generation.

**Traceability:** `src/parseopt/help.c`

#### `help_context`
A helper context used during help rendering.

Role evidenced by source:
- carries the active parse context and option-related state needed while producing help output.

Relationships:
- aggregates or references `parseopt` and `optdef`,
- is used internally by help rendering logic.

**Traceability:** `src/parseopt/help.c`

## Success Criteria

1. **Usage output parity**
   - Given equivalent parse-context data, the Rust module can emit usage text to a requested descriptor using both supported usage formatting paths.
   - Traceability: `parseopt_usage_std`, `parseopt_usage_sdash`, `parseopt_usage_fd`

2. **Usage variable handling parity**
   - Usage rendering in Rust initializes and applies usage-variable state from the parse context rather than relying on hard-coded static text alone.
   - Traceability: `set_usage_var`, `init_usage_vars`

3. **Help output parity**
   - Given equivalent parse-context and option-definition data, the Rust module emits help text to a requested descriptor, and the output is derived from the option set.
   - Traceability: `parseopt_help_fd`, `help_context`, `optsort`, `optdef`, `parseopt`

4. **Version output parity**
   - Given equivalent parse-context version data, the Rust module emits version output to a requested descriptor.
   - Traceability: `parseopt_version_fd`, `parseopt`

5. **Option setter coverage parity**
   - The Rust module implements all seven evidenced setter behaviors: increment, string copy, direct string assignment, allocating string assignment, set true, set false, and generic boolean set.
   - Traceability: `optset_incr`, `optset_string_copy`, `optset_string`, `optset_string_alloc`, `optset_true`, `optset_false`, `optset_bool`

6. **State update correctness for setters**
   - For representative valid inputs, each Rust setter updates the destination state associated with the supplied option definition in the same functional direction as the C source:
     - increment changes a numeric destination by one step,
     - string setters preserve the provided text according to their storage mode,
     - boolean setters produce the expected final boolean state.
   - Traceability: all functions in `src/parseopt/optset.c`

7. **Option display prefix parity**
   - The Rust module selects the visible option dash prefix from parse-context and option-definition inputs so that usage/help text reflects the same option style decisions as the source module.
   - Traceability: `option_dash`, plus usage/help renderers consuming option display text

8. **Descriptor-directed output**
   - The Rust module’s public output entry points for usage, help, and version each accept a caller-selected output target equivalent in purpose to the C file descriptor interface and write their output there.
   - Traceability: `parseopt_usage_fd`, `parseopt_help_fd`, `parseopt_version_fd`