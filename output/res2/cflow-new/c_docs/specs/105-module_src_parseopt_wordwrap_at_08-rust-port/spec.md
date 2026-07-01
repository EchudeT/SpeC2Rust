# Functional Specification: `module_src_parseopt_wordwrap_at_08`

## 1. Overview

This module provides position-state queries for a word-wrapping output context used by the option parsing/help formatting area of the project. The analyzed module file is `src/parseopt/wordwrap.c`, and the identified public behavior in scope is exposed through:

- `wordwrap_at_bol`
- `wordwrap_at_eol`

The Rust rewrite must preserve the observable behavior of these position-query operations against the module’s word-wrapping file/context state.

This specification covers only the functionality evidenced by the analyzed module contents: determining whether the current output position is at the beginning of a line or at the end of a line within the maintained wrapping state.

---

## 2. Feature Specification

### 2.1 Feature Summary

The module maintains a word-wrapping state object representing the current formatted-output position. It provides query functions that allow callers to ask whether the current position is:

- at the beginning of a line, or
- at the end of a line.

These queries are part of the formatting control flow for wrapped text emission.

### 2.2 In-Scope Functionality

The Rust version must implement behavior equivalent to the following module capabilities:

- Accept a word-wrapping context/state object corresponding to the C module’s `WORDWRAP_FILE` / `struct wordwrap_file`.
- Inspect the current tracked position information contained in that state.
- Report whether the current position is at line start.
- Report whether the current position is at line end.
- Return integer-style boolean results compatible with the original module behavior:
  - nonzero when the queried condition is true,
  - zero when the queried condition is false.

### 2.3 Out of Scope

The following are not required by this specification unless they are needed internally to support the evidenced behavior:

- defining new formatting features,
- adding new public APIs beyond the evidenced query behavior,
- thread-safety guarantees,
- serialization or persistence,
- FFI interfaces,
- recovery/error-reporting mechanisms not evidenced by the analyzed module entry points.

---

## 3. User Scenarios & Testing

### 3.1 Scenario: Check whether wrapped output is at line start

A caller responsible for formatting usage/help text has an active word-wrapping context and needs to decide whether indentation, prefix emission, or spacing rules for a new line should apply.

**Expected behavior:**
- When the current tracked position corresponds to the beginning of a line, the line-start query returns true/nonzero.
- Otherwise it returns false/zero.

**Test coverage:**
- Construct or prepare a wrapping state whose current position is line-start.
- Verify `at_bol` behavior returns true/nonzero.
- Advance or alter the state so the current position is no longer line-start.
- Verify `at_bol` behavior returns false/zero.

### 3.2 Scenario: Check whether wrapped output is at line end

A caller needs to know whether additional text can be placed on the current line or whether the current tracked position has reached the line boundary according to the wrapping state.

**Expected behavior:**
- When the current tracked position corresponds to the end of a line, the line-end query returns true/nonzero.
- Otherwise it returns false/zero.

**Test coverage:**
- Prepare a wrapping state whose current position is exactly at line-end according to the module’s tracked state.
- Verify `at_eol` behavior returns true/nonzero.
- Prepare a state that is still within line bounds.
- Verify `at_eol` behavior returns false/zero.

### 3.3 Scenario: Distinguish beginning-of-line from end-of-line state

A caller uses both queries to guide formatting decisions and must be able to distinguish the two conditions based on the maintained position state.

**Expected behavior:**
- The two queries reflect the current tracked state consistently.
- Each query depends on the same underlying wrapping context and position tracking.

**Test coverage:**
- Validate query results across multiple prepared states:
  - line-start state,
  - interior-of-line state,
  - line-end state.
- Confirm the results change appropriately with the state.

---

## 4. Requirements

### 4.1 Functional Requirements

#### FR-1: Beginning-of-line query
The Rust module shall provide behavior equivalent to `wordwrap_at_bol(WORDWRAP_FILE wf)` from `src/parseopt/wordwrap.c`, determining from the wrapping context whether the current position is at the beginning of a line.

**Traceability:** `wordwrap_at_bol` in `src/parseopt/wordwrap.c:208-212`; `struct wordwrap_file`; `struct position`.

#### FR-2: End-of-line query
The Rust module shall provide behavior equivalent to `wordwrap_at_eol(WORDWRAP_FILE wf)` from `src/parseopt/wordwrap.c`, determining from the wrapping context whether the current position is at the end of a line.

**Traceability:** `wordwrap_at_eol` in `src/parseopt/wordwrap.c:217-221`; `struct wordwrap_file`; `struct position`.

#### FR-3: Query against maintained wrapping state
Both queries shall operate on a word-wrapping context object that contains the position information required to evaluate current line position.

**Traceability:** `struct wordwrap_file` in `src/parseopt/wordwrap.c:72-95`; referenced `struct position` fields/usages.

#### FR-4: Integer-style truth semantics
The Rust rewrite shall preserve the original observable semantics of returning a truth value corresponding to the queried condition, matching the C module’s integer boolean convention at the behavioral level.

**Traceability:** function signatures returning `int` for `wordwrap_at_bol` and `wordwrap_at_eol`.

### 4.2 Key Entities

#### Word-wrapping context
A stateful entity corresponding to `struct wordwrap_file`, representing the current formatting/wrapping state for output and holding position-tracking data used by the query functions.

**Traceability:** `struct wordwrap_file` in `src/parseopt/wordwrap.c:72-95`.

#### Position
A position-tracking entity corresponding to `struct position`, used within the wrapping context to represent current line-related placement information needed to determine beginning-of-line and end-of-line conditions.

**Traceability:** multiple `struct position` declarations/usages in `src/parseopt/wordwrap.c`, including within `struct wordwrap_file`.

#### Relationship between entities
The word-wrapping context owns or contains position state, and the two exported query behaviors evaluate that position state to report current line-boundary status.

**Traceability:** `struct wordwrap_file` containing `struct position` members/usages; `wordwrap_at_bol`; `wordwrap_at_eol`.

---

## 5. Success Criteria

### 5.1 Behavioral Equivalence

1. The Rust rewrite provides line-start query behavior equivalent to `wordwrap_at_bol` for the same wrapping-state conditions.
2. The Rust rewrite provides line-end query behavior equivalent to `wordwrap_at_eol` for the same wrapping-state conditions.

### 5.2 State-Based Correctness

3. For a prepared wrapping context representing beginning-of-line, the line-start query returns true and the line-end query reflects the corresponding maintained state.
4. For a prepared wrapping context representing interior-of-line, both queries return false unless the maintained state indicates otherwise.
5. For a prepared wrapping context representing end-of-line, the line-end query returns true and the line-start query reflects the corresponding maintained state.

### 5.3 Interface-Level Compatibility

6. The Rust module’s observable query results are directly usable by callers making boolean formatting decisions in the same way as the C module’s `int` return values.
7. No required behavior evidenced by `wordwrap_at_bol`, `wordwrap_at_eol`, `struct wordwrap_file`, or `struct position` is omitted in the rewrite.