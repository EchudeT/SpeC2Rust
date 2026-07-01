# spec.md

## Title

Rust Port Functional Specification for `module_test`

## Metadata

- Project: `cflow-new`
- Module: `module_test`
- Category: `module`
- Target branch: `120-module_test-rust-port`
- Source basis: `test/multi.c`, `test/recursion.c`, `test/simple.c`
- Generation date: 2026-06-11

## Overview

`module_test` is a small test-oriented module composed of independent arithmetic and recursion examples. Its functionality is defined by three source files that exercise:

- simple arithmetic composition,
- helper-driven computation and formatted output,
- recursive numeric computation for Fibonacci and factorial.

The Rust rewrite must preserve the observable behavior evidenced by these source files. The module does not define persistent state or custom structured data types; its behavior is function-based and numeric.

## Feature Specification

### Summary

The Rust version must implement the same functional behaviors as the C sources:

1. Basic arithmetic helpers for addition and multiplication.
2. A composed computation that combines helper functions to produce a numeric result.
3. A helper-driven execution path that invokes an internal helper, computes a doubled value, and prints the result.
4. Recursive Fibonacci calculation.
5. Recursive factorial calculation.
6. Entrypoints equivalent to the source test programs where present.

### Functional Scope

#### 1. Simple arithmetic operations

From `test/simple.c`, the module provides arithmetic helper behavior:

- `add` computes the sum of two integer inputs.
- `mul` computes the product of two integer inputs.

The Rust port must preserve these arithmetic results for equivalent integer inputs.

#### 2. Composed arithmetic computation

Also from `test/simple.c`, the module includes `compute`, which uses the arithmetic helpers to produce a derived integer result.

The Rust port must preserve:

- that `compute` is defined in terms of the helper arithmetic behavior already present in the module,
- that the result remains consistent with the source behavior for the same input.

`orphan` is also present and returns a fixed integer value. The Rust version must preserve that fixed return behavior where this function is carried over.

#### 3. Helper-driven execution and output

From `test/multi.c`, the module includes:

- `helper`, a no-argument helper routine,
- `twice`, which returns a doubled form of its integer input,
- `run`, which invokes `helper`, computes `twice(42)`, and writes the resulting integer followed by a newline using formatted output.

The Rust version must preserve the externally observable behavior of `run`:

- it calls the helper path before producing the final result,
- it computes the doubled value of `42`,
- it outputs the resulting decimal integer followed by a newline.

#### 4. Recursive Fibonacci

From `test/recursion.c`, `fib` computes Fibonacci numbers recursively.

The Rust version must preserve the recursive numeric behavior as evidenced by the source function for integer inputs in the supported domain of the original code.

#### 5. Recursive factorial

From `test/recursion.c`, `fact` computes factorial values recursively.

The Rust version must preserve the recursive numeric behavior as evidenced by the source function for integer inputs in the supported domain of the original code.

#### 6. Program-style entrypoints

The source module contains `main` functions in `test/recursion.c` and `test/simple.c`. These demonstrate executable usage of the module functionality.

The Rust rewrite must provide equivalent executable behavior for these test flows, preserving the computations and outputs that are directly evidenced by the source files. It does not need to invent a unified public API beyond what is necessary to preserve those behaviors.

## User Scenarios & Testing

### Scenario 1: Use arithmetic helpers directly

A caller uses the simple arithmetic portion of the module to add or multiply integer values.

Expected support:

- adding two integers returns their sum,
- multiplying two integers returns their product.

Testing guidance:

- verify representative integer pairs for `add`,
- verify representative integer pairs for `mul`.

Traceability: `test/simple.c` functions `add`, `mul`.

### Scenario 2: Compute a derived result through helper composition

A caller invokes the module’s composed computation rather than performing individual arithmetic steps manually.

Expected support:

- `compute` accepts the same kind of integer input as the source,
- it returns the same derived result as the original C implementation.

Testing guidance:

- validate `compute` against known inputs and expected outputs derived from the source behavior,
- confirm that helper arithmetic behavior is reflected in the final result.

Traceability: `test/simple.c` function `compute`, with supporting functions `add`, `mul`.

### Scenario 3: Retrieve the fixed orphan value

A caller invokes the standalone fixed-value function.

Expected support:

- the function returns `42`.

Testing guidance:

- a direct call returns exactly `42`.

Traceability: `test/simple.c` function `orphan`.

### Scenario 4: Run the helper-driven multi-function flow

A caller executes the multi-function test flow.

Expected support:

- the helper path is invoked,
- the value `42` is doubled,
- the decimal result is printed with a trailing newline.

Testing guidance:

- capture standard output from the Rust equivalent of `run`,
- verify the output text is exactly `84\n`.

Traceability: `test/multi.c` functions `helper`, `twice`, `run`.

### Scenario 5: Compute recursive Fibonacci values

A caller requests Fibonacci values through the recursive function.

Expected support:

- the Rust implementation returns the same results as the C source for representative valid inputs.

Testing guidance:

- verify base-case inputs from the source behavior,
- verify several small positive inputs where expected Fibonacci values are unambiguous.

Traceability: `test/recursion.c` function `fib`.

### Scenario 6: Compute recursive factorial values

A caller requests factorial values through the recursive function.

Expected support:

- the Rust implementation returns the same results as the C source for representative valid inputs.

Testing guidance:

- verify base-case inputs from the source behavior,
- verify several small positive inputs where expected factorial values are unambiguous.

Traceability: `test/recursion.c` function `fact`.

### Scenario 7: Execute source-equivalent test binaries

A user runs executable test flows corresponding to the original source files with `main` functions.

Expected support:

- the Rust port provides equivalent executable outcomes for the simple and recursion test cases,
- outputs and computed values match the original behavior evidenced in those files.

Testing guidance:

- run the Rust equivalents of the original executable flows,
- compare observed outputs to the C source behavior.

Traceability: `test/simple.c` function `main`; `test/recursion.c` function `main`.

## Requirements

### Functional Requirements

#### FR-1: Integer addition

The module shall provide integer addition behavior equivalent to `add` in `test/simple.c`, returning the sum of two integer inputs.

Traceability: `test/simple.c:add`.

#### FR-2: Integer multiplication

The module shall provide integer multiplication behavior equivalent to `mul` in `test/simple.c`, returning the product of two integer inputs.

Traceability: `test/simple.c:mul`.

#### FR-3: Fixed-value function

The module shall provide a function equivalent to `orphan` in `test/simple.c` that returns the constant integer `42`.

Traceability: `test/simple.c:orphan`.

#### FR-4: Derived arithmetic computation

The module shall provide a computation equivalent to `compute` in `test/simple.c`, producing the same integer result as the source implementation for the same input values.

Traceability: `test/simple.c:compute`.

#### FR-5: No-argument helper routine in multi-function flow

The module shall provide a no-argument helper routine equivalent in role to `helper` in `test/multi.c`, sufficient to preserve the behavior of the source execution flow.

Traceability: `test/multi.c:helper`, `test/multi.c:run`.

#### FR-6: Doubling function

The module shall provide a function equivalent to `twice` in `test/multi.c`, returning double the provided integer input.

Traceability: `test/multi.c:twice`.

#### FR-7: Multi-function execution flow and output

The module shall provide behavior equivalent to `run` in `test/multi.c` that:
- invokes the helper routine,
- computes the doubled value of `42`,
- emits the resulting decimal integer followed by a newline.

Traceability: `test/multi.c:run`.

#### FR-8: Recursive Fibonacci computation

The module shall provide recursive Fibonacci behavior equivalent to `fib` in `test/recursion.c`, returning the same values as the source implementation for supported inputs.

Traceability: `test/recursion.c:fib`.

#### FR-9: Recursive factorial computation

The module shall provide recursive factorial behavior equivalent to `fact` in `test/recursion.c`, returning the same values as the source implementation for supported inputs.

Traceability: `test/recursion.c:fact`.

#### FR-10: Executable recursion test flow

The Rust port shall preserve the executable behavior represented by `main` in `test/recursion.c`, including the same source-driven use of recursive computations and corresponding output behavior.

Traceability: `test/recursion.c:main`, with `fib` and `fact`.

#### FR-11: Executable simple test flow

The Rust port shall preserve the executable behavior represented by `main` in `test/simple.c`, including the same source-driven use of arithmetic/composed computations and corresponding output behavior.

Traceability: `test/simple.c:main`, with `add`, `mul`, `orphan`, `compute`.

### Key Entities

This module has no evidenced custom structs, unions, enums, or persistent object models.

The key entities are functional and scalar:

- **Integer inputs and outputs**: all evidenced computations operate on integers and return integer results.
- **Arithmetic helper functions**: `add`, `mul`, and `twice` transform integer inputs into integer outputs.
- **Recursive numeric functions**: `fib` and `fact` map a single integer input to a recursively computed integer result.
- **Execution flows**: `compute`, `run`, and the source `main` functions compose helper functions and, in evidenced cases, produce console output.

Relationships:

- `compute` depends on arithmetic helper behavior from `add` and `mul`.
- `run` depends on `helper` and `twice`.
- recursion test execution depends on `fib` and `fact`.
- simple test execution depends on the arithmetic and composed computation functions.

## Success Criteria

### SC-1: Arithmetic correctness

For representative test inputs, the Rust implementations corresponding to `add`, `mul`, and `twice` return the same integer results as the C source behavior.

Traceability: `test/simple.c:add`, `test/simple.c:mul`, `test/multi.c:twice`.

### SC-2: Fixed-value correctness

The Rust implementation corresponding to `orphan` returns exactly `42`.

Traceability: `test/simple.c:orphan`.

### SC-3: Composed computation correctness

For representative valid inputs, the Rust implementation corresponding to `compute` returns the same results as the C source implementation.

Traceability: `test/simple.c:compute`.

### SC-4: Multi-function output equivalence

Executing the Rust equivalent of `run` produces exactly the decimal output `84\n`.

Traceability: `test/multi.c:run`, `test/multi.c:twice`.

### SC-5: Fibonacci equivalence

For a set of representative small valid inputs covering base and non-base cases, the Rust implementation corresponding to `fib` matches the C source results exactly.

Traceability: `test/recursion.c:fib`.

### SC-6: Factorial equivalence

For a set of representative small valid inputs covering base and non-base cases, the Rust implementation corresponding to `fact` matches the C source results exactly.

Traceability: `test/recursion.c:fact`.

### SC-7: Executable flow preservation

The Rust port’s executable behaviors corresponding to the source `main` functions reproduce the same computations and printed outputs as evidenced in `test/simple.c` and `test/recursion.c`.

Traceability: `test/simple.c:main`, `test/recursion.c:main`.

### SC-8: No unsupported feature expansion

The Rust rewrite does not require capabilities that are not evidenced in the source module, such as persistence, custom data models, concurrency guarantees, serialization, or foreign-function interfaces.

Traceability: absence of such features across `test/multi.c`, `test/recursion.c`, `test/simple.c`.