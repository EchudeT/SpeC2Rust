# Functional Specification: `module_src_parseopt_wordwrap_at_08`

## 1. Overview

This module provides line-position queries for a word-wrapping output context. Its evidenced public behavior is limited to reporting whether the current output position is at the beginning of a line or at the end of a line within a `WORDWRAP_FILE` state object.

The Rust rewrite must preserve this observable behavior for the module represented by `src/parseopt/wordwrap.c`, specifically the functionality exposed through:

- `wordwrap_at_bol`
- `wordwrap_at_eol`

## 2. Feature Specification

### 2.1 Supported Functionality

The Rust version must implement the following module functionality:

- Determine whether a word-wrap context is currently positioned at the beginning of a line.
- Determine whether a word-wrap context is currently positioned at the end of a line.

These checks must operate against the module’s maintained word-wrap state, represented in C by `struct wordwrap_file`, and its tracked positions, represented in C by `struct position`.

### 2.2 Functional Boundary

The required boundary for this module is narrow:

- It is a state-query module for word-wrapping position status.
- It does not, based on the provided evidence, define broader formatting policy as part of this specification.
- It must not invent additional externally visible capabilities beyond the two evidenced position checks.

## 3. User Scenarios & Testing

### 3.1 Scenario: Check whether output is at line start

A caller has an active word-wrap file/context and needs to know whether the next output position is at the beginning of a line.

**Expected behavior**
- Calling the beginning-of-line query returns a boolean-like integer result indicating current line-start status.

**Testable outcome**
- For a context whose tracked position corresponds to line start, the function reports true/non-zero.
- For a context whose tracked position does not correspond to line start, the function reports false/zero.

**Traceability**
- `wordwrap_at_bol`
- `struct wordwrap_file`
- `struct position`

### 3.2 Scenario: Check whether output is at line end

A caller has an active word-wrap file/context and needs to know whether the current output position has reached the line end boundary.

**Expected behavior**
- Calling the end-of-line query returns a boolean-like integer result indicating current line-end status.

**Testable outcome**
- For a context whose tracked position corresponds to line end, the function reports true/non-zero.
- For a context whose tracked position does not correspond to line end, the function reports false/zero.

**Traceability**
- `wordwrap_at_eol`
- `struct wordwrap_file`
- `struct position`

### 3.3 Scenario: Distinguish line-start and line-end states independently

A caller uses both queries to make formatting decisions and requires that they reflect the current state of the same word-wrap context without contradiction caused by unrelated state.

**Expected behavior**
- Both queries inspect the same context object.
- Each result reflects its own predicate only: beginning-of-line status for one function, end-of-line status for the other.

**Testable outcome**
- Repeated calls on the same unchanged context yield stable results.
- A context may be tested for both conditions without altering the context merely by querying it.

**Traceability**
- `wordwrap_at_bol`
- `wordwrap_at_eol`
- `struct wordwrap_file`

## 4. Requirements

### 4.1 Functional Requirements

#### FR-1: Beginning-of-line status query
The module shall provide a function that accepts a word-wrap context and reports whether the current position is at the beginning of a line.

**Traceability**
- `wordwrap_at_bol` in `src/parseopt/wordwrap.c:208-212`

#### FR-2: End-of-line status query
The module shall provide a function that accepts a word-wrap context and reports whether the current position is at the end of a line.

**Traceability**
- `wordwrap_at_eol` in `src/parseopt/wordwrap.c:217-221`

#### FR-3: Query behavior is derived from maintained wrap state
The results of both status queries shall be derived from the module’s maintained word-wrap state object and its tracked positional data.

**Traceability**
- `struct wordwrap_file` in `src/parseopt/wordwrap.c:72-95`
- `struct position` occurrences referenced throughout `src/parseopt/wordwrap.c`

#### FR-4: Query operations are state-observing
The status-query functions shall act as observers of the supplied word-wrap context and must not require unrelated side effects to obtain a result.

**Traceability**
- `wordwrap_at_bol` in `src/parseopt/wordwrap.c:208-212`
- `wordwrap_at_eol` in `src/parseopt/wordwrap.c:217-221`

### 4.2 Key Entities

#### `wordwrap_file`
The primary module state object representing a word-wrapping file/context. It owns or references the positional state needed to determine line-boundary conditions.

**Traceability**
- `struct wordwrap_file` in `src/parseopt/wordwrap.c:72-95`

#### `position`
A positional state structure used by the module to represent tracked output location information relevant to line-boundary checks.

**Traceability**
- `struct position` occurrences in `src/parseopt/wordwrap.c`, including lines `38-42`, `47`, `53`, `60`, `67`, `81`, `82`, `84`, `326`, `327`, `331`, `332`, `366`

#### Relationship: `wordwrap_file` contains or relies on `position`
The word-wrap context maintains one or more positional values that are used to answer beginning-of-line and end-of-line queries.

**Traceability**
- `struct wordwrap_file` in `src/parseopt/wordwrap.c:72-95`
- embedded/referenced `struct position` entries within that definition

## 5. Success Criteria

### 5.1 Behavioral correctness

- The Rust module exposes equivalents of the two evidenced status checks: beginning-of-line and end-of-line.
- Each check returns a boolean-like result consistent with the current tracked state of the supplied word-wrap context.

**Traceability**
- `wordwrap_at_bol`
- `wordwrap_at_eol`

### 5.2 State-based consistency

- Given the same unchanged context, repeated calls to the Rust beginning-of-line query return the same result.
- Given the same unchanged context, repeated calls to the Rust end-of-line query return the same result.
- Calling either query does not by itself change the observed line-boundary status of the context.

**Traceability**
- `wordwrap_at_bol`
- `wordwrap_at_eol`
- `struct wordwrap_file`

### 5.3 Structure fidelity

- The Rust rewrite preserves the concept of a word-wrap context carrying positional state sufficient to answer both line-boundary queries.
- The mapping from context state to reported beginning-of-line/end-of-line status remains functionally equivalent to the C module.

**Traceability**
- `struct wordwrap_file`
- `struct position`
- `wordwrap_at_bol`
- `wordwrap_at_eol`

### 5.4 Scenario coverage

The Rust rewrite is acceptable only if tests demonstrate:

- a context recognized as at beginning-of-line returns true/non-zero for the BOL query,
- a context not at beginning-of-line returns false/zero for the BOL query,
- a context recognized as at end-of-line returns true/non-zero for the EOL query,
- a context not at end-of-line returns false/zero for the EOL query,
- paired queries on the same unchanged context are observational only.

**Traceability**
- `wordwrap_at_bol`
- `wordwrap_at_eol`
- `struct wordwrap_file`