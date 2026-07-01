# spec.md

## Title

Rust Port Functional Specification: `main_root`

## Summary

`main_root` is the top-level executable module for the `c4` project. In the analyzed sources, it contains:

- the primary `c4` program entry point in `c4.c`
- token advancement, expression parsing, and statement parsing routines used by that executable
- a separate minimal `hello.c` executable entry point

The Rust rewrite on branch `001-main_root-rust-port` must preserve the observable behavior evidenced by these entry points and parser-driving routines. The module’s scope is limited to command-line execution behavior and the parsing/compilation control flow represented by `next`, `expr`, `stmt`, and `main`.

## Feature Specification

### Feature Overview

The Rust version must implement a main executable module that:

1. starts the `c4` program from command-line arguments
2. reads and processes source input through the same top-level parsing flow evidenced by the C module
3. advances lexical input into tokens
4. parses expressions with precedence-aware behavior
5. parses statements
6. provides the separate minimal `hello` program behavior represented by `hello.c`

These features are directly evidenced by:

- `c4.c:48-132` — `next`
- `c4.c:134-282` — `expr`
- `c4.c:284-331` — `stmt`
- `c4.c:333-528` — `main`
- `hello.c:3-7` — `main`

### In-Scope Behavior

#### 1. Top-level `c4` executable behavior

The module must provide the primary executable entry point that accepts command-line arguments and drives program execution. Based on the available analysis, this includes:

- accepting `argc` / `argv` style invocation inputs
- performing top-level setup required before parsing
- invoking the parsing flow rooted in token advancement, expression parsing, and statement parsing
- terminating with an integer process exit code

The Rust rewrite must preserve the same category of behavior: a command-line compiler/interpreter-style executable entry point that controls the full module lifecycle.

#### 2. Token advancement

The module must support advancing through the source stream/token stream via `next`.

Observable functional boundary:

- consume input from the current parsing position
- determine the next token used by later parsing stages
- make the next token available to expression and statement parsing

The Rust rewrite must preserve token progression as a distinct operation that the parser relies on.

#### 3. Expression parsing

The module must support expression parsing through `expr(int lev)`.

Observable functional boundary:

- parse expressions from the current token position
- honor a precedence/level parameter during parsing
- cooperate with `next` to consume tokens and progress input
- produce the parsing-side effects needed by the surrounding compile/execute flow

The Rust version must preserve expression parsing semantics at the module boundary: the parser must accept a precedence level and consume the corresponding expression from the input.

#### 4. Statement parsing

The module must support statement parsing through `stmt()`.

Observable functional boundary:

- parse statements from the current token position
- invoke expression parsing where required by statement forms
- consume the statement input and advance parser state

The Rust rewrite must preserve statement-level parsing as a top-level parser action.

#### 5. Minimal hello executable behavior

The module must also preserve the independent behavior represented by `hello.c`, which contains a second `main` entry point. The analysis evidences only a minimal standalone executable behavior. The Rust rewrite must therefore include an equivalent minimal entry-point behavior for this module’s scope, without inferring additional functionality not shown in the input.

## User Scenarios & Testing

### Scenario 1: Run the main `c4` executable from the command line

**Given** the user invokes the Rust port of the main `c4` executable with command-line arguments
**When** the program starts
**Then** it must enter the top-level execution path corresponding to `c4.c` `main`
**And** it must process input through the module’s parsing flow
**And** it must return an integer-compatible process result.

**Test approach:**

- invoke the executable with representative command-line arguments
- verify the process starts successfully
- verify it reaches parsing logic without crashing on valid invocation structure
- verify it exits with a deterministic status code for the tested input

### Scenario 2: Advance through tokens during parsing

**Given** source input is being processed
**When** parsing requests the next token
**Then** the token advancement routine must consume input from the current position
**And** make the resulting token state available to subsequent parser steps.

**Test approach:**

- provide representative source snippets
- drive parsing through repeated token advancement
- verify parser-visible token progression occurs in source order
- verify no token is skipped or repeated for the tested snippets unless required by source content

### Scenario 3: Parse an expression with precedence-sensitive control

**Given** the parser is positioned at the start of an expression
**When** expression parsing is invoked with a precedence/level argument
**Then** the parser must consume the expression from the current position
**And** respect the supplied parsing level in deciding how far to parse
**And** leave the parser positioned for the next enclosing parse step.

**Test approach:**

- provide source inputs containing nested or chained expressions
- invoke expression parsing at different levels as applicable to the parser flow
- verify the parse consumes the expected expression span
- verify the next parser action begins at the correct following token

### Scenario 4: Parse a statement

**Given** the parser is positioned at the start of a statement
**When** statement parsing is invoked
**Then** it must consume the complete statement represented at that location
**And** use expression parsing as needed for expression-bearing statements
**And** leave parser state ready for the next statement or enclosing block boundary.

**Test approach:**

- provide representative statement inputs
- invoke statement parsing from statement boundaries
- verify the parser advances beyond each complete statement
- verify expression subparsing occurs where statements require it

### Scenario 5: Run the minimal hello program entry point

**Given** the user invokes the Rust equivalent of the `hello.c` program
**When** the executable starts
**Then** it must follow the minimal standalone entry-point behavior evidenced by `hello.c`
**And** terminate successfully according to that behavior.

**Test approach:**

- build and run the minimal executable target
- verify the process executes the standalone path
- verify successful termination behavior matches the C version’s observed effect

## Requirements

### Functional Requirements

#### FR-1: Primary executable entry point

The module shall provide a primary executable entry point corresponding to `c4.c` `main` that accepts command-line invocation inputs and controls the overall program flow.

**Traceability:** `c4.c:333-528` `main(int argc, char **argv)`

#### FR-2: Token advancement capability

The module shall provide token advancement functionality corresponding to `next`, used to move from the current parse position to the next token in the input stream.

**Traceability:** `c4.c:48-132` `next()`

#### FR-3: Expression parsing capability

The module shall provide expression parsing functionality corresponding to `expr(int lev)` that parses from the current token position while honoring a precedence/level input.

**Traceability:** `c4.c:134-282` `expr(int lev)`

#### FR-4: Statement parsing capability

The module shall provide statement parsing functionality corresponding to `stmt()` that parses statements from the current token position.

**Traceability:** `c4.c:284-331` `stmt()`

#### FR-5: Parser flow integration

The module shall integrate token advancement, expression parsing, and statement parsing into the top-level execution flow so that the primary executable can process source input through these stages.

**Traceability:** combined behavior evidenced by `c4.c:48-331`, invoked from `c4.c:333-528`

#### FR-6: Integer-compatible process termination for main executable

The primary executable entry point shall terminate with an integer-compatible process result corresponding to the C `main` contract.

**Traceability:** `c4.c:333-528` `int main(int argc, char **argv)`

#### FR-7: Minimal standalone hello entry point

The module shall preserve the separate minimal executable behavior represented by `hello.c` through an equivalent Rust entry point or executable target.

**Traceability:** `hello.c:3-7` `main()`

#### FR-8: Integer-compatible process termination for hello executable

The minimal standalone hello entry point shall terminate with an integer-compatible process result corresponding to the C `main` contract.

**Traceability:** `hello.c:3-7` `int main()`

### Key Entities

The analysis does not provide named struct or type definitions for this module. The following functional entities are nevertheless directly evidenced by the functions and their relationships.

#### 1. Source input

The input text or program content being consumed by the parser-driving routines.

**Relationship to module behavior:**

- consumed incrementally by `next`
- interpreted by `expr`
- interpreted by `stmt`
- supplied under control of the primary `main`

**Traceability:** implied by `next`, `expr`, `stmt`, and `main` in `c4.c`

#### 2. Current token / parser token state

The current tokenized view of input that parser routines operate on.

**Relationship to module behavior:**

- produced/updated by `next`
- read by `expr`
- read by `stmt`

**Traceability:** functional role evidenced by the existence and sequencing of `next`, `expr`, and `stmt` in `c4.c`

#### 3. Expression parse level

The precedence or parsing level parameter supplied to `expr`.

**Relationship to module behavior:**

- controls expression parsing extent/priority
- links caller parser context to expression parsing behavior

**Traceability:** `c4.c:134-282` `expr(int lev)`

#### 4. Process invocation arguments

The command-line invocation inputs accepted by the primary executable.

**Relationship to module behavior:**

- accepted by `main`
- used to determine top-level execution behavior

**Traceability:** `c4.c:333-528` `main(int argc, char **argv)`

#### 5. Executable entry points

Two separate executable starts are evidenced:

- primary `c4` entry point
- minimal `hello` entry point

**Relationship to module behavior:**

- each initiates its own program path
- each returns an integer-compatible result

**Traceability:** `c4.c:333-528`, `hello.c:3-7`

## Success Criteria

### SC-1: Main executable parity

A Rust executable corresponding to `c4.c` can be invoked from the command line and reaches the top-level processing flow without omission of the parser-driving stages evidenced in the C module.

**Measured by:**

- successful build of the Rust executable
- successful process startup
- integration tests confirming execution reaches token and parser flow for representative inputs

**Traceability:** `c4.c:333-528`, with parser stages from `c4.c:48-331`

### SC-2: Token advancement parity

For representative source inputs, the Rust port advances through tokens in a way that supports downstream parsing, with observable parser progress after each advancement step.

**Measured by:**

- parser-state tests showing ordered token progression across representative snippets
- no premature termination or stalled parser state on those snippets

**Traceability:** `c4.c:48-132` `next()`

### SC-3: Expression parsing parity

For representative expression inputs, invoking Rust expression parsing with a precedence/level parameter consumes the intended expression span and leaves parser state positioned for the next enclosing parse step.

**Measured by:**

- targeted parsing tests covering simple and nested/chained expressions
- verification that parser position after parsing matches expected expression boundaries

**Traceability:** `c4.c:134-282` `expr(int lev)`

### SC-4: Statement parsing parity

For representative statement inputs, Rust statement parsing consumes complete statements and correctly coordinates with expression parsing where statements include expressions.

**Measured by:**

- statement-level parsing tests
- verification that parser state advances to the next statement boundary or enclosing delimiter after each parsed statement

**Traceability:** `c4.c:284-331` `stmt()`, coordinated with `c4.c:134-282`

### SC-5: Exit-code compatibility

Both Rust entry points return integer-compatible process outcomes matching the success/failure behavior of their C counterparts for tested invocation paths.

**Measured by:**

- process-level tests asserting exit status for representative runs of the main executable
- process-level tests asserting exit status for the minimal hello executable

**Traceability:** `c4.c:333-528`, `hello.c:3-7`

### SC-6: Hello executable preservation

The Rust port preserves the separate minimal executable behavior represented by `hello.c` without folding it into unrelated functionality.

**Measured by:**

- existence of a distinct runnable target or equivalent clearly separate entry path
- successful execution of that path in tests

**Traceability:** `hello.c:3-7`