# Functional Specification: module_src_set_level_15

## Document Control

- Project: `cflow-new`
- Module: `module_src_set_level_15`
- Category: `module_cluster`
- Rust branch: `078-module_src_set_level_15-rust-port`
- Source basis: `src/main.c`, `src/output.c`
- Generation date: 2026-06-17

## 1. Feature Specification

### 1.1 Purpose

This module is responsible for configuring level-based output formatting from option input. Its observable role is to accept user-supplied level-format settings and apply them to the output subsystem so that rendering at a given nesting level uses the configured indentation or marking behavior.

The Rust rewrite must preserve the behavior evidenced by:

- `set_level_indent(const char *str)` in `src/main.c`
- `set_level_mark(int lev, int mark)` in `src/output.c`

### 1.2 Scope of Functionality

The module provides two linked behaviors:

1. **Parse and apply level-indentation configuration from string input**
   - A string-form option value is accepted.
   - The string is interpreted as level-related formatting configuration.
   - Parsed settings are applied to output-level formatting state.

2. **Assign mark behavior for a specific level**
   - A level number and mark value are accepted.
   - The output formatting state for that exact level is updated.

These behaviors together define the module boundary: it translates option/configuration input into per-level output formatting settings.

### 1.3 Out of Scope

The following are not evidenced by the analyzed sources for this module and must not be added as module requirements:

- New public configuration formats beyond those accepted by the C module
- Thread-safety guarantees
- Persistence or serialization of formatting settings
- Recovery, rollback, or transactional updates
- Cross-process or networked configuration
- Benchmark or performance promises beyond preserving functional behavior

## 2. User Scenarios & Testing

### 2.1 Scenario: Apply indentation configuration from an option string

**Given**
- the program receives an option or internal configuration string intended to define level indentation behavior

**When**
- the module processes that string through the level-indentation configuration path

**Then**
- the string is interpreted according to the existing C behavior
- the resulting level-format settings are applied to the output subsystem
- subsequent output formatting uses the configured per-level indentation behavior

**Test focus**
- verify that accepted input strings produce the same effective level settings as the C implementation
- verify that output formatting changes are visible in later output generation

### 2.2 Scenario: Update a specific level mark

**Given**
- a caller determines a target output level and a mark value

**When**
- the module applies the level mark setting

**Then**
- only the specified level’s mark setting is updated
- later output for that level reflects the configured mark behavior

**Test focus**
- verify exact level targeting
- verify that the stored setting affects output at that level

### 2.3 Scenario: Combine string-based indentation setup with explicit per-level marking

**Given**
- indentation-related configuration has already been applied from a string
- a per-level mark is then set for one or more levels

**When**
- output is generated for nested levels

**Then**
- indentation and mark settings coexist without losing previously applied level-format state
- output behavior remains consistent with the composed settings used by the C module

**Test focus**
- verify ordering does not invalidate applied settings
- verify multi-level output formatting reflects both categories of configuration

### 2.4 Scenario: Option-definition-driven invocation

**Given**
- the module participates in option parsing using option-definition records present in `src/main.c`

**When**
- the relevant option path selects level-format configuration behavior

**Then**
- the correct configuration routine is invoked
- the resulting module state matches what direct invocation would produce

**Test focus**
- verify integration between option definitions and level-format configuration
- verify no mismatch between parsed option values and applied level state

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Level-indentation configuration input
The Rust module shall accept string input for level-indentation configuration and interpret it using the same accepted behavior as `set_level_indent` in `src/main.c`.

**Traceability:** `src/main.c:324-367`, `set_level_indent`

#### FR-2: Application of parsed indentation settings
The Rust module shall convert accepted level-indentation input into changes in output-formatting state used for level-based rendering.

**Traceability:** `src/main.c:324-367`, `set_level_indent`; `src/output.c:30-38`, `set_level_mark`

#### FR-3: Per-level mark assignment
The Rust module shall support assigning a mark value to a specified output level, preserving the same level-targeted behavior as `set_level_mark`.

**Traceability:** `src/output.c:30-38`, `set_level_mark`

#### FR-4: Configuration affects later output behavior
The Rust module shall ensure that applied indentation and mark settings remain in effect for subsequent output operations that depend on level formatting.

**Traceability:** `src/main.c:324-367`, `set_level_indent`; `src/output.c:30-38`, `set_level_mark`

#### FR-5: Option-parsing integration
The Rust rewrite shall preserve the module’s participation in the option-processing flow represented by the `parseopt` and `optdef` usage sites in `src/main.c`, such that level-format configuration is triggered from the same option-definition context.

**Traceability:** `src/main.c:370, 404, 415, 423, 430, 447, 455, 462, 469, 477, 484, 493, 506`; `set_level_indent`

### 3.2 Key Entities

#### Entity: Level indentation configuration input
A string-valued configuration input consumed by the level-format configuration path.

**Role**
- carries user- or option-supplied formatting directives for levels

**Traceability**
- `set_level_indent(const char *str)` in `src/main.c`

#### Entity: Level mark setting
A pair of values consisting of:
- a level identifier
- a mark value to apply at that level

**Role**
- expresses direct assignment of mark behavior for one nesting level

**Traceability**
- `set_level_mark(int lev, int mark)` in `src/output.c`

#### Entity: Option-definition records
Anonymous structures identified as `parseopt` and `optdef` in `src/main.c`.

**Role**
- connect command-line or internal option parsing to configuration handlers
- provide the integration path through which level-format configuration is selected

**Traceability**
- anonymous `struct parseopt`
- anonymous `struct optdef`
- usage sites in `src/main.c:370, 404, 415, 423, 430, 447, 455, 462, 469, 477, 484, 493, 506`

#### Entity: Option type records
Anonymous `struct option_type` definitions in `src/main.c`.

**Role**
- represent option-related configuration categories used by the surrounding option system that reaches this module behavior

**Traceability**
- anonymous `struct option_type` at `src/main.c:31-35, 107, 122, 241`

### 3.3 Entity Relationships

- Option-definition records select configuration behavior during option parsing.
- That option flow can deliver a string-valued level-format input to the level-indentation configuration path.
- Parsed indentation settings and direct level mark assignments both update level-based output formatting state.
- Later output rendering consumes that state to determine formatting by nesting level.

## 4. Success Criteria

### 4.1 Behavioral Equivalence

1. **String configuration parity**
   - For every level-indentation input accepted by the C module, the Rust module accepts the same input and produces the same effective level-format result.

   **Traceability:** `set_level_indent` in `src/main.c`

2. **Per-level mark parity**
   - For any tested `(level, mark)` pair accepted by the C module, the Rust module applies the mark to the same target level with the same observed effect on output formatting.

   **Traceability:** `set_level_mark` in `src/output.c`

3. **State persistence across later output**
   - After configuration is applied, later output operations that depend on nesting level use the updated settings rather than default or stale settings.

   **Traceability:** `set_level_indent`, `set_level_mark`

### 4.2 Integration Correctness

4. **Option-flow preservation**
   - The Rust rewrite preserves the option-processing path needed to reach level-format configuration from the option-definition context evidenced in `src/main.c`.

   **Traceability:** anonymous `parseopt` / `optdef` structures and usage sites in `src/main.c`

5. **No unsupported feature expansion**
   - The Rust module does not require new configuration concepts, new public APIs, or unrelated runtime guarantees beyond the functionality evidenced in the analyzed C sources.

   **Traceability:** module scope derived from `src/main.c` and `src/output.c`

### 4.3 Testability Expectations

6. **Scenario coverage**
   - Automated tests for the Rust module cover:
     - string-based level-indentation configuration
     - direct per-level mark assignment
     - combined use of both behaviors
     - invocation through the option-processing path

7. **Observable output impact**
   - Tests demonstrate that configuration changes affect actual level-based output behavior, not only internal state mutation.