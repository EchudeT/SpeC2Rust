# spec.md

## Title

Rust Port Functional Specification: `module_src_parseopt_optdef_04`

## Status

Draft

## Scope

This specification defines the functional behavior that must be preserved when rewriting the `src/parseopt/help.c` and `src/parseopt/parseopt.c` module cluster from `cflow-new` into Rust on branch `101-module_src_parseopt_optdef_04-rust-port`.

The covered behavior is limited to:

- formatting and printing option help entries,
- resolving option aliases for help presentation,
- grouping and sorting option definitions for display,
- handling standard and single-dash option presentation variants,
- locating an option’s short-name form where needed by parsing or display support.

This specification does not define unrelated argument parsing behavior outside the evidenced module boundaries.

## Feature Specification

### Overview

This module cluster is responsible for turning internal option definition data into user-facing help output, with stable ordering and correct handling of aliases and option argument placeholders.

The Rust version must preserve the following observable behaviors evidenced by the analyzed functions and supporting types:

1. **Help output generation for individual options**
   - The module formats option names and their argument placeholders for display.
   - It supports both standard multi-form option output and a single-dash formatting variant.
   - It decides how adjacent or related option definitions are combined or emitted during help rendering.

2. **Alias resolution for display**
   - When an option is an alias of another option, help formatting uses the effective underlying option definition rather than presenting aliased metadata as independent semantics where alias unwrapping is required.

3. **Display ordering**
   - Option entries are ordered through explicit comparison and group sorting logic.
   - Sorting behavior is used to produce deterministic help output rather than relying on source declaration order alone.

4. **Argument placeholder formatting**
   - Option arguments are printed with delimiter-aware formatting and argument-count tracking so that option synopsis text reflects the expected argument structure of an option.

5. **Short option name discovery**
   - The module can find the short-name representation associated with an option definition when such a form exists.

### Functional Boundary

The Rust port must implement the same functional boundary as evidenced by:

- help rendering behavior centered on `print_arg`, `print_option_std`, `print_option_sdash`, and `print_option`,
- alias normalization through `opt_unalias`,
- ordering behavior through `merge`, `optcmp`, `sethead`, and `sort_group`,
- short-name lookup through `find_short_name`.

No additional capabilities are required beyond these documented behaviors.

## User Scenarios & Testing

### Scenario 1: Print a standard help entry for an option with argument placeholders

A caller has a parsed option-definition table and requests help output.
When an option takes one or more arguments, the module prints the option names together with the expected argument placeholder text in the standard display form.

The Rust version must support tests that verify:

- the option name is printed,
- argument placeholder text is included when the option requires arguments,
- delimiter-sensitive formatting is preserved,
- adjacent related option forms are displayed consistently.

### Scenario 2: Print a help entry for a single-dash style option

A caller requests help output for options that are displayed in a single-dash style variant.
The module formats that entry using the single-dash presentation rules rather than the standard variant.

The Rust version must support tests that verify:

- single-dash help output is distinguishable from the standard format,
- the same underlying option definition can be rendered correctly in this mode,
- option arguments, if any, remain attached to the displayed option form.

### Scenario 3: Display help for aliased options without duplicating alias semantics

An option definition is an alias of another definition.
When help is generated, the module resolves the alias to its effective target before determining the displayable option semantics.

The Rust version must support tests that verify:

- alias resolution reaches the underlying effective definition,
- help text formatting uses the target option’s argument/display characteristics,
- alias handling does not change deterministic ordering behavior.

### Scenario 4: Produce deterministic sorted help output for a group of options

A caller generates a help section from a set of option definitions whose display order is not already presentation-ready.
The module sorts the options according to internal comparison rules and emits them in stable grouped order.

The Rust version must support tests that verify:

- the same input option set always produces the same display order,
- comparison-driven ordering is applied across a group,
- merged sorting behavior produces a fully ordered result for the group.

### Scenario 5: Find a short-name form for an option

A caller needs to determine whether an option has a short-name variant.
The module searches the available option definition data and returns the matching short-name form when present.

The Rust version must support tests that verify:

- a short-name match is found when one exists,
- absence of a short-name form is reported distinctly,
- lookup behavior is consistent with help formatting expectations.

## Requirements

### Functional Requirements

#### FR-1: Help entry rendering
The module shall render individual help entries from internal option definitions for user-facing help output.

**Traceability:** `print_option`, `print_option_std`, `print_option_sdash` in `src/parseopt/help.c`.

#### FR-2: Argument placeholder rendering
The module shall include option argument placeholders in help output when an option definition indicates argument usage.

**Traceability:** `print_arg` in `src/parseopt/help.c`; `optdef` references in `help.c`.

#### FR-3: Delimiter-aware argument formatting
The module shall preserve delimiter-sensitive formatting when rendering option arguments within help entries.

**Traceability:** `print_arg` in `src/parseopt/help.c`.

#### FR-4: Alias normalization for help behavior
The module shall resolve option aliases to their effective underlying option definitions where help formatting logic requires unaliased option semantics.

**Traceability:** `opt_unalias` in `src/parseopt/help.c`; `optdef` references in `help.c`.

#### FR-5: Support for standard help presentation
The module shall support a standard formatting path for option help output.

**Traceability:** `print_option_std` in `src/parseopt/help.c`; `help_context` in `help.c`.

#### FR-6: Support for single-dash help presentation
The module shall support a single-dash formatting path for option help output.

**Traceability:** `print_option_sdash` in `src/parseopt/help.c`; `help_context` in `help.c`.

#### FR-7: Conditional option emission across neighboring entries
The module shall determine option output with awareness of neighboring option definitions when rendering help entries.

**Traceability:** `print_option_std`, `print_option_sdash`, `print_option` in `src/parseopt/help.c` (all accept current and neighboring indices or equivalent context).

#### FR-8: Deterministic option comparison
The module shall compare option definitions according to internal display-order rules used for help sorting.

**Traceability:** `optcmp` in `src/parseopt/help.c`.

#### FR-9: Group sorting for help display
The module shall sort groups of option definitions before help output is produced.

**Traceability:** `sort_group`, `sethead` in `src/parseopt/help.c`; `optsort` in `help.c`.

#### FR-10: Merge-based ordered combination
The module shall combine partially ordered option ranges into a final ordered sequence for help presentation.

**Traceability:** `merge` in `src/parseopt/help.c`.

#### FR-11: Short-name discovery
The module shall locate the short-name form associated with an option definition when such a form exists.

**Traceability:** `find_short_name` in `src/parseopt/parseopt.c`.

### Key Entities

#### `optdef`
Represents an option definition used by both parsing support and help generation.

Functional role evidenced by the module:

- source of option names and aliases,
- source of argument-taking characteristics,
- item being compared, sorted, and rendered for help output.

**Traceability:** referenced throughout `help.c`; `find_short_name` in `parseopt.c`.

#### `help_context`
Represents the active context used while rendering help entries.

Functional role evidenced by the module:

- carries the parse/help state needed by option-printing functions,
- provides access to option arrays and formatting decisions during rendering.

**Traceability:** `print_option_std`, `print_option_sdash`, `print_option` in `src/parseopt/help.c`.

#### `optsort`
Represents the working state for sorting option groups.

Functional role evidenced by the module:

- stores data needed to establish and update group ordering,
- supports head-setting and group sort operations.

**Traceability:** `sethead`, `sort_group` in `src/parseopt/help.c`.

#### `parseopt_help_format`
Represents help-format configuration data used by help-generation logic.

Functional role evidenced by the module:

- participates in the formatting environment for help output,
- influences how option definitions are turned into displayed text.

**Traceability:** structure declared in `src/parseopt/help.c`.

#### `usage_var_def`
Represents usage/help variable definitions involved in formatting usage-related text.

Functional role evidenced by the module:

- contributes structured formatting data referenced by help code,
- supports substitution or organization of usage text elements.

**Traceability:** structure declarations in `src/parseopt/help.c`.

#### Relationships

- `help_context` uses `optdef` items as the primary input to help rendering.
- `optsort` organizes collections of `optdef` items into display order.
- alias relationships exist between `optdef` items and are resolved before some rendering decisions.
- `parseopt_help_format` and related usage-format data influence how `optdef` content is emitted.

## Success Criteria

### Behavioral Acceptance Criteria

1. **Equivalent help entry coverage**
   - For representative option definitions covering no-argument, argument-taking, aliased, and single-dash cases, the Rust port produces help entries matching the C module’s observable formatting decisions.
   - **Traceability:** `print_arg`, `print_option_std`, `print_option_sdash`, `print_option`.

2. **Correct alias handling**
   - For aliased option definitions, the Rust port resolves to the same effective definition used by the C module for help behavior.
   - **Traceability:** `opt_unalias`.

3. **Deterministic sorted output**
   - Given the same option-definition group input, repeated executions of the Rust port produce the same help ordering as the C module.
   - **Traceability:** `optcmp`, `merge`, `sethead`, `sort_group`.

4. **Short-name lookup parity**
   - For options with and without short-name forms, the Rust port returns presence/absence results equivalent to the C module.
   - **Traceability:** `find_short_name`.

5. **No unsupported feature expansion**
   - The Rust rewrite remains limited to the documented behaviors of help formatting, alias resolution, display ordering, and short-name discovery, without introducing unrelated public functionality.
   - **Traceability:** bounded by analyzed files `src/parseopt/help.c` and `src/parseopt/parseopt.c`.

### Test Completion Criteria

The Rust port shall be considered functionally complete for this module when:

- tests cover standard help rendering,
- tests cover single-dash help rendering,
- tests cover alias-based help behavior,
- tests cover ordered group output,
- tests cover short-name lookup,
- all such tests demonstrate output or decision parity with the source module for the covered cases.