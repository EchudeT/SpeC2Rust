# Specification: module_src_parseopt_optdef_04

- **Project**: cflow-new
- **Module**: module_src_parseopt_optdef_04
- **Category**: module_cluster
- **Rust branch**: `101-module_src_parseopt_optdef_04-rust-port`
- **Source basis**: `src/parseopt/help.c`, `src/parseopt/parseopt.c`
- **Generation date**: 2026-06-11

## 1. Overview

This module is responsible for option-definition presentation and related option-definition lookup support within the parseopt subsystem.

From the analyzed sources, its functional scope is limited to:

- formatting and printing option names and option arguments for help/usage output,
- resolving option aliases to their underlying option definition before display,
- ordering option definitions for grouped help presentation,
- selecting between standard and short-dash display styles for an option entry,
- locating an option's short-name form from an option definition.

The Rust rewrite must preserve the observable behavior of these responsibilities as used by the surrounding parseopt system. The rewrite must not add unrelated capabilities beyond these evidenced behaviors.

## 2. Feature Specification

### 2.1 Help and usage option rendering

The module shall render command-line option definitions into human-readable help output.

Supported rendering behavior evidenced by the source includes:

- printing option arguments together with an option entry,
- handling delimiter-sensitive argument printing,
- tracking argument-consumption state during formatting,
- printing option entries in at least two display styles:
  - a standard option style,
  - a short-dash-oriented style,
- producing a final option line through a dispatcher that selects the appropriate formatting path.

This behavior is traceable to `print_arg`, `print_option_std`, `print_option_sdash`, and `print_option` in `src/parseopt/help.c`.

### 2.2 Alias normalization for display

Before an option is displayed, the module shall be able to resolve an alias option definition to the effective underlying option definition used for presentation decisions.

This behavior is traceable to `opt_unalias` in `src/parseopt/help.c`.

### 2.3 Option ordering for help output

The module shall support deterministic ordering of option definitions for help output.

The ordering functionality shall include:

- comparing two option definitions according to module-defined help ordering rules,
- merging ordered subranges during sort processing,
- maintaining sortable group metadata,
- sorting option groups for presentation.

This behavior is traceable to `optcmp`, `merge`, `sethead`, and `sort_group` in `src/parseopt/help.c`.

### 2.4 Short-name lookup support

The module shall support lookup of an option definition's short-name representation when such a form exists.

This behavior is traceable to `find_short_name` in `src/parseopt/parseopt.c`.

## 3. User Scenarios & Testing

### 3.1 Scenario: Help output lists options in a stable, intended order

A caller prepares parseopt option definitions and requests help output. The module sorts or traverses those definitions according to the module's help-ordering rules, then prints them in that order.

**Expected result**:
- option entries appear in deterministic order for the same input definitions,
- grouped ordering behavior is preserved,
- the Rust port matches the C module's ordering for equivalent inputs.

**Test focus**:
- multiple options with different names,
- groups with more than one member,
- mixed long/short option sets,
- repeated runs yielding identical order.

### 3.2 Scenario: Help output displays option aliases as their effective option

A caller includes an alias option definition in the option table and requests help rendering. The module resolves the alias to the underlying effective option before making display decisions.

**Expected result**:
- alias handling does not produce an inconsistent or duplicate-format display path,
- the displayed characteristics correspond to the resolved option definition where the C code does so.

**Test focus**:
- direct option definitions,
- alias definitions referencing another option,
- mixed alias and non-alias entries in one help listing.

### 3.3 Scenario: Help output prints option arguments correctly

A caller requests rendering for options that accept arguments. The module prints the option name together with its argument notation, using the module's delimiter and argument-usage rules.

**Expected result**:
- required argument notation is shown when applicable,
- formatting respects delimiter behavior used by the C module,
- argument-use tracking prevents malformed output for multi-part option rendering.

**Test focus**:
- option with no argument,
- option with one argument,
- option entries where formatting state changes across rendering steps.

### 3.4 Scenario: The renderer chooses the correct display style

A caller requests printing of an option entry. Depending on option characteristics and context, the module selects the standard style or the short-dash style.

**Expected result**:
- the same option/context combinations choose the same display path as the C module,
- output text structure matches the corresponding C behavior.

**Test focus**:
- standard long-option cases,
- short-dash-oriented cases,
- neighboring options where formatting may depend on the next entry.

### 3.5 Scenario: Short-name lookup is available during parseopt processing

A caller needs the short-name form associated with an option definition. The module searches the definition and returns the short-name result used by surrounding parseopt logic.

**Expected result**:
- a valid short-name indicator is returned when one exists,
- absence of a short name is represented consistently with the C behavior.

**Test focus**:
- option with short name,
- option without short name,
- alias-related definitions if the surrounding code presents such inputs.

## 4. Requirements

### 4.1 Functional Requirements

#### FR-1: Render option arguments in help output
The Rust module shall format and emit option argument text for help output, including delimiter-aware behavior and argument-use tracking consistent with the C module.

**Traceability**: `print_arg` in `src/parseopt/help.c`.

#### FR-2: Resolve aliases for display logic
The Rust module shall resolve an aliased option definition to its effective underlying option definition before downstream help-formatting decisions that depend on the resolved option.

**Traceability**: `opt_unalias` in `src/parseopt/help.c`.

#### FR-3: Support standard option-entry formatting
The Rust module shall format option help entries using the standard display style used by the C module.

**Traceability**: `print_option_std` in `src/parseopt/help.c`.

#### FR-4: Support short-dash option-entry formatting
The Rust module shall format option help entries using the short-dash-oriented display style used by the C module.

**Traceability**: `print_option_sdash` in `src/parseopt/help.c`.

#### FR-5: Dispatch option rendering through module-defined selection logic
The Rust module shall provide the same rendering decision behavior as the C module when choosing how to print an option entry.

**Traceability**: `print_option` in `src/parseopt/help.c`.

#### FR-6: Compare options according to help-order rules
The Rust module shall compare option definitions using ordering rules equivalent to the C module's help comparator.

**Traceability**: `optcmp` in `src/parseopt/help.c`.

#### FR-7: Merge ordered ranges during sorting
The Rust module shall support merging ordered option-definition ranges as part of producing final help-output order.

**Traceability**: `merge` in `src/parseopt/help.c`.

#### FR-8: Maintain sortable group head metadata
The Rust module shall support initialization/update of group-head state needed by the module's option-group sorting behavior.

**Traceability**: `sethead` in `src/parseopt/help.c`.

#### FR-9: Sort option groups for presentation
The Rust module shall sort option groups for help presentation using behavior equivalent to the C module.

**Traceability**: `sort_group` in `src/parseopt/help.c`.

#### FR-10: Find an option definition's short name
The Rust module shall provide short-name lookup for an option definition, including a consistent representation for "not found" cases.

**Traceability**: `find_short_name` in `src/parseopt/parseopt.c`.

### 4.2 Key Entities

#### `optdef`
The option-definition entity is the central record used by this module. It represents a command-line option and supplies the information needed for:

- alias resolution,
- short-name lookup,
- ordering comparisons,
- help-output rendering.

**Traceability**: referenced throughout `src/parseopt/help.c` and `src/parseopt/parseopt.c`, including `print_arg`, `opt_unalias`, `print_option_*`, `optcmp`, `find_short_name`.

#### `parseopt`
The parseopt entity provides the surrounding option-processing context from which help-formatting and usage-related data are obtained.

**Traceability**: local anonymous struct references in `src/parseopt/help.c`.

#### `help_context`
The help-context entity bundles the state needed while rendering option help entries, including access to parseopt state and option collections.

**Traceability**: `struct help_context` in `src/parseopt/help.c`, used by `print_option_std`, `print_option_sdash`, `print_option`.

#### `optsort`
The optsort entity holds sorting/grouping state used to order option definitions for help presentation.

**Traceability**: `struct optsort`, `sethead`, `sort_group` in `src/parseopt/help.c`.

#### `parseopt_help_format`
This formatting entity captures help-format configuration used by the help-rendering path.

**Traceability**: `struct parseopt_help_format` in `src/parseopt/help.c`.

#### `usage_var_def`
This usage-related entity represents variable/placeholder definitions used when constructing usage/help text.

**Traceability**: `struct usage_var_def` occurrences in `src/parseopt/help.c`.

## 5. Success Criteria

### 5.1 Behavioral parity
For the option-definition inputs exercised by existing module callers, the Rust implementation produces help/usage option text that is equivalent in content and structure to the C module for:

- argument rendering,
- alias-aware display behavior,
- standard-format entries,
- short-dash-format entries.

**Traceability**: `print_arg`, `opt_unalias`, `print_option_std`, `print_option_sdash`, `print_option`.

### 5.2 Ordering parity
For the same collections of option definitions and grouping context, the Rust implementation yields the same option ordering as the C module.

**Traceability**: `optcmp`, `merge`, `sethead`, `sort_group`.

### 5.3 Short-name lookup parity
For representative option definitions with and without short names, the Rust implementation returns the same lookup result category as the C module.

**Traceability**: `find_short_name`.

### 5.4 Deterministic repeated output
Running the Rust implementation multiple times with identical option-definition input and formatting context yields identical ordering and help-output text.

**Traceability**: module behavior centered on `sort_group`, `merge`, `optcmp`, and `print_option`.

### 5.5 Scope fidelity
The Rust rewrite confines itself to the evidenced responsibilities of this module: option-definition formatting, alias normalization for display, presentation ordering, and short-name lookup support.

**Traceability**: all analyzed functions across `src/parseopt/help.c` and `src/parseopt/parseopt.c`.

## 6. Out of Scope

The Rust rewrite specification does not require, because the provided analysis does not evidence them:

- new public APIs beyond those needed to preserve existing module behavior,
- new command-line parsing semantics,
- configuration formats, serialization, or persistence,
- thread-safety guarantees,
- FFI layers,
- recovery or diagnostic systems beyond existing behavior,
- performance targets or benchmarks.