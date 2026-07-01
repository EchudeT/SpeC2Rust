# spec.md

## Title

Rust Port Functional Specification: `main_root`

## Status

Draft

## Summary

`main_root` is the top-level executable module for the `c4` project. From the analyzed sources, it provides the command-line entry flow and the core front-end execution path used to process source input, including token advancement, expression parsing, statement parsing, and program startup. The module also includes a minimal alternate executable entry in `hello.c` that prints a simple greeting.

The Rust rewrite on branch `001-main_root-rust-port` must preserve the observed executable behavior and functional boundaries present in:

- `c4.c`: `next`, `expr`, `stmt`, `main`
- `hello.c`: `main`

This specification covers only functionality evidenced by those files and functions.

## Scope

### In Scope

- Top-level executable startup behavior for the `c4` main program.
- Processing of command-line invocation for the main compiler/interpreter executable path.
- Source-token advancement behavior used by parsing flow.
- Expression parsing behavior driven by precedence level.
- Statement parsing behavior.
- Support for the separate minimal `hello` executable behavior present in `hello.c`.

### Out of Scope

- New language features not evidenced by the analyzed module.
- New public APIs beyond what is needed to preserve executable behavior.
- Concurrency, persistence, networking, FFI, or recovery capabilities.
- Refactoring requirements beyond preserving module behavior.

## Feature Specification

### Feature 1: Main executable startup and control flow

The module shall provide the top-level entry behavior for the primary `c4` executable.

Observed functional role from `c4.c`:

- Accept process arguments through `main(int argc, char **argv)`.
- Initialize and run the main program flow.
- Drive source processing through the parser-related functions in this module.
- Terminate with an integer process exit status.

The Rust version must implement equivalent startup behavior such that invoking the ported executable causes the same top-level processing path to occur for supported inputs.

### Feature 2: Token advancement for parsing

The module shall provide token advancement behavior through `next`.

Observed functional role from `c4.c`:

- Advance from the current lexical position to the next token.
- Supply the parser with current-token state needed by expression and statement parsing.

The Rust version must preserve this functional boundary: parsing operations must depend on a token-advancement step that recognizes and moves through source input in the same effective order as the C module.

### Feature 3: Expression parsing

The module shall parse expressions through `expr(int lev)`.

Observed functional role from `c4.c`:

- Parse expressions according to a precedence or level argument.
- Participate in the main program’s handling of source constructs.

The Rust version must implement expression parsing that accepts a parsing level and produces the same parsing outcomes, accepted/rejected forms, and parser progress expected by the original module for supported source inputs.

### Feature 4: Statement parsing

The module shall parse statements through `stmt`.

Observed functional role from `c4.c`:

- Parse statement-level language constructs.
- Cooperate with expression parsing and token advancement during source processing.

The Rust version must preserve statement parsing behavior for the constructs handled by the original module.

### Feature 5: Alternate minimal hello executable behavior

The module set also includes a separate simple executable entry in `hello.c`.

Observed functional role from `hello.c`:

- Provide a `main()` entry point.
- Print a simple greeting and exit.

The Rust rewrite must preserve this distinct minimal behavior if the port retains an equivalent binary target for `hello.c`.

## User Scenarios & Testing

### Scenario 1: Running the main `c4` executable with source input

A user invokes the main executable with command-line arguments expected by the original program. The program starts, processes the invocation, and enters its normal source-processing flow.

#### Expected result

- The executable starts without requiring any new arguments or configuration not evidenced in the C module.
- The source-processing path is entered.
- Parsing proceeds through token advancement, expression parsing, and statement parsing as needed.
- The process ends with an integer exit status.

#### Test guidance

- Run the Rust executable with the same invocation patterns used for the C version.
- Verify that startup succeeds or fails in the same situations as the original.
- Compare observable output and exit status for representative valid and invalid inputs.

### Scenario 2: Parsing expressions encountered in source code

A user provides source text containing expressions supported by the original `c4` program. During compilation or interpretation, the parser advances through tokens and parses expressions at the required precedence level.

#### Expected result

- Expression-containing input accepted by the C module is also accepted by the Rust port.
- Expression-containing input rejected by the C module is also rejected by the Rust port.
- Parsing consumes input in a way consistent with the original module’s behavior.

#### Test guidance

- Use source samples with simple and nested expressions.
- Include precedence-sensitive cases.
- Compare acceptance/rejection behavior and observable diagnostics where available.

### Scenario 3: Parsing statements encountered in source code

A user provides source text containing statement forms supported by the original module. The parser processes those statements as part of the main program flow.

#### Expected result

- Supported statement forms are parsed successfully.
- Unsupported or malformed statements are handled consistently with the original behavior.
- Statement parsing integrates correctly with expression parsing and token advancement.

#### Test guidance

- Use representative statement samples taken from original project usage.
- Include malformed statement inputs to compare failure behavior.
- Verify that statement boundaries are consumed consistently.

### Scenario 4: Running the minimal hello program

A user runs the executable corresponding to `hello.c`.

#### Expected result

- The program prints the greeting produced by the original C version.
- The program exits successfully.

#### Test guidance

- Execute the Rust equivalent of the hello target.
- Compare standard output and exit code to the C version.

## Requirements

### Functional Requirements

#### FR-1: Provide primary executable entry behavior

Traceability: `c4.c` → `main`

The Rust module shall provide the primary executable entry path corresponding to the `c4.c` program and accept command-line arguments for invocation.

#### FR-2: Execute the module’s top-level processing flow

Traceability: `c4.c` → `main`

When invoked, the Rust primary executable shall perform the same top-level processing flow as the original module, including entering source-processing/parsing behavior required by the input.

#### FR-3: Advance through source tokens for parser consumption

Traceability: `c4.c` → `next`

The Rust port shall implement token advancement behavior that moves through source input and updates parser-visible current-token state in support of parsing.

#### FR-4: Parse expressions by precedence level

Traceability: `c4.c` → `expr`

The Rust port shall implement expression parsing that is parameterized by a parsing level and preserves the original module’s accepted and rejected expression forms for supported inputs.

#### FR-5: Parse statements in source input

Traceability: `c4.c` → `stmt`

The Rust port shall implement statement parsing consistent with the original module’s handling of statement constructs and parser progression.

#### FR-6: Integrate token advancement, expression parsing, and statement parsing in the main program flow

Traceability: `c4.c` → `main`, `next`, `expr`, `stmt`

The Rust port shall preserve the functional relationship whereby the main executable path uses token advancement and parsing operations together to process source input.

#### FR-7: Return an integer process status from the primary executable

Traceability: `c4.c` → `main`

The Rust primary executable shall terminate with a process exit status corresponding to the original program’s observable success/failure behavior.

#### FR-8: Provide the minimal hello executable behavior

Traceability: `hello.c` → `main`

If the Rust port includes the target corresponding to `hello.c`, it shall provide a standalone executable entry that prints the same greeting as the original and exits.

### Key Entities

The analysis input does not enumerate named struct or type definitions for this module. The key entities below are therefore defined only at the behavioral level evidenced by the functions.

#### Entity 1: Source input stream

Traceability: `c4.c` → `next`, `expr`, `stmt`, `main`

The source input stream is the program text being processed by the main executable. It is consumed incrementally during token advancement and parsing.

Relationship:
- Read by the top-level executable flow.
- Advanced by tokenization behavior.
- Interpreted by expression and statement parsing.

#### Entity 2: Current token state

Traceability: `c4.c` → `next`, `expr`, `stmt`

The current token state represents the parser-visible token produced by the most recent advancement step.

Relationship:
- Produced or updated by `next`.
- Read by `expr` and `stmt`.
- Determines parser decisions as source processing proceeds.

#### Entity 3: Expression parse level

Traceability: `c4.c` → `expr(int lev)`

The expression parse level is the precedence/control input used to govern expression parsing behavior.

Relationship:
- Supplied to expression parsing.
- Influences which expression forms are consumed at a given point.

#### Entity 4: Statement parse context

Traceability: `c4.c` → `stmt`, `expr`, `next`

The statement parse context is the parser state under which a statement is recognized and consumed.

Relationship:
- Depends on current token state.
- May invoke expression parsing.
- Progresses through token advancement.

#### Entity 5: Process invocation arguments

Traceability: `c4.c` → `main(int argc, char **argv)`

The process invocation arguments are the command-line inputs provided to the primary executable.

Relationship:
- Consumed by the top-level executable entry.
- Influence startup and source-processing behavior.

## Success Criteria

### SC-1: Primary executable parity

Traceability: `c4.c` → `main`

For a defined regression set of command-line invocations used with the C program, the Rust primary executable shall produce matching success/failure outcomes and process exit status classifications.

### SC-2: Parsing flow parity for representative inputs

Traceability: `c4.c` → `main`, `next`, `expr`, `stmt`

For representative valid source inputs used by the original module, the Rust port shall complete source processing without introducing failures not present in the C version.

### SC-3: Rejection parity for malformed inputs

Traceability: `c4.c` → `expr`, `stmt`, `main`

For representative malformed or unsupported source inputs, the Rust port shall reject or fail in the same usage situations as the C module.

### SC-4: Expression handling parity

Traceability: `c4.c` → `expr`

For a regression suite containing precedence-sensitive expressions supported by the original module, the Rust port shall match the C module’s acceptance/rejection behavior.

### SC-5: Statement handling parity

Traceability: `c4.c` → `stmt`

For a regression suite containing representative statement forms handled by the original module, the Rust port shall match the C module’s acceptance/rejection behavior.

### SC-6: Hello target parity

Traceability: `hello.c` → `main`

If the hello target is included in the Rust port, running it shall produce the same greeting text on standard output and a successful exit status.

## Constraints and Notes

- No additional externally visible capabilities shall be introduced unless required to preserve the behavior evidenced by `c4.c` or `hello.c`.
- Because the provided analysis does not include explicit type definitions, this specification intentionally defines entities at the behavioral level rather than inventing concrete Rust data types.
- The Rust rewrite may reorganize internals, but observable executable behavior covered by this specification must remain equivalent.