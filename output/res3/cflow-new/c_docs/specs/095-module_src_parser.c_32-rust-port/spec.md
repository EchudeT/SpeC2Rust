# spec.md

## Title

Rust Functional Specification for `module_src_parser.c_32`

## Metadata

- **Project**: `cflow-new`
- **Source module**: `src/parser.c`
- **Module name**: `module_src_parser.c_32`
- **Module category**: `module_cluster`
- **Target branch**: `095-module_src_parser.c_32-rust-port`
- **Generation date**: `2026-06-17`

## Overview

This module slice provides parser-side support for recording source-code references and for resetting parser state related to the current static caller context.

The Rust rewrite must preserve the observed behavior of the C module segment represented by:

- `reference(char *name, int line)` in `src/parser.c:1484-1496`
- `reset_static_caller(void)` in `src/parser.c:1501-1507`

The functional scope evidenced by this module is limited to:

1. accepting a referenced name together with a source line number and registering that reference in parser-managed state, and
2. clearing the parser’s retained static-caller state so later parsing proceeds without the previous caller context.

No additional capabilities are in scope unless directly required to preserve these behaviors.

## Feature Specification

### Feature 1: Record a parsed reference

The module must support recording a reference identified during parsing.

A reference operation consists of:

- receiving a symbol or identifier name,
- receiving the source line associated with that occurrence,
- updating parser-managed state so the reference is retained for later parser/call-graph use.

The Rust version must preserve the behavior that this action is parser-stateful rather than a pure transformation. The operation is expected to contribute information to the parser’s current accumulated results.

### Feature 2: Reset static caller context

The module must support resetting the parser’s current static caller context.

A reset operation consists of:

- clearing the retained caller-related static state maintained by the parser,
- ensuring subsequent parsing or reference recording is not attributed to the previously retained static caller context.

The Rust version must preserve the behavioral boundary that this operation is a state reset, not a broader parser reinitialization.

## User Scenarios & Testing

### Scenario 1: Parser records a reference occurrence

**Given** the parser is processing source input and has active parser-managed state
**When** it identifies a reference name and the source line for that occurrence
**Then** the module records that reference in parser state for later use by the surrounding parser workflow.

**Test evidence target**: call the Rust equivalent of `reference` with a name and line value; verify parser state reflects one recorded reference event attributable to that input.

### Scenario 2: Multiple references are recorded across parsing

**Given** parser state is active across multiple parse events
**When** the parser reports several reference occurrences with names and line numbers
**Then** the module retains each occurrence in parser-managed state without losing earlier ones unless higher-level parser logic clears them.

**Test evidence target**: perform repeated reference-recording operations and verify all expected entries are represented in resulting parser state.

### Scenario 3: Static caller context is cleared before later parsing

**Given** the parser has retained a static caller context from prior parsing activity
**When** the reset operation is invoked
**Then** subsequent parser activity proceeds with that static caller context cleared.

**Test evidence target**: establish parser state with caller context, invoke the Rust equivalent of `reset_static_caller`, and verify later actions are no longer associated with the prior caller state.

### Scenario 4: Reset is limited to caller-context state

**Given** the parser has accumulated other state in addition to static caller context
**When** the reset operation is invoked
**Then** only the caller-related retained static state is cleared by this module operation, while unrelated parser state remains governed by its own lifecycle.

**Test evidence target**: verify reset changes caller-context-dependent behavior without implying full parser-state destruction.

## Requirements

### Functional Requirements

#### FR-1: Reference intake
The module shall accept a reference name and a source line number as input for a reference-recording operation.

**Traceability**: `reference` at `src/parser.c:1484-1496`

#### FR-2: Reference state update
The module shall update parser-managed state to retain the reported reference occurrence.

**Traceability**: `reference` at `src/parser.c:1484-1496`

#### FR-3: Repeated reference support
The module shall support multiple reference-recording operations during parser execution.

**Traceability**: parser-stateful behavior implied by `reference` at `src/parser.c:1484-1496`

#### FR-4: Static caller reset
The module shall provide an operation that clears retained static caller state.

**Traceability**: `reset_static_caller` at `src/parser.c:1501-1507`

#### FR-5: Post-reset independence from prior caller state
After static caller reset, subsequent parser operations affected by caller context shall no longer use the previously retained static caller state.

**Traceability**: `reset_static_caller` at `src/parser.c:1501-1507`

#### FR-6: Scoped reset behavior
The reset operation shall be limited to static caller state and shall not be specified as a full parser reset.

**Traceability**: `reset_static_caller` naming and bounded function scope at `src/parser.c:1501-1507`

### Key Entities

#### 1. Reference occurrence
A parser-discovered usage identified by:

- a name
- a source line number

This entity is the direct input to the module’s reference-recording behavior.

**Traceability**: `reference(char *name, int line)` at `src/parser.c:1484-1496`

#### 2. Static caller context
Parser-retained caller-related state used across parse actions until explicitly cleared.

This entity is manipulated by the reset behavior and affects attribution of later parse activity.

**Traceability**: `reset_static_caller(void)` at `src/parser.c:1501-1507`

#### 3. Parser-managed aggregate state
The broader internal parser state that stores or links recorded references and caller context.

The provided anonymous structures in `src/parser.c`, including parser-local state structures and the `balance_state` family, indicate that these operations occur within structured parser state rather than as standalone values. The Rust rewrite must preserve the functional relationships, even if the concrete representation changes.

**Traceability**:
- anonymous struct definitions in `src/parser.c:21-27`, `57-61`
- `struct obstack` mention at `src/parser.c:53`
- `struct balance_state` occurrences at `src/parser.c:475-516`
- behavior exercised by `reference` and `reset_static_caller`

#### Relationships

- A **reference occurrence** is added into **parser-managed aggregate state**.
- **Static caller context** is part of or linked to **parser-managed aggregate state**.
- Resetting **static caller context** changes how later parser events are interpreted, without itself defining a full reset of **parser-managed aggregate state**.

## Success Criteria

### SC-1: Reference recording correctness
For any invocation corresponding to `reference(name, line)`, the Rust module records a reference occurrence that preserves the provided name and line association in parser-managed state.

**Traceability**: `reference` at `src/parser.c:1484-1496`

### SC-2: Repeated reference retention
Across a sequence of reference-recording invocations, the Rust module retains all expected recorded occurrences in parser state unless cleared by separate higher-level logic.

**Traceability**: `reference` at `src/parser.c:1484-1496`

### SC-3: Static caller reset correctness
After invoking the Rust equivalent of `reset_static_caller()`, prior static caller context is no longer observed by subsequent parser operations that depend on caller attribution.

**Traceability**: `reset_static_caller` at `src/parser.c:1501-1507`

### SC-4: Reset scope preservation
Tests demonstrate that invoking static caller reset does not by itself constitute or require a full parser-state reset.

**Traceability**: bounded reset function `reset_static_caller` at `src/parser.c:1501-1507`

### SC-5: Behavioral parity at module boundary
The Rust rewrite exposes behavior sufficient for the surrounding parser workflow to:
1. register references with name and line, and
2. clear static caller context between parsing phases,

with no loss of these two evidenced capabilities.

**Traceability**:
- `reference` at `src/parser.c:1484-1496`
- `reset_static_caller` at `src/parser.c:1501-1507`

## Out of Scope

The Rust rewrite specification does not require, because they are not evidenced by the provided module slice:

- new public APIs beyond those needed to preserve the two observed behaviors,
- thread-safety guarantees,
- persistence or serialization,
- error recovery features not shown in the source evidence,
- performance targets or benchmarking criteria,
- FFI compatibility promises,
- extension of parser semantics beyond reference recording and static caller reset.