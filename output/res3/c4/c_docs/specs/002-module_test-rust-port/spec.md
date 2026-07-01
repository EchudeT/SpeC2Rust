# spec.md

## Title

Rust Port Functional Specification: `module_test`

## Status

Draft

## Context

This module is represented by two C test programs:

- `test/c4.c`: a small self-hosting-style compiler/interpreter test program with lexical analysis, expression parsing, statement parsing, and command-line driven execution.
- `test/hello.c`: a minimal standalone program used as a simple compile/run target.

The Rust rewrite on branch `002-module_test-rust-port` must preserve the observable functional behavior evidenced by these test files. The scope is limited to behavior exercised by the module files and their main functions.

## Feature Specification

### Summary

The Rust version must provide the same module-level behavior as the C test sources:

1. Execute a minimal standalone hello-style test program.
2. Execute the `c4` test program behavior, including:
   - token advancement over source input,
   - parsing of expressions,
   - parsing of statements,
   - command-line controlled processing and execution flow.

### In Scope

The Rust version must implement functionality corresponding to the following evidenced behaviors:

- A `hello`-style program entry that completes successfully.
- A compiler/interpreter-style test entry that:
  - accepts command-line arguments,
  - reads and processes source text,
  - advances through lexical tokens,
  - parses expressions with precedence-sensitive behavior,
  - parses statements,
  - performs the same top-level program flow as the C test program.

### Out of Scope

The Rust version must not claim or require capabilities not evidenced by the provided module files, including:

- new public APIs beyond what is needed to preserve module behavior,
- concurrency guarantees,
- serialization or persistence,
- network behavior,
- recovery/checkpointing,
- FFI interfaces,
- benchmark targets.

## User Scenarios & Testing

### Scenario 1: Run the minimal test program

A user builds and runs the Rust equivalent of `test/hello.c`.

**Expected result:**
- The program starts from its entry point and exits successfully.
- No extra input handling or extended behavior is required beyond the observed minimal program behavior.

**Test evidence:** `test/hello.c:3-7`

### Scenario 2: Run the compiler/interpreter test with command-line input

A user invokes the Rust equivalent of `test/c4.c` with command-line arguments expected by the original test program.

**Expected result:**
- The program accepts arguments through its main entry.
- The program performs the same top-level processing flow as the C version.
- Invalid or missing usage patterns are handled consistently with the original program’s observable behavior.

**Test evidence:** `test/c4.c:333-528`

### Scenario 3: Process source text token by token

A user provides source input to the Rust `c4` test program.

**Expected result:**
- The program advances through the source text in token-sized steps.
- Token progression supports later parsing stages.
- Token advancement remains consistent enough for expression and statement parsing to proceed as in the C version.

**Test evidence:** `test/c4.c:48-132`

### Scenario 4: Parse expressions from source input

A user provides source code containing expressions.

**Expected result:**
- The Rust version parses expressions according to the same precedence-driven parsing flow as the C version.
- Expression parsing integrates with token advancement.
- Expression parsing supports use inside larger statement and program parsing.

**Test evidence:** `test/c4.c:134-282`

### Scenario 5: Parse statements from source input

A user provides source code containing statements.

**Expected result:**
- The Rust version parses statements using the same statement-level control flow as the C version.
- Statement parsing can invoke expression parsing when required by the source.
- Statement parsing fits into the top-level program processing performed by `main`.

**Test evidence:** `test/c4.c:284-331`

## Requirements

### Functional Requirements

#### FR-1: Minimal standalone program execution

The module shall provide a Rust equivalent of the minimal `hello` test program that can start at `main` and terminate successfully with behavior consistent with the source test.

**Traceability:** `test/hello.c:3-7`

#### FR-2: Command-line driven top-level execution

The module shall provide a Rust equivalent of the `c4` test program entry that accepts command-line arguments and performs the same top-level processing flow as the C module.

**Traceability:** `test/c4.c:333-528`

#### FR-3: Token advancement over source input

The module shall implement token advancement behavior equivalent to `next`, enabling sequential consumption of source input for later parsing stages.

**Traceability:** `test/c4.c:48-132`

#### FR-4: Expression parsing

The module shall implement expression parsing behavior equivalent to `expr(int lev)`, including precedence-sensitive parsing flow required by the input accepted by the original test program.

**Traceability:** `test/c4.c:134-282`

#### FR-5: Statement parsing

The module shall implement statement parsing behavior equivalent to `stmt()`, including coordination with expression parsing where required.

**Traceability:** `test/c4.c:284-331`

#### FR-6: Integrated parsing flow

The module shall preserve the functional relationship among token advancement, expression parsing, statement parsing, and top-level execution so that the Rust version can process source input through the same staged flow as the C test program.

**Traceability:** `test/c4.c:48-132`, `test/c4.c:134-282`, `test/c4.c:284-331`, `test/c4.c:333-528`

### Key Entities

No named standalone structs or typedef-based core data structures were provided in the analysis input. The key functional entities evidenced by the module are behavioral parser/execution stages:

1. **Source Input**
   - The text being processed by the `c4` test program.
   - Consumed by token advancement and parsing stages.

2. **Token Stream State**
   - The current lexical position and tokenized view of the source input.
   - Produced/advanced by `next`.
   - Consumed by `expr` and `stmt`.

3. **Expression Parse State**
   - The current parsing context for an expression, including precedence level.
   - Driven by `expr(int lev)`.
   - Depends on token stream state.

4. **Statement Parse State**
   - The current parsing context for a statement.
   - Driven by `stmt()`.
   - May depend on expression parsing.

5. **Program Execution Context**
   - The top-level runtime state managed by `main`, including command-line inputs and overall processing flow.
   - Coordinates source loading, parsing progression, and execution behavior.

### Entity Relationships

- Source Input feeds Token Stream State.
- Token Stream State is advanced by lexical processing and consumed by both Expression Parse State and Statement Parse State.
- Statement Parse State may invoke or depend on Expression Parse State.
- Program Execution Context orchestrates all of the above during a run of the `c4` test program.

## Success Criteria

### SC-1: Hello program parity

The Rust rewrite includes a runnable equivalent of `test/hello.c` that exits successfully under the same basic invocation conditions as the original test.

**Traceability:** `test/hello.c:3-7`

### SC-2: Top-level `c4` execution parity

The Rust rewrite includes a runnable equivalent of `test/c4.c` whose main entry accepts command-line arguments and completes the same top-level processing flow for supported inputs.

**Traceability:** `test/c4.c:333-528`

### SC-3: Token progression supports parsing

For source inputs accepted by the original `c4` test program, the Rust rewrite advances through input in a way that allows downstream expression and statement parsing to proceed successfully.

**Traceability:** `test/c4.c:48-132`, `test/c4.c:134-282`, `test/c4.c:284-331`

### SC-4: Expression parsing parity

For expressions within the supported input set of the original module, the Rust rewrite parses them with the same functional precedence behavior as `expr(int lev)`.

**Traceability:** `test/c4.c:134-282`

### SC-5: Statement parsing parity

For statements within the supported input set of the original module, the Rust rewrite parses them with the same functional behavior as `stmt()` and integrates correctly with expression parsing.

**Traceability:** `test/c4.c:284-331`, `test/c4.c:134-282`

### SC-6: End-to-end staged processing parity

Given supported source input and invocation patterns, the Rust rewrite preserves the staged behavior evidenced by the C module: top-level setup, token advancement, expression/statement parsing, and completion of program processing.

**Traceability:** `test/c4.c:48-132`, `test/c4.c:134-282`, `test/c4.c:284-331`, `test/c4.c:333-528`