# spec.md

## Title

Functional Specification for `module_src_parseopt_help.c_11` Rust Port

## Metadata

- Project: `cflow-new`
- Source module: `src/parseopt/help.c`
- Module category: `module_cluster`
- Target branch: `108-module_src_parseopt_help.c_11-rust-port`
- Generation date: `2026-06-17`

## Overview

This module is responsible for producing formatted command-line help and usage output for a parse-option system.

The Rust rewrite must preserve the module’s observable behavior as a help-formatting component that:

- derives usage/help text from parser state and option definitions,
- formats usage variables and option descriptions,
- orders option entries for display,
- lays out help text using width-aware formatting rules,
- emits grouped help content through a formatting context.

The module is a presentation-layer companion to option parsing data. It does not define option parsing itself; it consumes parser and option-definition data to generate help output.

## Scope

### In Scope

The Rust version must implement behavior evidenced by `src/parseopt/help.c`, including:

- formatting usage/help text from parse-option metadata,
- representing formatting parameters and help-generation context,
- deriving displayable option entries from option definitions,
- sorting option entries for stable help presentation,
- wrapping and aligning help output according to formatting constraints,
- handling usage variable definitions used in usage text generation.

### Out of Scope

The Rust version is not required by this module specification to implement:

- option parsing logic itself,
- command execution,
- configuration loading,
- localization beyond source-evidenced text behavior,
- public APIs unrelated to help/usage generation.

## Feature Specification

### Feature Summary

The module generates user-facing help text for a command-line parser by traversing parser definitions and rendering a structured usage display.

The Rust implementation must support the following feature set.

### F1. Help formatting configuration

The module uses a dedicated formatting description (`parseopt_help_format`) to control how help text is laid out. The Rust version must preserve the role of this formatting state in determining:

- indentation and alignment of option text and descriptions,
- line-width-sensitive wrapping decisions,
- spacing between syntactic elements in usage/help output.

This formatting configuration is used by help-generation operations and must be consistently applied across the produced output.

**Traceability:** `struct parseopt_help_format` in `src/parseopt/help.c:29`

### F2. Usage variable definition support

The module represents usage variables through dedicated usage-variable definition records. The Rust version must support using these definitions when building usage text, including preserving the relationship between parser state and usage-variable metadata.

The specification does not require inventing additional variable semantics; only the observed role of usage variable definitions in help/usage generation must be preserved.

**Traceability:** `struct usage_var_def` in `src/parseopt/help.c:40-45`, `47`, `65`

### F3. Consumption of parse-option state

The module operates on parse-option parser state (`parseopt`) and derives display content from it. The Rust version must accept or represent equivalent parser metadata sufficient to:

- obtain usage-related information,
- enumerate option definitions relevant to help output,
- bind parser-level context into a help-generation pass.

**Traceability:** `struct parseopt` references in `src/parseopt/help.c:63`, `131`, `178`, `259`, `293`, `399`

### F4. Option-definition-based help generation

The module builds help content from option definitions (`optdef`). The Rust version must preserve this behavior by generating displayable option help entries from option-definition records, including both option syntax and associated descriptive text where present.

**Traceability:** `struct optdef` references in `src/parseopt/help.c:157`, `169`, `170`, `181`, `265`, `268`, `328`, `332`, `346`, `400`, `409`, `412`, `472`, `479`

### F5. Stable option ordering for help display

The module defines an option-sorting helper structure (`optsort`) and a helper utility (`min`) used during formatting-related operations. The Rust version must preserve deterministic ordering of option entries in generated help output, consistent with the source module’s role in preparing sorted display content.

The specification requires deterministic, repeatable ordering; it does not require exposing sorting as a public feature.

**Traceability:** `struct optsort` in `src/parseopt/help.c:330-337`, `365`; `min` in `src/parseopt/help.c:339-343`

### F6. Help-generation context management

The module maintains a dedicated help-generation context (`help_context`) that ties together parser state, option state, and formatting/output progress during help construction. The Rust version must preserve the role of this context as the coordinating state for one help-rendering operation.

**Traceability:** `struct help_context` in `src/parseopt/help.c:397-405`, `408`, `471`, `499`

### F7. Width-aware text layout

The module contains formatting logic evidenced by a formatting structure, context structure, and utility operations used during display generation. The Rust version must preserve behavior in which generated help text is formatted to fit within configured layout constraints, including:

- aligning option names and descriptions,
- wrapping longer descriptive text across multiple lines,
- continuing wrapped text with consistent indentation.

**Traceability:** `struct parseopt_help_format` in `src/parseopt/help.c:29`; `struct help_context` in `src/parseopt/help.c:397-405`; `min` in `src/parseopt/help.c:339-343`

## User Scenarios & Testing

### Scenario 1: Display basic command usage

A command integrates parse-option definitions and requests help/usage output. The module produces a usage line or usage section derived from parser metadata and usage-variable definitions.

#### Expected behavior

- Usage text reflects parser-provided usage information.
- Usage variables are incorporated into the output in their defined form.
- Output is formatted according to the module’s help-format configuration.

#### Test focus

- Verify that usage text is generated from parser state.
- Verify that usage-variable metadata contributes to the resulting usage display.
- Verify that layout remains readable and consistent.

**Traceability:** `parseopt`, `usage_var_def`, `parseopt_help_format`

### Scenario 2: Display a list of options with descriptions

A command has multiple option definitions, including short and/or long forms and descriptive help text. The module produces a help section listing those options.

#### Expected behavior

- All eligible option definitions are included in output.
- Each option entry includes its display syntax and associated descriptive text when defined.
- Option entries are aligned in a readable columnar form.

#### Test focus

- Verify inclusion of defined options.
- Verify formatting of option labels and descriptions.
- Verify alignment across entries of varying label widths.

**Traceability:** `optdef`, `help_context`, `parseopt_help_format`

### Scenario 3: Long descriptions wrap cleanly

Some option descriptions exceed the available help width.

#### Expected behavior

- Descriptions wrap onto continuation lines instead of overflowing.
- Continuation lines maintain the correct indentation relative to the option label column.
- Wrapping behavior is deterministic for the same formatting configuration.

#### Test focus

- Verify wrapped output under narrow and moderate widths.
- Verify continuation indentation.
- Verify no loss of description text during wrapping.

**Traceability:** `parseopt_help_format`, `help_context`, `min`

### Scenario 4: Help output ordering is deterministic

A command defines multiple options in a way that requires internal ordering for display.

#### Expected behavior

- The same parser metadata always yields the same help ordering.
- Ordering is stable across repeated invocations.
- Ordering behavior applies consistently to the generated option list.

#### Test focus

- Verify repeated runs produce identical option ordering.
- Verify display order is independent of unrelated runtime state.

**Traceability:** `optsort`, `optdef`

### Scenario 5: Mixed usage and option help in one rendering flow

A command emits both usage text and an option help section as part of a single help request.

#### Expected behavior

- A single help-rendering operation can coordinate parser data, usage variables, and option entries.
- Formatting remains consistent across sections.
- The context used for rendering preserves section-to-section coherence.

#### Test focus

- Verify combined output contains both usage and options sections.
- Verify consistent formatting rules across all sections.
- Verify no section corrupts or resets the formatting of the next unexpectedly.

**Traceability:** `help_context`, `parseopt`, `usage_var_def`, `optdef`

## Requirements

### Functional Requirements

#### FR-1: Generate usage/help output from parser metadata

The Rust module shall generate user-facing help/usage text by consuming parse-option parser state and related help metadata.

**Traceability:** `struct parseopt` in `src/parseopt/help.c`

#### FR-2: Support usage-variable-driven usage text

The Rust module shall incorporate usage variable definitions into generated usage output where parser metadata references them.

**Traceability:** `struct usage_var_def` in `src/parseopt/help.c:40-45`, `47`, `65`

#### FR-3: Generate option help entries from option definitions

The Rust module shall derive help-display entries from option-definition records and include option syntax plus descriptive text when available.

**Traceability:** `struct optdef` in `src/parseopt/help.c`

#### FR-4: Apply dedicated help-formatting rules

The Rust module shall apply a dedicated help-format configuration to control spacing, indentation, alignment, and width-sensitive rendering decisions.

**Traceability:** `struct parseopt_help_format` in `src/parseopt/help.c:29`

#### FR-5: Order option entries deterministically for display

The Rust module shall prepare option entries in a deterministic order for help output.

**Traceability:** `struct optsort` in `src/parseopt/help.c:330-337`, `365`

#### FR-6: Wrap long help text within formatting constraints

The Rust module shall wrap option-description text and other long help text to fit within active formatting width constraints while preserving readable indentation.

**Traceability:** `struct parseopt_help_format`, `struct help_context`, `min`

#### FR-7: Maintain per-render help context

The Rust module shall maintain rendering context that associates parser state, option data, and formatting progress for a single help-generation operation.

**Traceability:** `struct help_context` in `src/parseopt/help.c:397-405`, `408`, `471`, `499`

#### FR-8: Preserve repeated-run output consistency

Given the same parser metadata and formatting configuration, the Rust module shall produce the same help output across repeated invocations.

**Traceability:** `struct optsort`, `struct help_context`, formatting structures in `src/parseopt/help.c`

### Key Entities

#### `ParseoptHelpFormat`

Represents the formatting policy for help generation.

Role in the module:

- defines layout constraints used during rendering,
- influences indentation, alignment, and wrapping behavior,
- is consumed by the active help-generation context.

**Traceability:** `struct parseopt_help_format` in `src/parseopt/help.c:29`

#### `UsageVarDef`

Represents a usage-variable definition used to construct usage text.

Role in the module:

- provides metadata for variable portions of usage output,
- is associated with parser-level usage generation,
- participates in rendering usage lines/sections.

**Traceability:** `struct usage_var_def` in `src/parseopt/help.c:40-45`, `47`, `65`

#### `Parseopt`

Represents parser metadata consumed by this module.

Role in the module:

- supplies the source usage/help information,
- provides access to option definitions and related parser state,
- anchors the overall help-generation operation.

Relationship:

- one help-generation operation is based on one parser context,
- parser context is referenced by help context and usage handling.

**Traceability:** `struct parseopt` references in `src/parseopt/help.c`

#### `Optdef`

Represents an option definition used to build display entries.

Role in the module:

- serves as the source record for option syntax shown in help,
- carries or links to descriptive text used in help output,
- participates in ordering and formatting during rendering.

Relationship:

- option definitions are owned or referenced by parser metadata,
- ordered/display-ready option entries derive from these definitions.

**Traceability:** `struct optdef` references in `src/parseopt/help.c`

#### `Optsort`

Represents sorting state for option-display preparation.

Role in the module:

- supports deterministic ordering of options before rendering,
- links display ordering logic to option definitions.

Relationship:

- built from or associated with option definitions,
- used before or during help output assembly.

**Traceability:** `struct optsort` in `src/parseopt/help.c:330-337`, `365`

#### `HelpContext`

Represents the active state of a help-rendering pass.

Role in the module:

- ties together parser metadata, option data, and formatting state,
- tracks the current rendering operation,
- ensures formatting rules are applied consistently across output sections.

Relationship:

- references parser data and current option-related data,
- consumes help-format settings during rendering.

**Traceability:** `struct help_context` in `src/parseopt/help.c:397-405`, `408`, `471`, `499`

## Success Criteria

### SC-1: Usage generation correctness

For parser inputs containing usage metadata and usage variables, the Rust module produces usage output that includes both fixed and variable usage elements.

**Traceability:** `parseopt`, `usage_var_def`

### SC-2: Option help completeness

For parser inputs containing option definitions, the Rust module’s help output includes all defined displayable option entries expected by the source module behavior.

**Traceability:** `optdef`, `parseopt`

### SC-3: Deterministic ordering

Given identical parser metadata, repeated generation of help output produces identical option ordering.

**Traceability:** `optsort`

### SC-4: Width-constrained readability

For inputs with long descriptions and constrained formatting width, output remains width-aware by wrapping text onto continuation lines with preserved indentation rather than producing a single unstructured overflow line.

**Traceability:** `parseopt_help_format`, `help_context`, `min`

### SC-5: Consistent section formatting

When generating combined usage and option help, the Rust module applies one coherent formatting policy across both sections within the same rendering operation.

**Traceability:** `help_context`, `parseopt_help_format`

### SC-6: Stable output under repeated rendering

Given identical parser state and formatting settings, the entire generated help text is byte-for-byte stable across repeated invocations in the same environment.

**Traceability:** `help_context`, `optsort`, formatting structures in `src/parseopt/help.c`

## Acceptance Notes

- Conformance should be evaluated through golden-output tests built from representative parser metadata and option-definition sets.
- Tests should cover short help, full help, wrapped descriptions, mixed usage-plus-options output, and repeated-run stability.
- Any Rust API surface may differ from the C implementation, but the externally observable help-generation behavior described above must be preserved.