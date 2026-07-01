# spec.md

## Title

Rust Functional Specification for `src/parseopt/help.c`

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_src_parseopt_help.c_11`
- **Category**: `module_cluster`
- **Source File**: `src/parseopt/help.c`
- **Target Rust Branch**: `108-module_src_parseopt_help.c_11-rust-port`
- **Generation Date**: `2026-06-11`

## Overview

This module is responsible for producing command-line help and usage output from parser option definitions and help-format configuration. The Rust rewrite must preserve the module’s role as the formatter and presenter of usage text, option listings, and related help sections derived from parse-option metadata.

The source evidence shows that this module works with:
- parse-option configuration objects,
- option definition records,
- usage-variable substitution records,
- formatting context for help generation,
- option ordering support used during output generation.

The Rust version must implement the same functional boundary: transform parse-option metadata into formatted help/usage text suitable for command-line presentation.

## Feature Specification

### Summary

The module formats and emits help text for a command-line option parser. It consumes parser state and option definitions, derives presentation details from them, and outputs a structured help view that includes usage-related text and option descriptions.

### In-Scope Behavior

The Rust rewrite must support the following behaviors evidenced by the source module:

1. **Usage/help formatting from parser metadata**
   The module must accept parse-option data and produce help output based on that data rather than requiring manually assembled help text.

2. **Option-definition based help listing**
   The module must format help entries from option-definition records, including per-option display text and associated descriptive text when present.

3. **Usage variable handling**
   The module must support usage-related variable definitions referenced by the help-format logic so that generated usage/help text can include configured variable-driven substitutions or expansions.

4. **Help output layout using a formatting context**
   The module must maintain enough contextual information during generation to format help consistently across the whole output, including parser context, option ranges or collections, and computed presentation data.

5. **Option ordering for presentation**
   The module must support ordering or sorting of option definitions for display when generating help text.

6. **Bounded formatting calculations**
   The module must apply bounded size comparisons during formatting-related calculations, as evidenced by the local `min(size_t, size_t)` helper used by this module.

### Out of Scope

The Rust rewrite must not introduce unevidenced capabilities, including:
- new command-line parsing behavior,
- new public configuration features unrelated to help generation,
- persistence or serialization of help metadata,
- concurrency guarantees,
- FFI-facing interfaces,
- interactive help systems.

## User Scenarios & Testing

### Scenario 1: Generate basic command usage text

**Given** a parser configuration with defined options and usage formatting data
**When** help output is requested
**Then** the module produces usage text derived from the parser metadata
**And** the output is structured for command-line display.

**Test focus**:
- usage text is generated from parse-option data,
- formatting is deterministic for the same input,
- no required option entry is omitted.

### Scenario 2: Display a list of available options with descriptions

**Given** a set of option definitions that include names and help descriptions
**When** the module formats help output
**Then** each displayable option appears in the help listing
**And** each entry is associated with its descriptive text when defined.

**Test focus**:
- option entries are included once,
- descriptions align with the corresponding options,
- output remains readable for mixed short and long option forms if present in the source metadata.

### Scenario 3: Apply usage-variable driven formatting

**Given** help-format configuration with usage-variable definitions
**When** usage/help text is generated
**Then** the resulting output reflects those variable definitions in the formatted usage text.

**Test focus**:
- variable-backed formatting is applied,
- missing or empty variable content does not corrupt the surrounding help layout,
- repeated generation with the same variable definitions yields the same output.

### Scenario 4: Present options in module-defined order

**Given** option definitions that require presentation ordering
**When** the module prepares the option list for output
**Then** the displayed help follows the module’s established ordering behavior.

**Test focus**:
- ordering is stable for identical inputs,
- sorted output is used consistently across the whole option section,
- ordering logic does not drop entries.

### Scenario 5: Format long or uneven help content safely

**Given** option/help text with varying lengths
**When** the module computes formatting widths or output bounds
**Then** bounded calculations are applied so output preparation respects size comparisons used by the original module.

**Test focus**:
- formatting logic handles short and long strings,
- width-related calculations remain bounded,
- no malformed truncation or overrun-like behavior appears in generated output.

## Requirements

### Functional Requirements

#### FR-1: Generate help output from parse-option metadata
The Rust module shall generate help/usage output from parse-option structures represented in the original source by `struct parseopt` references in `src/parseopt/help.c`.

**Traceability**: `src/parseopt/help.c`; `struct parseopt` occurrences at lines 63, 131, 178, 259, 293, 399.

#### FR-2: Format option entries from option definitions
The Rust module shall format help entries from option definition records represented in the original source by `struct optdef`, preserving the relationship between option identity and displayed help text.

**Traceability**: `src/parseopt/help.c`; `struct optdef` occurrences at lines 157, 169, 170, 181, 265, 268, 328, 332, 346, 400, 409, 412, 472, 479.

#### FR-3: Support usage-format configuration
The Rust module shall support help/usage generation behavior controlled by a help-format configuration equivalent to `struct parseopt_help_format`.

**Traceability**: `src/parseopt/help.c`; `struct parseopt_help_format` at line 29.

#### FR-4: Support usage-variable definitions in formatted output
The Rust module shall support usage-variable definitions equivalent to `struct usage_var_def` and apply them in the generation of usage/help text where the source module does so.

**Traceability**: `src/parseopt/help.c`; `struct usage_var_def` at lines 40-45, 47, 65.

#### FR-5: Maintain generation context across help rendering
The Rust module shall maintain a help-generation context equivalent in role to `struct help_context` so that parser state, option collections, and derived formatting data remain available throughout help rendering.

**Traceability**: `src/parseopt/help.c`; `struct help_context` at lines 397-405, 408, 471, 499.

#### FR-6: Support ordered presentation of options
The Rust module shall support option ordering behavior equivalent in role to `struct optsort` during help generation.

**Traceability**: `src/parseopt/help.c`; `struct optsort` at lines 330-337, 365.

#### FR-7: Use bounded minimum comparisons in formatting calculations
The Rust module shall preserve the source module’s bounded comparison behavior where formatting logic requires selecting the lesser of two size values.

**Traceability**: `src/parseopt/help.c`; `min` at lines 339-343.

### Key Entities

#### `ParseoptHelpFormat`
A help-format configuration entity corresponding to `struct parseopt_help_format`. It defines formatting-related inputs used when producing usage/help text.

**Relationship**:
- used alongside parser metadata,
- may reference usage-variable definitions conceptually through the formatting process.

**Traceability**: `src/parseopt/help.c:29`.

#### `UsageVarDef`
A usage-variable definition entity corresponding to `struct usage_var_def`. It represents named or structured variable content used by usage/help formatting.

**Relationship**:
- consumed by help-format logic,
- contributes to final usage text generation.

**Traceability**: `src/parseopt/help.c:40-45, 47, 65`.

#### `Parseopt`
A parser metadata entity corresponding to `struct parseopt`. It is the primary source of command/option information for help generation.

**Relationship**:
- owns or references option definitions,
- is referenced by the help-generation context,
- is interpreted using help-format configuration.

**Traceability**: `src/parseopt/help.c:63, 131, 178, 259, 293, 399`.

#### `Optdef`
An option definition entity corresponding to `struct optdef`. It represents one command-line option as displayed in generated help.

**Relationship**:
- belongs to or is derived from parser metadata,
- may be collected and ordered for output,
- is rendered by the help-generation context into visible help entries.

**Traceability**: `src/parseopt/help.c:157, 169, 170, 181, 265, 268, 328, 332, 346, 400, 409, 412, 472, 479`.

#### `Optsort`
An ordering/support entity corresponding to `struct optsort`. It represents the state or data needed to sort/order option definitions for presentation.

**Relationship**:
- references option definitions,
- is used before or during final help rendering.

**Traceability**: `src/parseopt/help.c:330-337, 332, 365`.

#### `HelpContext`
A rendering-context entity corresponding to `struct help_context`. It bundles parser references, option-related data, and derived formatting state needed while generating help output.

**Relationship**:
- references `Parseopt`,
- references one or more `Optdef` records,
- drives the final rendering flow.

**Traceability**: `src/parseopt/help.c:397-405, 399, 400, 408, 471, 472, 479, 499`.

## Success Criteria

1. **Functional parity of help generation**
   For equivalent parse-option metadata and help-format inputs, the Rust module generates usage/help output covering the same functional content categories as the C module: usage text, option entries, and option descriptions where defined.
   **Traceability**: `struct parseopt`, `struct optdef`, `struct parseopt_help_format`, `src/parseopt/help.c`.

2. **Usage-variable support preserved**
   Inputs equivalent to the original module’s usage-variable definitions influence generated usage/help text in the Rust version, and tests demonstrate that variable-driven formatting is applied.
   **Traceability**: `struct usage_var_def`, `src/parseopt/help.c`.

3. **Option ordering preserved**
   The Rust module presents options in a deterministic order consistent with the source module’s ordering role, with no dropped or duplicated option entries in generated help.
   **Traceability**: `struct optsort`, `struct optdef`, `src/parseopt/help.c`.

4. **Context-driven rendering preserved**
   The Rust design includes a rendering context equivalent in function to the original `help_context`, and tests show that help generation can be performed from parser plus option metadata without requiring out-of-module state.
   **Traceability**: `struct help_context`, `struct parseopt`, `struct optdef`.

5. **Bounded size logic preserved where applicable**
   Tests covering width/length-sensitive formatting confirm that formatting decisions using size comparisons behave correctly at boundary values and do not exceed the intended lesser bound.
   **Traceability**: `min(size_t, size_t)` at lines 339-343.

6. **Deterministic output for identical inputs**
   Repeated help generation with the same parser metadata, option definitions, usage variables, and formatting configuration produces the same output.
   **Traceability**: all core entities in `src/parseopt/help.c`.

## Notes for Rewrite Scope

- Preserve the module as a help/usage formatting component.
- Keep data-model and behavior mapping faithful to the source roles evidenced by the listed structures.
- Do not extend the scope beyond help generation and related formatting/orchestration.