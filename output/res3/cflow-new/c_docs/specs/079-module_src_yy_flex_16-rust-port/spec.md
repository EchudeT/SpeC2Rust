# spec.md

## Title

Rust Functional Specification for `module_src_yy_flex_16`

## Document Control

- Project: `cflow-new`
- Module: `module_src_yy_flex_16`
- Category: `module_cluster`
- Source basis: `src/c.c`
- Rust branch: `079-module_src_yy_flex_16-rust-port`
- Generation date: 2026-06-17

## 1. Feature Specification

### 1.1 Purpose

This module provides small lexer-support string utilities used within the generated scanner logic in `src/c.c`. The analyzed functions are:

- `yy_flex_strncpy`
- `yy_flex_strlen`

Their role is limited to internal handling of C-style strings for scanner/runtime support.

### 1.2 In-Scope Functionality

The Rust rewrite must implement the functional behavior evidenced by the source:

- Determine the length of a null-terminated byte string.
- Copy a specified number of characters from one string buffer into another.
- Preserve the internal utility role of these operations as scanner-support helpers associated with lexer buffer/state handling.

The module is tied to lexer runtime structures present in the same source file, especially `yy_buffer_state`, and exists to support operations performed around scanner-managed text and buffers.

### 1.3 Out of Scope

The Rust version must not introduce capabilities not evidenced by the source analysis, including:

- New public APIs beyond what is needed to preserve this module’s existing role.
- Unicode-aware string semantics.
- Automatic bounds recovery, resizing, or ownership-management behavior not evidenced by the original C utility functions.
- Thread-safety guarantees.
- Serialization, persistence, or external interoperability features.

## 2. User Scenarios & Testing

### 2.1 Scenario: Copy scanner text into another internal buffer

A scanner-internal operation needs to copy exactly `n` characters from one character buffer into another buffer already provisioned by surrounding scanner logic.

The Rust version must support:

- Copying the first `n` characters from a source string region into a destination string region.
- Leaving overall buffer management to the surrounding scanner runtime, consistent with the original utility role.

#### Testing expectations

- Given a source containing at least `n` characters, the destination receives those `n` characters in order.
- Copying zero characters performs no data transfer.
- The operation behaves as a fixed-count copy rather than a length-discovering copy.

### 2.2 Scenario: Compute the length of scanner-managed text

A scanner-internal operation needs the length of a null-terminated string stored in scanner-managed memory.

The Rust version must support:

- Returning the number of characters before the terminating null byte.

#### Testing expectations

- An empty string returns length `0`.
- A non-empty null-terminated string returns the count of characters preceding the first null terminator.
- Embedded data after the first null terminator is not included in the reported length.

### 2.3 Scenario: Use within lexer buffer/state processing

These utilities are used in the context of scanner runtime state represented by buffer-related structures in `src/c.c`.

The Rust version must support:

- Invocation from code that manages lexer buffer state and scanner text handling.
- Behavior consistent with internal helper utilities rather than standalone end-user functionality.

#### Testing expectations

- The utility functions can be exercised from Rust scanner-support code that models buffer-oriented lexer state.
- Their outputs are suitable for use in code paths that manipulate scanner text and buffer contents.

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1: Fixed-count string copy

The module shall provide functionality equivalent to `yy_flex_strncpy` from `src/c.c:2688-2694`.

This functionality shall:

- Copy exactly the requested number of characters from a source character sequence to a destination character sequence.
- Preserve character order.
- Support `n = 0` as a no-op case.

Traceability:
- Function: `yy_flex_strncpy`
- File: `src/c.c`

#### FR-2: Null-terminated string length calculation

The module shall provide functionality equivalent to `yy_flex_strlen` from `src/c.c:2698-2705`.

This functionality shall:

- Compute the number of characters in a null-terminated string.
- Stop counting at the first null terminator.

Traceability:
- Function: `yy_flex_strlen`
- File: `src/c.c`

#### FR-3: Scanner-runtime support role

The Rust rewrite shall preserve these utilities as scanner/runtime support behavior used alongside lexer buffer management logic in the same module context.

This requirement means:

- The functionality must remain usable by code representing scanner buffer and text handling.
- The rewrite must align with the module’s internal helper purpose rather than redefining the functions as unrelated general-purpose services.

Traceability:
- Functions: `yy_flex_strncpy`, `yy_flex_strlen`
- Related types in same source: `yy_buffer_state`, `yy_trans_info`, `obstack`
- File: `src/c.c`

### 3.2 Key Entities

#### `yy_buffer_state`

A scanner buffer-state structure appearing multiple times in the analyzed source. It represents the lexer’s managed input-buffer context and is the primary surrounding state with which these helper string operations are associated.

Relationship to this module:
- The string utilities support text and buffer handling performed in the scanner runtime where `yy_buffer_state` is relevant.

Traceability:
- Type: `struct yy_buffer_state`
- File: `src/c.c`

#### `yy_trans_info`

A scanner transition-information structure used by generated lexer runtime logic.

Relationship to this module:
- It provides lexical state-machine context within the same scanner runtime where these string helpers operate, but no additional behavior beyond that relation is evidenced for this utility module.

Traceability:
- Type: `struct yy_trans_info`
- File: `src/c.c`

#### `obstack`

A storage-management structure present in the same source file.

Relationship to this module:
- It is part of the broader runtime environment in which scanner text/buffer operations exist, but no direct functional requirement beyond coexistence in module context is evidenced for the two analyzed functions.

Traceability:
- Type: `struct obstack`
- File: `src/c.c`

## 4. Success Criteria

### 4.1 Behavioral Correctness

- A Rust implementation of the fixed-count copy behavior produces destination contents identical to the first `n` source characters for representative inputs, including `n = 0`.
  Traceability: `yy_flex_strncpy`, `src/c.c:2688-2694`

- A Rust implementation of the string-length behavior returns the count of bytes before the first null terminator for representative empty and non-empty inputs.
  Traceability: `yy_flex_strlen`, `src/c.c:2698-2705`

### 4.2 Module Role Preservation

- The implemented Rust functionality can be called from scanner-support code associated with lexer buffer-state handling without requiring new capabilities not evidenced by the source module.
  Traceability: `yy_flex_strncpy`, `yy_flex_strlen`, `yy_buffer_state`, `src/c.c`

### 4.3 Scope Conformance

- The Rust rewrite limits itself to the evidenced utility behavior of counted string copy and null-terminated string length calculation.
- No additional documented module responsibilities are introduced beyond the scanner-support role evidenced in `src/c.c`.

Traceability:
- Functions: `yy_flex_strncpy`, `yy_flex_strlen`
- File: `src/c.c`