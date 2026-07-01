# spec.md

## Title

Rust Functional Specification for `module_src_parseopt_optdef_04`

## Metadata

- Project: `cflow-new`
- Module: `module_src_parseopt_optdef_04`
- Category: `module_cluster`
- Source files:
  - `src/parseopt/help.c`
  - `src/parseopt/parseopt.c`
- Rust branch: `101-module_src_parseopt_optdef_04-rust-port`
- Generation date: `2026-06-17`

## Overview

This module is responsible for option-definition presentation behavior within the parseopt subsystem. The evidenced scope covers:

- formatting and printing option argument placeholders for help output,
- resolving option aliases to their effective option definition,
- ordering option definitions for help display,
- printing option entries in distinct display styles,
- locating an option’s short-name form from its definition.

The Rust rewrite must preserve the observed behavior of help-generation and option-definition handling represented by these source files and functions, without introducing additional capabilities beyond this scope.

## Feature Specification

### Summary

The Rust module must implement the functional behavior needed to:

1. interpret option-definition metadata for display purposes,
2. normalize aliased option definitions before display,
3. sort option definitions into the display order used by help text generation,
4. print individual options in at least the evidenced standard and single-dash display forms,
5. support help-layout decisions using shared formatting context,
6. detect or retrieve the short-name representation associated with an option definition.

### In-Scope Functional Behavior

#### 1. Help argument rendering

The module must render the argument portion of an option’s help entry based on the option definition and current formatting state. This includes:

- printing option argument placeholders when an option takes arguments,
- tracking argument-use state during formatting,
- inserting delimiters as required by the selected option display form.

This behavior is evidenced by `print_arg` in `src/parseopt/help.c`.

#### 2. Alias normalization for help/display logic

The module must resolve an option definition through aliasing so that help generation and related logic operate on the effective underlying option definition rather than a purely aliased wrapper when applicable.

This behavior is evidenced by `opt_unalias` in `src/parseopt/help.c`.

#### 3. Stable option ordering for help output

The module must provide ordering behavior for option definitions used by help rendering. The evidenced behavior includes:

- comparing options using module-defined comparison rules,
- merging partially ordered ranges,
- sorting grouped option collections,
- maintaining group-head metadata used during grouped ordering.

This behavior is evidenced by `merge`, `optcmp`, `sethead`, and `sort_group` in `src/parseopt/help.c`.

The Rust rewrite must preserve the same observable ordering of options for equivalent input definitions.

#### 4. Option help entry rendering in multiple display styles

The module must render option help lines in the display styles evidenced by the source:

- standard option display format,
- single-dash option display format.

This includes use of formatting context and neighboring option information when deciding how an option entry is printed.

This behavior is evidenced by `print_option_std`, `print_option_sdash`, and `print_option` in `src/parseopt/help.c`.

#### 5. Help formatting context usage

The module must use shared help-formatting context to drive option printing decisions, including access to parseopt-level settings and the option-definition set being rendered.

This behavior is evidenced by `struct help_context` usage across the help-printing functions in `src/parseopt/help.c`.

#### 6. Short-name lookup from option definitions

The module must be able to find the short-name representation associated with an option definition when such a form exists.

This behavior is evidenced by `find_short_name` in `src/parseopt/parseopt.c`.

## User Scenarios & Testing

### Scenario 1: Rendering help for a mixed option set

A caller prepares a parseopt definition containing multiple options, including long options, short options, and options with arguments. The module generates help output in which:

- each option appears in the expected display order,
- arguments are shown with the correct placeholder formatting,
- the correct display form is used for each option entry.

**Test expectation:** For a fixed option-definition set, the Rust output ordering and per-entry structure match the C module’s observable behavior.

### Scenario 2: Rendering help when aliases are present

A caller includes aliased options in the option-definition set. When help is generated, the module resolves aliases as needed so that displayed argument and naming information correspond to the effective option definition.

**Test expectation:** Alias-backed entries display consistently with their underlying option definition according to the original module behavior.

### Scenario 3: Printing options that use single-dash formatting

A caller uses option definitions that are rendered in the module’s single-dash style. The module prints these entries using the single-dash formatter rather than the standard formatter.

**Test expectation:** Options assigned to this style are rendered with the single-dash form, and formatting remains consistent with the C implementation.

### Scenario 4: Sorting grouped options for display

A caller requests help for options that belong to groups or otherwise require grouped ordering. The module sorts the options and preserves the grouping semantics reflected by the original ordering logic.

**Test expectation:** The Rust implementation reproduces the same relative order and grouping-visible behavior as the C implementation for equivalent inputs.

### Scenario 5: Looking up a short option name

A caller needs to determine whether an option definition has a short-name form. The module searches the option definition and returns the associated short name when present.

**Test expectation:** The Rust implementation identifies the same short-name result as the original function for representative option definitions with and without short aliases.

## Requirements

### Functional Requirements

#### FR-1: Argument placeholder printing
The module shall render an option’s argument notation for help output based on the option definition and formatting inputs, including delimiter handling and argument-use tracking.

**Traceability:** `print_arg` in `src/parseopt/help.c`.

#### FR-2: Alias resolution
The module shall resolve an option definition through its alias relationship before downstream help/display logic consumes the effective definition where the original module does so.

**Traceability:** `opt_unalias` in `src/parseopt/help.c`.

#### FR-3: Option comparison for ordering
The module shall compare option entries according to the module-defined ordering rules used for help display.

**Traceability:** `optcmp` in `src/parseopt/help.c`.

#### FR-4: Merge-based ordered range combination
The module shall combine partially ordered option ranges into a single ordered range consistent with the comparison behavior used by the original module.

**Traceability:** `merge` in `src/parseopt/help.c`.

#### FR-5: Group sort support
The module shall sort grouped option collections for help output and maintain the group-head relationship needed by that sorting behavior.

**Traceability:** `sethead`, `sort_group`, and `struct optsort` in `src/parseopt/help.c`.

#### FR-6: Standard-style option entry rendering
The module shall render individual option help entries in the standard display style using the help context and option-definition data.

**Traceability:** `print_option_std` in `src/parseopt/help.c`.

#### FR-7: Single-dash-style option entry rendering
The module shall render individual option help entries in the single-dash display style using the help context and option-definition data.

**Traceability:** `print_option_sdash` in `src/parseopt/help.c`.

#### FR-8: Style-dispatched option printing
The module shall select and execute the appropriate option-entry printing behavior for a given option and report completion/status consistent with the original module’s `print_option` behavior.

**Traceability:** `print_option` in `src/parseopt/help.c`.

#### FR-9: Help context driven formatting
The module shall use a shared help-formatting context that links parseopt-wide settings with the option-definition set being printed.

**Traceability:** `struct help_context` and its use in `print_option_std`, `print_option_sdash`, and `print_option` in `src/parseopt/help.c`.

#### FR-10: Short-name discovery
The module shall determine the short-name representation associated with an option definition when present.

**Traceability:** `find_short_name` in `src/parseopt/parseopt.c`.

### Key Entities

#### `optdef`
Represents an option definition consumed by help rendering, alias resolution, ordering, and short-name lookup. It is the central entity for all evidenced module behavior.

**Relationships:**
- used by argument rendering,
- may reference or behave as an alias to another effective option definition,
- participates in ordering and grouped sorting,
- is printed through style-specific help renderers,
- is searched for short-name information.

**Traceability:** referenced across `print_arg`, `opt_unalias`, `merge`, `print_option_std`, `print_option_sdash`, `print_option`, `optcmp`, and `find_short_name`.

#### `parseopt`
Represents parseopt-level configuration and option-definition ownership used by help-generation routines.

**Relationships:**
- participates in help context,
- supplies module-wide settings affecting option printing and usage formatting.

**Traceability:** referenced in `src/parseopt/help.c` structures and help-related functions.

#### `help_context`
Represents the active formatting context for help output.

**Relationships:**
- links `parseopt` state with the current option set,
- is passed to style-specific option printers and the general option printer.

**Traceability:** `struct help_context` in `src/parseopt/help.c`, used by `print_option_std`, `print_option_sdash`, `print_option`.

#### `optsort`
Represents sorting state for grouped option ordering.

**Relationships:**
- holds data needed by group sorting,
- uses head/group metadata maintained by `sethead`,
- is consumed by `sort_group`.

**Traceability:** `struct optsort`, `sethead`, `sort_group` in `src/parseopt/help.c`.

#### `parseopt_help_format`
Represents help-format configuration used by the help subsystem.

**Relationships:**
- contributes formatting rules used while producing help output.

**Traceability:** `struct parseopt_help_format` in `src/parseopt/help.c`.

#### `usage_var_def`
Represents usage/help variable definition data used by the help subsystem.

**Relationships:**
- contributes to the surrounding help/usage formatting environment in which option rendering occurs.

**Traceability:** `struct usage_var_def` declarations in `src/parseopt/help.c`.

## Success Criteria

1. For identical option-definition inputs, the Rust implementation produces the same observable option ordering in help output as the C module’s ordering logic.
   - **Traceability:** `optcmp`, `merge`, `sethead`, `sort_group`.

2. The Rust implementation renders option argument placeholders and delimiters consistently with the original module for options with and without arguments.
   - **Traceability:** `print_arg`.

3. The Rust implementation resolves aliases in help-related processing so that displayed option information matches the original module’s effective-definition behavior.
   - **Traceability:** `opt_unalias`.

4. The Rust implementation supports both evidenced option display styles and selects the correct rendering path for each printed option.
   - **Traceability:** `print_option_std`, `print_option_sdash`, `print_option`.

5. The Rust implementation uses a formatting context equivalent in role to the original help context when rendering option entries.
   - **Traceability:** `struct help_context`, `print_option_std`, `print_option_sdash`, `print_option`.

6. The Rust implementation returns the same short-name lookup result as the original module for representative option definitions with present and absent short names.
   - **Traceability:** `find_short_name`.

7. End-to-end help-generation tests covering mixed options, aliases, grouped ordering, and single-dash formatting pass against fixtures derived from the C module’s observable output.
   - **Traceability:** `src/parseopt/help.c`, `src/parseopt/parseopt.c`.

## Out of Scope

The Rust rewrite specification does not require capabilities not evidenced in the analyzed module, including:

- creation of new public interfaces unrelated to the existing help/option-definition behavior,
- thread-safety guarantees,
- serialization or persistence,
- FFI design,
- performance benchmarking targets,
- error recovery features not visible in the source evidence.