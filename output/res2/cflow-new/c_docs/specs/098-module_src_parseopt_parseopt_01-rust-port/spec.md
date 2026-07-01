# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_parseopt_01`

## Summary

This module provides option-related support behavior for the `cflow-new` parser layer in three areas:

1. setting option target values through predefined setter routines,
2. generating usage output,
3. generating help and version output.

The Rust rewrite must preserve the observable behavior evidenced by:

- `src/parseopt/optset.c`
- `src/parseopt/help.c`
- `src/parseopt/parseopt.c`

The module operates on an existing option parser state (`parseopt`) and option definitions (`optdef`). It does not define a full command-line parser by itself in the analyzed files; instead, it supports formatting and state mutation for options already described by the surrounding parseopt system.

## Scope

### In Scope

- Applying option values to caller-owned storage through standard setter behaviors.
- Selecting the visible dash form for an option.
- Producing usage text to a file descriptor.
- Producing help text to a file descriptor.
- Producing version text to a file descriptor.
- Resolving and initializing usage-template variables used by usage formatting.

### Out of Scope

- Full command-line tokenization and argument scanning beyond the evidenced support functions.
- Defining new option semantics not represented by the existing setters.
- Adding configuration formats, serialization, shell completion, localization frameworks, or interactive help systems.

---

## Feature Specification

### 1. Option value setter behavior

The module provides standard behaviors for writing parsed option results into storage referenced by an option definition.

The Rust version must implement setter behavior equivalent to the analyzed routines:

- incrementing a numeric target,
- copying a provided string into caller-owned storage,
- storing a provided string reference,
- storing an allocated copy of a provided string,
- forcing a boolean-like target to true,
- forcing a boolean-like target to false,
- toggling or setting a boolean-like target according to the routine’s defined behavior.

These setters are invoked with the parser context, an option definition, and an argument string pointer. Their observable effect is mutation of the option’s associated destination value and a status result indicating success or failure.

### 2. Usage variable preparation

The module supports usage text generation through internal variable substitution/initialization logic.

The Rust version must preserve the behavior of:

- extracting usage-related variable values from parser state and text sources,
- initializing all usage variables required before formatting usage output.

This behavior is used as a prerequisite to producing final usage text.

### 3. Usage output generation

The module emits usage information in at least two formatting styles evidenced by the code:

- a standard usage rendering,
- an alternate single-dash-oriented rendering.

The Rust version must preserve the externally visible result that `parseopt_usage_fd` writes usage text for a parser instance to a supplied file descriptor, choosing the module’s supported formatting path(s) as in the original behavior.

### 4. Help output generation

The module emits help text for the parser’s options and related command information.

The Rust version must preserve the behavior that help output is generated from parser state and option definitions, including presentation of option names and descriptions in formatted output written to a supplied file descriptor.

The existing code evidence also shows internal sorting/formatting support structures for help generation; the Rust rewrite must preserve the resulting output behavior, without needing to expose those internal structures publicly.

### 5. Version output generation

The module emits version text for the parser/program context to a supplied file descriptor.

The Rust version must preserve the behavior that version information is written when requested, using parser state as the source of the rendered content.

### 6. Option dash selection

The module provides logic for determining the dash prefix form to use when rendering an option.

The Rust version must preserve the behavior of returning the appropriate dash representation for a given parser context and option definition, for use by help/usage formatting.

---

## User Scenarios & Testing

### Scenario 1: Count-style option increments a target

A caller defines an option whose action is to increase a numeric counter each time the option is encountered.

**Expected support in Rust:**
- invoking the increment setter increases the target by one per call,
- the setter reports success,
- repeated invocations accumulate correctly.

**Testing focus:**
- single invocation changes `0 -> 1`,
- repeated invocation changes `1 -> 2 -> 3`,
- no unrelated parser fields are modified.

### Scenario 2: Option stores a provided string

A caller defines an option that accepts a string argument and stores it.

**Expected support in Rust:**
- one setter stores the provided string as the target value,
- another setter stores a copied/owned version when that mode is selected.

**Testing focus:**
- stored value matches the provided argument text,
- owned-copy behavior preserves the text independently of the original mutable input lifetime,
- status values distinguish success from allocation failure where applicable.

### Scenario 3: Flag option sets boolean state

A caller defines options such as enable/disable flags.

**Expected support in Rust:**
- `true` setter forces enabled state,
- `false` setter forces disabled state,
- boolean setter preserves the original routine’s observable state update semantics.

**Testing focus:**
- each setter writes the expected target state,
- argument text is ignored when the original behavior does not depend on it,
- return status indicates success.

### Scenario 4: Program prints usage to a file descriptor

A program requests usage output for its parser configuration.

**Expected support in Rust:**
- usage text is generated from parser state,
- output is written to the provided file descriptor,
- usage variable initialization occurs before formatting,
- supported formatting style matches original behavior for the same parser configuration.

**Testing focus:**
- non-empty usage output is written,
- expected command/option markers appear,
- output destination receives the data without requiring stdout/stderr specifically.

### Scenario 5: Program prints help for defined options

A program requests detailed help text.

**Expected support in Rust:**
- help output includes option renderings based on `optdef` data,
- visible option prefixes use the module’s dash-selection behavior,
- output is written to the provided file descriptor.

**Testing focus:**
- options appear in help output,
- descriptions appear with their associated options,
- formatting remains stable enough to preserve the original content structure.

### Scenario 6: Program prints version information

A program requests version output.

**Expected support in Rust:**
- version text is written to the specified file descriptor,
- output reflects parser/program version data configured in parser state.

**Testing focus:**
- non-empty version output is produced when version data exists,
- destination file descriptor receives the content.

### Scenario 7: Single-dash usage style is requested by parser configuration

A parser configuration triggers the alternate usage formatting path.

**Expected support in Rust:**
- usage output follows the alternate style rather than the standard style,
- option prefix rendering matches the original module behavior for this mode.

**Testing focus:**
- compare output from standard and alternate configurations,
- verify that the alternate configuration changes dash/prefix presentation as expected.

---

## Requirements

### Functional Requirements

#### FR-1: Standard option setters
The module shall provide setter behaviors that update option-associated storage for increment, string copy, string store, allocated string store, true, false, and boolean-setting cases, corresponding to the routines in `src/parseopt/optset.c`.

#### FR-2: Setter status reporting
Each setter behavior shall return a success/failure status compatible with the original module’s observable contract, including failure when owned string allocation cannot be completed, as evidenced by `optset_string_alloc` in `src/parseopt/optset.c`.

#### FR-3: Usage variable initialization
Before formatting usage text, the module shall initialize the usage variables required by the usage formatter, as evidenced by `set_usage_var` and `init_usage_vars` in `src/parseopt/help.c`.

#### FR-4: Usage output emission
The module shall generate usage text from parser state and write it to a caller-supplied file descriptor, as evidenced by `parseopt_usage_fd` in `src/parseopt/help.c`.

#### FR-5: Multiple usage formatting behaviors
The module shall support the usage formatting behaviors evidenced by `parseopt_usage_std` and `parseopt_usage_sdash` in `src/parseopt/help.c`, with the selected output behavior determined by parser state as in the original module.

#### FR-6: Help output emission
The module shall generate formatted help text for parser options and write it to a caller-supplied file descriptor, as evidenced by `parseopt_help_fd` in `src/parseopt/help.c`.

#### FR-7: Version output emission
The module shall generate version text and write it to a caller-supplied file descriptor, as evidenced by `parseopt_version_fd` in `src/parseopt/help.c`.

#### FR-8: Option prefix selection
The module shall determine the visible dash prefix for an option from parser state and option definition data, as evidenced by `option_dash` in `src/parseopt/parseopt.c`.

#### FR-9: Help output uses option definitions
Help and usage generation shall derive rendered option information from parser-owned option definition data (`optdef`), as evidenced throughout `src/parseopt/help.c`.

#### FR-10: File-descriptor-oriented output contract
Usage, help, and version functions shall target a supplied file descriptor rather than assuming a fixed output stream, as evidenced by `parseopt_usage_fd`, `parseopt_help_fd`, and `parseopt_version_fd`.

### Key Entities

#### `parseopt`
The parser state object is the central context for this module. It supplies:

- configuration affecting usage/help/version output,
- access to option definitions,
- source values for usage variables,
- output text content such as program metadata.

It is consumed by all major behaviors in the analyzed files.

#### `optdef`
An option definition describes an individual option and its associated storage/action metadata. In this module it is used for:

- selecting the option’s visible dash form,
- rendering options in usage/help text,
- locating destination storage for setter routines.

`optdef` instances are associated with a `parseopt` context.

#### `parseopt_help_format`
This structure represents internal help/usage formatting state in `help.c`. The Rust rewrite must preserve the formatting behavior it supports, but does not need to expose this structure publicly unless required by the surrounding port architecture.

#### `usage_var_def`
This structure represents a usage-template variable definition/value relationship used during usage preparation. It links usage placeholder processing to parser-derived content.

#### Help-related internal context structures
The analyzed code includes internal sorting and context structures such as `optsort` and `help_context`. These support ordering and formatting of help output. Their internal representation may change in Rust, but the resulting help output behavior must remain functionally equivalent.

#### Relationship summary
- `parseopt` owns or references multiple `optdef` entries.
- usage/help/version generation reads from `parseopt`, and help/usage also read from `optdef`.
- setter routines use `optdef` metadata to mutate caller-visible option target storage.
- usage variable structures mediate between parser state and rendered usage text.

---

## Success Criteria

### SC-1: Setter equivalence
For every setter behavior evidenced in `src/parseopt/optset.c`, tests demonstrate the same target-state mutation and return-status behavior for representative valid inputs.

### SC-2: Allocation-sensitive string storage
Tests demonstrate that the owned-string setter stores text content correctly on success and reports failure on forced allocation failure paths, traceable to `optset_string_alloc`.

### SC-3: Usage output availability
Given a populated parser context, calling the Rust equivalent of `parseopt_usage_fd` writes non-empty usage output to the supplied file descriptor.

### SC-4: Usage style preservation
Given parser configurations that exercise both usage formatting paths evidenced by `parseopt_usage_std` and `parseopt_usage_sdash`, tests show distinct output consistent with the selected style.

### SC-5: Help output coverage
Given parser contexts with defined options, calling the Rust equivalent of `parseopt_help_fd` writes help output that includes rendered option entries and their associated descriptive text.

### SC-6: Version output availability
Given parser context containing version data, calling the Rust equivalent of `parseopt_version_fd` writes non-empty version output to the supplied file descriptor.

### SC-7: Option dash behavior preservation
Tests covering representative option definitions confirm that the Rust equivalent of `option_dash` returns the same visible dash form as the original module for the same parser configuration.

### SC-8: File descriptor targeting
Integration tests verify that usage, help, and version output can be directed to caller-selected file descriptors and captured without requiring process-global stdout/stderr behavior.

### SC-9: Traceable scope conformance
The Rust rewrite exposes and implements only the functionality evidenced by `help.c`, `optset.c`, and `parseopt.c` for this module boundary, with no dependency on invented capabilities outside those behaviors.