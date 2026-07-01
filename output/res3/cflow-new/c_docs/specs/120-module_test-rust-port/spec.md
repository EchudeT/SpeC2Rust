# spec.md

## Overview

This specification defines the required behavior for the Rust rewrite of the `module_test` module from the `cflow-new` project, targeting branch `120-module_test-rust-port`.

The source module is composed of small test-oriented functions that exercise:
- simple arithmetic composition,
- cross-function invocation,
- recursion,
- and observable console output.

The Rust version must preserve the functional behavior evidenced by the C sources in:
- `test/multi.c`
- `test/recursion.c`
- `test/simple.c`

No persistent state, shared module state, or custom data structures are evidenced by the source module.

---

## Feature Specification

### Summary

The module provides a set of standalone computation and test-style execution routines with the following functional areas:

1. **Simple arithmetic helpers**
   - integer addition,
   - integer multiplication,
   - composition of helper functions into a higher-level computation.

2. **Multi-function call flow**
   - a helper routine that can be invoked for side-effect-free flow coverage,
   - a function that transforms an integer input,
   - a runner routine that coordinates calls and emits a numeric result to standard output.

3. **Recursive numeric computation**
   - Fibonacci calculation,
   - factorial calculation,
   - execution entry that prints a recursive result.

4. **Constant-return helper**
   - a function returning a fixed integer value independent of input.

### Rust Rewrite Scope

The Rust version must implement the same functional surface evidenced by the C module:

- preserve standalone function behavior for arithmetic and recursive routines,
- preserve composed call behavior where one function invokes others to produce a result,
- preserve observable printing behavior of the routines that print integer results followed by a newline,
- preserve the existence of side-effect-free helper behavior where evidenced.

The Rust rewrite must not introduce new module responsibilities beyond the behaviors directly supported by the source files.

---

## User Scenarios & Testing

### Scenario 1: Basic arithmetic helpers are used as building blocks

A caller uses arithmetic helper routines to combine integers.

Expected support:
- adding two integers returns their sum,
- multiplying two integers returns their product,
- a higher-level computation may call these helpers to derive a final integer result.

Verification:
- unit tests call the arithmetic functions directly with representative integer inputs,
- unit tests verify the composed computation matches the result produced by applying the helper operations in the same order as the source behavior.

Traceability:
- `add`, `mul`, `compute` in `test/simple.c`

### Scenario 2: A fixed-value helper is called independently

A caller invokes the constant-return routine and expects a fixed integer result regardless of context.

Expected support:
- the routine returns `42`.

Verification:
- a unit test calls the function and asserts the returned value is `42`.

Traceability:
- `orphan` in `test/simple.c`

### Scenario 3: A runner executes a small call chain and emits output

A caller invokes the multi-function runner to exercise internal call flow.

Expected support:
- the runner invokes the helper routine,
- the runner invokes the integer-transform routine with the evidenced input,
- the runner prints the resulting integer followed by a newline.

Verification:
- an integration-style test captures standard output from the runner,
- the captured output matches the expected decimal integer text and newline.

Traceability:
- `helper`, `twice`, `run` in `test/multi.c`

### Scenario 4: Recursive Fibonacci computation is available

A caller requests the Fibonacci value for an integer input using the recursive routine.

Expected support:
- the function returns the same integer result as the C source logic for supported inputs.

Verification:
- unit tests cover base-case inputs and at least one recursive input,
- results are checked against the C-defined behavior.

Traceability:
- `fib` in `test/recursion.c`

### Scenario 5: Recursive factorial computation is available

A caller requests the factorial value for an integer input using the recursive routine.

Expected support:
- the function returns the same integer result as the C source logic for supported inputs.

Verification:
- unit tests cover base-case inputs and at least one recursive input,
- results are checked against the C-defined behavior.

Traceability:
- `fact` in `test/recursion.c`

### Scenario 6: Recursive test entry prints a computed result

A caller invokes the recursion test entry routine and expects printed output.

Expected support:
- the routine computes a recursive result using the source-defined call path,
- the routine prints the integer result followed by a newline.

Verification:
- an integration-style test captures standard output,
- the output exactly matches the expected text produced by the C behavior.

Traceability:
- `main` in `test/recursion.c`

### Scenario 7: Simple test entry prints a computed result

A caller invokes the simple test entry routine and expects printed output.

Expected support:
- the routine computes a value through the simple arithmetic path defined in the source,
- the routine prints the integer result followed by a newline.

Verification:
- an integration-style test captures standard output,
- the output exactly matches the expected text produced by the C behavior.

Traceability:
- `main` in `test/simple.c`, with computation rooted in `compute`

---

## Requirements

### Functional Requirements

#### FR-1: Provide integer addition behavior
The module shall provide a function that accepts two integer inputs and returns their arithmetic sum.

Traceability:
- `add` in `test/simple.c`

#### FR-2: Provide integer multiplication behavior
The module shall provide a function that accepts two integer inputs and returns their arithmetic product.

Traceability:
- `mul` in `test/simple.c`

#### FR-3: Provide composed computation using helper arithmetic routines
The module shall provide a computation routine that derives its result through calls consistent with the arithmetic helper flow evidenced in the source module.

Traceability:
- `compute`, with related helpers `add` and `mul`, in `test/simple.c`

#### FR-4: Provide a fixed-value integer routine
The module shall provide a function that returns the constant integer value `42`.

Traceability:
- `orphan` in `test/simple.c`

#### FR-5: Provide a side-effect-free helper call target
The module shall provide a callable helper routine that performs no observable output or returned-value behavior.

Traceability:
- `helper` in `test/multi.c`

#### FR-6: Provide integer doubling behavior
The module shall provide a function that accepts an integer input and returns the doubled value consistent with the source behavior.

Traceability:
- `twice` in `test/multi.c`

#### FR-7: Provide a runner that executes internal call flow and prints a result
The module shall provide a routine that:
1. invokes the helper routine,
2. invokes the integer-doubling routine with the source-defined input,
3. prints the resulting integer followed by a newline.

Traceability:
- `run`, with related calls to `helper` and `twice`, in `test/multi.c`

#### FR-8: Provide recursive Fibonacci computation
The module shall provide a recursive Fibonacci function whose return values match the source logic for the same integer inputs.

Traceability:
- `fib` in `test/recursion.c`

#### FR-9: Provide recursive factorial computation
The module shall provide a recursive factorial function whose return values match the source logic for the same integer inputs.

Traceability:
- `fact` in `test/recursion.c`

#### FR-10: Provide a recursion-driven print entry
The module shall provide an entry routine that computes a recursive result using the source-defined path and prints the resulting integer followed by a newline.

Traceability:
- `main` in `test/recursion.c`, with related recursive functions in `test/recursion.c`

#### FR-11: Provide a simple-computation print entry
The module shall provide an entry routine that computes a result through the simple arithmetic path and prints the resulting integer followed by a newline.

Traceability:
- `main` in `test/simple.c`, with related computation in `compute`

### Key Entities

No custom structs, records, or container types are evidenced in this module.

The key functional entities are standalone integer-processing routines and print-entry routines:

- **Arithmetic helper functions**
  - relationship: used as building blocks for higher-level computation.
  - traceability: `add`, `mul` in `test/simple.c`

- **Composed computation function**
  - relationship: depends on arithmetic helpers to produce a final integer result.
  - traceability: `compute` in `test/simple.c`

- **Constant-return function**
  - relationship: independent utility function with no evidenced consumers in-module.
  - traceability: `orphan` in `test/simple.c`

- **Call-flow helper and runner functions**
  - relationship: `run` invokes `helper` and `twice`, then prints the produced value.
  - traceability: `helper`, `twice`, `run` in `test/multi.c`

- **Recursive computation functions**
  - relationship: recursion-based numeric calculators; used directly and by the recursion print entry.
  - traceability: `fib`, `fact`, `main` in `test/recursion.c`

---

## Success Criteria

### Behavioral Correctness

1. The Rust implementation returns the same integer results as the C module for direct calls to:
   - addition,
   - multiplication,
   - constant-return,
   - doubling,
   - Fibonacci,
   - factorial.

   Traceability:
   - `add`, `mul`, `orphan`, `twice`, `fib`, `fact`

2. The Rust implementation preserves the source-defined composed computation behavior for the higher-level arithmetic routine.

   Traceability:
   - `compute` with `add` and `mul`

3. Invoking the multi-function runner produces exactly one decimal integer line of output matching the source-defined call sequence result.

   Traceability:
   - `run` with `helper` and `twice`

4. Invoking the recursion print entry produces exactly one decimal integer line of output matching the source-defined recursive computation result.

   Traceability:
   - `main` in `test/recursion.c`

5. Invoking the simple-computation print entry produces exactly one decimal integer line of output matching the source-defined arithmetic computation result.

   Traceability:
   - `main` in `test/simple.c`

### Testability

6. Each standalone computation routine can be validated with direct unit tests using integer inputs and expected integer outputs.

   Traceability:
   - `add`, `mul`, `orphan`, `twice`, `fib`, `fact`, `compute`

7. Each print-producing routine can be validated by capturing standard output and comparing exact output text including the trailing newline.

   Traceability:
   - `run`, `main` in `test/recursion.c`, `main` in `test/simple.c`

### Scope Adherence

8. The Rust rewrite introduces no required public functionality beyond the arithmetic, recursive, call-flow, constant-return, and print behaviors evidenced by the source module.

   Traceability:
   - all listed source functions in `test/multi.c`, `test/recursion.c`, `test/simple.c`