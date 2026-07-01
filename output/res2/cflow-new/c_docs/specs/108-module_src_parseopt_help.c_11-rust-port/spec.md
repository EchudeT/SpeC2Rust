# spec.md

## Title

Functional Specification for `module_src_parseopt_help.c_11` Rust Port

## Document Control

- Project: `cflow-new`
- Module: `module_src_parseopt_help.c_11`
- Category: `module_cluster`
- Source file: `src/parseopt/help.c`
- Target Rust branch: `108-module_src_parseopt_help.c_11-rust-port`
- Generation date: `2026-06-17`

## 1. Feature Specification

### 1.1 Purpose

This module is responsible for generating command-line help and usage output for the parse option subsystem.

The Rust rewrite must preserve the module behavior evidenced by `src/parseopt/help.c`, namely:

- formatting usage/help text from parser state and option definitions,
- representing usage-variable substitutions used inside usage text,
- organizing option definitions for display,
- ordering displayed options in a stable, user-facing help listing,
- rendering help output with line-width-aware formatting and alignment.

### 1.2 In-Scope Functionality

Based on the analyzed file and its internal data structures, the Rust version must implement the following functional boundary:

1. Accept parse-option state and option-definition data as input to help generation.
2. Produce usage/help text that combines:
   - overall command usage text,
   - option names and argument forms,
   - associated descriptive text,
   - variable-derived substitutions used by usage text formatting.
3. Support internal help-format configuration through a formatting context comparable to `parseopt_help_format`.
4. Sort or otherwise order options for presentation using the behavior associated with the option-sorting context (`optsort`) and comparison support evidenced by `min` and the sorting-related structures.
5. Use a help-generation context comparable to `help_context` to coordinate parser state, current option selection, and output formatting state.

### 1.3 Out of Scope

The specification does not require any capability not evidenced by `src/parseopt/help.c`. In particular, the Rust port must not add requirements for:

- new public command-line parsing features,
- configuration persistence,
- serialization,
- thread-safety guarantees,
- FFI interfaces,
- error recovery behaviors beyond those needed for help text generation,
- benchmarking or performance promises beyond preserving module behavior.

## 2. User Scenarios & Testing

### 2.1 Scenario: Display command usage summary

A command invokes the parse-option help subsystem to print a concise usage summary. The module receives parser metadata and usage text templates and returns or emits formatted usage output.

The Rust version must support testing that:

- a usage summary is produced from parser-owned usage information,
- usage variables are substituted consistently where defined,
- the result is formatted as user-facing help text rather than raw internal data.

Traceability: `struct parseopt_help_format`, `struct usage_var_def`, `struct parseopt`, `struct help_context` in `src/parseopt/help.c`.

### 2.2 Scenario: Display option list with aligned descriptions

A command has multiple options, each with one or more display names and a description. The module formats a readable help section where option spellings and descriptive text are aligned according to formatting rules.

The Rust version must support testing that:

- option entries are rendered as a list,
- descriptions are associated with the correct option entry,
- line layout respects the module’s formatting role, including spacing/alignment behavior driven by the help format context.

Traceability: `struct optdef`, `struct parseopt_help_format`, `struct help_context` in `src/parseopt/help.c`.

### 2.3 Scenario: Display options in sorted presentation order

When help is generated, option definitions are prepared for display in a deterministic order rather than arbitrary declaration order.

The Rust version must support testing that:

- a collection of option definitions can be ordered for display,
- the ordering is repeatable for the same input,
- sorting behavior is tied to option-display generation rather than unrelated parser behavior.

Traceability: `struct optsort`, `struct optdef`, `min` in `src/parseopt/help.c`.

### 2.4 Scenario: Handle mixed option widths and wrapped text

Some options have short display forms, while others have longer display forms or argument placeholders. The help generator must still produce readable output when option labels and descriptions have different lengths.

The Rust version must support testing that:

- long option labels do not corrupt neighboring text,
- description text remains associated with the correct option,
- formatting logic handles width-sensitive layout decisions.

Traceability: `struct parseopt_help_format`, `struct help_context`, `min` in `src/parseopt/help.c`.

### 2.5 Scenario: Usage text depends on named variables

Usage/help text may include placeholders or variable-based fragments whose values are defined separately.

The Rust version must support testing that:

- usage-variable definitions are recognized by name,
- the corresponding values are inserted into generated usage/help text,
- absent or unrelated variables do not affect unrelated output sections.

Traceability: `struct usage_var_def` in `src/parseopt/help.c`.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Help generation from parser and option metadata

The module shall generate help/usage output from parse-option state and associated option definitions.

Traceability: `struct parseopt`, `struct optdef`, `struct help_context` in `src/parseopt/help.c`.

#### FR-2: Usage text formatting

The module shall format command usage text using a dedicated help-format configuration context.

Traceability: `struct parseopt_help_format` in `src/parseopt/help.c`.

#### FR-3: Usage-variable substitution

The module shall support named usage-variable definitions and use them during usage/help text generation where applicable.

Traceability: `struct usage_var_def` in `src/parseopt/help.c`.

#### FR-4: Option entry rendering

The module shall render option definitions into user-visible help entries that include option identifiers and associated descriptive text.

Traceability: `struct optdef` in `src/parseopt/help.c`.

#### FR-5: Width-aware help layout

The module shall apply width-sensitive layout behavior when formatting help output so that option text and descriptions are presented readably across varying entry lengths.

Traceability: `struct parseopt_help_format`, `struct help_context`, `min` in `src/parseopt/help.c`.

#### FR-6: Deterministic option ordering for help display

The module shall prepare option definitions for display in a deterministic presentation order suitable for help output.

Traceability: `struct optsort`, `struct optdef`, `min` in `src/parseopt/help.c`.

#### FR-7: Help-generation context coordination

The module shall maintain a help-generation context that ties together parser input, option-definition input, and current formatting state during rendering.

Traceability: `struct help_context` in `src/parseopt/help.c`.

### 3.2 Key Entities

#### Parse Option State

A parser-related entity represented by `struct parseopt`. It supplies the command-level context from which usage/help output is derived.

Relationship:
- referenced by the help-generation context,
- associated with collections of option definitions,
- contributes top-level usage information.

Traceability: `struct parseopt` occurrences in `src/parseopt/help.c`.

#### Option Definition

An option-description entity represented by `struct optdef`. It describes an individual command-line option as displayed in help output.

Relationship:
- consumed by help rendering,
- may be collected and ordered through the option-sorting entity,
- associated with descriptive text shown in the help listing.

Traceability: `struct optdef` occurrences in `src/parseopt/help.c`.

#### Help Format Configuration

A formatting entity represented by `struct parseopt_help_format`. It controls presentation characteristics used while producing usage/help output.

Relationship:
- applied by the help-generation context,
- influences width/alignment behavior for rendered text.

Traceability: `struct parseopt_help_format` in `src/parseopt/help.c`.

#### Usage Variable Definition

A named substitution entity represented by `struct usage_var_def`. It provides replacement values used while constructing usage/help text.

Relationship:
- consulted during usage rendering,
- affects textual expansion in command usage output.

Traceability: `struct usage_var_def` in `src/parseopt/help.c`.

#### Option Sorting Context

A display-order entity represented by `struct optsort`. It supports arranging option definitions for help presentation.

Relationship:
- references option definitions,
- used before or during help listing generation to establish display order.

Traceability: `struct optsort` in `src/parseopt/help.c`.

#### Help Generation Context

An orchestration entity represented by `struct help_context`. It holds the active state needed to render help output.

Relationship:
- references parser state,
- references one or more option definitions,
- applies help-format configuration during output generation.

Traceability: `struct help_context` in `src/parseopt/help.c`.

## 4. Success Criteria

### 4.1 Behavioral Equivalence Criteria

1. For the same parser state, option definitions, and usage-variable inputs represented by the source module entities, the Rust version produces the same logical help content sections as the C module:
   - usage text,
   - option listing,
   - option descriptions.

   Traceability: `struct parseopt`, `struct optdef`, `struct usage_var_def`, `struct help_context`.

2. The Rust version preserves deterministic help-display ordering for option definitions for repeated runs with identical input.

   Traceability: `struct optsort`, `struct optdef`, `min`.

3. The Rust version preserves width-aware formatting behavior such that option labels and descriptions remain readable and correctly associated under mixed entry lengths.

   Traceability: `struct parseopt_help_format`, `struct help_context`, `min`.

### 4.2 Testability Criteria

1. Tests can construct parser/option input state and verify that generated help output contains expected usage text and expected option entries.

   Traceability: `struct parseopt`, `struct optdef`.

2. Tests can provide usage-variable definitions and verify that expected substitutions appear in generated usage/help text.

   Traceability: `struct usage_var_def`.

3. Tests can provide the same set of option definitions in repeated executions and verify identical display order in the resulting help output.

   Traceability: `struct optsort`, `struct optdef`.

4. Tests can cover both short and long option display forms and verify that help text remains properly structured and legible.

   Traceability: `struct parseopt_help_format`, `struct help_context`, `min`.

### 4.3 Port Completion Criteria

The Rust port is complete for this module when:

- all in-scope behaviors listed in Section 1.2 are implemented,
- all functional requirements in Section 3.1 are satisfied,
- scenario-based tests in Section 2 are supported,
- no extra externally visible capabilities are introduced beyond the evidenced help/usage generation role of `src/parseopt/help.c`.