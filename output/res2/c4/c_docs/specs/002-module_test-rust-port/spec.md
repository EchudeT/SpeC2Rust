# Specification: module_test

- **Project**: c4
- **Module**: module_test
- **Category**: module
- **Rust branch**: `002-module_test-rust-port`
- **Source basis**: `test/c4.c`, `test/hello.c`
- **Generation date**: 2026-06-07

## 1. Overview

This module provides test-program behavior used alongside the c4 project’s self-hosting and sample execution flow.

From the analyzed sources, the module contains:

- a small sample program in `test/hello.c` with a trivial `main`
- a larger test/compiler-driver style program in `test/c4.c` whose visible behavior is organized around:
  - token advancement (`next`)
  - expression handling (`expr`)
  - statement handling (`stmt`)
  - command-line driven program execution (`main`)

The Rust rewrite must preserve the functional behavior evidenced by these files: it must support the test-oriented parsing/execution flow represented by `test/c4.c`, and it must preserve the existence and behavior of the minimal sample program represented by `test/hello.c`.

## 2. Feature Specification

### 2.1 Module purpose

The module serves as a test-facing program component with two roles:

1. **Minimal sample program support**
   - `test/hello.c` represents a simple standalone program entry point.
   - The Rust version must preserve this sample-program role as a minimal executable behavior used for validation/examples.

2. **Parsing and statement/expression processing flow**
   - `test/c4.c` exposes a top-level flow that reads program input, advances through tokens, processes expressions and statements, and runs through a command-line controlled main routine.
   - The Rust version must preserve this functional pipeline.

### 2.2 Functional scope to preserve

The Rust version must implement the following evidenced module behavior:

- Advance through source input as discrete tokens or equivalent parse units.
- Process expressions with precedence/level-sensitive handling as indicated by `expr(int lev)`.
- Process statements as higher-level language constructs as indicated by `stmt()`.
- Provide a command-line entry path that initializes and drives the module’s processing flow.
- Preserve a minimal standalone hello-style test program behavior corresponding to `test/hello.c`.

### 2.3 Explicit non-goals

The specification does not require any capability not evidenced by the source analysis. In particular, the Rust rewrite is not required by this spec to add:

- new public APIs beyond what is needed to preserve module behavior
- thread-safety guarantees
- serialization
- FFI interfaces
- error recovery beyond the source-observable behavior
- performance or benchmark targets

## 3. User Scenarios & Testing

### Scenario 1: Run the main test/compiler-style program

**Given** the Rust port of the `test/c4.c` behavior is built
**When** a user invokes the program from the command line with source input as expected by the original test flow
**Then** the program enters its top-level main routine and performs the same overall processing stages evidenced by the C module:
- initialization from command-line inputs
- source progression through token advancement
- expression handling
- statement handling
- completion of the program flow with an integer process result

**Testing guidance**
- Provide an integration test or executable test that invokes the Rust program with representative input.
- Verify that the program reaches the same class of outcomes as the original C version for valid test inputs.

### Scenario 2: Parse and process expressions

**Given** source content containing expressions
**When** the Rust port processes the content
**Then** the module must advance through input and handle expressions according to the level-based expression routine represented by `expr(int lev)`.

**Testing guidance**
- Use inputs containing expression forms accepted by the original module.
- Verify that expression parsing/handling completes successfully where the original does.
- Verify that invalid or unsupported expression forms do not produce behavior outside the original module’s demonstrated scope.

### Scenario 3: Parse and process statements

**Given** source content containing statements
**When** the Rust port processes the content
**Then** the module must handle statement-level constructs through behavior corresponding to `stmt()` and integrate this with expression handling where required.

**Testing guidance**
- Use statement-bearing inputs accepted by the original module.
- Verify statement processing order and completion are consistent with the source program flow.

### Scenario 4: Advance through source input incrementally

**Given** source text to be processed
**When** the Rust port steps through the input
**Then** it must support incremental advancement behavior corresponding to `next()`, enabling subsequent expression and statement processing.

**Testing guidance**
- Exercise parsing on inputs requiring multiple advancement steps.
- Verify that progression through input is sufficient to support complete statement/expression handling.

### Scenario 5: Run the minimal hello-style sample program

**Given** the Rust replacement for the `test/hello.c` sample
**When** it is built and run
**Then** it must preserve the same minimal standalone program behavior represented by that source file.

**Testing guidance**
- Build and run the Rust sample executable or equivalent target.
- Verify successful execution and the same observable simple-program outcome as the C sample.

## 4. Requirements

## 4.1 Functional Requirements

### FR-1: Command-line driven top-level execution
The module shall provide a top-level executable flow corresponding to `main(int argc, char **argv)` in `test/c4.c`, accepting command-line invocation and returning an integer process status.

**Traceability**: `test/c4.c: main`

### FR-2: Incremental input advancement
The module shall support advancing through source input in discrete processing steps corresponding to `next()`.

**Traceability**: `test/c4.c: next`

### FR-3: Expression processing
The module shall process expressions through a precedence- or level-sensitive entry point corresponding to `expr(int lev)`.

**Traceability**: `test/c4.c: expr`

### FR-4: Statement processing
The module shall process statements through behavior corresponding to `stmt()`.

**Traceability**: `test/c4.c: stmt`

### FR-5: Integrated parsing/execution flow
The module shall integrate input advancement, expression processing, and statement processing into the main executable flow so that source input can be handled end-to-end.

**Traceability**: `test/c4.c: next`, `expr`, `stmt`, `main`

### FR-6: Minimal sample program preservation
The module shall preserve a minimal standalone test-program behavior corresponding to `main()` in `test/hello.c`.

**Traceability**: `test/hello.c: main`

## 4.2 Key Entities

The analysis input does not declare named core data structures, but the module’s functional behavior evidences the following required conceptual entities:

### KE-1: Source input
Program text or equivalent input consumed by the top-level test/compiler-style flow.

**Relationship**
- Consumed by input advancement.
- Supplies content for expression and statement processing.

**Traceability**: `test/c4.c: next`, `expr`, `stmt`, `main`

### KE-2: Current parse position / current token state
The current processing state that is advanced step-by-step and used by higher-level parsing routines.

**Relationship**
- Updated by `next`.
- Read by expression and statement processing.
- Participates in end-to-end flow driven by `main`.

**Traceability**: `test/c4.c: next`, `expr`, `stmt`, `main`

### KE-3: Expression level
The expression-processing level parameter used to control expression handling behavior.

**Relationship**
- Passed into expression processing.
- Influences how expression input is handled relative to surrounding parse state.

**Traceability**: `test/c4.c: expr`

### KE-4: Program entry points
Executable entry points for:
- the test/compiler-style program in `test/c4.c`
- the minimal sample program in `test/hello.c`

**Relationship**
- The main test flow orchestrates parsing/processing.
- The sample program provides a separate minimal execution target.

**Traceability**: `test/c4.c: main`, `test/hello.c: main`

## 5. Success Criteria

### SC-1: End-to-end top-level execution works
A Rust executable implementing the `test/c4.c` behavior can be invoked from the command line and completes with an integer exit status.

**Traceability**: `test/c4.c: main`

### SC-2: Input advancement supports parsing flow
For representative valid inputs used by the original module, the Rust port advances through source input successfully enough to enable continued parsing/processing.

**Traceability**: `test/c4.c: next`

### SC-3: Expression handling is preserved
For representative valid expression inputs accepted by the original module, the Rust port processes expressions through level-sensitive behavior without requiring unsupported new interfaces.

**Traceability**: `test/c4.c: expr`

### SC-4: Statement handling is preserved
For representative valid statement inputs accepted by the original module, the Rust port processes statements successfully within the overall program flow.

**Traceability**: `test/c4.c: stmt`

### SC-5: Integrated flow is preserved
For representative source inputs, the Rust port demonstrates correct integration of advancement, expression processing, statement processing, and top-level execution consistent with the original module’s purpose.

**Traceability**: `test/c4.c: next`, `expr`, `stmt`, `main`

### SC-6: Minimal sample program still runs
The Rust replacement for the `test/hello.c` sample builds and runs successfully, preserving the sample’s minimal standalone behavior.

**Traceability**: `test/hello.c: main`

## 6. Acceptance Notes

- Acceptance should be based on observable behavior of the analyzed module files only.
- If the original C module accepts only a limited class of test inputs, the Rust rewrite is only required to preserve that evidenced behavior.
- No additional functionality should be treated as required unless it is traceable to the analyzed sources.