# spec.md

## Title

Rust Functional Specification for `main_root`

## Overview

`main_root` is the executable-entry module for the `c4` project. Based on `c4.c` and `hello.c`, it provides:

- the top-level program entry behavior for the `c4` compiler/interpreter executable,
- lexical token advancement for source text,
- expression parsing,
- statement parsing,
- and a minimal standalone hello-world style entry program represented by `hello.c`.

The Rust rewrite on branch `001-main_root-rust-port` must preserve the observable behavior represented by these source files. The specification covers only functionality evidenced by the analyzed module files and named functions.

## Scope

This specification applies to functionality traceable to:

- `c4.c`
  - `next`
  - `expr`
  - `stmt`
  - `main`
- `hello.c`

It does not define new capabilities beyond those files.

## Feature Specification

### Feature 1: Program entry for the `c4` executable

The module must provide the main executable behavior corresponding to `c4.c:main`.

This behavior includes, as evidenced by the module analysis:

- accepting command-line invocation,
- acting as the top-level coordinator for compilation or interpretation flow,
- driving source processing through token advancement and parser routines,
- and terminating with an integer process result.

The Rust version must preserve the role of `main` as the single top-level entry path for the `c4` executable behavior represented in `c4.c`.

### Feature 2: Lexical token advancement

The module must provide token-reading behavior corresponding to `c4.c:next`.

This feature is responsible for advancing through source input and producing the next token state consumed by parsing logic. The Rust rewrite must preserve:

- sequential advancement through source text,
- parser-visible token progression,
- and compatibility with expression and statement parsing flow.

The specification does not require a separate public API beyond what is necessary to preserve module behavior.

### Feature 3: Expression parsing

The module must provide expression parsing behavior corresponding to `c4.c:expr(int lev)`.

This feature must parse expressions according to precedence or level-directed parsing behavior implied by the `lev` parameter. The Rust rewrite must preserve:

- parsing of expressions from the current token stream position,
- use of token advancement during expression parsing,
- and parser state changes needed for continued statement or top-level parsing.

### Feature 4: Statement parsing

The module must provide statement parsing behavior corresponding to `c4.c:stmt()`.

This feature must parse source statements using the current token stream and expression parsing support. The Rust rewrite must preserve:

- recognition and handling of statements from source input,
- use of expression parsing where statements require expressions,
- and advancement of parser state so subsequent input may be processed.

### Feature 5: Minimal hello-world style executable entry

The module also includes `hello.c:main`, which evidences a minimal standalone executable behavior separate from the main compiler/interpreter entry.

The Rust rewrite must preserve the existence of this module-level functionality in a form appropriate to the Rust project structure if that file is included in scope for the port. At minimum, the behavior must remain a simple executable entry that completes successfully and reflects the intent of the original minimal sample program.

## User Scenarios & Testing

### Scenario 1: Running the primary `c4` executable

A user invokes the Rust port from the command line with program arguments.

Expected support:

- the executable starts through its top-level entry,
- reads the provided invocation context,
- processes source according to the module’s parsing flow,
- and exits with an integer status.

Traceability: `c4.c:main`

### Scenario 2: Processing source text that requires repeated token advancement

A source file contains multiple lexical items and parser-relevant symbols.

Expected support:

- the parser can repeatedly advance through tokens,
- each advancement updates parser-visible current-token state,
- and parsing can continue until the input needed by the module flow has been consumed.

Traceability: `c4.c:next`, `c4.c:main`

### Scenario 3: Parsing expressions with precedence-sensitive behavior

A source input contains expressions whose interpretation depends on parsing level or precedence.

Expected support:

- expression parsing can be entered with a level parameter,
- parsing consumes the relevant portion of the token stream,
- and the parser state after completion is suitable for enclosing parsing flow.

Traceability: `c4.c:expr`

### Scenario 4: Parsing statements that contain expressions

A source input contains statements, including statements that embed expressions.

Expected support:

- statement parsing begins at the current token position,
- invokes expression parsing when needed,
- and leaves parser state advanced past the statement.

Traceability: `c4.c:stmt`, `c4.c:expr`

### Scenario 5: Running the minimal hello program

A user builds and runs the hello-style program represented by `hello.c`.

Expected support:

- the executable enters through its `main`,
- performs the minimal program behavior represented by the original file,
- and exits successfully.

Traceability: `hello.c:main`

### Testing Guidance

The Rust port must be testable through black-box scenarios derived from the above behaviors:

1. **Executable entry test**
   - Invoke the main Rust executable with command-line arguments.
   - Verify it starts and returns an integer exit status.
   - Traceability: `c4.c:main`

2. **Token progression test**
   - Provide representative source input with multiple tokens.
   - Verify parsing flow depends on repeated advancement and reaches later constructs.
   - Traceability: `c4.c:next`

3. **Expression parsing test**
   - Provide input containing expressions requiring precedence-sensitive parsing.
   - Verify expression parsing consumes the intended input segment without blocking later parsing.
   - Traceability: `c4.c:expr`

4. **Statement parsing test**
   - Provide input containing one or more statements, including statement-contained expressions.
   - Verify statements are consumed in sequence.
   - Traceability: `c4.c:stmt`

5. **Hello executable test**
   - Run the Rust equivalent of the hello sample.
   - Verify successful completion and preserved minimal behavior.
   - Traceability: `hello.c:main`

## Requirements

### Functional Requirements

#### FR-1: Top-level executable control

The Rust module shall provide top-level executable behavior corresponding to `c4.c:main`, including command-line entry and integer process termination.

Traceability: `c4.c:main`

#### FR-2: Source token advancement

The Rust module shall provide token advancement behavior corresponding to `c4.c:next` so that parsing flow can move sequentially through source input.

Traceability: `c4.c:next`

#### FR-3: Expression parsing by level

The Rust module shall provide expression parsing behavior corresponding to `c4.c:expr(int lev)` and shall accept an expression parsing level/precedence input that influences parsing behavior.

Traceability: `c4.c:expr`

#### FR-4: Statement parsing

The Rust module shall provide statement parsing behavior corresponding to `c4.c:stmt()`.

Traceability: `c4.c:stmt`

#### FR-5: Parser integration

The Rust module shall ensure statement parsing can depend on expression parsing and that parsing routines operate over a shared advancing token stream.

Traceability: `c4.c:next`, `c4.c:expr`, `c4.c:stmt`

#### FR-6: Main-flow parser orchestration

The Rust module shall ensure the primary executable flow can invoke and coordinate token advancement and parsing routines during source processing.

Traceability: `c4.c:main`, `c4.c:next`, `c4.c:expr`, `c4.c:stmt`

#### FR-7: Minimal hello-style entry behavior

The Rust project shall preserve the minimal standalone executable behavior evidenced by `hello.c:main` in an equivalent Rust form when porting that file’s functionality.

Traceability: `hello.c:main`

### Key Entities

No named struct or type definitions were provided in the analysis input for this module. The following functional entities are nevertheless evidenced by the functions and their relationships:

#### Entity 1: Source input stream

A sequential source text being read and parsed.

Relationships:

- consumed by token advancement,
- indirectly consumed by expression and statement parsing through token progression,
- initiated or managed by top-level program flow.

Traceability: `c4.c:next`, `c4.c:main`

#### Entity 2: Current token state

The parser-visible notion of the current lexical token.

Relationships:

- produced or updated by token advancement,
- consumed by expression parsing,
- consumed by statement parsing.

Traceability: `c4.c:next`, `c4.c:expr`, `c4.c:stmt`

#### Entity 3: Expression parse level

The precedence or level input used during expression parsing.

Relationships:

- supplied to expression parsing,
- affects how expressions are consumed from the token stream.

Traceability: `c4.c:expr`

#### Entity 4: Parser state

The evolving state of parsing over the current source input.

Relationships:

- advanced by `next`,
- read and updated by `expr`,
- read and updated by `stmt`,
- orchestrated by `main`.

Traceability: `c4.c:next`, `c4.c:expr`, `c4.c:stmt`, `c4.c:main`

#### Entity 5: Process invocation context

The command-line invocation information provided to the main executable.

Relationships:

- accepted by primary program entry,
- used to drive top-level execution behavior.

Traceability: `c4.c:main`

## Success Criteria

### SC-1: Primary entry preservation

A Rust executable corresponding to `c4.c` can be invoked from the command line and completes through a top-level main path with an integer exit status.

Traceability: `c4.c:main`

### SC-2: Token-driven parsing preservation

Given representative source input, the Rust port advances through multiple tokens in sequence and supports continued parsing across those token boundaries.

Traceability: `c4.c:next`

### SC-3: Expression parsing preservation

Given representative expression input, the Rust port parses expressions using level-sensitive behavior and leaves parsing positioned for subsequent processing.

Traceability: `c4.c:expr`

### SC-4: Statement parsing preservation

Given representative statement input, the Rust port parses statements and advances beyond each parsed statement.

Traceability: `c4.c:stmt`

### SC-5: Integrated parse-flow preservation

The Rust primary executable successfully coordinates token advancement, expression parsing, and statement parsing as part of its source-processing flow.

Traceability: `c4.c:main`, `c4.c:next`, `c4.c:expr`, `c4.c:stmt`

### SC-6: Hello sample preservation

A Rust equivalent of the hello sample can be executed successfully and preserves the minimal standalone behavior represented by `hello.c`.

Traceability: `hello.c:main`