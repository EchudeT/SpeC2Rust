# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_parseopt_01`

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_parseopt_01`
- Category: `module_cluster`
- Target branch: `098-module_src_parseopt_parseopt_01-rust-port`
- Source scope:
  - `src/parseopt/help.c`
  - `src/parseopt/optset.c`
  - `src/parseopt/parseopt.c`
- Generation date: `2026-06-17`

## 1. Feature Specification

### 1.1 Purpose

This module provides parse-option support behavior in three areas:

1. option value setters used by a parse-option framework,
2. usage/help/version text emission to a file descriptor,
3. option prefix selection for display.

The Rust rewrite must preserve these functional boundaries and user-visible behaviors as evidenced by the source module.

### 1.2 In-Scope Functionality

The Rust version must implement the following module behaviors:

- Set option target values through predefined setter routines for:
  - incrementing a numeric target,
  - assigning string values,
  - assigning string values by copy/allocation mode,
  - setting boolean-like targets to true, false, or a fixed boolean value.
- Produce usage output for a parse-option definition set, including:
  - standard usage formatting,
  - single-dash-oriented usage formatting,
  - writing usage output to a caller-specified file descriptor.
- Produce full help output for a parse-option definition set and write it to a caller-specified file descriptor.
- Produce version output and write it to a caller-specified file descriptor.
- Resolve the display dash/prefix associated with an option for presentation purposes.

### 1.3 Behavioral Scope

The module is presentation- and assignment-oriented. It does not itself define the full command-line parsing engine in the provided evidence. The Rust rewrite must therefore preserve only the functionality directly evidenced here:

- formatting and emitting help/usage/version text from parse-option metadata,
- applying option values to bound storage through setter callbacks,
- determining an option’s display prefix string.

### 1.4 Output and Side Effects

The Rust version must support side effects equivalent to the C module:

- mutation of option-bound storage through setter functions,
- writing formatted text to an output sink corresponding to a file descriptor,
- initialization and use of internal usage/help formatting variables derived from parse-option state.

## 2. User Scenarios & Testing

### 2.1 Scenario: Print standard command usage

A caller has a populated parse-option context and needs a concise usage summary.
The module writes usage text to a chosen output destination and reflects the option metadata in that summary.

**Test expectations**
- Given a parse-option context with defined options and usage metadata, usage output is produced.
- Output is written to the destination specified by the caller.
- Option prefixes shown in the output match the module’s option-dash selection behavior.

### 2.2 Scenario: Print help text for a command

A caller requests full help for a command-line interface.
The module formats and emits help text based on the defined options and available help metadata.

**Test expectations**
- Help output is produced for a valid parse-option context.
- Help output includes option-oriented information derived from option definitions.
- Output is written to the destination specified by the caller.

### 2.3 Scenario: Print version text

A caller requests version information for the command associated with the parse-option context.
The module emits version text to the requested destination.

**Test expectations**
- Version output is produced when version information is available in the parse-option context.
- Output is written to the destination specified by the caller.

### 2.4 Scenario: Option increments a counter

A command-line option is configured to count occurrences.
When the corresponding setter is invoked, the bound numeric target is incremented.

**Test expectations**
- Each invocation increases the target by one.
- The setter reports completion through its return convention.

### 2.5 Scenario: Option stores a provided string directly

A command-line option is configured to store an argument string without copying.
When the setter is invoked, the bound target refers to the provided string value.

**Test expectations**
- The target is updated to the provided argument value.
- The setter reports completion through its return convention.

### 2.6 Scenario: Option stores a copied or allocated string

A command-line option is configured to preserve an argument value independently of the caller’s buffer lifetime.
When the setter is invoked, the bound target receives a copied/allocated string value according to the corresponding setter behavior.

**Test expectations**
- The target receives string content matching the provided argument.
- The copied/allocated storage behavior is distinct from direct pointer assignment.
- The setter reports completion through its return convention.

### 2.7 Scenario: Option forces a boolean state

A command-line option is configured to force a target state on activation.
When the setter is invoked, the target becomes true, false, or a defined boolean value.

**Test expectations**
- The target state matches the setter’s intended value.
- The setter reports completion through its return convention.

### 2.8 Scenario: Usage format depends on option style

A caller uses a parse-option context whose display style requires single-dash usage formatting rather than standard formatting.
The module emits usage text using the appropriate style.

**Test expectations**
- Distinct usage formatting paths are supported.
- The selected formatting path affects how options are presented.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Option setter support

The Rust module shall provide setter behaviors corresponding to the source module’s option-setting functions in `src/parseopt/optset.c`.

Traceability:
- `optset_incr`
- `optset_string_copy`
- `optset_string`
- `optset_string_alloc`
- `optset_true`
- `optset_false`
- `optset_bool`

#### FR-2: Incrementing option target

The module shall support a setter that increments a target value each time the option is applied.

Traceability:
- `optset_incr`

#### FR-3: Direct string assignment

The module shall support a setter that assigns the provided argument string to the option’s target without requiring copied ownership semantics.

Traceability:
- `optset_string`

#### FR-4: String copy/allocation assignment

The module shall support setter behaviors that store a provided argument string using copied or allocated storage semantics distinct from direct string assignment.

Traceability:
- `optset_string_copy`
- `optset_string_alloc`

#### FR-5: Boolean state assignment

The module shall support setters that assign option targets to true, false, or a fixed boolean state when the option is applied.

Traceability:
- `optset_true`
- `optset_false`
- `optset_bool`

#### FR-6: Usage variable preparation

The module shall derive and initialize internal usage-formatting variables from parse-option context data before generating usage/help text.

Traceability:
- `set_usage_var`
- `init_usage_vars`
- `usage_var_def`
- `parseopt`

#### FR-7: Standard usage generation

The module shall support generation of standard usage text for a parse-option context.

Traceability:
- `parseopt_usage_std`

#### FR-8: Single-dash usage generation

The module shall support generation of single-dash-oriented usage text for a parse-option context.

Traceability:
- `parseopt_usage_sdash`

#### FR-9: Usage output emission

The module shall provide an operation that writes generated usage text for a parse-option context to a caller-specified output destination corresponding to a file descriptor.

Traceability:
- `parseopt_usage_fd`

#### FR-10: Help output emission

The module shall provide an operation that writes help text for a parse-option context to a caller-specified output destination corresponding to a file descriptor.

Traceability:
- `parseopt_help_fd`

#### FR-11: Version output emission

The module shall provide an operation that writes version text for a parse-option context to a caller-specified output destination corresponding to a file descriptor.

Traceability:
- `parseopt_version_fd`

#### FR-12: Option display prefix resolution

The module shall provide option-prefix selection behavior for display, returning the dash/prefix string associated with an option in the context of a parse-option configuration.

Traceability:
- `option_dash`
- `parseopt`
- `optdef`

#### FR-13: Option metadata-driven formatting

Usage and help output shall be derived from parse-option and option-definition metadata rather than hardcoded command text.

Traceability:
- `parseopt_usage_std`
- `parseopt_usage_sdash`
- `parseopt_usage_fd`
- `parseopt_help_fd`
- `optdef`
- `parseopt`

### 3.2 Key Entities

#### `parseopt`

Represents the parse-option context used throughout the module. It is the primary input for:

- usage generation,
- help generation,
- version generation,
- usage variable initialization,
- option prefix selection,
- option setter invocation context.

Relationships:
- owns or references option definitions (`optdef`),
- supplies data used to derive usage variables,
- provides contextual information needed for display formatting.

Traceability:
- referenced across `help.c`, `optset.c`, and `parseopt.c`

#### `optdef`

Represents an option definition within a parse-option context.

Relationships:
- is consumed by setter behaviors when an option is applied,
- is inspected for help/usage presentation,
- is used when resolving display prefixes.

Traceability:
- referenced across `help.c`, `optset.c`, and `parseopt.c`

#### `usage_var_def`

Represents the definition of a usage formatting variable used while preparing usage text.

Relationships:
- used with `parseopt` during usage variable setup,
- supports substitution/initialization before output generation.

Traceability:
- `set_usage_var`
- `init_usage_vars`

#### `parseopt_help_format`

Represents formatting configuration/state for help presentation.

Relationships:
- supports help and usage text layout decisions.

Traceability:
- `src/parseopt/help.c`

#### `help_context`

Represents working context used during help generation.

Relationships:
- ties together parse-option context and option-definition data during help formatting.

Traceability:
- `src/parseopt/help.c`

#### `optsort`

Represents temporary option-ordering data used while preparing help presentation.

Relationships:
- associates option definitions with ordering/grouping behavior for output formatting.

Traceability:
- `src/parseopt/help.c`

## 4. Success Criteria

### 4.1 Functional Equivalence Criteria

1. The Rust module can emit usage text for a valid parse-option context to a caller-selected output sink corresponding to the original file-descriptor-based behavior.
   - Traceability: `parseopt_usage_fd`

2. The Rust module supports both standard usage formatting and single-dash usage formatting paths.
   - Traceability: `parseopt_usage_std`, `parseopt_usage_sdash`

3. The Rust module can emit help text for a valid parse-option context to a caller-selected output sink corresponding to the original file-descriptor-based behavior.
   - Traceability: `parseopt_help_fd`

4. The Rust module can emit version text for a valid parse-option context to a caller-selected output sink corresponding to the original file-descriptor-based behavior.
   - Traceability: `parseopt_version_fd`

5. The Rust module supports option setter behaviors for increment, string assignment, copied/allocated string storage, and boolean assignment.
   - Traceability: `optset_incr`, `optset_string`, `optset_string_copy`, `optset_string_alloc`, `optset_true`, `optset_false`, `optset_bool`

6. Invoking the increment setter repeatedly changes the bound target by one per invocation.
   - Traceability: `optset_incr`

7. Invoking the direct string setter updates the bound target to the provided string argument.
   - Traceability: `optset_string`

8. Invoking copied/allocated string setters stores string content matching the provided argument while preserving distinct behavior from direct assignment.
   - Traceability: `optset_string_copy`, `optset_string_alloc`

9. Invoking boolean setters updates the bound target to the expected state for each setter.
   - Traceability: `optset_true`, `optset_false`, `optset_bool`

10. The Rust module resolves an option’s display dash/prefix consistently for output formatting.
    - Traceability: `option_dash`

11. Usage/help generation in the Rust rewrite is driven by parse-option and option-definition metadata rather than fixed literal output unrelated to context.
    - Traceability: `parseopt_usage_std`, `parseopt_usage_sdash`, `parseopt_help_fd`, `parseopt`, `optdef`

### 4.2 Testability Criteria

The rewrite is acceptable when automated tests demonstrate:

- usage output is produced for at least one standard-format context,
- usage output is produced for at least one single-dash-format context,
- help output is produced for a context containing multiple option definitions,
- version output is produced for a context containing version metadata,
- each setter function updates its bound target according to the expected semantics,
- option prefix resolution returns the expected display prefix for representative option definitions.