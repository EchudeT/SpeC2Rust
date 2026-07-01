# spec.md

## Title

Rust Port Functional Specification: `module_test`

## Document Control

- **Project**: `cflow-new`
- **Module**: `module_test`
- **Category**: `module`
- **Target Rust Branch**: `120-module_test-rust-port`
- **Generation Date**: `2026-06-17`

## Overview

`module_test` is a small test-oriented module composed of standalone arithmetic, recursion, and call-flow examples. Its behavior is defined entirely by free functions operating on integer inputs and by simple console output in selected entry-point-style functions.

The Rust rewrite must preserve the observable behavior evidenced by the source module:

- simple integer arithmetic through dedicated helper functions,
- composition of helper functions into a higher-level computation,
- recursive computation of Fibonacci and factorial values,
- invocation flow where one function calls another and emits a computed value to standard output.

No persistent state, custom structs, or shared module-level data structures are evidenced in the source module.

## Feature Specification

### Summary of Provided Functionality

The module provides three functional groups:

1. **Simple arithmetic helpers**
   - addition of two integers,
   - multiplication of two integers,
   - a constant-returning helper,
   - a composed computation built from helper calls.

2. **Recursive mathematical computation**
   - Fibonacci computation for integer input,
   - factorial computation for integer input,
   - a main-style function that evaluates recursion results and prints output.

3. **Basic multi-function call flow**
   - a helper function callable for side-effect-free flow participation,
   - a function that doubles an integer,
   - a runner that invokes both and prints the doubled result.

### Rust Port Scope

The Rust version must implement the same functional boundaries evidenced by the C module:

- preserve all function-level behaviors that are externally observable from the module,
- preserve integer-input/integer-output semantics for arithmetic and recursive functions,
- preserve the presence of composed execution paths where one function invokes others,
- preserve standard-output emission where the C module prints numeric results.

The Rust version must not introduce unsupported capabilities beyond those behaviors.

## User Scenarios & Testing

### Scenario 1: Simple arithmetic helper usage

A caller uses the module to perform basic arithmetic operations by invoking dedicated helper functions with integer arguments.

**Expected support in Rust**
- The addition operation returns the arithmetic sum of its two inputs.
- The multiplication operation returns the arithmetic product of its two inputs.

**Representative tests**
- Calling the addition helper with two integers returns their sum.
- Calling the multiplication helper with two integers returns their product.

### Scenario 2: Composed computation usage

A caller invokes the module’s composed computation function, which derives a result by combining internal helper functions rather than by performing a single direct operation.

**Expected support in Rust**
- The composed computation returns a deterministic integer result for a given integer input.
- The result must reflect use of the module’s arithmetic helpers and constant-returning helper, matching the C module’s behavior.

**Representative tests**
- For a selected integer input, the composed computation returns the same integer result as the C version.
- Repeated calls with the same input return the same result.

### Scenario 3: Constant helper usage

A caller invokes the constant-returning helper independently.

**Expected support in Rust**
- The function returns the constant integer value evidenced by the C source.

**Representative tests**
- The helper returns `42` on every call.

### Scenario 4: Recursive computation usage

A caller requests Fibonacci or factorial values by passing integer arguments to the corresponding recursive functions.

**Expected support in Rust**
- Fibonacci returns the same values as the C implementation for supported test inputs.
- Factorial returns the same values as the C implementation for supported test inputs.

**Representative tests**
- Base-case inputs return the same values as the original module.
- Small positive integer inputs return the same recursive results as the original module.

### Scenario 5: Multi-call execution flow with printed result

A caller invokes the runner function that exercises an internal helper, computes a doubled value from a fixed integer argument, and prints the numeric result.

**Expected support in Rust**
- The helper is invoked as part of the flow.
- The doubling function is called with the fixed input value evidenced by the C source.
- The printed output matches the C module’s emitted numeric line.

**Representative tests**
- Captured standard output from the runner matches the expected line from the original module.

### Scenario 6: Main-style recursion demonstration

A caller executes the recursion example’s entry-point-style function.

**Expected support in Rust**
- The function evaluates recursive computation and prints the same numeric output as the C module.

**Representative tests**
- Captured standard output from this entry point matches the original module’s output for the same built-in invocation path.

### Scenario 7: Main-style simple demonstration

A caller executes the simple example’s entry-point-style function.

**Expected support in Rust**
- The function triggers the module’s simple computation path and preserves the original observable output behavior.

**Representative tests**
- Captured standard output and return behavior match the original module for the same invocation path.

## Requirements

### Functional Requirements

- **FR-1 Arithmetic helpers**
  The module shall provide integer addition and integer multiplication operations that return the arithmetic result of their inputs.
  **Traceability**: `add`, `mul` in `test/simple.c`.

- **FR-2 Constant-returning helper**
  The module shall provide a function that always returns the constant integer value `42`.
  **Traceability**: `orphan` in `test/simple.c`.

- **FR-3 Composed computation**
  The module shall provide a computation function that returns an integer derived from its integer input through calls to module helper functions, preserving the original C behavior for all supported inputs.
  **Traceability**: `compute` with `add`, `mul`, `orphan` in `test/simple.c`.

- **FR-4 Doubling operation**
  The module shall provide a function that returns the doubled value of an integer input.
  **Traceability**: `twice` in `test/multi.c`.

- **FR-5 Multi-function runner flow**
  The module shall provide a runner function that invokes the local helper function, computes the doubled value of the fixed input `42`, and writes the resulting integer followed by a newline to standard output.
  **Traceability**: `helper`, `twice`, `run` in `test/multi.c`.

- **FR-6 Fibonacci recursion**
  The module shall provide a recursive Fibonacci computation whose returned values match the C implementation for the same integer inputs.
  **Traceability**: `fib` in `test/recursion.c`.

- **FR-7 Factorial recursion**
  The module shall provide a recursive factorial computation whose returned values match the C implementation for the same integer inputs.
  **Traceability**: `fact` in `test/recursion.c`.

- **FR-8 Recursion example output**
  The module shall provide an entry-point-style function for the recursion example that prints the same numeric output as the C implementation.
  **Traceability**: `main` in `test/recursion.c`, using `fib` and/or `fact` as evidenced by that file.

- **FR-9 Simple example output path**
  The module shall provide an entry-point-style function for the simple example that preserves the original module’s observable behavior, including its computation path and any emitted output.
  **Traceability**: `main`, `compute` in `test/simple.c`.

### Key Entities

This module does not define custom data structures, records, or object-like entities.

The key functional entities are stateless free functions:

- **Arithmetic functions**: perform addition, multiplication, and doubling on integer values.
- **Recursive functions**: compute Fibonacci and factorial values from integer inputs.
- **Composed functions**: derive results by invoking helper functions and, in some cases, emit output.
- **Entry-point-style functions**: exercise module behavior and expose observable output.

**Relationships**
- The composed computation depends on arithmetic helpers and the constant-returning helper.
- The runner depends on the local helper and doubling function.
- The recursion example entry point depends on recursive computation functions.
- The simple example entry point depends on the composed computation path.

## Success Criteria

- **SC-1 Arithmetic correctness**
  For a representative set of integer test inputs, Rust addition and multiplication results exactly match the C module results.
  **Traceability**: `add`, `mul` in `test/simple.c`.

- **SC-2 Constant helper correctness**
  The Rust constant-returning helper returns `42` on every invocation.
  **Traceability**: `orphan` in `test/simple.c`.

- **SC-3 Composed computation parity**
  For representative valid integer inputs, the Rust composed computation returns exactly the same results as the C module.
  **Traceability**: `compute` in `test/simple.c`.

- **SC-4 Doubling correctness**
  For representative integer inputs, the Rust doubling function returns exactly twice the input, matching the C implementation.
  **Traceability**: `twice` in `test/multi.c`.

- **SC-5 Runner output parity**
  Invoking the Rust runner produces the same single-line numeric standard output as the C runner for its built-in flow.
  **Traceability**: `run` in `test/multi.c`.

- **SC-6 Fibonacci parity**
  For base cases and small positive integer inputs used in module tests, the Rust Fibonacci function returns the same values as the C function.
  **Traceability**: `fib` in `test/recursion.c`.

- **SC-7 Factorial parity**
  For base cases and small positive integer inputs used in module tests, the Rust factorial function returns the same values as the C function.
  **Traceability**: `fact` in `test/recursion.c`.

- **SC-8 Recursion entry-point output parity**
  Captured standard output from the Rust recursion example entry point matches the C module output exactly.
  **Traceability**: `main` in `test/recursion.c`.

- **SC-9 Simple entry-point behavior parity**
  Captured observable behavior of the Rust simple example entry point matches the C module for the same execution path.
  **Traceability**: `main` in `test/simple.c`.