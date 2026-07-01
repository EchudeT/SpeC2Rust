# Specification: `module_src_parser.c_32` Rust Port

- **Project**: `cflow-new`
- **Source module**: `src/parser.c`
- **Module category**: `module_cluster`
- **Rust branch**: `095-module_src_parser.c_32-rust-port`
- **Generation date**: `2026-06-17`

## 1. Overview

This module segment provides parser-side support for recording symbol references and for resetting parser state related to the current static caller context.

The Rust rewrite must preserve the observed functional boundaries of the C module segment identified by:

- `reference(char *name, int line)`
- `reset_static_caller(void)`

The rewrite is expected to integrate into the parser subsystem so that:

- a named source-level reference can be recorded with an associated line number, and
- parser state that tracks the current static caller can be cleared on demand.

This specification is limited to the behavior evidenced by the identified functions and the nearby parser state structures in `src/parser.c`. It does not define new public APIs or capabilities beyond those boundaries.

## 2. Feature Specification

### 2.1 Reference Recording

The module must support parser-driven recording of a reference identified by:

- a symbol or name string, and
- a source line number.

Behaviorally, this means the Rust version must accept parser input representing a reference occurrence and propagate that information into the parser’s maintained reference/call relationship state in the same role served by `reference` in the C module.

The specification does not require a specific storage strategy, but the resulting behavior must preserve:

- association of the provided name with the provided line number,
- use within parser-managed state rather than as a standalone utility,
- compatibility with repeated calls during parsing.

### 2.2 Static Caller State Reset

The module must support explicit clearing of the parser’s static caller context.

Behaviorally, this means the Rust version must provide the equivalent effect of `reset_static_caller`:

- after reset, any parser state representing the current static caller context is considered cleared,
- subsequent parsing actions operate from that cleared state until a new caller context is established elsewhere in the parser.

This reset capability must be usable independently of whether references were recorded previously.

### 2.3 Parser State Participation

The identified functionality operates within parser-owned state. The presence of internal parser structures, including parser-local anonymous structs and balance-related state, indicates this module is not an isolated service but part of a broader parser state machine.

The Rust rewrite must therefore preserve these functions as stateful parser operations, not as stateless text helpers.

## 3. User Scenarios & Testing

## 3.1 Scenario: Record a reference while parsing source input

**Given** the parser has recognized a reference occurrence in source text
**When** the parser invokes the module with a reference name and source line number
**Then** the module records that reference in parser-managed state for later use by the surrounding parser/call-flow logic.

### Acceptance checks

- A non-empty name can be submitted with a valid line number.
- Multiple reference submissions during one parse session are accepted.
- Each submission preserves the supplied line number association.

## 3.2 Scenario: Reset static caller context between parsing contexts

**Given** parser state currently includes a static caller context
**When** the parser invokes the reset operation
**Then** the static caller context is cleared.

### Acceptance checks

- Reset can be invoked without additional parameters.
- After reset, parser behavior reflects absence of a current static caller until another one is established by surrounding parser logic.
- Repeated reset calls do not require a prior successful reference recording.

## 3.3 Scenario: Continue parsing after a reset

**Given** the static caller context has been reset
**When** the parser later records a new reference
**Then** the new reference is handled under the post-reset parser state rather than inheriting stale caller context.

### Acceptance checks

- A reset does not disable future reference recording.
- State observed after reset is distinct from state before reset in the aspect of current static caller context.

## 3.4 Scenario: Handle repeated reference recording during one parse flow

**Given** source parsing encounters several references on different lines
**When** the parser invokes the reference-recording operation for each occurrence
**Then** the module consistently accepts each occurrence and associates each one with its provided line number.

### Acceptance checks

- Sequential invocations are supported.
- Different names and different line numbers are preserved per invocation.
- No reset is required between normal successive reference recordings.

## 4. Requirements

### 4.1 Functional Requirements

#### FR-1: Record named references with line information
The Rust module shall implement the behavior of `reference` from `src/parser.c` by accepting a reference name and line number and recording that reference within parser-managed state.

**Traceability**: `src/parser.c`, `reference` at lines 1484-1496.

#### FR-2: Support repeated reference recording during parsing
The Rust module shall allow the reference-recording behavior to be invoked multiple times within a parser run without requiring reinitialization between calls.

**Traceability**: `src/parser.c`, `reference` at lines 1484-1496.

#### FR-3: Reset static caller context
The Rust module shall implement the behavior of `reset_static_caller` from `src/parser.c` by clearing the parser’s current static caller state.

**Traceability**: `src/parser.c`, `reset_static_caller` at lines 1501-1507.

#### FR-4: Permit reset independent of reference recording order
The Rust module shall support invocation of static caller reset regardless of whether references have already been recorded in the current parse flow.

**Traceability**: `src/parser.c`, `reset_static_caller` at lines 1501-1507; parser-state role implied by `reference` at lines 1484-1496.

#### FR-5: Preserve parser-state semantics
The Rust module shall preserve the stateful parser role of these operations so they act on maintained parser context rather than as detached pure string-processing helpers.

**Traceability**: `src/parser.c`; `reference` at lines 1484-1496; `reset_static_caller` at lines 1501-1507; parser-local state structures including `struct balance_state` and other anonymous parser structs.

### 4.2 Key Entities

#### Entity: Reference occurrence
A parser-recognized occurrence identified by a name and a source line number.

**Relationship**:
- Consumed by the reference-recording operation.
- Associated with parser-maintained reference/caller state.

**Traceability**: `reference(char *name, int line)` in `src/parser.c`.

#### Entity: Static caller context
Parser-maintained state representing the current static caller context.

**Relationship**:
- Cleared by the reset operation.
- Influences how subsequent parser actions are interpreted by surrounding parser logic.

**Traceability**: `reset_static_caller(void)` in `src/parser.c`.

#### Entity: Parser internal state
The internal mutable state used by parser operations, evidenced by multiple anonymous structs and `balance_state` structures in `src/parser.c`.

**Relationship**:
- Owns or mediates reference occurrence handling.
- Owns the static caller context that can be reset.
- Provides the stateful environment in which the two identified functions operate.

**Traceability**: anonymous structs in `src/parser.c` including lines 21-27, 57-61, and `balance_state` occurrences around lines 475-516.

## 5. Success Criteria

### 5.1 Functional Equivalence Criteria

- The Rust port provides behavior equivalent to calling `reference(name, line)` for parser-recognized references, including use of both the supplied name and supplied line number.
- The Rust port provides behavior equivalent to calling `reset_static_caller()` to clear current static caller state.
- After a reset, subsequent reference handling does not rely on stale static caller context from before the reset.

### 5.2 Testable Criteria

- A test that records at least two references with different names and line numbers passes only if both invocations are accepted and preserve their respective line associations.
- A test that establishes parser state, invokes static caller reset, and inspects resulting parser behavior passes only if the static caller context is cleared.
- A test that invokes reset more than once passes only if repeated resets remain valid and do not require intervening setup.
- A test that records a reference after reset passes only if the operation succeeds under cleared caller context.

### 5.3 Scope Conformance Criteria

- The Rust rewrite stays within the evidenced functionality of reference recording and static caller reset from `src/parser.c`.
- The Rust rewrite does not require introduction of unrelated capabilities not evidenced by the identified functions or parser state structures.