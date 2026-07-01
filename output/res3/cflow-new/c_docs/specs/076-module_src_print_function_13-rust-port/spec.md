# spec.md

## Title

Rust Functional Specification: `module_src_print_function_13`

## Metadata

- **Project**: `cflow-new`
- **Module**: `module_src_print_function_13`
- **Category**: `module_cluster`
- **Rust branch**: `076-module_src_print_function_13-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module is responsible for emitting function-level output for symbols that represent callable program elements. The analyzed behavior is centered on two printing operations:

- printing a function name in GNU-style output, with awareness of whether the function has a subtree;
- printing a function entry through the generic output flow.

The Rust rewrite must preserve the observed functional boundary of this module: given a function symbol and the surrounding output context, it produces the corresponding textual function representation used by the project’s output pipeline.

## Scope

### In Scope

- Formatting and emitting a function symbol’s printed representation.
- Supporting the GNU-specific function-name output path.
- Supporting the output-layer function printing path.
- Using the module’s symbol/output entities as the source of printed content.

### Out of Scope

- Symbol discovery, parsing, or semantic analysis.
- Building complete call graphs or traversing unrelated graph structure beyond what is required to decide the printed function representation.
- Introducing new output formats or public APIs not evidenced by the analyzed module.
- Persistence, serialization, concurrency guarantees, or error-recovery features not evidenced by the source module.

## Feature Specification

### Feature: Function-level output emission

The module provides function-specific printing behavior within the project’s output subsystem.

The Rust version must implement behavior equivalent to the analyzed C module for these two observed cases:

1. **GNU-style function name printing**
   - Accept a symbol representing a function-like entity.
   - Emit the symbol’s function name in the GNU output path.
   - Account for whether the symbol has a subtree, because that status is part of the function’s printed behavior boundary.

2. **General function entry printing**
   - Accept a symbol in the output layer’s function-printing path.
   - Emit the function representation expected by the module’s output driver flow.
   - Integrate with the module’s output symbol/driver model so that function printing remains usable inside the existing output pipeline.

### Behavioral Notes

- The module is output-producing, not analysis-producing.
- The behavior is symbol-driven: the printed result is derived from the provided symbol and output context.
- The `has_subtree` input is behaviorally significant for GNU-style printing and must remain part of the Rust implementation’s supported logic.
- The rewrite must preserve observable output behavior for equivalent symbols and equivalent subtree state.

## User Scenarios & Testing

### Scenario 1: Print a function name in GNU output mode

**Given**
- a symbol corresponding to a function;
- GNU-style output is being generated;
- the caller knows whether the function has a subtree;

**When**
- the module is asked to print that function name;

**Then**
- the module emits the GNU-style function name representation for that symbol;
- the emitted result reflects the provided subtree presence state.

**Testing expectations**
- Verify that a function symbol can be printed through the GNU path.
- Verify that changing `has_subtree` can affect the emitted function representation when required by the original behavior.
- Verify that the symbol identity used for output is preserved correctly.

### Scenario 2: Print a function through the general output pipeline

**Given**
- a symbol corresponding to a function;
- the output subsystem is using this module’s function-printing path;

**When**
- the module is asked to print the function entry;

**Then**
- the module emits the function representation expected by the output subsystem;
- the output is compatible with the surrounding driver/symbol output flow.

**Testing expectations**
- Verify that a function symbol can be printed through the general output path.
- Verify that the produced output is emitted in the same place and role as in the C behavior.
- Verify that function printing works with the module’s output entities rather than requiring unrelated external formatting steps.

### Scenario 3: Use the same symbol model across output paths

**Given**
- the same logical function symbol is passed through different output contexts supported by this module;

**When**
- the GNU-specific and general output printing paths are invoked separately;

**Then**
- each path emits its own expected representation;
- both paths remain driven by the same underlying symbol information.

**Testing expectations**
- Verify that both observed entry points can operate on equivalent function symbols.
- Verify that output differences are attributable to output mode/path, not to accidental symbol reinterpretation.

## Requirements

### Functional Requirements

#### FR-1: GNU-style function name output
The Rust module shall provide behavior equivalent to `print_function_name(Symbol *sym, int has_subtree)` for printing a function symbol’s name in the GNU output path.
**Traceability**: `src/gnu.c`, `print_function_name`

#### FR-2: Subtree-aware GNU output behavior
The Rust module shall preserve the behavioral significance of the subtree-presence input when printing GNU-style function names.
**Traceability**: `src/gnu.c`, `print_function_name`

#### FR-3: General function output
The Rust module shall provide behavior equivalent to `print_function(Symbol *symp)` for printing a function symbol through the output subsystem’s general function-printing path.
**Traceability**: `src/output.c`, `print_function`

#### FR-4: Symbol-driven output
The Rust module shall derive printed function output from the provided symbol entity rather than from external inferred state not represented in the module’s input/output model.
**Traceability**: `print_function_name`, `print_function`, `struct output_symbol`

#### FR-5: Output-pipeline compatibility
The Rust module shall remain compatible with the module’s driver-based output flow, so that function printing participates correctly in the output subsystem.
**Traceability**: `src/output.c`, `struct output_driver`, `print_function`

#### FR-6: Support for module-local output symbol representation
The Rust module shall support the output symbol representation used by the analyzed module where function-printing behavior depends on symbol-related output state.
**Traceability**: `src/gnu.c: struct output_symbol`, `src/output.c: struct output_symbol`

### Key Entities

#### Symbol
A program symbol that identifies the function being printed. It is the primary input to both observed printing behaviors.
**Relationship**: consumed by GNU-style function-name printing and by the general output-layer function-printing path.
**Traceability**: `print_function_name(Symbol *sym, ...)`, `print_function(Symbol *symp)`

#### Output Symbol
A module-local representation that associates symbol-related state with output behavior.
**Relationship**: bridges symbol information into emitted output in both analyzed files.
**Traceability**: `src/gnu.c: struct output_symbol`, `src/output.c: struct output_symbol`

#### Output Driver
A module-local output abstraction representing the active output mode/driver.
**Relationship**: provides the surrounding output context in which function-printing behavior is used.
**Traceability**: `src/output.c: struct output_driver`

#### Linked List / Linked List Entry
Module-local list entities used within the output subsystem.
**Relationship**: part of the output module’s supporting data model and relevant context for how output-related entities are organized, though not themselves a new functional surface of this spec.
**Traceability**: `src/output.c: struct linked_list`, `struct linked_list_entry`

## Success Criteria

1. **GNU path equivalence**
   For equivalent function symbols and subtree flags, the Rust implementation emits GNU-style function-name output equivalent in observable behavior to the C module.
   **Traceability**: `src/gnu.c`, `print_function_name`

2. **General output path equivalence**
   For equivalent function symbols in the output subsystem, the Rust implementation emits function output equivalent in observable behavior to the C module’s general function-printing path.
   **Traceability**: `src/output.c`, `print_function`

3. **Subtree-sensitive behavior preserved**
   Tests demonstrate that the GNU function-name printing path accepts and honors subtree presence state as part of output behavior.
   **Traceability**: `src/gnu.c`, `print_function_name`

4. **Entity compatibility preserved**
   The Rust implementation operates on symbol and output-context entities corresponding to the analyzed module’s symbol/output abstractions, without requiring unsupported new capabilities or external workflows.
   **Traceability**: `struct output_symbol`, `struct output_driver`, `print_function_name`, `print_function`

5. **Scenario coverage achieved**
   The Rust test suite includes at minimum the scenarios defined in this document for GNU function-name printing, general function printing, and symbol reuse across both paths.
   **Traceability**: all scenarios above; `print_function_name`, `print_function`