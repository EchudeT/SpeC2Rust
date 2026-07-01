# spec.md

## Overview

- **Project**: `cflow-new`
- **Module**: `module_doc_ack.c_02`
- **Category**: `module_cluster`
- **Source basis**: `doc/ack.c`
- **Primary function**: `ack(a, b) -> u_long`

This module provides a single numeric computation: the two-argument Ackermann function over unsigned long integer inputs. The Rust rewrite must preserve the same externally observable behavior for supported inputs: given two unsigned integer arguments, it returns the corresponding unsigned integer result defined by the module's recursive rules.

## Feature Specification

### Summary

The module computes the Ackermann function for two non-negative integer inputs.

### Functional scope

The Rust version must implement the same mathematical behavior evidenced by the C module's `ack` function:

- When the first argument is zero, the result is the second argument plus one.
- When the first argument is non-zero and the second argument is zero, the result is the Ackermann value of the first argument minus one and one.
- When both arguments are non-zero, the result is the Ackermann value of the first argument minus one and the Ackermann value of the first argument and second argument minus one.

### Out of scope

The following are not evidenced by the module input and must not be added to the specification as required behavior:

- Additional public APIs beyond the single documented computation
- Input parsing or formatting
- Error-reporting interfaces
- Overflow protection guarantees beyond the behavior of the target integer type
- Iterative alternatives, memoization, or performance features

## User Scenarios & Testing

### Scenario 1: Base case evaluation

A caller needs the module to evaluate the function for a zero first argument.

- Input: `ack(0, b)`
- Expected behavior: returns `b + 1`

**Representative tests**
- `ack(0, 0) == 1`
- `ack(0, 5) == 6`

### Scenario 2: Second-argument zero reduction

A caller evaluates the function where the first argument is positive and the second argument is zero.

- Input: `ack(a, 0)` where `a > 0`
- Expected behavior: returns `ack(a - 1, 1)`

**Representative tests**
- `ack(1, 0) == ack(0, 1) == 2`
- `ack(2, 0) == ack(1, 1) == 3`

### Scenario 3: General recursive evaluation

A caller evaluates the function for two positive arguments.

- Input: `ack(a, b)` where `a > 0` and `b > 0`
- Expected behavior: returns `ack(a - 1, ack(a, b - 1))`

**Representative tests**
- `ack(1, 1) == 3`
- `ack(2, 2) == 7`
- `ack(3, 2) == 29`

### Scenario 4: Deterministic repeated use

A caller invokes the function multiple times with the same inputs.

- Input: repeated calls with identical arguments
- Expected behavior: each call returns the same value

**Representative test**
- Two or more invocations of `ack(2, 1)` each return `5`

## Requirements

### Functional Requirements

#### FR-1: Two-argument unsigned integer computation

The module shall expose behavior equivalent to a function taking two unsigned integer inputs and producing one unsigned integer output, corresponding to `ack(a, b)` in `doc/ack.c`.

**Traceability**: `doc/ack.c`, function `ack`

#### FR-2: Base rule

The module shall return `b + 1` when the first input `a` equals zero.

**Traceability**: `doc/ack.c`, function `ack`

#### FR-3: Zero-second-argument recursive rule

The module shall return the value equivalent to `ack(a - 1, 1)` when `a > 0` and `b == 0`.

**Traceability**: `doc/ack.c`, function `ack`

#### FR-4: General recursive rule

The module shall return the value equivalent to `ack(a - 1, ack(a, b - 1))` when `a > 0` and `b > 0`.

**Traceability**: `doc/ack.c`, function `ack`

#### FR-5: Pure functional result behavior

For the same pair of inputs, the module shall produce the same output value on repeated calls.

**Traceability**: `doc/ack.c`, function `ack`

### Key Entities

#### Entity: Ackermann input pair

- A pair of non-negative integer values represented by the function parameters `a` and `b`.
- Relationship: together they define the unique function result.

#### Entity: Ackermann result

- A non-negative integer value represented by the function return value.
- Relationship: derived solely from the input pair by the recursive rules defined above.

No additional module-specific data structures are evidenced in the source basis.

## Success Criteria

### Behavioral correctness

- The Rust module returns correct values for the base-rule tests:
  - `ack(0, 0) == 1`
  - `ack(0, 5) == 6`

### Recursive-case correctness

- The Rust module returns correct values for zero-second-argument tests:
  - `ack(1, 0) == 2`
  - `ack(2, 0) == 3`

- The Rust module returns correct values for general recursive tests:
  - `ack(1, 1) == 3`
  - `ack(2, 2) == 7`
  - `ack(3, 2) == 29`

### Rule conformance

- For tested inputs where `a > 0` and `b == 0`, results satisfy:
  - `ack(a, 0) == ack(a - 1, 1)`

- For tested inputs where `a > 0` and `b > 0`, results satisfy:
  - `ack(a, b) == ack(a - 1, ack(a, b - 1))`

### Interface preservation

- The Rust rewrite provides one module function with behavior equivalent to the C module's single documented computation from `doc/ack.c`.