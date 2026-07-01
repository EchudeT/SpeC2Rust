# spec.md

## Title

Rust Functional Specification for `module_src_set_level_15`

## Metadata

- Project: `cflow-new`
- Module: `module_src_set_level_15`
- Category: `module_cluster`
- Rust branch: `078-module_src_set_level_15-rust-port`
- Generation date: `2026-06-17`

## Overview

This module is responsible for configuring level-based output formatting used by the program’s flow display logic. The analyzed C sources show two behaviors in scope:

- parsing and applying a level indentation specification from textual input
- setting a per-level mark value used by output formatting

The Rust rewrite must preserve the observable behavior of these formatting configuration operations so that higher-level option parsing and output generation continue to work as they do in the C project.

## Scope

Included in scope for this module:

- accepting a textual level-indentation setting and applying it to the program’s formatting state
- accepting a level number and mark value and applying that mark to the formatting state for that level
- participating in command-line or option-driven configuration through the option-related structures present in `src/main.c`

Out of scope:

- defining new formatting features not evidenced by the analyzed functions
- redesigning unrelated command-line parsing behavior
- changing output semantics beyond what is required to preserve the behavior of level indentation and level marking

## Source Traceability

Primary traced behaviors:

- `set_level_indent` — `src/main.c:324-367`
- `set_level_mark` — `src/output.c:30-38`

Primary traced entities:

- option-related anonymous structures in `src/main.c`, including `option_type`, `parseopt`, and `optdef`, which indicate that formatting configuration is driven by parsed options

## Feature Specification

### Feature: Level indentation configuration

The module must support configuring indentation behavior for output levels from a string input. The Rust version must interpret the provided string as a level-formatting directive and update the module’s formatting state accordingly.

This behavior is evidenced by `set_level_indent(const char *str)` in `src/main.c`. Because the function accepts textual input rather than a pre-parsed numeric structure, the Rust version must preserve the ability to receive indentation configuration in string form from surrounding option-processing logic.

The Rust implementation must preserve these functional boundaries:

- accept an indentation specification as text
- validate and interpret that specification according to the existing module behavior
- apply the resulting indentation settings to the level-formatting state used by output generation
- integrate with option-driven configuration flow rather than requiring a new configuration mechanism

### Feature: Per-level mark configuration

The module must support assigning a mark value to a specific output level. The Rust version must apply a level identifier and a mark value to the formatting state used by output routines.

This behavior is evidenced by `set_level_mark(int lev, int mark)` in `src/output.c`.

The Rust implementation must preserve these functional boundaries:

- accept a level identifier
- accept a mark value for that level
- update the formatting state so later output rendering can use the configured mark for that level

### Feature: Option-driven formatting configuration participation

The analyzed `src/main.c` includes multiple option-related anonymous structures (`option_type`, `parseopt`, `optdef`) around the same area as `set_level_indent`, showing that this module participates in a larger option parsing/configuration path.

The Rust version must therefore remain usable from an option-processing flow that:

- maps parsed option definitions to formatting actions
- supplies textual indentation settings to the indentation configuration behavior
- supplies parsed numeric level and mark values to the per-level marking behavior

The rewrite must preserve this integration role without inventing a separate user-facing API requirement beyond what the module already serves.

## User Scenarios & Testing

### Scenario 1: User provides a level indentation setting through program options

A user invokes the program with an option that ultimately supplies an indentation specification string to this module.

Expected behavior:

- the specification is accepted in textual form
- the module applies the indentation configuration to internal formatting state
- subsequent output generation uses the updated level indentation behavior

Test guidance:

- provide representative valid indentation-setting inputs through the option path
- confirm that output formatting changes in a way consistent with the C module’s behavior
- verify that the Rust port accepts the same class of textual input as the C module

### Scenario 2: User configures a mark for a specific level

A user invokes the program with an option or configuration path that results in a level number and mark value being passed to this module.

Expected behavior:

- the target level is identified correctly
- the requested mark is stored for that level
- subsequent output formatting for that level reflects the configured mark

Test guidance:

- configure at least one non-default level/mark pair
- generate output that exercises that level
- confirm that the mark used by the Rust version matches the C version

### Scenario 3: Multiple formatting settings are applied before output

A user provides more than one formatting-related option during a single invocation, such as indentation-related configuration and level-specific mark configuration.

Expected behavior:

- each setting is applied through the same option-driven configuration flow
- applying one setting does not prevent the other from taking effect
- resulting output reflects the combined formatting state

Test guidance:

- apply indentation configuration and one or more level mark settings together
- compare resulting output behavior against the C implementation
- verify that ordering of supported option processing does not break the final formatting state where the C version would not

### Scenario 4: Invalid or unsupported indentation text is encountered

A user provides indentation text that is not accepted by the existing module behavior.

Expected behavior:

- the Rust version handles the input consistently with the C behavior for that case
- it must not silently invent a new interpretation for unsupported input
- observable program behavior must remain aligned with the original module’s handling path

Test guidance:

- derive invalid samples from the accepted input grammar observed in the C module
- compare Rust and C outcomes for acceptance/rejection and resulting formatting state

## Requirements

### Functional Requirements

#### FR-1: Accept textual indentation configuration

The module shall accept a level indentation specification in string form and process it as formatting configuration.

Traceability:

- `set_level_indent` in `src/main.c:324-367`

#### FR-2: Apply parsed indentation settings to level-formatting state

After processing a valid indentation specification, the module shall update the formatting state used by later output generation so that level indentation behavior reflects the provided specification.

Traceability:

- `set_level_indent` in `src/main.c:324-367`

#### FR-3: Preserve option-path compatibility for indentation settings

The module shall remain compatible with an option-processing flow that provides indentation settings through option-related definitions and parsed option state.

Traceability:

- `set_level_indent` in `src/main.c:324-367`
- option-related anonymous structures in `src/main.c` (`option_type`, `parseopt`, `optdef`)

#### FR-4: Accept per-level mark assignments

The module shall accept a level identifier together with a mark value and process that pair as output-formatting configuration.

Traceability:

- `set_level_mark` in `src/output.c:30-38`

#### FR-5: Apply mark settings to the addressed level

When a level and mark are provided, the module shall update the formatting state for that level so later output formatting can use the assigned mark.

Traceability:

- `set_level_mark` in `src/output.c:30-38`

#### FR-6: Preserve original behavior for valid and invalid formatting inputs

For inputs handled by the analyzed functions, the Rust rewrite shall match the C module’s observable behavior in whether configuration is accepted, how it affects formatting state, and how subsequent output is formatted.

Traceability:

- `set_level_indent` in `src/main.c:324-367`
- `set_level_mark` in `src/output.c:30-38`

### Key Entities

#### Formatting configuration state

A module-level formatting state is implied by both analyzed functions because each updates output-related behavior without returning a value. This state includes:

- indentation configuration derived from textual level-indentation input
- mark configuration associated with specific levels

Relationships:

- `set_level_indent` updates the indentation-related portion of this state
- `set_level_mark` updates the per-level mark portion of this state

Traceability:

- `set_level_indent` in `src/main.c:324-367`
- `set_level_mark` in `src/output.c:30-38`

#### Option definition and parsed option entities

The anonymous `option_type`, `parseopt`, and `optdef` structures in `src/main.c` represent the option-definition and option-parsing layer that supplies configuration values to this module.

Relationships:

- option definitions describe available configuration inputs
- parsed option state delivers user-supplied values
- formatting configuration functions consume those values to update formatting state

Traceability:

- anonymous `option_type` structures in `src/main.c`
- anonymous `parseopt` and `optdef` structures in `src/main.c`

#### Level identifier

A level identifier is an integer-like value used to address a specific output level for mark configuration.

Relationships:

- paired with a mark value in `set_level_mark`
- selects which level’s formatting state is updated

Traceability:

- `set_level_mark(int lev, int mark)` in `src/output.c:30-38`

#### Mark value

A mark value is an integer-like formatting value associated with a specific level.

Relationships:

- supplied together with a level identifier
- stored in per-level formatting configuration and later used by output formatting logic

Traceability:

- `set_level_mark(int lev, int mark)` in `src/output.c:30-38`

## Success Criteria

### Behavioral equivalence

- For a representative set of valid indentation specification inputs accepted by the C module, the Rust version accepts the same inputs and produces matching output-formatting effects.
- For representative level/mark assignments, the Rust version updates formatting behavior for the addressed level in the same way as the C module.
- When indentation and per-level mark configuration are both applied in one run, resulting formatted output matches the C module for the same input sequence.

Traceability:

- `set_level_indent` in `src/main.c:324-367`
- `set_level_mark` in `src/output.c:30-38`

### Invalid-input consistency

- For tested invalid or unsupported indentation specification inputs, the Rust version’s observable behavior matches the C module’s behavior for acceptance/rejection and resulting formatting state.

Traceability:

- `set_level_indent` in `src/main.c:324-367`

### Option-flow compatibility

- The Rust rewrite can be driven by the surrounding option-processing path without requiring new categories of configuration input beyond textual indentation settings and numeric level/mark assignments already evidenced in the C sources.

Traceability:

- `set_level_indent` in `src/main.c:324-367`
- option-related anonymous structures in `src/main.c`
- `set_level_mark` in `src/output.c:30-38`

### No functional regression within module scope

- The rewrite preserves all evidenced behaviors in scope for this module and does not require callers to adopt new semantics for configuring level indentation or level marks.

Traceability:

- `set_level_indent` in `src/main.c:324-367`
- `set_level_mark` in `src/output.c:30-38`