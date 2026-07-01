# spec.md

## Title

Rust Port Functional Specification: `module_test`

## Metadata

- Project: `c4`
- Module: `module_test`
- Category: `module`
- Rust branch: `002-module_test-rust-port`
- Source basis:
  - `test/c4.c`
  - `test/hello.c`
- Generation date: 2026-06-06

## Overview

`module_test` contains test programs that exercise compilation and execution behavior.

From the available source evidence, the module includes:

- a self-contained compiler/interpreter-style test program in `test/c4.c` built around token advancement, expression parsing, statement parsing, and a command-line entry point;
- a minimal standalone hello-world style program in `test/hello.c`.

The Rust rewrite must preserve the functional role of these test programs as executable module artifacts. The specification is limited to behavior evidenced by the listed source files and functions.

## Scope

### In Scope

- Reproducing the observable behavior represented by:
  - lexical/token progression via `next`
  - expression handling via `expr`
  - statement handling via `stmt`
  - command-line driven execution flow in `test/c4.c` `main`
  - execution of the simple standalone `test/hello.c` program
- Supporting the module as test-oriented executable behavior rather than as a reusable library API

### Out of Scope

- New language features, diagnostics, or command-line options not evidenced by the source module
- Public APIs beyond what is required to preserve executable behavior
- Concurrency, persistence, serialization, FFI, recovery systems, or performance targets not evidenced in the source files

## Feature Specification

### Feature 1: Token Progression for Source Processing

The module shall support advancing through source input in a way that enables subsequent parsing stages.

This feature is evidenced by `next` in `test/c4.c`. The Rust version must implement equivalent behavior sufficient for the rest of the `c4` test program flow to operate correctly.

Observed functional role:

- consume or advance to the next meaningful token or parsing unit;
- maintain parser progress so that expression and statement handling can proceed in sequence;
- participate in the overall compilation or interpretation pipeline driven from `main`.

### Feature 2: Expression Handling

The module shall support processing expressions with precedence-sensitive behavior.

This feature is evidenced by `expr(int lev)` in `test/c4.c`. The Rust version must implement expression handling that accepts a precedence or level parameter and uses it to drive correct expression processing for the test program.

Observed functional role:

- parse and/or compile expression constructs;
- respect the supplied level parameter in determining expression handling;
- integrate with token progression and statement handling.

### Feature 3: Statement Handling

The module shall support processing statements as top-level executable or compilable units within the test program.

This feature is evidenced by `stmt` in `test/c4.c`. The Rust version must implement equivalent statement handling behavior as part of the language-processing flow exercised by the test module.

Observed functional role:

- recognize and process statements;
- invoke expression handling where statements contain expressions;
- operate within the sequencing controlled by the program entry point.

### Feature 4: Command-Line Driven Test Program Execution

The module shall provide an executable entry point for the `c4` test program.

This feature is evidenced by `main(int argc, char **argv)` in `test/c4.c`. The Rust version must preserve the externally observable role of this executable: accepting command-line arguments and driving the test program’s full processing flow.

Observed functional role:

- accept command-line invocation;
- initialize and run the processing pipeline;
- coordinate token progression, expression handling, and statement handling;
- terminate with an integer process result.

### Feature 5: Minimal Standalone Hello Program

The module shall provide a minimal executable corresponding to `test/hello.c`.

This feature is evidenced by `main()` in `test/hello.c`. The Rust version must preserve this file’s role as a simple standalone executable test artifact.

Observed functional role:

- provide a valid runnable program entry point;
- perform the simple behavior represented by the original file;
- terminate successfully when run under the same intended test conditions.

## User Scenarios & Testing

### Scenario 1: Running the Main `c4` Test Executable

A developer runs the Rust port of the `test/c4.c` executable with command-line arguments corresponding to the original test usage.

Expected support:

- the executable starts from its entry point;
- command-line inputs are accepted;
- source processing advances through tokens;
- expressions and statements are handled as part of the run;
- the program exits with an integer status consistent with the original behavior for the same input class.

Suggested validation:

- execute the Rust binary with representative arguments used for the original test;
- confirm successful completion for valid usage;
- confirm observable output and exit status match the original for the same test input.

### Scenario 2: Processing Expression-Containing Input

A developer uses the main test executable with input that requires expression handling.

Expected support:

- expression processing is invoked during program flow;
- precedence-level-based handling occurs;
- expression-containing constructs are accepted and processed in a manner compatible with the original test program.

Suggested validation:

- provide representative input that exercises nested or precedence-relevant expressions;
- compare output, acceptance, and exit behavior between C and Rust versions.

### Scenario 3: Processing Statement-Oriented Input

A developer uses the main test executable with input containing one or more statements.

Expected support:

- statement processing is invoked;
- statements that include expressions are handled through the expression path;
- sequencing through multiple statements works as in the source module.

Suggested validation:

- provide representative statement-based input;
- confirm the Rust executable processes the same input successfully where the C version does.

### Scenario 4: Running the Minimal Hello Test Program

A developer runs the Rust equivalent of `test/hello.c` as a sanity-check executable.

Expected support:

- the executable launches and performs the same simple behavior as the original program;
- it exits successfully under normal execution.

Suggested validation:

- run the original and Rust versions;
- compare observable output and exit code.

## Requirements

### Functional Requirements

#### FR-1: Executable Entry for `c4` Test Program

The Rust module shall provide an executable corresponding to `test/c4.c` that accepts command-line arguments and initiates the program flow represented by the original `main`.

**Traceability:** `test/c4.c` — `main`

#### FR-2: Token Advancement Support

The Rust module shall implement token or parsing-unit advancement behavior required by the `c4` test program so that parsing can proceed incrementally through input.

**Traceability:** `test/c4.c` — `next`

#### FR-3: Expression Processing with Level Parameter

The Rust module shall implement expression processing that accepts a level/precedence parameter and uses that parameter in handling expressions for the `c4` test program.

**Traceability:** `test/c4.c` — `expr`

#### FR-4: Statement Processing

The Rust module shall implement statement processing for the `c4` test program and shall support statements that require expression processing.

**Traceability:** `test/c4.c` — `stmt`, `expr`

#### FR-5: Coordinated Parsing/Processing Flow

The Rust module shall coordinate token advancement, expression processing, and statement processing under the control of the main executable flow so that the overall `c4` test program remains runnable.

**Traceability:** `test/c4.c` — `main`, `next`, `expr`, `stmt`

#### FR-6: Integer Process Termination for `c4` Executable

The Rust equivalent of the `test/c4.c` executable shall terminate with a process status corresponding to the original program’s integer `main` result for equivalent invocation conditions.

**Traceability:** `test/c4.c` — `main`

#### FR-7: Minimal Standalone Hello Executable

The Rust module shall provide an executable corresponding to `test/hello.c` that preserves the original file’s simple standalone runtime behavior.

**Traceability:** `test/hello.c` — `main`

#### FR-8: Successful Normal Termination of Hello Executable

The Rust equivalent of `test/hello.c` shall terminate successfully under normal invocation consistent with the original file’s behavior.

**Traceability:** `test/hello.c` — `main`

### Key Entities

No named core data structures were provided in the analysis input.

The module’s evidenced functional entities are behavioral stages within the test program:

- **Token progression state**
  - implied by `next`
  - represents the current position in source processing needed by later parsing stages

- **Expression-processing context**
  - implied by `expr(int lev)`
  - includes the supplied level/precedence input and current parser state

- **Statement-processing context**
  - implied by `stmt`
  - depends on current parser state and may invoke expression processing

- **Program invocation context**
  - implied by `main(int argc, char **argv)` in `test/c4.c`
  - includes command-line arguments and top-level execution control

- **Standalone hello execution**
  - implied by `main()` in `test/hello.c`
  - represents a minimal independent executable path

Relationships:

- program invocation control drives token progression and parsing behavior in the `c4` executable;
- statement processing depends on token progression and may depend on expression processing;
- expression processing depends on current token progression state and the supplied level parameter;
- the hello executable is independent of the `c4` executable flow.

## Success Criteria

### SC-1: `c4` Executable Availability

A Rust executable corresponding to `test/c4.c` is buildable and runnable from the specified Rust branch.

**Traceability:** `test/c4.c` — `main`

### SC-2: Argument-Driven Execution

When invoked with representative command-line arguments used by the original test program, the Rust `c4` executable accepts the arguments and completes execution without missing required top-level behavior.

**Traceability:** `test/c4.c` — `main`

### SC-3: Parsing Flow Preservation

For representative valid inputs that exercise token progression, expression handling, and statement handling, the Rust `c4` executable exhibits the same accept/reject outcome and equivalent observable execution behavior as the C version.

**Traceability:** `test/c4.c` — `next`, `expr`, `stmt`, `main`

### SC-4: Expression Scenario Coverage

At least one validation case exercising expression handling with precedence-sensitive structure produces equivalent observable behavior between the Rust and C versions.

**Traceability:** `test/c4.c` — `expr`

### SC-5: Statement Scenario Coverage

At least one validation case exercising statement processing produces equivalent observable behavior between the Rust and C versions.

**Traceability:** `test/c4.c` — `stmt`

### SC-6: Hello Executable Preservation

A Rust executable corresponding to `test/hello.c` runs successfully and matches the original program’s observable behavior for normal execution.

**Traceability:** `test/hello.c` — `main`

### SC-7: Exit Status Compatibility

For the tested normal execution paths of both executables, the Rust versions return process exit statuses compatible with the corresponding original C programs.

**Traceability:** `test/c4.c` — `main`; `test/hello.c` — `main`

## Constraints and Notes

- This specification is intentionally limited to behavior evidenced by the analyzed files and functions.
- Because no explicit data structure definitions were provided in the analysis input, key entities are described only at the functional-context level.
- The Rust rewrite should preserve observable executable behavior, not C-specific internal structure.