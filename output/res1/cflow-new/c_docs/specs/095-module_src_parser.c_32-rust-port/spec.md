# spec.md

## Title
Rust Functional Specification for `module_src_parser.c_32`

## Status
Draft

## Scope
This specification defines the functional behavior that must be preserved when rewriting the `src/parser.c` module segment represented by `module_src_parser.c_32` into Rust on branch `095-module_src_parser.c_32-rust-port`.

The covered evidence in this module segment is limited to:

- `reference(char *name, int line)` in `src/parser.c:1484-1496`
- `reset_static_caller(void)` in `src/parser.c:1501-1507`

The Rust rewrite must preserve the observable behavior of these functions and the state interactions they imply, without adding unsupported capabilities.

## Feature Specification

### Summary
This module segment manages parser-side reference recording and parser state reset related to static caller tracking.

It provides two functional behaviors:

1. Recording a named reference associated with a source line.
2. Resetting the parser's remembered static-caller state so later parsing proceeds from a cleared caller context.

### In-Scope Behavior
The Rust version must implement:

- acceptance of a symbol or identifier name together with a line number for reference handling;
- integration of that reference into the parser's maintained state in the same logical situations as the C module;
- reset of the module's static-caller tracking state to an empty or neutral state.

### Out-of-Scope Behavior
This specification does not require any behavior not evidenced by the module input, including:

- new parsing capabilities;
- new public APIs beyond Rust equivalents needed to preserve module behavior;
- concurrency guarantees;
- persistence or serialization;
- recovery features beyond the original behavior.

## User Scenarios & Testing

### Scenario 1: Record a reference encountered during parsing
A parser workflow identifies a reference to a named symbol at a known source line and passes that information to the module.

Expected result:

- the module accepts the name and line number;
- the reference is recorded in parser-managed state consistent with the original module behavior;
- subsequent parser logic can observe that the reference was registered.

### Scenario 2: Record multiple references across parsing progress
During parsing of a source file, the parser encounters more than one referenced name on different lines and invokes reference handling repeatedly.

Expected result:

- each invocation is processed independently;
- later invocations do not erase earlier recorded reference effects unless that is part of pre-existing parser state rules;
- line association remains tied to the invocation that supplied it.

### Scenario 3: Clear static caller state before a new parse context
The parser reaches a boundary where any remembered static-caller context must not leak into future parsing decisions and invokes the reset function.

Expected result:

- the module clears the static-caller tracking state;
- later parsing proceeds as though no static caller is currently remembered.

### Scenario 4: Reset followed by new reference activity
After clearing static-caller state, parsing continues and new references are encountered.

Expected result:

- resetting static-caller state does not prevent future valid reference recording;
- new reference recording operates from the cleared caller context.

### Testing Guidance
The Rust port must be tested with at least:

- a test that invokes reference recording with a valid name and line and verifies state change;
- a test that invokes reference recording multiple times and verifies cumulative or sequential behavior matches the original module contract;
- a test that sets or simulates non-empty static-caller state, calls reset, and verifies the cleared state;
- a test that performs reset followed by reference recording and verifies both behaviors remain valid in sequence.

## Requirements

### Functional Requirements

#### FR-1 Reference registration
The module shall provide behavior equivalent to `reference(char *name, int line)` for registering a named reference together with its source line.

Traceability: `src/parser.c:1484-1496`

#### FR-2 Name-carrying reference input
The reference-registration behavior shall accept a reference name as input and use that input to identify the referenced entity in parser-managed state.

Traceability: `reference(char *name, int line)` in `src/parser.c:1484-1496`

#### FR-3 Line-aware reference input
The reference-registration behavior shall accept a line number as input and preserve the association between the recorded reference event and that line.

Traceability: `reference(char *name, int line)` in `src/parser.c:1484-1496`

#### FR-4 Repeated invocation support
The module shall support repeated reference-registration calls during a parsing run without requiring module reinitialization between calls.

Traceability: repeated callable behavior implied by `reference` in `src/parser.c:1484-1496`

#### FR-5 Static caller reset
The module shall provide behavior equivalent to `reset_static_caller(void)` that clears the current static-caller tracking state.

Traceability: `src/parser.c:1501-1507`

#### FR-6 Reset to neutral caller context
After static-caller reset completes, subsequent parser behavior that depends on static-caller tracking shall observe a neutral or empty caller state rather than any previously remembered one.

Traceability: `reset_static_caller(void)` in `src/parser.c:1501-1507`

#### FR-7 Sequential interoperability
The module shall allow `reset_static_caller` and `reference` behavior to be used in sequence within one parsing workflow.

Traceability: both functions in `src/parser.c:1484-1507`

### Key Entities

#### Entity: Reference event input
A reference event consists of:

- a reference name;
- a source line number.

Relationship:
- consumed by the module's reference-recording behavior.

Traceability: `reference(char *name, int line)` in `src/parser.c:1484-1496`

#### Entity: Static caller state
The module maintains parser state representing the currently remembered static caller context.

Relationship:
- reset by `reset_static_caller`;
- may affect surrounding parser behavior before and after reset.

Traceability: `reset_static_caller(void)` in `src/parser.c:1501-1507`

#### Entity: Parser-managed internal state
The module participates in broader parser state updates, including reference tracking and caller-context tracking.

Relationship:
- updated by `reference`;
- partially cleared by `reset_static_caller`.

Traceability: both functions in `src/parser.c:1484-1507`

#### Entity: Local parser support structures
This source file also defines internal structures used by parser logic, including anonymous internal structs and `balance_state`. For this module segment, they are implementation-internal and only relevant insofar as the Rust port may need equivalent internal state representation to preserve behavior.

Traceability:
- anonymous structs in `src/parser.c:21-27`, `53`, `57-61`
- `struct balance_state` references in `src/parser.c:475-516`

## Success Criteria

### SC-1 Functional parity for reference handling
Given a reference name and line number, the Rust module records the reference with behavior equivalent to the original `reference` function.

Traceability: `src/parser.c:1484-1496`

### SC-2 Correct line association
For tested inputs with distinct line numbers, the Rust module preserves the line association for each reference event as required by parser behavior.

Traceability: `reference(char *name, int line)` in `src/parser.c:1484-1496`

### SC-3 Repeatable reference processing
When reference handling is invoked multiple times in a single parsing workflow, the Rust module processes each invocation without unintended loss of prior effects beyond original-module behavior.

Traceability: `reference` in `src/parser.c:1484-1496`

### SC-4 Static caller state is cleared
After invoking the Rust equivalent of `reset_static_caller`, any previously set static-caller context is no longer present.

Traceability: `src/parser.c:1501-1507`

### SC-5 Reset and later reference handling coexist correctly
In tests that call static-caller reset and then perform additional reference registration, the Rust module exhibits correct reset behavior and continues accepting reference events.

Traceability: `src/parser.c:1484-1507`

### SC-6 No unsupported behavioral expansion
The Rust rewrite exposes no additional required functionality beyond the evidenced behaviors of reference recording and static-caller reset.

Traceability: module scope defined by `src/parser.c:1484-1507`