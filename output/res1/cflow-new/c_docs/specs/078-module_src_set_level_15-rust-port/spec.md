# spec.md

## Title

Functional Specification: `module_src_set_level_15` Rust Port

## Metadata

- Project: `cflow-new`
- Module: `module_src_set_level_15`
- Category: `module_cluster`
- Source files: `src/main.c`, `src/output.c`
- Rust branch: `078-module_src_set_level_15-rust-port`
- Generation date: `2026-06-11`

## Overview

This module is responsible for configuring level-based output formatting behavior in the program. The analyzed functionality shows two focused responsibilities:

1. Parsing and applying an indentation pattern from textual input.
2. Setting a per-level mark flag used by output formatting.

The Rust rewrite must preserve the observable behavior of these responsibilities as used by the program’s option/configuration flow and output layer.

This specification is limited to behavior evidenced by:

- `set_level_indent(const char *str)` in `src/main.c`
- `set_level_mark(int lev, int mark)` in `src/output.c`
- the option-related structures present in `src/main.c`, which indicate that these settings participate in command/configuration option processing

## Feature Specification

### Feature 1: Level indentation configuration from text input

The module accepts a string-based indentation specification and applies it to the program’s level-formatting configuration.

The Rust version must implement behavior equivalent to `set_level_indent`:

- Accept a textual indentation specification.
- Interpret that specification as the source of level indentation settings.
- Update the module/program formatting state so subsequent level-based output uses the configured indentation behavior.
- Support use through the same configuration path implied by the option parsing structures in `src/main.c`.

The specification does not require inventing a new user-facing syntax beyond the syntax accepted by the C module. The Rust port must preserve accepted input forms and resulting formatting behavior for all inputs currently handled by the C implementation.

### Feature 2: Per-level mark assignment

The module supports assigning a mark state to a specific output level.

The Rust version must implement behavior equivalent to `set_level_mark`:

- Accept a target level identifier.
- Accept a mark value for that level.
- Apply the mark setting to the corresponding level in output formatting state.
- Ensure subsequent output generation can observe the configured mark state for that level.

This feature is configuration behavior only; no additional output API is implied beyond preserving the effect on the existing output system.

### Feature 3: Integration with existing option/configuration definitions

The presence of repeated `parseopt` and `optdef` definitions in `src/main.c` evidences that this module’s behaviors are tied to command-line or configuration option handling.

The Rust version must therefore:

- Preserve the ability for indentation and level mark settings to be supplied through the module’s existing option/configuration flow.
- Preserve the mapping between option handling and the underlying formatting state changes implemented by this module.
- Maintain behavior consistent with the source module’s role as a configuration stage for output formatting.

## User Scenarios & Testing

### Scenario 1: Configure indentation pattern before generating output

A user supplies an indentation-related option or configuration value that is processed by the program. The module interprets the provided string and updates level-formatting configuration. When output is later generated, level nesting reflects the configured indentation behavior.

#### Test expectations

- Given a valid indentation specification accepted by the C module, the Rust version accepts the same input.
- After configuration is applied, later output formatting uses the same indentation behavior as the C module.
- No unrelated formatting setting is changed by applying indentation configuration.

### Scenario 2: Change mark behavior for a specific level

A program path sets a mark for a specific level through the module’s level-mark configuration behavior. Output subsequently produced for that level reflects the updated mark state.

#### Test expectations

- For a selected level and mark value accepted by the C module, the Rust version applies the same level-targeted setting.
- The setting affects the specified level rather than all levels indiscriminately.
- Existing settings for other levels remain unchanged unless the C module also changes them.

### Scenario 3: Apply multiple configuration updates in sequence

The program processes configuration in stages, such as option parsing followed by output generation. Indentation configuration and one or more level mark updates are applied before output is produced.

#### Test expectations

- Applying indentation settings followed by level mark settings yields the same final formatting state as the C module.
- Applying multiple mark updates to different levels preserves per-level distinctions.
- Later updates override earlier state only where the C module does so.

### Scenario 4: Option-driven configuration path

A user invokes the program through its standard option/configuration interface. The module receives configuration values through the option definitions associated with `parseopt`/`optdef` structures and updates formatting state.

#### Test expectations

- The Rust rewrite preserves the option-to-behavior linkage evidenced in `src/main.c`.
- Configuration values routed through option processing produce the same formatting state changes as direct internal invocation would.
- Unsupported or malformed handling is not expanded beyond current C behavior.

## Requirements

### Functional Requirements

#### FR-1: Indentation specification handling

The module shall accept a string input for level indentation configuration and apply it to the formatting state used for level-based output.

**Traceability:** `set_level_indent` in `src/main.c`

#### FR-2: Indentation behavior preservation

For every indentation specification currently accepted by the C module, the Rust version shall produce the same resulting formatting behavior in subsequent output.

**Traceability:** `set_level_indent` in `src/main.c`

#### FR-3: Level-specific mark setting

The module shall allow assignment of a mark value to a specific level.

**Traceability:** `set_level_mark` in `src/output.c`

#### FR-4: Mark state persistence for later output

After a level mark is set, the updated mark state shall remain in effect for subsequent output formatting until changed by later configuration, matching the C module’s behavior.

**Traceability:** `set_level_mark` in `src/output.c`

#### FR-5: Configuration-path integration

The module shall remain usable through the option/configuration processing path indicated by the option-related structures in `src/main.c`.

**Traceability:** `struct parseopt`, `struct optdef`, `struct option_type` in `src/main.c`

#### FR-6: No unsupported feature expansion

The Rust rewrite shall not introduce new functional scope beyond the evidenced behaviors of indentation configuration, per-level mark setting, and their participation in option-driven formatting configuration.

**Traceability:** limited evidenced scope from `src/main.c` and `src/output.c`

### Key Entities

#### 1. Option definition entities

- `option_type`
- `parseopt`
- `optdef`

These structures represent the configuration/option-processing layer through which formatting-related settings are defined and routed.

**Relationship to module behavior:**
They connect user- or program-supplied option values to the formatting state changes implemented by indentation and level-mark functions.

**Traceability:** anonymous structs listed from `src/main.c`

#### 2. Indentation specification input

A string value provided to the indentation-setting logic.

**Relationship to module behavior:**
This input is consumed by the indentation configuration function and translated into level-formatting state.

**Traceability:** `set_level_indent(const char *str)` in `src/main.c`

#### 3. Level identifier and mark value

A pair of integer inputs representing the target level and the mark state to assign.

**Relationship to module behavior:**
These inputs are consumed by the level-mark function to update per-level output formatting configuration.

**Traceability:** `set_level_mark(int lev, int mark)` in `src/output.c`

#### 4. Output formatting state

The internal program state that determines how level-based output is rendered, specifically with respect to indentation and level marks.

**Relationship to module behavior:**
Both main functions mutate this state; later output generation depends on it.

**Traceability:** inferred directly from `set_level_indent` and `set_level_mark` responsibilities across `src/main.c` and `src/output.c`

## Success Criteria

### SC-1: Behavioral equivalence for indentation inputs

For a regression suite built from indentation specifications accepted by the C module, the Rust port produces the same resulting output indentation behavior in all tested cases.

**Traceability:** `set_level_indent` in `src/main.c`

### SC-2: Behavioral equivalence for level mark updates

For a regression suite covering valid `(level, mark)` combinations used by the C module, the Rust port applies the same level-targeted mark behavior as the C implementation.

**Traceability:** `set_level_mark` in `src/output.c`

### SC-3: Option-path parity

When configuration values are supplied through the program’s existing option-processing path, the Rust port reaches the same indentation and level-mark state as the C module.

**Traceability:** option-related structures and configuration role in `src/main.c`

### SC-4: State isolation across updates

Tests that set indentation and then update one or more specific level marks demonstrate that each operation affects only the corresponding part of formatting state, matching C behavior.

**Traceability:** `set_level_indent` in `src/main.c`; `set_level_mark` in `src/output.c`

### SC-5: Scope compliance

Review of the Rust module confirms that it implements only the evidenced responsibilities of indentation configuration, per-level mark configuration, and option-flow integration, without adding unrelated public functionality.

**Traceability:** evidenced module scope from `src/main.c` and `src/output.c`

## Out of Scope

The following are not required by this specification because they are not evidenced by the analyzed module inputs:

- New public APIs beyond what is needed to preserve existing module behavior
- New configuration syntaxes
- Thread-safety guarantees
- Serialization or persistence formats
- FFI interfaces
- Performance or benchmark targets
- Recovery semantics beyond the source module’s current behavior